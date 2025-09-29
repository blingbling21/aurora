//! # 历史数据获取模块
//! 
//! 这个模块提供从交易所API获取历史K线数据的功能。
//! 主要支持Binance交易所的REST API，可以获取指定时间范围和间隔的历史数据。
//! 
//! ## 功能特性
//! 
//! - **批量数据获取**: 支持获取大量历史K线数据
//! - **数据格式转换**: 自动将交易所格式转换为标准Kline格式
//! - **CSV导出**: 将获取的数据保存为CSV格式便于分析
//! - **错误处理**: 完整的网络和数据错误处理机制
//! - **请求限制**: 自动处理API请求频率限制
//! 
//! ## 使用示例
//! 
//! ```rust,no_run
//! use aurora_data::BinanceHistoricalDownloader;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let downloader = BinanceHistoricalDownloader::new();
//! 
//! // 下载BTCUSDT的1小时数据
//! let klines = downloader.fetch_klines(
//!     "BTCUSDT",
//!     "1h", 
//!     Some(1640995200000), // 开始时间
//!     Some(1641081600000), // 结束时间
//!     Some(500)            // 限制数量
//! ).await?;
//! 
//! println!("获取到 {} 条K线数据", klines.len());
//! # Ok(())
//! # }
//! ```

use aurora_core::Kline;
use crate::{DataError, DataResult, DataSourceConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tracing::{info, debug, warn, error};
use anyhow;

/// Binance API返回的原始K线数据格式
/// 
/// 这个结构体对应Binance API返回的数组格式数据。
/// 每个字段都是字符串格式，需要转换为相应的数值类型。
/// 
/// ## 字段说明
/// 
/// 0. 开盘时间 (timestamp)
/// 1. 开盘价 (string)  
/// 2. 最高价 (string)
/// 3. 最低价 (string)
/// 4. 收盘价 (string)
/// 5. 成交量 (string)
/// 6. 收盘时间 (timestamp)
/// 7. 成交额 (string)
/// 8. 成交笔数 (integer)
/// 9. 主动买入成交量 (string)
/// 10. 主动买入成交额 (string)
/// 11. 忽略字段
/// 
/// ## 注意事项
/// 
/// Binance返回的所有价格和数量字段都是字符串格式，
/// 这是为了保持精度，避免浮点数精度问题。
#[derive(Debug, Clone, Deserialize, Serialize)]
struct BinanceKline(
    /// 开盘时间（毫秒时间戳）
    i64,    
    /// 开盘价（字符串格式，保持精度）
    String, 
    /// 最高价（字符串格式，保持精度）
    String, 
    /// 最低价（字符串格式，保持精度）
    String, 
    /// 收盘价（字符串格式，保持精度）
    String, 
    /// 成交量（字符串格式，保持精度）
    String, 
    /// 收盘时间（毫秒时间戳）
    i64,    
    /// 成交额（字符串格式）
    String, 
    /// 成交笔数
    i64,    
    /// 主动买入成交量（字符串格式）
    String, 
    /// 主动买入成交额（字符串格式）
    String, 
    /// 忽略字段（通常为"0"）
    String, 
);

impl From<BinanceKline> for Kline {
    /// 将Binance原始数据转换为标准Kline格式
    /// 
    /// 这个转换过程包括：
    /// 1. 字符串价格转换为f64浮点数
    /// 2. 提取必要的字段
    /// 3. 处理解析错误（使用默认值0.0）
    /// 
    /// # 错误处理
    /// 
    /// 如果字符串解析失败，将使用0.0作为默认值。
    /// 在生产环境中，应该考虑更严格的错误处理。
    fn from(binance_kline: BinanceKline) -> Self {
        Kline {
            timestamp: binance_kline.0,
            open: binance_kline.1.parse().unwrap_or(0.0),
            high: binance_kline.2.parse().unwrap_or(0.0),
            low: binance_kline.3.parse().unwrap_or(0.0),
            close: binance_kline.4.parse().unwrap_or(0.0),
            volume: binance_kline.5.parse().unwrap_or(0.0),
        }
    }
}

/// Binance历史数据下载器
/// 
/// 这个结构体提供了从Binance交易所获取历史K线数据的功能。
/// 它封装了HTTP客户端和配置，提供了简洁的API接口。
/// 
/// ## 功能特性
/// 
/// - **自动重试**: 网络错误时自动重试
/// - **请求限制**: 自动处理API请求频率限制
/// - **批量获取**: 支持获取大量历史数据
/// - **数据验证**: 验证返回数据的有效性
/// 
/// ## 使用示例
/// 
/// ```rust
/// use aurora_data::BinanceHistoricalDownloader;
/// use aurora_data::DataSourceConfig;
/// 
/// // 使用默认配置
/// let downloader = BinanceHistoricalDownloader::new();
/// 
/// // 使用自定义配置
/// let config = DataSourceConfig::new("https://api.binance.com")
///     .with_timeout(60);
/// let downloader = BinanceHistoricalDownloader::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct BinanceHistoricalDownloader {
    /// HTTP客户端，用于发送API请求
    client: Client,
    
    /// 数据源配置，包含URL、超时等设置
    config: DataSourceConfig,
}

