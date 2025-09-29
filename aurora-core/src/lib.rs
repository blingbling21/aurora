//! Aurora 核心库 - 定义基础数据结构和通用接口
//!
//! 本模块提供了 Aurora 量化交易框架的核心抽象，包括：
//! - 市场数据结构（K线数据）
//! - 市场事件系统
//! - 交易信号定义
//! - 数据源和策略的统一接口
//!
//! # 示例
//!
//! ```rust
//! use aurora_core::{Kline, Signal, MarketEvent};
//!
//! // 创建K线数据
//! let kline = Kline {
//!     timestamp: 1640995200000,
//!     open: 100.0,
//!     high: 105.0,
//!     low: 95.0,
//!     close: 102.0,
//!     volume: 1000.0,
//! };
//!
//! // 创建市场事件
//! let event = MarketEvent::Kline(kline);
//! ```

use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

/// K线数据结构
///
/// 表示一个时间周期内的价格和成交量信息，是技术分析的基础数据。
///
/// # 字段说明
///
/// * `timestamp` - 时间戳（Unix毫秒）
/// * `open` - 开盘价
/// * `high` - 最高价
/// * `low` - 最低价
/// * `close` - 收盘价
/// * `volume` - 成交量
///
/// # 示例
///
/// ```rust
/// use aurora_core::Kline;
///
/// let kline = Kline {
///     timestamp: 1640995200000, // 2022-01-01 00:00:00 UTC
///     open: 46000.0,
///     high: 47000.0,
///     low: 45500.0,
///     close: 46500.0,
///     volume: 123.45,
/// };
///
/// assert_eq!(kline.open, 46000.0);
/// assert_eq!(kline.close, 46500.0);
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Kline {
    /// Unix时间戳（毫秒）
    pub timestamp: i64,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub volume: f64,
}

/// 市场事件枚举
///
/// 统一不同类型的市场数据输出，提供事件驱动架构的基础。
/// 目前支持K线事件，未来可扩展支持逐笔成交、订单簿等事件。
///
/// # 变体
///
/// * `Kline(Kline)` - K线数据事件
///
/// # 示例
///
/// ```rust
/// use aurora_core::{MarketEvent, Kline};
///
/// let kline = Kline {
///     timestamp: 1640995200000,
///     open: 100.0,
///     high: 105.0,
///     low: 95.0,
///     close: 102.0,
///     volume: 1000.0,
/// };
///
/// let event = MarketEvent::Kline(kline);
/// match event {
///     MarketEvent::Kline(k) => println!("收到K线: 收盘价 {}", k.close),
/// }
/// ```
#[derive(Debug, Clone)]
pub enum MarketEvent {
    /// K线数据事件
    Kline(Kline),
    // 未来可扩展：
    // Trade(Trade),      // 逐笔成交事件
    // OrderBook(OrderBook), // 订单簿事件
}

/// 交易信号枚举
///
/// 表示策略产生的交易决策信号。
///
/// # 变体
///
/// * `Buy` - 买入信号
/// * `Sell` - 卖出信号
/// * `Hold` - 持有/观望信号
///
/// # 示例
///
/// ```rust
/// use aurora_core::Signal;
///
/// let signal = Signal::Buy;
/// match signal {
///     Signal::Buy => println!("执行买入"),
///     Signal::Sell => println!("执行卖出"),
///     Signal::Hold => println!("继续持有"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    /// 买入信号
    Buy,
    /// 卖出信号
    Sell,
    /// 持有/观望信号
    Hold,
}

/// 信号事件
///
/// 包含交易信号及其相关元数据，如触发价格和时间戳。
///
/// # 字段说明
///
/// * `signal` - 交易信号类型
/// * `price` - 触发信号时的价格
/// * `timestamp` - 信号产生的时间戳
///
/// # 示例
///
/// ```rust
/// use aurora_core::{SignalEvent, Signal};
///
/// let signal_event = SignalEvent {
///     signal: Signal::Buy,
///     price: 46500.0,
///     timestamp: 1640995200000,
/// };
///
/// assert_eq!(signal_event.signal, Signal::Buy);
/// assert_eq!(signal_event.price, 46500.0);
/// ```
#[derive(Debug, Clone)]
pub struct SignalEvent {
    /// 交易信号类型
    pub signal: Signal,
    /// 触发价格
    pub price: f64,
    /// 时间戳（Unix毫秒）
    pub timestamp: i64,
}

