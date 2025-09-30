//! Aurora Core 核心模块集成测试

use aurora_core::{Kline, MarketEvent, Signal, SignalEvent, Strategy, DataSource};
use async_trait::async_trait;
use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver};

/// 模拟策略用于测试
struct MockStrategy {
    signals: Vec<SignalEvent>,
    signal_index: usize,
}

impl MockStrategy {
    fn new() -> Self {
        Self {
            signals: Vec::new(),
            signal_index: 0,
        }
    }
    
    fn with_signals(signals: Vec<SignalEvent>) -> Self {
        Self {
            signals,
            signal_index: 0,
        }
    }
}

impl Strategy for MockStrategy {
    fn on_market_event(&mut self, _event: &MarketEvent) -> Option<SignalEvent> {
        if self.signal_index < self.signals.len() {
            let signal = self.signals[self.signal_index].clone();
            self.signal_index += 1;
            Some(signal)
        } else {
            None
        }
    }
}

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

/// 创建测试用的K线数据
fn create_test_klines() -> Vec<Kline> {
    vec![
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        },
        Kline {
            timestamp: 1640995260000,
            open: 102.0,
            high: 108.0,
            low: 98.0,
            close: 106.0,
            volume: 1200.0,
        },
        Kline {
            timestamp: 1640995320000,
            open: 106.0,
            high: 112.0,
            low: 102.0,
            close: 110.0,
            volume: 1100.0,
        },
        Kline {
            timestamp: 1640995380000,
            open: 110.0,
            high: 115.0,
            low: 105.0,
            close: 108.0,
            volume: 950.0,
        },
        Kline {
            timestamp: 1640995440000,
            open: 108.0,
            high: 112.0,
            low: 103.0,
            close: 105.0,
            volume: 800.0,
        },
    ]
}

/// 测试Kline结构的基本功能
#[test]
fn test_kline_basic() {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    assert_eq!(kline.timestamp, 1640995200000);
    assert_eq!(kline.open, 100.0);
    assert_eq!(kline.high, 105.0);
    assert_eq!(kline.low, 95.0);
    assert_eq!(kline.close, 102.0);
    assert_eq!(kline.volume, 1000.0);
}

/// 测试Kline的Clone和Debug traits
#[test]
fn test_kline_traits() {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    // 测试Clone
    let cloned_kline = kline.clone();
    assert_eq!(kline.timestamp, cloned_kline.timestamp);
    assert_eq!(kline.open, cloned_kline.open);
    
    // 测试Debug
    let debug_string = format!("{:?}", kline);
    assert!(debug_string.contains("1640995200000"));
    assert!(debug_string.contains("100.0"));
}

/// 测试Signal枚举的基本功能
#[test]
fn test_signal_basic() {
    let buy_signal = Signal::Buy;
    let sell_signal = Signal::Sell;
    let hold_signal = Signal::Hold;
    
    // 测试匹配
    match buy_signal {
        Signal::Buy => {},
        _ => panic!("应该是买入信号"),
    }
    
    match sell_signal {
        Signal::Sell => {},
        _ => panic!("应该是卖出信号"),
    }
    
    match hold_signal {
        Signal::Hold => {},
        _ => panic!("应该是持有信号"),
    }
}

/// 测试SignalEvent结构
#[test]
fn test_signal_event() {
    let signal_event = SignalEvent {
        signal: Signal::Buy,
        price: 100.0,
        timestamp: 1640995200000,
    };
    
    assert_eq!(signal_event.signal, Signal::Buy);
    assert_eq!(signal_event.price, 100.0);
    assert_eq!(signal_event.timestamp, 1640995200000);
}

/// 测试MarketEvent枚举的基本功能
#[test]
fn test_market_event_basic() {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    let event = MarketEvent::Kline(kline.clone());
    
    match event {
        MarketEvent::Kline(k) => {
            assert_eq!(k.timestamp, kline.timestamp);
            assert_eq!(k.close, kline.close);
        }
    }
}

/// 测试MarketEvent的Clone和Debug traits
#[test]
fn test_market_event_traits() {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    let event = MarketEvent::Kline(kline);
    
    // 测试Clone
    let cloned_event = event.clone();
    match (event, cloned_event) {
        (MarketEvent::Kline(k1), MarketEvent::Kline(k2)) => {
            assert_eq!(k1.timestamp, k2.timestamp);
        }
    }
}

