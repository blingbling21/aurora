use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

/// 核心数据结构：K线
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Kline {
    pub timestamp: i64, // Unix Millisecond Timestamp
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// 市场事件枚举，统一数据源的输出
#[derive(Debug, Clone)]
pub enum MarketEvent {
    Kline(Kline),
    // 未来可扩展，如逐笔成交 Trade, 订单簿 OrderBook 等
}

/// 交易信号枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

/// 包含信号和一些元数据，如触发价格
#[derive(Debug, Clone)]
pub struct SignalEvent {
    pub signal: Signal,
    pub price: f64,
    pub timestamp: i64,
}

/// 定义异步数据源的统一接口
#[async_trait]
pub trait DataSource {
    /// 返回一个无界通道的接收端，用于接收市场事件
    async fn start(&mut self) -> anyhow::Result<UnboundedReceiver<MarketEvent>>;
}

/// 定义策略模块的统一接口
pub trait Strategy: Send + Sync {
    /// 每次市场事件发生时调用
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_creation() {
        let kline = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };
        
        assert_eq!(kline.open, 100.0);
        assert_eq!(kline.close, 102.0);
    }

    #[test]
    fn test_signal_equality() {
        assert_eq!(Signal::Buy, Signal::Buy);
        assert_ne!(Signal::Buy, Signal::Sell);
    }

    #[test]
    fn test_signal_event_creation() {
        let signal_event = SignalEvent {
            signal: Signal::Buy,
            price: 102.0,
            timestamp: 1640995200000,
        };
        
        assert!(matches!(signal_event.signal, Signal::Buy));
        assert_eq!(signal_event.price, 102.0);
    }
}
