//! 集成测试 - 测试数据源和策略接口的集成使用

use aurora_core::{DataSource, Strategy, MarketEvent, SignalEvent, Kline, Signal};
use async_trait::async_trait;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use anyhow::Result;

/// 模拟数据源用于测试
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
    async fn start(&mut self) -> Result<UnboundedReceiver<MarketEvent>> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // 发送所有K线数据
        for kline in self.klines.drain(..) {
            tx.send(MarketEvent::Kline(kline)).unwrap();
        }
        
        Ok(rx)
    }
}

/// 模拟策略用于测试
struct MockStrategy {
    threshold: f64,
}

impl MockStrategy {
    fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

impl Strategy for MockStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        match event {
            MarketEvent::Kline(kline) => {
                if kline.close > self.threshold {
                    Some(SignalEvent {
                        signal: Signal::Buy,
                        price: kline.close,
                        timestamp: kline.timestamp,
                    })
                } else if kline.close < self.threshold * 0.9 {
                    Some(SignalEvent {
                        signal: Signal::Sell,
                        price: kline.close,
                        timestamp: kline.timestamp,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[tokio::test]
async fn test_data_source_strategy_integration() {
    // 准备测试数据
    let test_klines = vec![
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0, // 超过阈值，应该产生买入信号
            volume: 1000.0,
        },
        Kline {
            timestamp: 1640995260000,
            open: 105.0,
            high: 108.0,
            low: 102.0,
            close: 103.0, // 在阈值附近，不应该产生信号
            volume: 800.0,
        },
        Kline {
            timestamp: 1640995320000,
            open: 103.0,
            high: 104.0,
            low: 88.0,
            close: 90.0, // 低于卖出阈值，应该产生卖出信号
            volume: 1200.0,
        },
    ];

    // 创建模拟数据源
    let mut data_source = MockDataSource::new(test_klines);
    let mut strategy = MockStrategy::new(102.0); // 阈值设为102

    // 启动数据源
    let mut event_receiver = data_source.start().await.unwrap();
    
    let mut signals = Vec::new();
    
    // 处理所有事件
    while let Ok(event) = event_receiver.try_recv() {
        if let Some(signal_event) = strategy.on_market_event(&event) {
            signals.push(signal_event);
        }
    }

    // 验证结果
    assert_eq!(signals.len(), 2); // 应该有2个信号
    
    // 第一个信号应该是买入（价格105.0 > 102.0）
    assert!(matches!(signals[0].signal, Signal::Buy));
    assert_eq!(signals[0].price, 105.0);
    
    // 第二个信号应该是卖出（价格90.0 < 102.0 * 0.9 = 91.8）
    assert!(matches!(signals[1].signal, Signal::Sell));
    assert_eq!(signals[1].price, 90.0);
}

#[tokio::test]
async fn test_empty_data_source() {
    let mut data_source = MockDataSource::new(vec![]);
    let mut event_receiver = data_source.start().await.unwrap();
    
    // 空数据源应该立即结束
    let result = event_receiver.try_recv();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_strategy_no_signals() {
    let test_klines = vec![
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 101.0,
            low: 99.0,
            close: 100.5, // 在阈值范围内，不应产生信号
            volume: 1000.0,
        },
    ];

    let mut data_source = MockDataSource::new(test_klines);
    let mut strategy = MockStrategy::new(102.0);
    let mut event_receiver = data_source.start().await.unwrap();
    
    let mut signal_count = 0;
    while let Ok(event) = event_receiver.try_recv() {
        if let Some(_) = strategy.on_market_event(&event) {
            signal_count += 1;
        }
    }

    assert_eq!(signal_count, 0);
}

#[test]
fn test_kline_serialization() {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    // 测试序列化
    let json = serde_json::to_string(&kline).unwrap();
    assert!(json.contains("1640995200000"));
    assert!(json.contains("100.0"));
    assert!(json.contains("102.0"));
    
    // 测试反序列化
    let deserialized: Kline = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, kline);
}

#[test]
fn test_signal_debug_format() {
    let buy_signal = Signal::Buy;
    let sell_signal = Signal::Sell;
    let hold_signal = Signal::Hold;
    
    assert_eq!(format!("{:?}", buy_signal), "Buy");
    assert_eq!(format!("{:?}", sell_signal), "Sell");
    assert_eq!(format!("{:?}", hold_signal), "Hold");
}

#[test]
fn test_signal_event_debug_format() {
    let signal_event = SignalEvent {
        signal: Signal::Buy,
        price: 102.0,
        timestamp: 1640995200000,
    };
    
    let debug_str = format!("{:?}", signal_event);
    assert!(debug_str.contains("Buy"));
    assert!(debug_str.contains("102.0"));
    assert!(debug_str.contains("1640995200000"));
}