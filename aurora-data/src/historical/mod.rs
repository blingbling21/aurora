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
//! ## 模块组织
//! 
//! - `types`: 数据类型定义
//! - `downloader`: 下载器实现
//! - `utils`: 工具函数
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

pub mod downloader;
pub mod types;
pub mod utils;

pub use downloader::BinanceHistoricalDownloader;
pub use utils::download_data;

#[cfg(test)]
use utils::save_to_csv;

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::Kline;
    use crate::DataSourceConfig;

    /// 测试模块公共接口
    #[test]
    fn test_public_interface() {
        // 测试创建下载器
        let _downloader = BinanceHistoricalDownloader::new();
        
        // 测试使用自定义配置
        let config = DataSourceConfig::new("https://api.binance.com")
            .with_timeout(60);
        let _downloader_with_config = BinanceHistoricalDownloader::with_config(config);
    }
    
    /// 测试工具函数
    #[tokio::test]
    async fn test_utils() {
        let klines = vec![
            Kline {
                timestamp: 1640995200000,
                open: 50000.0,
                high: 51000.0,
                low: 49000.0,
                close: 50500.0,
                volume: 100.0,
            }
        ];
        
        let temp_file = "test_mod_utils.csv";
        let result = save_to_csv(&klines, temp_file).await;
        assert!(result.is_ok());
        
        // 清理测试文件
        if std::path::Path::new(temp_file).exists() {
            std::fs::remove_file(temp_file).expect("删除测试文件失败");
        }
    }
}