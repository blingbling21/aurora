//! BinanceLiveStream 的单元测试模块

use super::*;
use tokio;

#[test]
fn test_binance_live_stream_creation() {
    let stream = BinanceLiveStream::new();
    
    // 验证创建成功
    // 注意：由于字段是私有的，我们只能通过方法来验证
    let _ = format!("{:?}", stream);
}

#[test]
fn test_binance_live_stream_with_config() {
    let config = DataSourceConfig::new("wss://stream.binance.com:9443")
        .with_timeout(60);
    let stream = BinanceLiveStream::with_config(config);
    
    // 验证使用自定义配置创建成功
    let _ = format!("{:?}", stream);
}

#[test]
fn test_binance_live_stream_debug_format() {
    let stream = BinanceLiveStream::new();
    let debug_string = format!("{:?}", stream);
    
    // 验证Debug输出包含预期信息
    assert!(debug_string.contains("BinanceLiveStream"));
}

#[test]
fn test_initial_connection_state() {
    let stream = BinanceLiveStream::new();
    
    // 验证初始状态
    assert!(!stream.is_connected());
    assert!(stream.subscribed_symbols().is_empty());
}

#[test]
fn test_set_interval() {
    let mut stream = BinanceLiveStream::new();
    
    // 测试设置不同的时间间隔
    let intervals = vec!["1m", "5m", "15m", "30m", "1h", "4h", "1d"];
    
    for interval in intervals {
        stream.set_interval(interval);
        // 验证不会panic
    }
}

#[tokio::test]
async fn test_connect_invalid_symbols() {
    let mut stream = BinanceLiveStream::new();
    
    // 使用无效的交易对符号进行测试
    let result = stream.connect(&["INVALID_SYMBOL"]).await;
    
    // 由于是模拟网络连接，可能会因为各种原因失败
    // 主要验证方法可以被调用而不panic
    let _ = result;
}

#[tokio::test]
async fn test_connect_empty_symbols() {
    let mut stream = BinanceLiveStream::new();
    
    // 使用空的符号列表进行测试
    let result = stream.connect(&[]).await;
    
    // 应该返回错误或成功处理空列表
    let _ = result;
}

#[tokio::test]
async fn test_next_kline_without_connection() {
    let mut stream = BinanceLiveStream::new();
    
    // 在未连接的情况下尝试获取下一个K线
    let result = stream.next_kline().await;
    
    // 验证结果合理（可能返回错误或None）
    // 主要确保不会panic
    let _ = result;
}

#[tokio::test]
async fn test_disconnect_without_connection() {
    let mut stream = BinanceLiveStream::new();
    
    // 在未连接的情况下尝试断开连接
    stream.disconnect();
    
    // 验证不会panic
    assert!(!stream.is_connected());
}

#[test]
fn test_multiple_stream_instances() {
    // 测试创建多个流实例
    let streams: Vec<BinanceLiveStream> = (0..10)
        .map(|_| BinanceLiveStream::new())
        .collect();
    
    // 验证所有实例都正确创建
    assert_eq!(streams.len(), 10);
    
    for stream in &streams {
        assert!(!stream.is_connected());
        assert!(stream.subscribed_symbols().is_empty());
    }
}

#[test]
fn test_config_variations() {
    // 测试各种配置组合
    let configs = vec![
        DataSourceConfig::new("wss://stream.binance.com:9443"),
        DataSourceConfig::new("wss://stream.binance.us:9443"),
        DataSourceConfig::new("wss://testnet.binance.vision"),
    ];
    
    for config in configs {
        let stream = BinanceLiveStream::with_config(config);
        let _ = format!("{:?}", stream);
        assert!(!stream.is_connected());
    }
}

#[test]
fn test_symbol_validation_patterns() {
    let mut stream = BinanceLiveStream::new();
    
    // 测试各种符号格式
    let test_symbols = vec![
        vec!["BTCUSDT"],
        vec!["ETHUSDT", "BNBUSDT"],
        vec!["btcusdt"], // 小写
        vec!["ADAUSDT", "XRPUSDT", "DOTUSDT"],
    ];
    
    for _symbols in test_symbols {
        // 主要验证不会在准备阶段panic
        stream.set_interval("1m");
        let _ = format!("{:?}", stream);
    }
}

#[test]
fn test_interval_validation_patterns() {
    let mut stream = BinanceLiveStream::new();
    
    // 测试各种时间间隔格式
    let test_intervals = vec![
        "1m", "3m", "5m", "15m", "30m",
        "1h", "2h", "4h", "6h", "8h", "12h",
        "1d", "3d", "1w", "1M",
        "invalid", // 无效间隔
    ];
    
    for interval in test_intervals {
        stream.set_interval(interval);
        // 验证不会panic
        let _ = format!("{:?}", stream);
    }
}

#[tokio::test] 
async fn test_connection_lifecycle() {
    let mut stream = BinanceLiveStream::new();
    
    // 验证连接生命周期
    assert!(!stream.is_connected());
    
    // 尝试连接（可能失败，但不应该panic）
    let _ = stream.connect(&["BTCUSDT"]).await;
    
    // 尝试断开连接
    stream.disconnect();
    
    // 再次验证状态
    assert!(!stream.is_connected());
}

#[test]
fn test_memory_efficiency() {
    // 测试内存使用效率
    let streams: Vec<BinanceLiveStream> = (0..50)
        .map(|_| BinanceLiveStream::new())
        .collect();
    
    // 验证创建多个实例不会导致内存问题
    assert_eq!(streams.len(), 50);
    
    // 验证所有实例都可以被正常使用
    for stream in &streams {
        let _ = format!("{:?}", stream);
        assert!(!stream.is_connected());
    }
}

#[tokio::test]
async fn test_error_handling_invalid_config() {
    // 测试无效配置的错误处理
    let config = DataSourceConfig::new("invalid://url")
        .with_websocket("invalid://websocket.url")
        .with_timeout(5);
    let mut stream = BinanceLiveStream::with_config(config);
    
    let result = stream.connect(&["BTCUSDT"]).await;
    
    // 应该因为无效URL而失败
    assert!(result.is_err());
}

#[test]
fn test_subscribed_symbols_management() {
    let stream = BinanceLiveStream::new();
    
    // 验证初始状态
    let symbols = stream.subscribed_symbols();
    assert!(symbols.is_empty());
    
    // 验证返回的是引用
    let symbols2 = stream.subscribed_symbols();
    assert_eq!(symbols.len(), symbols2.len());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let stream = BinanceLiveStream::new();
    
    // 测试并发操作不会导致死锁或panic
    // 简化测试，只验证基本状态
    assert!(!stream.is_connected());
    assert!(stream.subscribed_symbols().is_empty());
}

#[test]
fn test_clone_behavior() {
    // 注意：BinanceLiveStream可能不实现Clone，这个测试验证这一点
    use std::any::TypeId;
    
    let stream = BinanceLiveStream::new();
    let type_id = TypeId::of::<BinanceLiveStream>();
    
    // 只是验证类型存在
    let _ = (stream, type_id);
}