impl BinanceHistoricalDownloader {
    /// 使用默认配置创建新的历史数据下载器
    /// 
    /// 默认配置包括：
    /// - Binance官方API URL
    /// - 30秒请求超时
    /// - 最多3次重试
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_data::BinanceHistoricalDownloader;
    /// 
    /// let downloader = BinanceHistoricalDownloader::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(DataSourceConfig::default())
    }
    
    /// 使用指定配置创建历史数据下载器
    /// 
    /// # 参数
    /// 
    /// * `config` - 数据源配置
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_data::{BinanceHistoricalDownloader, DataSourceConfig};
    /// 
    /// let config = DataSourceConfig::new("https://api.binance.com")
    ///     .with_timeout(60);
    /// let downloader = BinanceHistoricalDownloader::with_config(config);
    /// ```
    pub fn with_config(config: DataSourceConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            config,
        }
    }
    
    /// 获取K线历史数据
    /// 
    /// 从Binance API获取指定交易对和时间范围的K线数据。
    /// 
    /// # 参数
    /// 
    /// * `symbol` - 交易对符号，如"BTCUSDT"
    /// * `interval` - 时间间隔，如"1m", "5m", "1h", "1d"
    /// * `start_time` - 开始时间（毫秒时间戳），可选
    /// * `end_time` - 结束时间（毫秒时间戳），可选  
    /// * `limit` - 返回数据条数限制（最大1000），可选
    /// 
    /// # 返回值
    /// 
    /// 成功时返回K线数据向量，失败时返回DataError
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use aurora_data::BinanceHistoricalDownloader;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let downloader = BinanceHistoricalDownloader::new();
    /// 
    /// // 获取最近500条1小时K线
    /// let klines = downloader.fetch_klines(
    ///     "BTCUSDT", 
    ///     "1h", 
    ///     None, 
    ///     None, 
    ///     Some(500)
    /// ).await?;
    /// 
    /// println!("获取到 {} 条K线数据", klines.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> DataResult<Vec<Kline>> {
        // 构建API请求URL
        let url = format!("{}/api/v3/klines", self.config.base_url);
        
        // 构建请求参数
        let mut params = vec![
            ("symbol", symbol.to_uppercase()),
            ("interval", interval.to_string()),
        ];
        
        // 添加可选参数
        if let Some(start) = start_time {
            params.push(("startTime", start.to_string()));
        }
        
        if let Some(end) = end_time {
            params.push(("endTime", end.to_string()));
        }
        
        // 设置数据条数限制，默认500条，最大1000条
        let limit_value = limit.unwrap_or(500).min(1000);
        params.push(("limit", limit_value.to_string()));

        info!("正在请求历史K线数据: {}, 间隔: {}, 限制: {}", symbol, interval, limit_value);
        debug!("API请求URL: {}", url);
        debug!("请求参数: {:?}", params);

        // 执行带重试的HTTP请求
        let response = self.make_request_with_retry(&url, &params).await?;
        
        // 解析JSON响应
        let binance_klines: Vec<BinanceKline> = response
            .json()
            .await
            .map_err(|e| DataError::ParseError(format!("JSON解析失败: {}", e)))?;

        info!("成功获取到 {} 条K线数据", binance_klines.len());

        // 转换为标准Kline格式并验证数据
        let klines: Vec<Kline> = binance_klines
            .into_iter()
            .map(Kline::from)
            .filter(|kline| self.validate_kline(kline))
            .collect();

        if klines.is_empty() {
            return Err(DataError::ApiError("未获取到有效的K线数据".to_string()));
        }

        Ok(klines)
    }
    
    /// 下载K线数据并保存到CSV文件
    /// 
    /// 这是一个便利方法，结合了数据获取和CSV保存功能。
    /// 
    /// # 参数
    /// 
    /// * `symbol` - 交易对符号
    /// * `interval` - 时间间隔
    /// * `start_time` - 开始时间（毫秒时间戳）
    /// * `end_time` - 结束时间（毫秒时间戳）
    /// * `output_path` - 输出文件路径
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use aurora_data::BinanceHistoricalDownloader;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let downloader = BinanceHistoricalDownloader::new();
    /// 
    /// downloader.download_klines(
    ///     "BTCUSDT",
    ///     "1h",
    ///     1640995200000, // 2022-01-01 00:00:00 UTC
    ///     1641081600000, // 2022-01-02 00:00:00 UTC  
    ///     "btc_1h.csv"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: i64,
        end_time: i64,
        output_path: &str,
    ) -> DataResult<()> {
        // 分批获取数据，避免单次请求数据量过大
        let mut all_klines = Vec::new();
        let mut current_start = start_time;
        
        // 计算每个间隔的毫秒数，用于分批请求
        let interval_ms = self.parse_interval_to_ms(interval)?;
        let batch_size = 1000; // Binance API最大限制
        
        while current_start < end_time {
            // 计算这一批次的结束时间
            let batch_end = std::cmp::min(
                current_start + (batch_size as i64 * interval_ms),
                end_time
            );
            
            info!("获取数据批次: {} 到 {}", current_start, batch_end);
            
            // 获取这一批次的数据
            let batch_klines = self.fetch_klines(
                symbol,
                interval,
                Some(current_start),
                Some(batch_end),
                Some(batch_size),
            ).await?;
            
            if batch_klines.is_empty() {
                info!("没有更多数据可获取");
                break;
            }
            
            // 添加到总结果中，避免重复数据
            all_klines.extend(batch_klines);
            
            // 更新下一批次的开始时间
            if let Some(last_kline) = all_klines.last() {
                current_start = last_kline.timestamp + interval_ms;
            } else {
                break;
            }
            
            // 避免请求过于频繁，添加小延迟
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            info!("当前已获取 {} 条K线数据", all_klines.len());
        }
        
        info!("总共获取到 {} 条K线数据", all_klines.len());
        
        // 保存到CSV文件
        self.save_to_csv(&all_klines, output_path).await?;
        
        info!("数据已保存到: {}", output_path);
        Ok(())
    }
    
    /// 执行带重试机制的HTTP请求
    /// 
    /// 这个方法实现了自动重试逻辑，在网络错误时会重试指定次数。
    /// 
    /// # 参数
    /// 
    /// * `url` - 请求URL
    /// * `params` - 查询参数
    /// 
    /// # 返回值
    /// 
    /// 成功时返回HTTP响应，失败时返回DataError
    async fn make_request_with_retry(
        &self,
        url: &str,
        params: &[(&str, String)],
    ) -> DataResult<reqwest::Response> {
        let mut last_error = None;
        
        for attempt in 1..=self.config.max_retries {
            match self.client.get(url).query(params).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    } else {
                        let error_msg = format!(
                            "API请求失败，状态码: {}, 响应: {:?}",
                            response.status(),
                            response.text().await.unwrap_or_default()
                        );
                        return Err(DataError::ApiError(error_msg));
                    }
                }
                Err(e) => {
                    warn!("请求失败 (尝试 {}/{}): {}", attempt, self.config.max_retries, e);
                    last_error = Some(e);
                    
                    if attempt < self.config.max_retries {
                        // 指数退避延迟
                        let delay = Duration::from_millis(1000 * (1 << (attempt - 1)));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(DataError::NetworkError(format!(
            "重试 {} 次后仍然失败: {}",
            self.config.max_retries,
            last_error.unwrap()
        )))
    }
    
    /// 验证K线数据的有效性
    /// 
    /// 检查K线数据是否符合基本的有效性要求。
    /// 
    /// # 参数
    /// 
    /// * `kline` - 待验证的K线数据
    /// 
    /// # 返回值
    /// 
    /// 如果数据有效返回true，否则返回false
    fn validate_kline(&self, kline: &Kline) -> bool {
        // 检查基本的数据有效性
        if kline.high < kline.low {
            warn!("发现无效K线数据: 最高价 {} 小于最低价 {}", kline.high, kline.low);
            return false;
        }
        
        if kline.open < 0.0 || kline.high < 0.0 || kline.low < 0.0 || kline.close < 0.0 {
            warn!("发现无效K线数据: 包含负价格");
            return false;
        }
        
        if kline.volume < 0.0 {
            warn!("发现无效K线数据: 成交量为负数");
            return false;
        }
        
        if kline.timestamp <= 0 {
            warn!("发现无效K线数据: 时间戳无效");
            return false;
        }
        
        true
    }
    
    /// 解析时间间隔字符串为毫秒数
    /// 
    /// 将Binance的时间间隔格式（如"1m", "5m", "1h", "1d"）转换为毫秒数。
    /// 
    /// # 参数
    /// 
    /// * `interval` - 时间间隔字符串
    /// 
    /// # 返回值
    /// 
    /// 成功时返回毫秒数，失败时返回DataError
    fn parse_interval_to_ms(&self, interval: &str) -> DataResult<i64> {
        match interval {
            "1m" => Ok(60 * 1000),
            "3m" => Ok(3 * 60 * 1000),
            "5m" => Ok(5 * 60 * 1000),
            "15m" => Ok(15 * 60 * 1000),
            "30m" => Ok(30 * 60 * 1000),
            "1h" => Ok(60 * 60 * 1000),
            "2h" => Ok(2 * 60 * 60 * 1000),
            "4h" => Ok(4 * 60 * 60 * 1000),
            "6h" => Ok(6 * 60 * 60 * 1000),
            "8h" => Ok(8 * 60 * 60 * 1000),
            "12h" => Ok(12 * 60 * 60 * 1000),
            "1d" => Ok(24 * 60 * 60 * 1000),
            "3d" => Ok(3 * 24 * 60 * 60 * 1000),
            "1w" => Ok(7 * 24 * 60 * 60 * 1000),
            "1M" => Ok(30 * 24 * 60 * 60 * 1000), // 近似值
            _ => Err(DataError::ConfigError(format!("不支持的时间间隔: {}", interval))),
        }
    }
    
    /// 将K线数据保存为CSV文件
    /// 
    /// 这个方法是异步的，可以处理大量数据而不阻塞。
    /// 
    /// # 参数
    /// 
    /// * `klines` - K线数据切片
    /// * `file_path` - 输出文件路径
    /// 
    /// # 返回值
    /// 
    /// 成功时返回()，失败时返回DataError
    async fn save_to_csv(&self, klines: &[Kline], file_path: &str) -> DataResult<()> {
        let mut writer = csv::Writer::from_path(file_path)
            .map_err(|e| DataError::IoError(format!("创建CSV文件失败: {}", e)))?;
        
        // 写入表头
        writer.write_record(&["timestamp", "open", "high", "low", "close", "volume"])
            .map_err(|e| DataError::IoError(format!("写入CSV表头失败: {}", e)))?;
        
        // 写入数据行
        for kline in klines {
            writer.write_record(&[
                kline.timestamp.to_string(),
                kline.open.to_string(),
                kline.high.to_string(),
                kline.low.to_string(),
                kline.close.to_string(),
                kline.volume.to_string(),
            ]).map_err(|e| DataError::IoError(format!("写入CSV数据失败: {}", e)))?;
        }
        
        writer.flush()
            .map_err(|e| DataError::IoError(format!("刷新CSV文件失败: {}", e)))?;
            
        Ok(())
    }
}