/// 异步数据源统一接口
///
/// 定义了数据源的标准行为，使得回测引擎和实时引擎可以透明地
/// 切换不同的数据来源（如历史CSV文件或实时WebSocket流）。
///
/// # 方法
///
/// * `start()` - 启动数据源并返回事件接收器
///
/// # 实现要求
///
/// 实现此trait的类型必须是 `Send + Sync` 以支持多线程环境。
///
/// # 示例
///
/// ```rust,no_run
/// use aurora_core::{DataSource, MarketEvent};
/// use tokio::sync::mpsc::UnboundedReceiver;
/// use async_trait::async_trait;
/// use anyhow::Result;
///
/// struct MockDataSource;
///
/// #[async_trait]
/// impl DataSource for MockDataSource {
///     async fn start(&mut self) -> Result<UnboundedReceiver<MarketEvent>> {
///         let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
///         // 启动数据生成逻辑...
///         Ok(rx)
///     }
/// }
/// ```
#[async_trait]
pub trait DataSource {
    /// 启动数据源并返回事件接收器
    ///
    /// # 返回值
    ///
    /// 返回一个无界通道的接收端，用于接收 `MarketEvent`
    ///
    /// # 错误
    ///
    /// 如果数据源启动失败，返回错误信息
    async fn start(&mut self) -> anyhow::Result<UnboundedReceiver<MarketEvent>>;
}

/// 策略统一接口
///
/// 定义了交易策略的标准行为，支持事件驱动的信号生成。
/// 所有策略都必须实现此trait以与回测和实时引擎兼容。
///
/// # 方法
///
/// * `on_market_event()` - 处理市场事件并可能产生交易信号
///
/// # 实现要求
///
/// 实现此trait的类型必须是 `Send + Sync` 以支持多线程环境。
///
/// # 示例
///
/// ```rust
/// use aurora_core::{Strategy, MarketEvent, SignalEvent, Signal, Kline};
///
/// struct SimpleStrategy;
///
/// impl Strategy for SimpleStrategy {
///     fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
///         match event {
///             MarketEvent::Kline(kline) => {
///                 // 简单策略：价格上涨就买入
///                 if kline.close > kline.open {
///                     Some(SignalEvent {
///                         signal: Signal::Buy,
///                         price: kline.close,
///                         timestamp: kline.timestamp,
///                     })
///                 } else {
///                     None
///                 }
///             }
///         }
///     }
/// }
/// ```
pub trait Strategy: Send + Sync {
    /// 处理市场事件并可能产生交易信号
    ///
    /// # 参数
    ///
    /// * `event` - 市场事件引用
    ///
    /// # 返回值
    ///
    /// 如果策略决定产生交易信号，返回 `Some(SignalEvent)`；
    /// 否则返回 `None`
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试K线数据结构创建
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
        assert_eq!(kline.high, 105.0);
        assert_eq!(kline.low, 95.0);
        assert_eq!(kline.volume, 1000.0);
        assert_eq!(kline.timestamp, 1640995200000);
    }

    /// 测试K线数据克隆和相等性
    #[test]
    fn test_kline_clone_and_equality() {
        let kline1 = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };
        
        let kline2 = kline1.clone();
        assert_eq!(kline1, kline2);
        
        let kline3 = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 103.0, // 不同的收盘价
            volume: 1000.0,
        };
        
        assert_ne!(kline1, kline3);
    }

    /// 测试信号的相等性
    #[test]
    fn test_signal_equality() {
        assert_eq!(Signal::Buy, Signal::Buy);
        assert_eq!(Signal::Sell, Signal::Sell);
        assert_eq!(Signal::Hold, Signal::Hold);
        
        assert_ne!(Signal::Buy, Signal::Sell);
        assert_ne!(Signal::Buy, Signal::Hold);
        assert_ne!(Signal::Sell, Signal::Hold);
    }

    /// 测试信号事件创建
    #[test]
    fn test_signal_event_creation() {
        let signal_event = SignalEvent {
            signal: Signal::Buy,
            price: 102.0,
            timestamp: 1640995200000,
        };
        
        assert!(matches!(signal_event.signal, Signal::Buy));
        assert_eq!(signal_event.price, 102.0);
        assert_eq!(signal_event.timestamp, 1640995200000);
    }

    /// 测试市场事件创建和匹配
    #[test]
    fn test_market_event_creation() {
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
                assert_eq!(k, kline);
            }
        }
    }

    /// 测试K线数据的有效性验证
    #[test]
    fn test_kline_validity_concepts() {
        let kline = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };
        
        // 验证价格逻辑关系
        assert!(kline.high >= kline.open);
        assert!(kline.high >= kline.close);
        assert!(kline.low <= kline.open);
        assert!(kline.low <= kline.close);
        assert!(kline.volume >= 0.0);
        assert!(kline.timestamp > 0);
    }

    /// 测试信号克隆
    #[test]
    fn test_signal_clone() {
        let original = Signal::Buy;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// 测试MarketEvent克隆
    #[test]
    fn test_market_event_clone() {
        let kline = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };
        
        let event1 = MarketEvent::Kline(kline);
        let event2 = event1.clone();
        
        // 由于MarketEvent没有实现PartialEq，我们只能测试克隆是否成功
        match (&event1, &event2) {
            (MarketEvent::Kline(k1), MarketEvent::Kline(k2)) => {
                assert_eq!(k1, k2);
            }
        }
    }
}
