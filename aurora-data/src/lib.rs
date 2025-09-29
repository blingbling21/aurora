//! # Aurora Data Library
//! 
//! 这个crate提供了量化交易系统的数据获取和处理功能。它支持从多种数据源
//! 获取历史数据和实时数据，包括REST API和WebSocket连接。
//! 
//! ## 主要功能
//! 
//! - **历史数据获取**: 从交易所REST API获取K线历史数据
//! - **实时数据流**: 通过WebSocket连接获取实时市场数据
//! - **数据存储**: 将获取的数据保存为CSV格式
//! - **数据验证**: 确保数据的完整性和有效性
//! 
//! ## 支持的数据源
//! 
//! - **Binance**: 支持REST API和WebSocket
//! - 可扩展支持其他交易所
//! 
//! ## 使用示例
//! 
//! ### 历史数据获取
//! 
//! ```rust,no_run
//! use aurora_data::BinanceHistoricalDownloader;
//! use tokio;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let downloader = BinanceHistoricalDownloader::new();
//!     
//!     // 下载BTCUSDT的1小时K线数据
//!     downloader.download_klines(
//!         "BTCUSDT",
//!         "1h",
//!         1640995200000, // 开始时间戳
//!         1641081600000, // 结束时间戳
//!         "btc_data.csv"
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ### 实时数据流
//! 
//! ```rust,no_run
//! use aurora_data::BinanceLiveStream;
//! use aurora_core::DataSource;
//! use tokio;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut stream = BinanceLiveStream::new();
//!     
//!     // 连接到实时数据流
//!     stream.connect(&["BTCUSDT", "ETHUSDT"]).await?;
//!     
//!     // 接收数据
//!     while let Some(kline) = stream.next_kline().await? {
//!         println!("收到K线: {}", kline.close);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## 配置选项
//! 
//! - **请求限制**: 自动处理API请求频率限制
//! - **重试机制**: 网络错误时自动重试
//! - **数据格式**: 统一的K线数据格式输出
//! 
//! ## 错误处理
//! 
//! 所有公开函数都返回 `Result` 类型，包含详细的错误信息。
//! 常见错误包括网络连接失败、API限制、数据格式错误等。

// 导入必要类型
use std::error::Error;
use std::fmt;

/// 数据获取相关的错误类型
/// 
/// 这个枚举定义了在数据获取过程中可能遇到的各种错误。
/// 使用统一的错误类型有助于错误处理和调试。
#[derive(Debug, Clone)]
pub enum DataError {
    /// 网络连接错误
    /// 包含底层的网络错误信息
    NetworkError(String),
    
    /// API响应错误
    /// 当API返回错误状态码或无效响应时触发
    ApiError(String),
    
    /// 数据解析错误
    /// 当接收到的数据格式不符合预期时触发
    ParseError(String),
    
    /// 文件操作错误
    /// 在读写文件时发生错误
    IoError(String),
    
    /// 配置错误
    /// 当传入的参数或配置无效时触发
    ConfigError(String),
    
    /// WebSocket连接错误
    /// WebSocket特有的连接或通信错误
    WebSocketError(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            DataError::ApiError(msg) => write!(f, "API错误: {}", msg),
            DataError::ParseError(msg) => write!(f, "数据解析错误: {}", msg),
            DataError::IoError(msg) => write!(f, "文件操作错误: {}", msg),
            DataError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            DataError::WebSocketError(msg) => write!(f, "WebSocket错误: {}", msg),
        }
    }
}

impl Error for DataError {}

/// 数据获取结果类型
/// 
/// 这是一个便利类型别名，用于所有可能返回错误的数据操作。
pub type DataResult<T> = Result<T, DataError>;

/// 通用的数据源配置
/// 
/// 这个结构体包含了连接到数据源所需的通用配置参数。
/// 不同的数据源可以扩展这个基础配置。
#[derive(Debug, Clone)]
pub struct DataSourceConfig {
    /// API基础URL
    pub base_url: String,
    
    /// WebSocket URL
    pub ws_url: Option<String>,
    
    /// API密钥（如果需要）
    pub api_key: Option<String>,
    
    /// 密钥（如果需要）
    pub secret_key: Option<String>,
    