/// 从Binance下载历史K线数据的便利函数
/// 
/// 这是一个向后兼容的函数，提供了简单的接口来下载历史数据。
/// 建议在新代码中使用BinanceHistoricalDownloader结构体。
/// 
/// # 参数
/// 
/// * `symbol` - 交易对符号
/// * `interval` - 时间间隔
/// * `start_time` - 开始时间字符串（暂未实现）
/// * `end_time` - 结束时间字符串（暂未实现） 
/// * `output` - 输出文件路径
/// 
/// # 返回值
/// 
/// 成功时返回()，失败时返回anyhow::Result错误
/// 
/// # 示例
/// 
/// ```rust,no_run
/// use aurora_data::historical::download_data;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// download_data("BTCUSDT", "1h", None, None, Some("btc.csv")).await?;
/// # Ok(())
/// # }
/// ```
/// 
/// # 注意
/// 
/// 这个函数目前还不支持时间范围参数，将在后续版本中实现。
pub async fn download_data(
    symbol: &str,
    interval: &str,
    start_time: Option<&str>,
    end_time: Option<&str>,
    output: Option<&str>,
) -> anyhow::Result<()> {
    // 创建下载器实例
    let downloader = BinanceHistoricalDownloader::new();
    
    // 处理时间参数（目前暂不支持）
    if start_time.is_some() || end_time.is_some() {
        warn!("时间参数功能正在开发中，当前版本将获取最新的1000条数据");
    }

    // 获取数据
    let klines = downloader.fetch_klines(
        symbol,
        interval,
        None, // 暂时不支持时间范围
        None,
        Some(1000),
    ).await
    .map_err(|e| anyhow::anyhow!("获取数据失败: {}", e))?;

    // 保存到文件
    let default_path = format!("{}_{}.csv", symbol.to_lowercase(), interval);
    let output_path = output.unwrap_or(&default_path);
    
    save_to_csv(&klines, output_path).await
        .map_err(|e| anyhow::anyhow!("保存文件失败: {}", e))?;
    
    info!("数据已保存到: {}", output_path);
    Ok(())
}

