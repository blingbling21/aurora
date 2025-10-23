// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 数据管理API

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use aurora_core::Kline;
use serde::Deserialize;
use std::fs;
use tracing::{debug, info, warn};

use crate::error::{WebError, WebResult};
use crate::models::{DataFileItem, FetchDataRequest, SuccessResponse};
use crate::state::AppState;

/// 数据路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/list", get(list_data_files))
        .route("/fetch", post(fetch_historical_data))
        .route("/klines", get(get_klines_data))
        .route("/{filename}", get(get_data_file).delete(delete_data_file))
}

/// 列出所有数据文件
async fn list_data_files(State(state): State<AppState>) -> WebResult<Json<SuccessResponse<Vec<DataFileItem>>>> {
    debug!("列出数据文件");

    let mut files = Vec::new();
    let entries = fs::read_dir(&state.data_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    let dt: chrono::DateTime<chrono::Utc> = t.into();
                    Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                });

            // 尝试读取CSV文件获取记录数
            let record_count = count_csv_records(&path);

            files.push(DataFileItem {
                filename: filename.clone(),
                path: path.to_string_lossy().to_string(),
                size,
                modified: modified.unwrap_or_else(|| "未知".to_string()),
                record_count,
            });
        }
    }

    info!("找到 {} 个数据文件", files.len());
    Ok(Json(SuccessResponse::new(files)))
}

/// 获取数据文件信息
async fn get_data_file(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> WebResult<Json<SuccessResponse<DataFileItem>>> {
    debug!("获取数据文件信息: {}", filename);

    let data_path = state.data_dir.join(&filename);
    if !data_path.exists() {
        return Err(WebError::DataNotFound(format!(
            "数据文件不存在: {}",
            filename
        )));
    }

    let metadata = fs::metadata(&data_path)?;
    let size = metadata.len();
    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| {
            let dt: chrono::DateTime<chrono::Utc> = t.into();
            Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
        });

    let record_count = count_csv_records(&data_path);

    let file_info = DataFileItem {
        filename: filename.clone(),
        path: data_path.to_string_lossy().to_string(),
        size,
        modified: modified.unwrap_or_else(|| "未知".to_string()),
        record_count,
    };

    Ok(Json(SuccessResponse::new(file_info)))
}