    /// 请求超时时间（秒）
    pub timeout_secs: u64,
    
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for DataSourceConfig {
    /// 创建默认配置
    /// 
    /// 默认配置适用于公开的API访问，不包含认证信息。
    fn default() -> Self {
        Self {
            base_url: "https://api.binance.com".to_string(),
            ws_url: Some("wss://stream.binance.com:9443".to_string()),
            api_key: None,
            secret_key: None,
            timeout_secs: 30,
            max_retries: 3,
        }
    }
}

impl DataSourceConfig {
    /// 创建新的数据源配置
    /// 
    /// # 参数
    /// 
    /// * `base_url` - API基础URL
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_data::DataSourceConfig;
    /// 
    /// let config = DataSourceConfig::new("https://api.binance.com");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }
    
    /// 设置WebSocket URL
    /// 
    /// # 参数
    /// 
    /// * `ws_url` - WebSocket连接URL
    /// 
    /// # 返回值
    /// 
    /// 返回修改后的配置实例
    pub fn with_websocket(mut self, ws_url: &str) -> Self {
        self.ws_url = Some(ws_url.to_string());
        self
    }
    
    /// 设置API认证信息
    /// 
    /// # 参数
    /// 
    /// * `api_key` - API密钥
    /// * `secret_key` - 密钥
    /// 
    /// # 返回值
    /// 
    /// 返回修改后的配置实例
    pub fn with_auth(mut self, api_key: &str, secret_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self.secret_key = Some(secret_key.to_string());
        self
    }
    
    /// 设置超时时间
    /// 
    /// # 参数
    /// 
    /// * `timeout_secs` - 超时时间（秒）
    /// 
    /// # 返回值
    /// 
    /// 返回修改后的配置实例
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }
    
    /// 设置最大重试次数
    /// 
    /// # 参数
    /// 
    /// * `max_retries` - 最大重试次数
    /// 
    /// # 返回值
    /// 
    /// 返回修改后的配置实例
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试默认配置创建
    #[test]
    fn test_default_config() {
        let config = DataSourceConfig::default();
        
        assert_eq!(config.base_url, "https://api.binance.com");
        assert_eq!(config.ws_url, Some("wss://stream.binance.com:9443".to_string()));
        assert_eq!(config.api_key, None);
        assert_eq!(config.secret_key, None);
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.max_retries, 3);
    }

    /// 测试配置构建器模式
    #[test]
    fn test_config_builder() {
        let config = DataSourceConfig::new("https://testapi.com")
            .with_websocket("wss://testws.com")
            .with_auth("test_key", "test_secret")
            .with_timeout(60)
            .with_max_retries(5);
            
        assert_eq!(config.base_url, "https://testapi.com");
        assert_eq!(config.ws_url, Some("wss://testws.com".to_string()));
        assert_eq!(config.api_key, Some("test_key".to_string()));
        assert_eq!(config.secret_key, Some("test_secret".to_string()));
        assert_eq!(config.timeout_secs, 60);
        assert_eq!(config.max_retries, 5);
    }

    /// 测试错误类型显示
    #[test]
    fn test_error_display() {
        let errors = vec![
            DataError::NetworkError("连接失败".to_string()),
            DataError::ApiError("API限制".to_string()),
            DataError::ParseError("JSON格式错误".to_string()),
            DataError::IoError("文件不存在".to_string()),
            DataError::ConfigError("参数无效".to_string()),
            DataError::WebSocketError("连接断开".to_string()),
        ];
        
        for error in errors {
            let error_str = format!("{}", error);
            assert!(!error_str.is_empty());
            println!("Error: {}", error_str);
        }
    }

    /// 测试错误类型克隆
    #[test]
    fn test_error_clone() {
        let original = DataError::NetworkError("测试错误".to_string());
        let cloned = original.clone();
        
        match (&original, &cloned) {
            (DataError::NetworkError(msg1), DataError::NetworkError(msg2)) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("克隆后类型不匹配"),
        }
    }

    /// 测试配置克隆
    #[test]
    fn test_config_clone() {
        let original = DataSourceConfig::new("https://test.com")
            .with_auth("key", "secret");
            
        let cloned = original.clone();
        
        assert_eq!(original.base_url, cloned.base_url);
        assert_eq!(original.api_key, cloned.api_key);
        assert_eq!(original.secret_key, cloned.secret_key);
    }
}

// 内部模块声明
pub mod historical;
pub mod live;

// 重新导出主要的公共类型和函数
pub use historical::BinanceHistoricalDownloader;
pub use live::BinanceLiveStream;