/// 将K线数据保存为CSV文件的便利函数
/// 
/// 这是一个异步版本的CSV保存函数，提供向后兼容性。
/// 
/// # 参数
/// 
/// * `klines` - K线数据切片
/// * `file_path` - 输出文件路径
/// 
/// # 返回值
/// 
/// 成功时返回()，失败时返回DataError
/// 
/// # 示例
/// 
/// ```rust,no_run
/// use aurora_data::historical::save_to_csv;
/// use aurora_core::Kline;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let klines = vec![
///     Kline {
///         timestamp: 1640995200000,
///         open: 50000.0,
///         high: 51000.0,
///         low: 49000.0,
///         close: 50500.0,
///         volume: 100.0,
///     }
/// ];
/// 
/// save_to_csv(&klines, "output.csv").await?;
/// # Ok(())
/// # }
/// ```
pub async fn save_to_csv(klines: &[Kline], file_path: &str) -> DataResult<()> {
    let mut writer = csv::Writer::from_path(file_path)
        .map_err(|e| DataError::IoError(format!("创建CSV文件失败: {}", e)))?;
    
    // 写入表头
    writer.write_record(&["timestamp", "open", "high", "low", "close", "volume"])
        .map_err(|e| DataError::IoError(format!("写入CSV表头失败: {}", e)))?;
    
    // 写入数据行
    for kline in klines {
        writer.write_record(&[
            kline.timestamp.to_string(),
            kline.open.to_string(),
            kline.high.to_string(),
            kline.low.to_string(),
            kline.close.to_string(),
            kline.volume.to_string(),
        ]).map_err(|e| DataError::IoError(format!("写入CSV数据失败: {}", e)))?;
    }
    
    writer.flush()
        .map_err(|e| DataError::IoError(format!("刷新CSV文件失败: {}", e)))?;
        
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tokio;
    use approx::assert_relative_eq;

    /// 创建测试用的BinanceKline数据
    fn create_test_binance_kline() -> BinanceKline {
        BinanceKline(
            1640995200000,              // 开盘时间
            "50000.00".to_string(),     // 开盘价
            "51000.00".to_string(),     // 最高价
            "49000.00".to_string(),     // 最低价
            "50500.00".to_string(),     // 收盘价
            "100.0".to_string(),        // 成交量
            1640998800000,              // 收盘时间
            "5050000.0".to_string(),    // 成交额
            1000,                       // 成交笔数
            "50.0".to_string(),         // 主动买入成交量
            "2525000.0".to_string(),    // 主动买入成交额
            "0".to_string(),            // 忽略字段
        )
    }

    /// 创建测试用的Kline数据
    fn create_test_kline() -> Kline {
        Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        }
    }

    /// 测试Binance K线数据转换
    #[test]
    fn test_binance_kline_conversion() {
        let binance_kline = create_test_binance_kline();
        let kline: Kline = binance_kline.into();
        
        // 验证转换后的数据
        assert_eq!(kline.timestamp, 1640995200000);
        assert_relative_eq!(kline.open, 50000.0, epsilon = 1e-6);
        assert_relative_eq!(kline.high, 51000.0, epsilon = 1e-6);
        assert_relative_eq!(kline.low, 49000.0, epsilon = 1e-6);
        assert_relative_eq!(kline.close, 50500.0, epsilon = 1e-6);
        assert_relative_eq!(kline.volume, 100.0, epsilon = 1e-6);
    }

    /// 测试包含无效数据的Binance K线转换
    #[test]
    fn test_binance_kline_conversion_with_invalid_data() {
        let binance_kline = BinanceKline(
            1640995200000,
            "invalid_price".to_string(), // 无效的价格字符串
            "51000.00".to_string(),
            "49000.00".to_string(),
            "50500.00".to_string(),
            "invalid_volume".to_string(), // 无效的成交量字符串
            1640998800000,
            "5050000.0".to_string(),
            1000,
            "50.0".to_string(),
            "2525000.0".to_string(),
            "0".to_string(),
        );
        
        let kline: Kline = binance_kline.into();
        
        // 无效数据应该转换为0.0
        assert_eq!(kline.open, 0.0);
        assert_eq!(kline.volume, 0.0);
        // 有效数据应该正常转换
        assert_eq!(kline.high, 51000.0);
    }

    /// 测试BinanceHistoricalDownloader创建
    #[test]
    fn test_downloader_creation() {
        // 测试默认配置
        let downloader = BinanceHistoricalDownloader::new();
        assert_eq!(downloader.config.base_url, "https://api.binance.com");
        assert_eq!(downloader.config.timeout_secs, 30);
        assert_eq!(downloader.config.max_retries, 3);
        
        // 测试自定义配置
        let config = DataSourceConfig::new("https://testapi.com")
            .with_timeout(60)
            .with_max_retries(5);
        let downloader = BinanceHistoricalDownloader::with_config(config);
        assert_eq!(downloader.config.base_url, "https://testapi.com");
        assert_eq!(downloader.config.timeout_secs, 60);
        assert_eq!(downloader.config.max_retries, 5);
    }

    /// 测试时间间隔解析
    #[test]
    fn test_parse_interval_to_ms() {
        let downloader = BinanceHistoricalDownloader::new();
        
        // 测试各种有效间隔
        assert_eq!(downloader.parse_interval_to_ms("1m").unwrap(), 60 * 1000);
        assert_eq!(downloader.parse_interval_to_ms("5m").unwrap(), 5 * 60 * 1000);
        assert_eq!(downloader.parse_interval_to_ms("1h").unwrap(), 60 * 60 * 1000);
        assert_eq!(downloader.parse_interval_to_ms("1d").unwrap(), 24 * 60 * 60 * 1000);
        assert_eq!(downloader.parse_interval_to_ms("1w").unwrap(), 7 * 24 * 60 * 60 * 1000);
        
        // 测试无效间隔
        let result = downloader.parse_interval_to_ms("invalid");
        assert!(result.is_err());
        
        if let Err(DataError::ConfigError(msg)) = result {
            assert!(msg.contains("不支持的时间间隔"));
        } else {
            panic!("期望ConfigError");
        }
    }

    /// 测试K线数据验证
    #[test]
    fn test_validate_kline() {
        let downloader = BinanceHistoricalDownloader::new();
        
        // 有效的K线数据
        let valid_kline = create_test_kline();
        assert!(downloader.validate_kline(&valid_kline));
        
        // 无效的K线数据：最高价小于最低价
        let invalid_kline1 = Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 49000.0, // 最高价小于最低价
            low: 51000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!downloader.validate_kline(&invalid_kline1));
        
        // 无效的K线数据：负价格
        let invalid_kline2 = Kline {
            timestamp: 1640995200000,
            open: -50000.0, // 负价格
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!downloader.validate_kline(&invalid_kline2));
        
        // 无效的K线数据：负成交量
        let invalid_kline3 = Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: -100.0, // 负成交量
        };
        assert!(!downloader.validate_kline(&invalid_kline3));
        
        // 无效的K线数据：无效时间戳
        let invalid_kline4 = Kline {
            timestamp: -1, // 无效时间戳
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!downloader.validate_kline(&invalid_kline4));
    }

    /// 测试CSV保存功能
    #[tokio::test]
    async fn test_save_to_csv() {
        let klines = vec![
            create_test_kline(),
            Kline {
                timestamp: 1640998800000,
                open: 50500.0,
                high: 52000.0,
                low: 50000.0,
                close: 51500.0,
                volume: 200.0,
            },
        ];
        
        let temp_file = "test_klines_async.csv";
        
        // 测试保存功能
        let result = save_to_csv(&klines, temp_file).await;
        assert!(result.is_ok(), "保存CSV失败: {:?}", result);
        
        // 验证文件是否创建
        assert!(Path::new(temp_file).exists());
        
        // 读取并验证文件内容
        let content = fs::read_to_string(temp_file).expect("读取文件失败");
        assert!(content.contains("timestamp,open,high,low,close,volume")); // 表头
        assert!(content.contains("1640995200000")); // 第一行数据
        assert!(content.contains("50000")); // 价格数据
        
        // 清理测试文件
        if Path::new(temp_file).exists() {
            fs::remove_file(temp_file).expect("删除测试文件失败");
        }
    }

    /// 测试空数据保存
    #[tokio::test]
    async fn test_save_empty_csv() {
        let klines: Vec<Kline> = vec![];
        let temp_file = "test_empty.csv";
        
        let result = save_to_csv(&klines, temp_file).await;
        assert!(result.is_ok());
        
        // 验证只有表头
        if Path::new(temp_file).exists() {
            let content = fs::read_to_string(temp_file).expect("读取文件失败");
            let lines: Vec<&str> = content.lines().collect();
            assert_eq!(lines.len(), 1); // 只有表头行
            assert_eq!(lines[0], "timestamp,open,high,low,close,volume");
            
            fs::remove_file(temp_file).expect("删除测试文件失败");
        }
    }

    /// 测试无效文件路径
    #[tokio::test]
    async fn test_save_to_invalid_path() {
        let klines = vec![create_test_kline()];
        
        // 使用无效路径（包含不存在的目录）
        let invalid_path = "/invalid/directory/test.csv";
        let result = save_to_csv(&klines, invalid_path).await;
        
        assert!(result.is_err());
        if let Err(DataError::IoError(msg)) = result {
            assert!(msg.contains("创建CSV文件失败"));
        } else {
            panic!("期望IoError");
        }
    }

    /// 测试BinanceKline的序列化和反序列化
    #[test]
    fn test_binance_kline_serde() {
        let original = create_test_binance_kline();
        
        // 测试序列化
        let serialized = serde_json::to_string(&original).expect("序列化失败");
        assert!(!serialized.is_empty());
        
        // 测试反序列化
        let deserialized: BinanceKline = serde_json::from_str(&serialized).expect("反序列化失败");
        
        // 验证数据一致性
        assert_eq!(original.0, deserialized.0); // 时间戳
        assert_eq!(original.1, deserialized.1); // 开盘价
        assert_eq!(original.4, deserialized.4); // 收盘价
    }

    /// 测试BinanceKline克隆
    #[test]
    fn test_binance_kline_clone() {
        let original = create_test_binance_kline();
        let cloned = original.clone();
        
        // 验证克隆后的数据
        assert_eq!(original.0, cloned.0);
        assert_eq!(original.1, cloned.1);
        assert_eq!(original.4, cloned.4);
    }

    /// 测试下载器克隆
    #[test]
    fn test_downloader_clone() {
        let original = BinanceHistoricalDownloader::new();
        let cloned = original.clone();
        
        // 验证配置一致性
        assert_eq!(original.config.base_url, cloned.config.base_url);
        assert_eq!(original.config.timeout_secs, cloned.config.timeout_secs);
        assert_eq!(original.config.max_retries, cloned.config.max_retries);
    }

    /// 测试边界条件：极端时间戳
    #[test]
    fn test_extreme_timestamps() {
        let downloader = BinanceHistoricalDownloader::new();
        
        // 测试零时间戳
        let zero_timestamp_kline = Kline {
            timestamp: 0,
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 50.0,
        };
        assert!(!downloader.validate_kline(&zero_timestamp_kline));
        
        // 测试负时间戳
        let negative_timestamp_kline = Kline {
            timestamp: -1000,
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 50.0,
        };
        assert!(!downloader.validate_kline(&negative_timestamp_kline));
        
        // 测试很大的时间戳（应该有效）
        let large_timestamp_kline = Kline {
            timestamp: 9999999999999, // 很大的未来时间戳
            open: 100.0,
            high: 110.0,
            low: 90.0,
            close: 105.0,
            volume: 50.0,
        };
        assert!(downloader.validate_kline(&large_timestamp_kline));
    }

    /// 测试批量数据验证性能
    #[test]
    fn test_batch_validation_performance() {
        let downloader = BinanceHistoricalDownloader::new();
        
        // 创建大量测试数据
        let mut klines = Vec::new();
        for i in 0..1000 {
            klines.push(Kline {
                timestamp: 1640995200000 + i * 60000, // 每分钟递增
                open: 50000.0 + i as f64,
                high: 51000.0 + i as f64,
                low: 49000.0 + i as f64,
                close: 50500.0 + i as f64,
                volume: 100.0,
            });
        }
        
        // 验证所有数据
        let start = std::time::Instant::now();
        let valid_count = klines.iter()
            .filter(|kline| downloader.validate_kline(kline))
            .count();
        let duration = start.elapsed();
        
        assert_eq!(valid_count, 1000); // 所有数据都应该有效
        println!("批量验证1000条数据耗时: {:?}", duration);
        
        // 验证性能在合理范围内（应该很快）
        assert!(duration.as_millis() < 100, "验证耗时过长: {:?}", duration);
    }
}