/// 删除数据文件
async fn delete_data_file(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> WebResult<Json<SuccessResponse<String>>> {
    info!("删除数据文件: {}", filename);

    let data_path = state.data_dir.join(&filename);
    if !data_path.exists() {
        return Err(WebError::DataNotFound(format!(
            "数据文件不存在: {}",
            filename
        )));
    }

    fs::remove_file(&data_path)?;

    info!("成功删除数据文件: {}", filename);
    Ok(Json(SuccessResponse::new(format!(
        "数据文件已删除: {}",
        filename
    ))))
}

/// 统计CSV文件记录数
fn count_csv_records(path: &std::path::Path) -> Option<usize> {
    csv::Reader::from_path(path)
        .ok()
        .and_then(|mut reader| {
            let count = reader.records().count();
            Some(count)
        })
}

/// 获取历史数据
async fn fetch_historical_data(
    State(state): State<AppState>,
    Json(req): Json<FetchDataRequest>,
) -> WebResult<Json<SuccessResponse<String>>> {
    info!(
        "开始获取历史数据: {} {} {} ({} 到 {})",
        req.exchange, req.symbol, req.interval, req.start_date, req.end_date
    );

    // 生成文件名
    let filename = req.filename.unwrap_or_else(|| {
        format!(
            "{}_{}_{}_{}_to_{}.csv",
            req.exchange.to_lowercase(),
            req.symbol.to_lowercase(),
            req.interval,
            req.start_date.replace("-", ""),
            req.end_date.replace("-", "")
        )
    });

    let file_path = state.data_dir.join(&filename);

    // 检查文件是否已存在
    if file_path.exists() {
        warn!("数据文件已存在: {}", filename);
        return Err(WebError::DataError(format!(
            "数据文件已存在: {}。如需重新下载，请先删除现有文件。",
            filename
        )));
    }

    // 将日期字符串转换为时间戳（毫秒）
    let start_timestamp = parse_date_to_timestamp(&req.start_date)?;
    let end_timestamp = parse_date_to_timestamp(&req.end_date)?;

    // 验证时间范围
    if start_timestamp >= end_timestamp {
        return Err(WebError::InvalidRequest(
            "开始日期必须早于结束日期".to_string(),
        ));
    }

    // 根据交易所选择合适的下载器
    match req.exchange.to_lowercase().as_str() {
        "binance" => {
            // 使用 Binance 下载器
            use aurora_data::BinanceHistoricalDownloader;

            info!("使用 Binance 下载器获取数据");
            let downloader = BinanceHistoricalDownloader::new();

            // 执行下载
            downloader
                .download_klines(
                    &req.symbol.to_uppercase(),
                    &req.interval,
                    start_timestamp,
                    end_timestamp,
                    file_path.to_str().unwrap(),
                )
                .await
                .map_err(|e| {
                    warn!("数据下载失败: {}", e);
                    
                    // 提供更友好的错误消息
                    let error_msg = e.to_string();
                    if error_msg.contains("Invalid symbol") {
                        WebError::InvalidRequest(format!(
                            "交易对 '{}' 无效。请检查拼写，常见格式示例: BTCUSDT, ETHUSDT, BNBUSDT",
                            req.symbol
                        ))
                    } else if error_msg.contains("network") || error_msg.contains("Network") {
                        WebError::DataError(format!(
                            "网络连接失败，请检查网络连接后重试。详细错误: {}",
                            error_msg
                        ))
                    } else if error_msg.contains("timeout") || error_msg.contains("Timeout") {
                        WebError::DataError(format!(
                            "请求超时，可能是网络不稳定或数据量过大。建议缩小日期范围后重试。"
                        ))
                    } else {
                        WebError::DataError(format!("数据下载失败: {}", e))
                    }
                })?;

            info!("数据已成功下载到: {}", filename);
            
            // 读取下载的文件以获取实际的行数
            let record_count = count_csv_records(&file_path).unwrap_or(0);
            
            Ok(Json(SuccessResponse::new(format!(
                "成功下载 {} 条K线数据到文件: {}",
                record_count, filename
            ))))
        }
        "okx" | "bybit" | "coinbase" => {
            // 其他交易所暂未实现
            Err(WebError::InvalidRequest(format!(
                "交易所 {} 暂未支持，目前仅支持 Binance",
                req.exchange
            )))
        }
        _ => Err(WebError::InvalidRequest(format!(
            "不支持的交易所: {}",
            req.exchange
        ))),
    }
}

/// 将日期字符串（YYYY-MM-DD）转换为 Unix 时间戳（毫秒）
fn parse_date_to_timestamp(date_str: &str) -> WebResult<i64> {
    use chrono::NaiveDate;

    // 解析日期字符串
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| WebError::InvalidRequest(format!("日期格式错误: {}", e)))?;

    // 转换为 UTC 时间戳（开始时间为当天 00:00:00）
    let datetime = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| WebError::InvalidRequest("日期转换失败".to_string()))?;

    // 转换为时间戳（秒），然后转换为毫秒
    Ok(datetime.and_utc().timestamp() * 1000)
}

/// K线数据查询参数
#[derive(Debug, Deserialize)]
struct KlinesQuery {
    /// 数据文件路径（相对或绝对路径）
    path: String,
}

/// 获取K线数据
///
/// 从CSV文件加载K线数据并返回给前端,用于绘制交易图表
async fn get_klines_data(
    State(state): State<AppState>,
    Query(query): Query<KlinesQuery>,
) -> WebResult<Json<SuccessResponse<Vec<Kline>>>> {
    debug!("加载K线数据: {}", query.path);

    // 处理路径:如果是相对路径,则相对于 data_dir
    let data_path = if std::path::Path::new(&query.path).is_absolute() {
        std::path::PathBuf::from(&query.path)
    } else {
        state.data_dir.join(&query.path)
    };

    // 验证文件存在
    if !data_path.exists() {
        return Err(WebError::DataNotFound(format!(
            "数据文件不存在: {}",
            query.path
        )));
    }

    // 验证文件扩展名
    if data_path.extension().and_then(|s| s.to_str()) != Some("csv") {
        return Err(WebError::InvalidRequest(
            "只支持 CSV 格式的数据文件".to_string(),
        ));
    }

    // 读取CSV文件
    let mut reader = csv::Reader::from_path(&data_path)
        .map_err(|e| WebError::DataError(format!("无法读取CSV文件: {}", e)))?;

    let mut klines = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(kline) => klines.push(kline),
            Err(e) => {
                warn!("解析K线数据失败: {}", e);
                continue;
            }
        }
    }

    // 按时间戳排序
    klines.sort_by_key(|k: &Kline| k.timestamp);

    info!("成功加载 {} 条K线数据", klines.len());
    Ok(Json(SuccessResponse::new(klines)))
}
