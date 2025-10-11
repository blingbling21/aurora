# Aurora Strategy

Aurora 交易策略库 - 为量化交易系统提供策略框架和实现

## 概述

`aurora-strategy` 是 Aurora 量化交易框架的策略层组件，提供了统一的策略接口和常用交易策略的实现。它基于技术指标生成买卖信号，支持事件驱动的策略执行模式，适用于回测和实时交易环境。

## 主要功能

### 🎯 策略框架
- 统一的策略接口定义
- 事件驱动的执行模式
- 状态管理和持久化
- 灵活的信号生成机制

### 📊 技术分析策略
- 移动平均线交叉策略（MA Crossover）
- 更多策略正在开发中...

### 🔄 策略生命周期
- 策略创建和初始化
- 市场事件处理
- 信号生成
- 状态重置

## 核心概念

### Strategy Trait - 策略接口

所有策略都必须实现 `aurora_core::Strategy` trait：

```rust
pub trait Strategy: Send + Sync {
    /// 处理市场事件并可能产生交易信号
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent>;
}
```

**特点**:
- **事件驱动**: 响应市场数据变化
- **信号生成**: 返回买入/卖出/持有信号
- **状态管理**: 维护策略内部状态
- **线程安全**: 支持并发环境 (Send + Sync)

### 信号类型

策略可以生成三种类型的信号：

```rust
pub enum Signal {
    Buy,   // 买入信号
    Sell,  // 卖出信号
    Hold,  // 持有/观望信号
}
```

### 信号事件

包含信号及其相关元数据：

```rust
pub struct SignalEvent {
    pub signal: Signal,      // 信号类型
    pub price: f64,          // 触发价格
    pub timestamp: i64,      // 时间戳
}
```

## 策略实现

### MACrossoverStrategy - 移动平均线交叉策略

基于两条不同周期移动平均线的交叉生成交易信号的经典策略。

#### 策略原理

**金叉 (Golden Cross)** - 买入信号:
```
条件: 短期MA从下方穿越长期MA到上方
示例: MA(5) 从 < MA(20) 变为 > MA(20)
```

**死叉 (Death Cross)** - 卖出信号:
```
条件: 短期MA从上方穿越长期MA到下方
示例: MA(5) 从 > MA(20) 变为 < MA(20)
```

**持有信号**:
```
条件: 无交叉发生或数据不足
```

#### 基本使用

```rust
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Strategy, MarketEvent, Kline};

// 创建策略：5日线和20日线
let mut strategy = MACrossoverStrategy::new(5, 20);

// 处理K线数据
let kline = Kline {
    timestamp: 1640995200000,
    open: 50000.0,
    high: 50500.0,
    low: 49500.0,
    close: 50200.0,
    volume: 100.0,
};

let event = MarketEvent::Kline(kline);

// 获取交易信号
if let Some(signal_event) = strategy.on_market_event(&event) {
    match signal_event.signal {
        Signal::Buy => println!("🔔 金叉买入信号 @ {}", signal_event.price),
        Signal::Sell => println!("🔔 死叉卖出信号 @ {}", signal_event.price),
        Signal::Hold => println!("持有"),
    }
}
```

#### 策略参数

| 参数 | 说明 | 典型值 | 限制 |
|------|------|--------|------|
| `short_period` | 短期MA周期 | 5, 10, 20 | > 0 |
| `long_period` | 长期MA周期 | 20, 30, 60 | > short_period |

**参数选择建议**:
- **激进型**: (5, 20) - 信号频繁，适合短线
- **平衡型**: (10, 30) - 信号适中，较为稳健
- **保守型**: (20, 60) - 信号较少，适合长线

#### API 参考

```rust
impl MACrossoverStrategy {
    /// 创建新策略
    pub fn new(short_period: usize, long_period: usize) -> Self;
    
    /// 获取短期周期
    pub fn short_period(&self) -> usize;
    
    /// 获取长期周期
    pub fn long_period(&self) -> usize;
    
    /// 获取短期MA当前值
    pub fn short_ma_value(&self) -> Option<f64>;
    
    /// 获取长期MA当前值
    pub fn long_ma_value(&self) -> Option<f64>;
    
    /// 检查策略是否准备好
    pub fn is_ready(&self) -> bool;
    
    /// 重置策略状态
    pub fn reset(&mut self);
}
```

## 使用示例

### 基本示例

