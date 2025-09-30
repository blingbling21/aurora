//! # 实时数据流模块
//! 
//! 这个模块提供实时市场数据流的获取功能。主要通过WebSocket连接
//! 从交易所获取实时的K线数据、交易数据等市场信息。
//! 
//! ## 功能特性
//! 
//! - **WebSocket连接**: 维持与交易所的WebSocket连接
//! - **实时K线**: 获取实时更新的K线数据
//! - **自动重连**: 连接断开时自动重连
//! - **数据过滤**: 只处理完成的K线数据
//! - **错误处理**: 完整的连接和数据错误处理
//! 
//! ## 模块组织
//! 
//! - `stream`: 流实现
//! - `utils`: 工具函数
//! 
//! ## 使用示例
//! 
//! ```rust,no_run
//! use aurora_data::BinanceLiveStream;
//! use aurora_core::DataSource;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut stream = BinanceLiveStream::new();
//! 
//! // 连接到实时数据流
//! stream.connect(&["BTCUSDT", "ETHUSDT"]).await?;
//! 
//! // 接收数据
//! while let Some(kline) = stream.next_kline().await? {
//!     println!("收到K线: 价格 {}, 成交量 {}", kline.close, kline.volume);
//! }
//! # Ok(())
//! # }
//! ```

mod stream;
mod utils;

// 重新导出公共接口
pub use stream::BinanceLiveStream;
pub use utils::stream_data;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataSourceConfig;

    /// 测试模块公共接口
    #[test]
    fn test_public_interface() {
        // 测试创建流
        let _stream = BinanceLiveStream::new();
        
        // 测试使用自定义配置
        let config = DataSourceConfig::new("https://api.binance.com")
            .with_websocket("wss://stream.binance.com:9443");
        let _stream_with_config = BinanceLiveStream::with_config(config);
    }
    
    /// 测试设置时间间隔
    #[test]
    fn test_set_interval() {
        let mut stream = BinanceLiveStream::new();
        stream.set_interval("5m");
        // 由于字段是私有的，这里只验证方法调用不会panic
    }
    
    /// 测试连接状态
    #[test]
    fn test_connection_status() {
        let stream = BinanceLiveStream::new();
        assert!(!stream.is_connected());
        assert!(stream.subscribed_symbols().is_empty());
    }
}