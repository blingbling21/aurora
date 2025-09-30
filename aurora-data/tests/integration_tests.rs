//! Aurora Data 模块集成测试

use aurora_data::{
    DataError, DataResult, DataSourceConfig,
    BinanceHistoricalDownloader, BinanceLiveStream, CsvDataLoader,
};
use aurora_core::{Kline, MarketEvent, DataSource};
use tokio;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};
use async_trait::async_trait;

/// 创建测试用的K线数据
fn create_test_kline(open: f64, high: f64, low: f64, close: f64, volume: f64, timestamp: i64) -> Kline {
    Kline {
        open,
        high,
        low,
        close,
        volume,
        timestamp,
    }
}

/// Mock DataSource for testing
#[derive(Debug)]
struct MockDataSource {
    klines: Vec<Kline>,
}

impl MockDataSource {
    fn new(klines: Vec<Kline>) -> Self {
        Self { klines }
    }
}

#[async_trait]
impl DataSource for MockDataSource {
    async fn start(&mut self) -> anyhow::Result<UnboundedReceiver<MarketEvent>> {
        let (tx, rx) = unbounded_channel();
        
        // 发送所有K线数据
        for kline in &self.klines {
            let event = MarketEvent::Kline(kline.clone());
            tx.send(event).map_err(|e| anyhow::anyhow!("Failed to send event: {}", e))?;
        }
        
        Ok(rx)
    }
}

/// 测试 DataSourceConfig 基本功能
#[test]
fn test_data_source_config_basic() {
    let config = DataSourceConfig::default();
    
    assert_eq!(config.base_url, "https://api.binance.com");
    assert!(config.ws_url.is_some());
    assert!(config.api_key.is_none());
    assert!(config.secret_key.is_none());
    assert_eq!(config.timeout_secs, 30);
    assert_eq!(config.max_retries, 3);
}

/// 测试 DataSourceConfig 构建器模式
#[test]
fn test_data_source_config_builder() {
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

/// 测试 DataError 类型
#[test]
fn test_data_error_types() {
    let errors = vec![
        DataError::NetworkError("连接失败".to_string()),
        DataError::ApiError("API限制".to_string()),
        DataError::ParseError("JSON格式错误".to_string()),
        DataError::IoError("文件不存在".to_string()),
        DataError::FileNotFound("test.csv".to_string()),
        DataError::InvalidData("数据为空".to_string()),
        DataError::ConfigError("参数无效".to_string()),
        DataError::WebSocketError("连接断开".to_string()),
    ];
    
    for error in errors {
        let error_str = format!("{}", error);
        assert!(!error_str.is_empty());
        
        // 测试克隆
        let cloned = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned));
    }
}

/// 测试 CsvDataLoader 基本功能
#[test]
fn test_csv_data_loader_creation() {
    let _loader = CsvDataLoader::new();
    // 测试加载器创建成功
}

/// 测试 BinanceHistoricalDownloader 基本功能
#[test]
fn test_binance_historical_downloader_creation() {
    let _downloader = BinanceHistoricalDownloader::new();
    // 测试下载器创建成功
}

/// 测试 BinanceLiveStream 基本功能
#[test]
fn test_binance_live_stream_creation() {
    let _stream = BinanceLiveStream::new();
    // 测试流创建成功
}

/// 测试 MockDataSource 功能
#[tokio::test]
async fn test_mock_data_source() {
    let test_klines = vec![
        create_test_kline(100.0, 105.0, 95.0, 102.0, 1000.0, 1640995200000),
        create_test_kline(102.0, 108.0, 98.0, 106.0, 1500.0, 1640995260000),
    ];
    
    let mut data_source = MockDataSource::new(test_klines.clone());
    let mut receiver = data_source.start().await.unwrap();
    
    let mut received_klines = Vec::new();
    while let Ok(event) = receiver.try_recv() {
        if let MarketEvent::Kline(kline) = event {
            received_klines.push(kline);
        }
    }
    
    assert_eq!(received_klines.len(), test_klines.len());
}

/// 测试配置克隆功能
#[test]
fn test_config_clone() {
    let original = DataSourceConfig::new("https://test.com")
        .with_auth("key", "secret");
        
    let cloned = original.clone();
    
    assert_eq!(original.base_url, cloned.base_url);
    assert_eq!(original.api_key, cloned.api_key);
    assert_eq!(original.secret_key, cloned.secret_key);
}

/// 测试K线数据创建和验证
#[test]
fn test_kline_creation() {
    let kline = create_test_kline(100.0, 105.0, 95.0, 102.0, 1000.0, 1640995200000);
    
    assert_eq!(kline.open, 100.0);
    assert_eq!(kline.high, 105.0);
    assert_eq!(kline.low, 95.0);
    assert_eq!(kline.close, 102.0);
    assert_eq!(kline.volume, 1000.0);
    assert_eq!(kline.timestamp, 1640995200000);
}

/// 测试极端值处理
#[test]
fn test_extreme_values() {
    // 测试零值
    let zero_kline = create_test_kline(0.0, 0.0, 0.0, 0.0, 0.0, 0);
    assert_eq!(zero_kline.close, 0.0);
    
    // 测试大数值 
    let large_value = 1_000_000.0;
    let large_kline = create_test_kline(
        large_value, large_value, large_value, large_value, large_value, 
        1640995200000
    );
    assert_eq!(large_kline.close, large_value);
}

/// 测试数据一致性
#[test]
fn test_data_consistency() {
    let config1 = DataSourceConfig::default();
    let config2 = DataSourceConfig::default();
    
    // 默认配置应该相同
    assert_eq!(config1.base_url, config2.base_url);
    assert_eq!(config1.timeout_secs, config2.timeout_secs);
    assert_eq!(config1.max_retries, config2.max_retries);
}

/// 性能测试 - 大量数据处理
#[tokio::test]
async fn test_large_dataset_performance() {
    let large_dataset: Vec<Kline> = (0..1000)
        .map(|i| create_test_kline(
            100.0 + i as f64,
            105.0 + i as f64,
            95.0 + i as f64,
            102.0 + i as f64,
            1000.0,
            1640995200000 + i * 60000
        ))
        .collect();
    
    let mut data_source = MockDataSource::new(large_dataset.clone());
    
    let start = std::time::Instant::now();
    let mut receiver = data_source.start().await.unwrap();
    
    let mut count = 0;
    while let Ok(_event) = receiver.try_recv() {
        count += 1;
    }
    
    let duration = start.elapsed();
    
    assert_eq!(count, large_dataset.len());
    assert!(duration.as_secs() < 1, "处理1000个K线数据应该在1秒内完成");
}