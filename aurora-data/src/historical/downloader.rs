//! # 历史数据下载器实现
//! 
//! 包含了Binance历史数据下载器的具体实现。

use aurora_core::Kline;
use crate::{DataSourceConfig, DataError, DataResult};
use super::types::BinanceKline;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, debug, warn};

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
        let interval_ms = super::utils::parse_interval_to_ms(interval)?;
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
        super::utils::save_to_csv(&all_klines, output_path).await?;
        
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
}

#[cfg(test)]
mod tests;  // 外部测试模块