```rust
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Strategy, MarketEvent, Kline, Signal};

fn main() {
    // 创建策略
    let mut strategy = MACrossoverStrategy::new(5, 20);
    
    // 模拟K线数据
    let prices = vec![
        100.0, 102.0, 101.0, 103.0, 105.0,  // 前5个点
        104.0, 106.0, 108.0, 107.0, 109.0,  // 可能产生信号
        110.0, 108.0, 107.0, 105.0, 103.0,  // 趋势变化
    ];
    
    for (i, price) in prices.iter().enumerate() {
        let kline = Kline {
            timestamp: 1640995200000 + (i as i64 * 60000),
            open: *price,
            high: *price + 1.0,
            low: *price - 1.0,
            close: *price,
            volume: 100.0,
        };
        
        let event = MarketEvent::Kline(kline);
        
        if let Some(signal_event) = strategy.on_market_event(&event) {
            match signal_event.signal {
                Signal::Buy => {
                    println!("#{} 买入信号 @ {:.2}", i + 1, signal_event.price);
                    println!("   短期MA: {:.2}", strategy.short_ma_value().unwrap());
                    println!("   长期MA: {:.2}", strategy.long_ma_value().unwrap());
                }
                Signal::Sell => {
                    println!("#{} 卖出信号 @ {:.2}", i + 1, signal_event.price);
                    println!("   短期MA: {:.2}", strategy.short_ma_value().unwrap());
                    println!("   长期MA: {:.2}", strategy.long_ma_value().unwrap());
                }
                _ => {}
            }
        }
    }
}
```

### 在回测中使用

```rust
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Strategy, MarketEvent, Signal};
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建策略和投资组合
    let mut strategy = MACrossoverStrategy::new(10, 30);
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 加载历史K线数据
    let klines = load_historical_data("btc_1h.csv")?;
    
    println!("开始回测...");
    
    for kline in klines {
        let event = MarketEvent::Kline(kline.clone());
        
        // 策略生成信号
        if let Some(signal_event) = strategy.on_market_event(&event) {
            match signal_event.signal {
                Signal::Buy => {
                    // 执行买入
                    if let Ok(trade) = portfolio.execute_buy(
                        signal_event.price,
                        signal_event.timestamp
                    ).await {
                        println!("✅ 买入 @ {:.2}", trade.price);
                    }
                }
                Signal::Sell => {
                    // 执行卖出
                    if let Ok(trade) = portfolio.execute_sell(
                        signal_event.price,
                        signal_event.timestamp
                    ).await {
                        println!("✅ 卖出 @ {:.2}", trade.price);
                    }
                }
                Signal::Hold => {}
            }
        }
        
        // 更新权益曲线
        portfolio.update_equity(kline.timestamp, kline.close);
    }
    
    // 输出回测结果
    let metrics = portfolio.calculate_performance(30.0);
    metrics.print_report();
    
    Ok(())
}

fn load_historical_data(path: &str) -> anyhow::Result<Vec<Kline>> {
    // 从CSV加载数据的逻辑
    todo!()
}
```

### 在实时交易中使用

```rust
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Strategy, MarketEvent, Signal};
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化策略和投资组合
    let mut strategy = MACrossoverStrategy::new(10, 30);
    let mut portfolio = BasePortfolio::new(10000.0);
    
    println!("🚀 启动实时交易...");
    
    // 连接到实时数据流
    let mut stream = connect_to_market_stream().await?;
    
    while let Some(event) = stream.next().await {
        // 策略处理市场事件
        if let Some(signal_event) = strategy.on_market_event(&event) {
            match signal_event.signal {
                Signal::Buy => {
                    println!("📈 买入信号");
                    
                    if let Ok(trade) = portfolio.execute_buy(
                        signal_event.price,
                        signal_event.timestamp
                    ).await {
                        send_notification(&format!(
                            "买入成功 @ {:.2}",
                            trade.price
                        ));
                    }
                }
                Signal::Sell => {
                    println!("📉 卖出信号");
                    
                    if let Ok(trade) = portfolio.execute_sell(
                        signal_event.price,
                        signal_event.timestamp
                    ).await {
                        send_notification(&format!(
                            "卖出成功 @ {:.2}",
                            trade.price
                        ));
                    }
                }
                Signal::Hold => {}
            }
        }
        
        // 定期打印状态
        print_status(&strategy, &portfolio);
    }
    
    Ok(())
}

async fn connect_to_market_stream() -> anyhow::Result<MarketStream> {
    // WebSocket连接逻辑
    todo!()
}

fn send_notification(message: &str) {
    println!("🔔 {}", message);
}

fn print_status(strategy: &MACrossoverStrategy, portfolio: &BasePortfolio) {
    println!("策略状态:");
    println!("  短期MA: {:?}", strategy.short_ma_value());
    println!("  长期MA: {:?}", strategy.long_ma_value());
    println!("  就绪: {}", strategy.is_ready());
    println!("账户状态:");
    println!("  现金: {:.2}", portfolio.get_cash());
    println!("  持仓: {:.6}", portfolio.get_position());
}
```

