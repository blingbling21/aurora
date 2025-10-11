# Aurora Core

Aurora 核心库 - 量化交易框架的基础抽象层

## 概述

`aurora-core` 是 Aurora 量化交易框架的核心库，提供了系统中所有组件共享的基础数据结构和统一接口。它定义了市场数据、交易信号、数据源和策略的标准抽象，使得回测引擎和实时交易引擎可以透明地切换不同的数据来源和策略实现。

## 主要功能

- **统一数据结构**: 定义标准的 K线数据格式
- **事件系统**: 提供基于事件驱动的市场数据处理机制
- **交易信号**: 标准化的买入/卖出/持有信号定义
- **接口抽象**: 定义数据源和策略的统一接口

## 核心类型

### Kline - K线数据

表示一个时间周期内的价格和成交量信息：

```rust
pub struct Kline {
    pub timestamp: i64,  // Unix时间戳（毫秒）
    pub open: f64,       // 开盘价
    pub high: f64,       // 最高价
    pub low: f64,        // 最低价
    pub close: f64,      // 收盘价
    pub volume: f64,     // 成交量
}
```

### MarketEvent - 市场事件

统一不同类型的市场数据输出：

```rust
pub enum MarketEvent {
    Kline(Kline),
    // 未来可扩展: Trade, OrderBook 等
}
```

### Signal - 交易信号

表示策略产生的交易决策：

```rust
pub enum Signal {
    Buy,   // 买入信号
    Sell,  // 卖出信号
    Hold,  // 持有/观望信号
}
```

### SignalEvent - 信号事件

包含交易信号及其元数据：

```rust
pub struct SignalEvent {
    pub signal: Signal,    // 交易信号类型
    pub price: f64,        // 触发价格
    pub timestamp: i64,    // 时间戳
}
```

## 核心接口

### DataSource - 数据源接口

定义数据源的标准行为，支持异步操作：

```rust
#[async_trait]
pub trait DataSource {
    async fn start(&mut self) -> Result<UnboundedReceiver<MarketEvent>>;
}
```

**实现示例**:

```rust
use aurora_core::{DataSource, MarketEvent};
use async_trait::async_trait;

struct CsvDataSource {
    file_path: String,
}

#[async_trait]
impl DataSource for CsvDataSource {
    async fn start(&mut self) -> Result<UnboundedReceiver<MarketEvent>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        // 读取CSV并发送事件...
        Ok(rx)
    }
}
```

### Strategy - 策略接口

定义交易策略的标准行为：

```rust
pub trait Strategy: Send + Sync {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent>;
}
```

**实现示例**:

```rust
use aurora_core::{Strategy, MarketEvent, SignalEvent, Signal};

struct SimpleStrategy;

impl Strategy for SimpleStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        match event {
            MarketEvent::Kline(kline) => {
                // 实现策略逻辑...
                Some(SignalEvent {
                    signal: Signal::Buy,
                    price: kline.close,
                    timestamp: kline.timestamp,
                })
            }
        }
    }
}
```

## 使用示例

### 创建和使用 K线数据

```rust
use aurora_core::Kline;

let kline = Kline {
    timestamp: 1640995200000, // 2022-01-01 00:00:00 UTC
    open: 46000.0,
    high: 47000.0,
    low: 45500.0,
    close: 46500.0,
    volume: 123.45,
};

println!("收盘价: {}", kline.close);
```

### 处理市场事件

```rust
use aurora_core::{MarketEvent, Kline};

let kline = Kline {
    timestamp: 1640995200000,
    open: 100.0,
    high: 105.0,
    low: 95.0,
    close: 102.0,
    volume: 1000.0,
};

let event = MarketEvent::Kline(kline);

match event {
    MarketEvent::Kline(k) => {
        println!("收到K线: 收盘价 {}", k.close);
    }
}
```

### 处理交易信号

```rust
use aurora_core::{Signal, SignalEvent};

let signal_event = SignalEvent {
    signal: Signal::Buy,
    price: 46500.0,
    timestamp: 1640995200000,
};

match signal_event.signal {
    Signal::Buy => println!("执行买入，价格: {}", signal_event.price),
    Signal::Sell => println!("执行卖出，价格: {}", signal_event.price),
    Signal::Hold => println!("继续持有"),
}
```

## 依赖项

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
async-trait = "0.1"
anyhow = "1.0"
tokio = { version = "1.0", features = ["sync"] }
```

## 设计原则

1. **接口优先**: 通过 trait 定义标准接口，确保组件的可替换性
2. **事件驱动**: 使用事件模型处理市场数据，支持异步操作
3. **类型安全**: 利用 Rust 的类型系统确保数据安全性
4. **异步支持**: 使用 async/await 和 tokio 支持高性能并发
5. **序列化友好**: 所有数据结构支持 serde 序列化

## 扩展性

`aurora-core` 设计时考虑了未来的扩展：

- **MarketEvent**: 可以添加更多事件类型，如 Trade（逐笔成交）、OrderBook（订单簿）
- **Signal**: 可以扩展支持更复杂的信号类型，如部分买卖、止损止盈等
- **接口**: DataSource 和 Strategy 接口可以支持更多实现

## 相关 Crate

- **aurora-data**: 实现了 `DataSource` 接口，提供历史数据和实时数据源
- **aurora-strategy**: 实现了 `Strategy` 接口，提供各种交易策略
- **aurora-backtester**: 使用核心接口实现回测引擎
- **aurora-live**: 使用核心接口实现实时交易引擎

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。