/// 测试Strategy trait的基本功能
#[test]
fn test_strategy_trait() {
    let signals = vec![
        SignalEvent {
            signal: Signal::Buy,
            price: 100.0,
            timestamp: 1000,
        },
        SignalEvent {
            signal: Signal::Sell,
            price: 110.0,
            timestamp: 2000,
        },
    ];
    
    let mut strategy = MockStrategy::with_signals(signals);
    
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    let event = MarketEvent::Kline(kline);
    
    // 第一次调用应该返回买入信号
    let result1 = strategy.on_market_event(&event);
    assert!(result1.is_some());
    let signal1 = result1.unwrap();
    assert_eq!(signal1.signal, Signal::Buy);
    assert_eq!(signal1.price, 100.0);
    
    // 第二次调用应该返回卖出信号
    let result2 = strategy.on_market_event(&event);
    assert!(result2.is_some());
    let signal2 = result2.unwrap();
    assert_eq!(signal2.signal, Signal::Sell);
    assert_eq!(signal2.price, 110.0);
    
    // 第三次调用应该返回None
    let result3 = strategy.on_market_event(&event);
    assert!(result3.is_none());
}

/// 测试DataSource trait的基本功能
#[tokio::test]
async fn test_data_source_trait() -> Result<()> {
    let test_klines = create_test_klines();
    let expected_count = test_klines.len();
    let mut data_source = MockDataSource::new(test_klines);
    
    // 启动数据源
    let mut event_receiver = data_source.start().await?;
    
    let mut received_count = 0;
    while let Ok(event) = event_receiver.try_recv() {
        match event {
            MarketEvent::Kline(_) => received_count += 1,
        }
    }
    
    assert_eq!(received_count, expected_count);
    
    Ok(())
}

/// 测试数据源和策略的集成
#[tokio::test]
async fn test_data_source_strategy_integration() -> Result<()> {
    let test_klines = vec![
        Kline {
            timestamp: 1000,
            open: 100.0,
            high: 110.0,
            low: 95.0,
            close: 105.0, // 高于阈值
            volume: 1000.0,
        },
        Kline {
            timestamp: 2000,
            open: 105.0,
            high: 108.0,
            low: 102.0,
            close: 103.0, // 在阈值附近
            volume: 800.0,
        },
    ];
    
    let signals = vec![
        SignalEvent {
            signal: Signal::Buy,
            price: 105.0,
            timestamp: 1000,
        },
    ];
    
    let mut data_source = MockDataSource::new(test_klines);
    let mut strategy = MockStrategy::with_signals(signals);
    
    // 启动数据源
    let mut event_receiver = data_source.start().await?;
    
    let mut generated_signals = Vec::new();
    while let Ok(event) = event_receiver.try_recv() {
        if let Some(signal_event) = strategy.on_market_event(&event) {
            generated_signals.push(signal_event);
        }
    }
    
    assert_eq!(generated_signals.len(), 1);
    assert_eq!(generated_signals[0].signal, Signal::Buy);
    
    Ok(())
}

/// 测试数据结构的序列化和反序列化
#[test]
fn test_kline_serialization() -> Result<()> {
    let kline = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    // 序列化
    let json_string = serde_json::to_string(&kline)?;
    assert!(json_string.contains("1640995200000"));
    assert!(json_string.contains("100.0"));
    
    // 反序列化
    let deserialized_kline: Kline = serde_json::from_str(&json_string)?;
    assert_eq!(deserialized_kline.timestamp, kline.timestamp);
    assert_eq!(deserialized_kline.open, kline.open);
    assert_eq!(deserialized_kline.high, kline.high);
    assert_eq!(deserialized_kline.low, kline.low);
    assert_eq!(deserialized_kline.close, kline.close);
    assert_eq!(deserialized_kline.volume, kline.volume);
    
    Ok(())
}

/// 测试边界值处理
#[test]
fn test_boundary_values() {
    // 测试极小值
    let small_kline = Kline {
        timestamp: 1,
        open: 0.0001,
        high: 0.0002,
        low: 0.0001,
        close: 0.0001,
        volume: 0.001,
    };
    
    assert_eq!(small_kline.timestamp, 1);
    assert_eq!(small_kline.open, 0.0001);
    
    // 测试零值
    let zero_kline = Kline {
        timestamp: 0,
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0,
    };
    
    assert_eq!(zero_kline.volume, 0.0);
}

/// 测试数据结构的相等性比较
#[test]
fn test_equality() {
    let kline1 = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    let kline2 = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000.0,
    };
    
    let kline3 = Kline {
        timestamp: 1640995200000,
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 103.0, // 不同的收盘价
        volume: 1000.0,
    };
    
    // 测试相等的K线
    assert_eq!(kline1, kline2);
    
    // 测试不相等的K线
    assert_ne!(kline1, kline3);
    
    // 测试Signal的相等性
    let signal1 = Signal::Buy;
    let signal2 = Signal::Buy;
    let signal3 = Signal::Sell;
    
    assert_eq!(signal1, signal2);
    assert_ne!(signal1, signal3);
}

/// 测试类型安全性
#[test]
fn test_type_safety() {
    // 确保类型系统正确工作
    let _kline: Kline = Kline {
        timestamp: 0,
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0,
    };
    let _signal: Signal = Signal::Buy;
    let _event: MarketEvent = MarketEvent::Kline(_kline);
    
    // 这些应该能正常编译，证明类型定义正确
}