### 参数优化

```rust
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Strategy, MarketEvent};

fn optimize_parameters(klines: &[Kline]) -> (usize, usize, f64) {
    let mut best_params = (0, 0);
    let mut best_return = f64::MIN;
    
    // 遍历不同的参数组合
    for short in (5..=20).step_by(5) {
        for long in (20..=60).step_by(10) {
            if short >= long {
                continue;
            }
            
            // 测试这组参数
            let return_rate = backtest_with_params(klines, short, long);
            
            println!("测试参数 {}:{} - 收益: {:.2}%", short, long, return_rate * 100.0);
            
            if return_rate > best_return {
                best_return = return_rate;
                best_params = (short, long);
            }
        }
    }
    
    println!("\n最佳参数: {}:{}", best_params.0, best_params.1);
    println!("最佳收益: {:.2}%", best_return * 100.0);
    
    (best_params.0, best_params.1, best_return)
}

fn backtest_with_params(klines: &[Kline], short: usize, long: usize) -> f64 {
    let mut strategy = MACrossoverStrategy::new(short, long);
    let initial_capital = 10000.0;
    let mut equity = initial_capital;
    
    // 简化的回测逻辑
    for kline in klines {
        let event = MarketEvent::Kline(kline.clone());
        if let Some(signal_event) = strategy.on_market_event(&event) {
            // 模拟交易执行
            match signal_event.signal {
                Signal::Buy => {
                    // 买入逻辑
                }
                Signal::Sell => {
                    // 卖出逻辑
                }
                _ => {}
            }
        }
    }
    
    (equity - initial_capital) / initial_capital
}
```

### 策略状态管理

```rust
use aurora_strategy::MACrossoverStrategy;

fn main() {
    let mut strategy = MACrossoverStrategy::new(10, 30);
    
    // 检查策略是否准备好
    if !strategy.is_ready() {
        println!("⏳ 策略正在预热...");
    }
    
    // 获取策略信息
    println!("策略参数:");
    println!("  短期周期: {}", strategy.short_period());
    println!("  长期周期: {}", strategy.long_period());
    
    // 处理数据...
    
    // 获取当前指标值
    if let Some(short_ma) = strategy.short_ma_value() {
        println!("短期MA: {:.2}", short_ma);
    }
    
    if let Some(long_ma) = strategy.long_ma_value() {
        println!("长期MA: {:.2}", long_ma);
    }
    
    // 检查就绪状态
    if strategy.is_ready() {
        println!("✅ 策略已准备好");
    }
    
    // 重置策略（如果需要）
    strategy.reset();
    println!("🔄 策略已重置");
}
```

## 策略开发指南

### 创建自定义策略

实现 `Strategy` trait 以创建自定义策略：

```rust
use aurora_core::{Strategy, MarketEvent, SignalEvent, Signal, Kline};
use aurora_indicators::{RSI, MACD};

/// RSI + MACD 组合策略
pub struct RsiMacdStrategy {
    rsi: RSI,
    macd: MACD,
    rsi_oversold: f64,
    rsi_overbought: f64,
}

impl RsiMacdStrategy {
    pub fn new(rsi_period: usize) -> Self {
        Self {
            rsi: RSI::new(rsi_period),
            macd: MACD::default(),
            rsi_oversold: 30.0,
            rsi_overbought: 70.0,
        }
    }
}

impl Strategy for RsiMacdStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        match event {
            MarketEvent::Kline(kline) => {
                // 更新指标
                let rsi_value = self.rsi.update(kline.close)?;
                let macd_output = self.macd.update(kline.close);
                
                // 生成信号
                let signal = if rsi_value < self.rsi_oversold 
                    && macd_output.histogram > 0.0 {
                    Signal::Buy  // RSI超卖 + MACD金叉
                } else if rsi_value > self.rsi_overbought 
                    && macd_output.histogram < 0.0 {
                    Signal::Sell  // RSI超买 + MACD死叉
                } else {
                    Signal::Hold
                };
                
                if !matches!(signal, Signal::Hold) {
                    Some(SignalEvent {
                        signal,
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
```

### 策略设计建议

#### 1. 状态管理

```rust
pub struct MyStrategy {
    // 指标实例
    indicators: Vec<Box<dyn Indicator>>,
    
    // 历史值（用于比较）
    prev_values: HashMap<String, f64>,
    
    // 配置参数
    config: StrategyConfig,
}
```

#### 2. 参数验证

```rust
impl MyStrategy {
    pub fn new(param1: usize, param2: f64) -> Self {
        assert!(param1 > 0, "参数1必须大于0");
        assert!(param2 > 0.0 && param2 < 1.0, "参数2必须在0-1之间");
        
        // 初始化...
    }
}
```

#### 3. 信号过滤

```rust
fn should_generate_signal(&self, signal: Signal) -> bool {
    match signal {
        Signal::Buy => {
            // 检查是否已有持仓
            // 检查是否满足额外条件
            true
        }
        Signal::Sell => {
            // 检查是否有持仓可卖
            // 检查止损止盈条件
            true
        }
        Signal::Hold => false,
    }
}
```

#### 4. 错误处理

```rust
impl Strategy for MyStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        // 安全地处理可能的错误
        let result = match self.calculate_indicators(event) {
            Ok(indicators) => indicators,
            Err(e) => {
                eprintln!("指标计算错误: {}", e);
                return None;
            }
        };
        
        // 继续处理...
    }
}
```

## 性能考虑

### 计算效率

- ✅ 使用增量计算的指标（如EMA）
- ✅ 避免重复计算
- ✅ 缓存中间结果
- ❌ 避免在事件处理中进行耗时操作

### 内存使用

```rust
// 好的做法：使用滑动窗口
pub struct EfficientStrategy {
    ma: MA,  // 只存储必要的数据
}

// 避免：存储所有历史数据
pub struct InefficientStrategy {
    all_prices: Vec<f64>,  // 随时间增长
}
```

### 并发安全

策略必须是 `Send + Sync`：

```rust
pub struct ThreadSafeStrategy {
    // 使用线程安全的类型
    indicator: Arc<Mutex<SomeIndicator>>,
}
```

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_strategy_creation() {
        let strategy = MACrossoverStrategy::new(5, 20);
        assert_eq!(strategy.short_period(), 5);
        assert_eq!(strategy.long_period(), 20);
        assert!(!strategy.is_ready());
    }
    
    #[test]
    fn test_golden_cross() {
        let mut strategy = MACrossoverStrategy::new(2, 5);
        
        // 构造会产生金叉的数据
        let prices = vec![100.0, 100.0, 100.0, 100.0, 100.0, 105.0, 110.0];
        
        let mut signals = vec![];
        for (i, price) in prices.iter().enumerate() {
            let kline = create_test_kline(*price, i as i64);
            let event = MarketEvent::Kline(kline);
            
            if let Some(signal_event) = strategy.on_market_event(&event) {
                signals.push(signal_event.signal);
            }
        }
        
        // 验证产生了买入信号
        assert!(signals.contains(&Signal::Buy));
    }
    
    fn create_test_kline(price: f64, timestamp: i64) -> Kline {
        Kline {
            timestamp,
            open: price,
            high: price,
            low: price,
            close: price,
            volume: 100.0,
        }
    }
}
```

### 运行测试

```bash
# 运行所有测试
cargo test --package aurora-strategy

# 运行特定测试
cargo test --package aurora-strategy test_golden_cross

# 显示测试输出
cargo test --package aurora-strategy -- --nocapture
```

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
aurora-indicators = { path = "../aurora-indicators" }

[dev-dependencies]
approx = "0.5"
tokio = { version = "1.0", features = ["full"] }
```

## 常见问题

### Q: 策略什么时候会产生信号？

A: 只有当检测到明确的买入或卖出条件时才会产生信号。Hold信号通常不会返回SignalEvent。

### Q: 为什么策略初期没有信号？

A: 指标需要足够的数据点才能计算有效值。例如MA(20)需要至少20个数据点。

### Q: 如何避免频繁交易？

A: 可以添加额外的过滤条件，如最小持有时间、价格变化阈值等。

### Q: 策略可以同时使用多个指标吗？

A: 可以。在自定义策略中组合多个指标实例即可。

### Q: 如何处理数据缺失？

A: 使用 `Option` 类型安全地处理可能缺失的指标值。

## 相关 Crate

- **aurora-core**: 核心数据结构和Strategy trait定义
- **aurora-indicators**: 技术指标库
- **aurora-backtester**: 使用策略进行回测
- **aurora-live**: 使用策略进行实时交易
- **aurora-portfolio**: 执行策略生成的交易信号

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。