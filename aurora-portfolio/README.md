# Aurora Portfolio

Aurora 投资组合管理库 - 为量化交易系统提供专业的资金管理、风险控制和业绩分析

## 概述

`aurora-portfolio` 是 Aurora 量化交易框架的投资组合管理组件，提供完整的交易执行、资金管理、持仓跟踪、风险控制和业绩分析功能。它采用统一的接口设计，同时支持回测和实时交易环境，是构建量化交易系统的核心模块之一。

## 主要功能

### 💼 投资组合管理
- 现金余额管理
- 持仓数量跟踪
- 总权益实时计算
- 交易记录保存
- 权益曲线跟踪

### 📈 交易执行
- 多种订单类型支持（市价单、限价单、止损单、止盈单）
- 买入/卖出操作执行
- 订单状态管理
- 参数验证和错误处理

### 🛡️ 风险管理
- **投资组合层风控**
  - 最大回撤限制
  - 单日最大亏损限制
  - 连续亏损次数限制
  - 账户最低权益保护
- **订单层风控**
  - 止损价格设置
  - 止盈价格设置
  - 自动触发机制
- **风险监控**
  - 实时风险检查
  - 自动停止交易
  - 风险日志记录

### 💰 仓位管理
- **固定金额策略** - 每次使用固定金额交易
- **固定比例策略** - 按账户权益的固定比例分配
- **Kelly准则** - 根据胜率和盈亏比动态调整仓位
- **金字塔加仓** - 在盈利时逐步增加仓位
- 支持杠杆设置
- 最小交易金额保护

### 📊 业绩分析
- 总收益率计算
- 年化收益率
- 最大回撤分析
- 夏普比率
- 胜率统计
- 盈亏比计算

## 核心组件

### Portfolio Trait - 投资组合接口

定义了投资组合管理的标准行为：

```rust
#[async_trait]
pub trait Portfolio: Send + Sync {
    /// 执行买入操作
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade>;
    
    /// 执行卖出操作
    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade>;
    
    /// 获取总权益
    fn get_total_equity(&self, current_price: f64) -> f64;
    
    /// 获取现金余额
    fn get_cash(&self) -> f64;
    
    /// 获取持仓数量
    fn get_position(&self) -> f64;
    
    /// 获取交易记录
    fn get_trades(&self) -> &[Trade];
    
    /// 更新权益曲线
    fn update_equity(&mut self, timestamp: i64, current_price: f64);
    
    /// 获取权益曲线
    fn get_equity_curve(&self) -> &[EquityPoint];
    
    /// 计算业绩指标
    fn calculate_performance(&self, time_period_days: f64) -> PerformanceMetrics;
}
```

### BasePortfolio - 基础投资组合实现

提供投资组合管理的标准实现：

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建投资组合，初始资金 10000
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 执行买入
    let buy_trade = portfolio.execute_buy(50000.0, 1640995200000).await?;
    println!("买入: 价格={}, 数量={}", buy_trade.price, buy_trade.quantity);
    
    // 查询状态
    println!("现金: {}", portfolio.get_cash());
    println!("持仓: {}", portfolio.get_position());
    println!("总权益: {}", portfolio.get_total_equity(51000.0));
    
    // 更新权益曲线
    portfolio.update_equity(1640998800000, 51000.0);
    
    // 执行卖出
    let sell_trade = portfolio.execute_sell(52000.0, 1641002400000).await?;
    println!("卖出: 价格={}, 数量={}", sell_trade.price, sell_trade.quantity);
    
    Ok(())
}
```

**特点**:
- 全仓买卖策略
- 自动资金检查
- 交易记录追踪
- 权益曲线更新

### Trade - 交易记录

记录单次交易的完整信息：

```rust
use aurora_portfolio::{Trade, TradeSide};

// 交易记录结构
pub struct Trade {
    pub timestamp: i64,        // 交易时间戳
    pub side: TradeSide,       // 交易方向
    pub price: f64,            // 成交价格
    pub quantity: f64,         // 交易数量
    pub value: f64,            // 交易总价值
    pub fee: Option<f64>,      // 手续费（可选）
    pub note: Option<String>,  // 备注（可选）
}

// 交易方向
pub enum TradeSide {
    Buy,   // 买入
    Sell,  // 卖出
}
```

**创建交易记录**:

```rust
// 使用构建器模式
use aurora_portfolio::TradeBuilder;

let trade = TradeBuilder::new(
    TradeSide::Buy,
    50000.0,      // 价格
    0.2,          // 数量
    1640995200000 // 时间戳
)
.with_fee(5.0)
.with_note("开仓买入".to_string())
.build();

// 或者使用便捷方法
let buy_trade = Trade::new_buy(50000.0, 0.2, 1640995200000);
let sell_trade = Trade::new_sell(52000.0, 0.2, 1641002400000);
```

### PerformanceMetrics - 业绩指标

包含投资组合的各项关键业绩和风险指标：

```rust
pub struct PerformanceMetrics {
    pub total_return: f64,        // 总收益率（%）
    pub annualized_return: f64,   // 年化收益率（%）
    pub max_drawdown: f64,        // 最大回撤（%）
    pub sharpe_ratio: f64,        // 夏普比率
    pub win_rate: f64,            // 胜率（%）
    pub total_trades: usize,      // 总交易次数
    pub winning_trades: usize,    // 盈利交易次数
    pub losing_trades: usize,     // 亏损交易次数
    pub average_win: f64,         // 平均盈利
    pub average_loss: f64,        // 平均亏损
    pub profit_loss_ratio: f64,   // 盈亏比
}
```

**计算业绩指标**:

```rust
let metrics = portfolio.calculate_performance(30.0); // 30天周期

println!("总收益率: {:.2}%", metrics.total_return);
println!("年化收益率: {:.2}%", metrics.annualized_return);
println!("最大回撤: {:.2}%", metrics.max_drawdown);
println!("夏普比率: {:.2}", metrics.sharpe_ratio);
println!("胜率: {:.2}%", metrics.win_rate);
println!("盈亏比: {:.2}", metrics.profit_loss_ratio);

// 或者使用便捷方法打印完整报告
metrics.print_report();
```

### EquityPoint - 权益曲线点

记录特定时刻的投资组合权益状态：

```rust
pub struct EquityPoint {
    pub timestamp: i64,    // 时间戳
    pub equity: f64,       // 总权益
    pub drawdown: f64,     // 当前回撤（%）
}
```

**使用示例**:

```rust
// 更新权益曲线
portfolio.update_equity(1640995200000, 50000.0);
portfolio.update_equity(1640998800000, 51000.0);
portfolio.update_equity(1641002400000, 52000.0);

// 获取权益曲线
let equity_curve = portfolio.get_equity_curve();

for point in equity_curve {
    println!(
        "时间: {}, 权益: {:.2}, 回撤: {:.2}%",
        point.timestamp, point.equity, point.drawdown
    );
}
```

## 使用示例

### 基本用法 - 简单交易

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建投资组合
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 2. 执行买入
    match portfolio.execute_buy(45000.0, 1640995200000).await {
        Ok(trade) => {
            println!("✅ 买入成功");
            println!("   价格: {:.2}", trade.price);
            println!("   数量: {:.6}", trade.quantity);
            println!("   价值: {:.2}", trade.value);
        }
        Err(e) => println!("❌ 买入失败: {}", e),
    }
    
    // 3. 查询账户状态
    println!("现金余额: {:.2}", portfolio.get_cash());
    println!("持仓数量: {:.6}", portfolio.get_position());
    
    // 4. 更新权益
    portfolio.update_equity(1640998800000, 46000.0);
    
    // 5. 获取总权益
    let equity = portfolio.get_total_equity(46000.0);
    println!("总权益: {:.2}", equity);
    
    // 6. 执行卖出
    match portfolio.execute_sell(47000.0, 1641002400000).await {
        Ok(trade) => {
            println!("✅ 卖出成功");
            println!("   价格: {:.2}", trade.price);
            println!("   数量: {:.6}", trade.quantity);
            println!("   价值: {:.2}", trade.value);
        }
        Err(e) => println!("❌ 卖出失败: {}", e),
    }
    
    Ok(())
}
```

### 在回测中使用

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};
use aurora_core::Kline;

async fn run_backtest(klines: Vec<Kline>) -> anyhow::Result<()> {
    let mut portfolio = BasePortfolio::new(10000.0);
    
    for kline in klines {
        // 根据策略信号执行交易
        if should_buy(&kline) {
            if let Ok(trade) = portfolio.execute_buy(kline.close, kline.timestamp).await {
                println!("买入 @ {}", trade.price);
            }
        } else if should_sell(&kline) {
            if let Ok(trade) = portfolio.execute_sell(kline.close, kline.timestamp).await {
                println!("卖出 @ {}", trade.price);
            }
        }
        
        // 更新权益曲线
        portfolio.update_equity(kline.timestamp, kline.close);
    }
    
    // 计算业绩
    let metrics = portfolio.calculate_performance(30.0);
    metrics.print_report();
    
    Ok(())
}

fn should_buy(kline: &Kline) -> bool {
    // 策略逻辑
    true
}

fn should_sell(kline: &Kline) -> bool {
    // 策略逻辑
    false
}
```

### 在实时交易中使用

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};
use aurora_core::{MarketEvent, Signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 假设从WebSocket接收实时数据
    loop {
        let event = receive_market_event().await?;
        
        if let MarketEvent::Kline(kline) = event {
            // 策略生成信号
            let signal = generate_signal(&kline);
            
            // 执行交易
            match signal {
                Signal::Buy => {
                    if let Ok(_) = portfolio.execute_buy(
                        kline.close,
                        kline.timestamp
                    ).await {
                        println!("执行买入");
                    }
                }
                Signal::Sell => {
                    if let Ok(_) = portfolio.execute_sell(
                        kline.close,
                        kline.timestamp
                    ).await {
                        println!("执行卖出");
                    }
                }
                Signal::Hold => {
                    // 不操作
                }
            }
            
            // 更新权益
            portfolio.update_equity(kline.timestamp, kline.close);
            
            // 定期打印状态
            print_status(&portfolio, kline.close);
        }
    }
}

async fn receive_market_event() -> anyhow::Result<MarketEvent> {
    // WebSocket接收逻辑
    todo!()
}

fn generate_signal(kline: &Kline) -> Signal {
    // 策略逻辑
    Signal::Hold
}

fn print_status(portfolio: &BasePortfolio, price: f64) {
    println!("现金: {:.2}", portfolio.get_cash());
    println!("持仓: {:.6}", portfolio.get_position());
    println!("权益: {:.2}", portfolio.get_total_equity(price));
}
```

### 业绩分析

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 模拟一系列交易
    portfolio.execute_buy(45000.0, 1640995200000).await?;
    portfolio.update_equity(1640998800000, 46000.0);
    
    portfolio.execute_sell(47000.0, 1641002400000).await?;
    portfolio.update_equity(1641006000000, 47000.0);
    
    portfolio.execute_buy(46500.0, 1641009600000).await?;
    portfolio.update_equity(1641013200000, 48000.0);
    
    portfolio.execute_sell(49000.0, 1641016800000).await?;
    
    // 计算30天周期的业绩
    let metrics = portfolio.calculate_performance(30.0);
    
    // 详细分析
    println!("\n========== 业绩分析 ==========");
    println!("收益指标:");
    println!("  总收益率: {:.2}%", metrics.total_return);
    println!("  年化收益率: {:.2}%", metrics.annualized_return);
    
    println!("\n风险指标:");
    println!("  最大回撤: {:.2}%", metrics.max_drawdown);
    println!("  夏普比率: {:.2}", metrics.sharpe_ratio);
    
    println!("\n交易统计:");
    println!("  总交易: {} 次", metrics.total_trades);
    println!("  盈利: {} 次", metrics.winning_trades);
    println!("  亏损: {} 次", metrics.losing_trades);
    println!("  胜率: {:.2}%", metrics.win_rate);
    
    println!("\n盈亏分析:");
    println!("  平均盈利: {:.2}", metrics.average_win);
    println!("  平均亏损: {:.2}", metrics.average_loss);
    println!("  盈亏比: {:.2}", metrics.profit_loss_ratio);
    println!("================================\n");
    
    // 或者使用内置的报告打印
    metrics.print_report();
    
    Ok(())
}
```

### 交易记录管理

```rust
use aurora_portfolio::{Portfolio, BasePortfolio, TradeSide};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 执行多笔交易
    portfolio.execute_buy(45000.0, 1640995200000).await?;
    portfolio.execute_sell(47000.0, 1641002400000).await?;
    portfolio.execute_buy(46000.0, 1641009600000).await?;
    
    // 获取所有交易记录
    let trades = portfolio.get_trades();
    
    println!("交易历史 (共 {} 笔):", trades.len());
    println!("{:<20} {:<10} {:<12} {:<12} {:<12}", 
        "时间戳", "方向", "价格", "数量", "价值");
    println!("{}", "-".repeat(70));
    
    for trade in trades {
        let side = match trade.side {
            TradeSide::Buy => "买入",
            TradeSide::Sell => "卖出",
        };
        
        println!("{:<20} {:<10} {:<12.2} {:<12.6} {:<12.2}",
            trade.timestamp,
            side,
            trade.price,
            trade.quantity,
            trade.value
        );
    }
    
    Ok(())
}
```

## 交易规则

### 买入规则

1. **资金检查**: 现金必须足够买入至少最小单位
2. **参数验证**: 价格必须 > 0，时间戳必须 >= 0
3. **数量计算**: 使用全部现金买入（全仓模式）
4. **状态更新**: 增加持仓，清零现金
5. **记录保存**: 创建交易记录

```rust
// 买入条件
if cash > price * 0.001 {  // 至少能买0.001个单位
    let quantity = cash / price;  // 全仓买入
    // 执行交易...
}
```

### 卖出规则

1. **持仓检查**: 持仓数量必须 > 0
2. **参数验证**: 价格必须 > 0，时间戳必须 >= 0
3. **数量计算**: 卖出全部持仓（全仓模式）
4. **状态更新**: 清零持仓，增加现金
5. **记录保存**: 创建交易记录

```rust
// 卖出条件
if position > 0.0 {
    let quantity = position;  // 全仓卖出
    let value = quantity * price;
    // 执行交易...
}
```

### 交易限制

- ✅ 全仓模式：每次买卖使用全部资金/持仓
- ✅ 单向持仓：不支持做空
- ✅ 即时成交：按指定价格立即成交
- ❌ 无手续费：当前不模拟手续费（可扩展）
- ❌ 无滑点：不模拟滑点（可扩展）

## 业绩指标说明

### 收益指标

**总收益率** (Total Return):
```
总收益率 = (最终权益 - 初始权益) / 初始权益 × 100%
```

**年化收益率** (Annualized Return):
```
年化收益率 = ((最终权益 / 初始权益) ^ (1 / 年数) - 1) × 100%
```

### 风险指标

**最大回撤** (Max Drawdown):
```
回撤 = (历史最高权益 - 当前权益) / 历史最高权益 × 100%
最大回撤 = max(所有回撤值)
```

**夏普比率** (Sharpe Ratio):
```
夏普比率 = (平均收益率 - 无风险利率) / 收益率标准差
```
（当前实现假设无风险利率为0）

### 交易统计

**胜率** (Win Rate):
```
胜率 = 盈利交易次数 / 总交易次数 × 100%
```

**盈亏比** (Profit/Loss Ratio):
```
盈亏比 = 平均盈利 / |平均亏损|
```

## API 参考

### Portfolio Trait

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `execute_buy(price, timestamp)` | 执行买入 | `Result<Trade>` |
| `execute_sell(price, timestamp)` | 执行卖出 | `Result<Trade>` |
| `get_total_equity(price)` | 获取总权益 | `f64` |
| `get_cash()` | 获取现金余额 | `f64` |
| `get_position()` | 获取持仓数量 | `f64` |
| `get_trades()` | 获取交易记录 | `&[Trade]` |
| `update_equity(timestamp, price)` | 更新权益曲线 | `()` |
| `get_equity_curve()` | 获取权益曲线 | `&[EquityPoint]` |
| `calculate_performance(days)` | 计算业绩指标 | `PerformanceMetrics` |

### BasePortfolio

| 方法 | 说明 |
|------|------|
| `new(initial_cash)` | 创建新投资组合 |
| 实现了 `Portfolio` trait 的所有方法 |

### PerformanceMetrics

| 方法 | 说明 |
|------|------|
| `print_report()` | 打印完整业绩报告 |

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
serde = { version = "1.0", features = ["derive"] }
async-trait = "0.1"
tracing = "0.1"
anyhow = "1.0"
```

## 测试

```bash
# 运行所有测试
cargo test --package aurora-portfolio

# 运行集成测试
cargo test --package aurora-portfolio --test integration_tests

# 运行特定模块测试
cargo test --package aurora-portfolio portfolio::
cargo test --package aurora-portfolio trade::
cargo test --package aurora-portfolio analytics::
```

## 扩展功能

### 计划中的功能

- [ ] 分仓管理（部分买卖）
- [ ] 手续费模拟
- [ ] 滑点模拟
- [ ] 止损止盈
- [ ] 多品种持仓
- [ ] 保证金管理
- [ ] 风险控制规则
- [ ] 更多业绩指标

### 自定义实现

实现 `Portfolio` trait 以创建自定义投资组合：

```rust
use aurora_portfolio::{Portfolio, Trade};
use async_trait::async_trait;
use anyhow::Result;

pub struct CustomPortfolio {
    // 自定义字段
}

#[async_trait]
impl Portfolio for CustomPortfolio {
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        // 自定义买入逻辑
        // 例如：部分买入、动态仓位等
        todo!()
    }
    
    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        // 自定义卖出逻辑
        todo!()
    }
    
    // 实现其他方法...
}
```

## 常见问题

### Q: 为什么只支持全仓买卖？

A: 当前版本为了简化实现，采用全仓模式。分仓管理功能计划在后续版本中添加。

### Q: 如何模拟手续费？

A: `Trade` 结构已包含 `fee` 字段，可以在执行交易时设置。未来版本将自动计算手续费。

### Q: 能否同时持有多个品种？

A: 当前版本只支持单品种。多品种组合管理功能计划在后续版本中添加。

### Q: 业绩指标准确吗？

A: 业绩指标的计算遵循行业标准公式，但具体准确性取决于数据质量和更新频率。

### Q: 如何持久化交易记录？

A: 当前交易记录存储在内存中。可以通过序列化 `Trade` 结构保存到文件或数据库。

## 设计原则

1. **接口抽象**: 通过 trait 定义标准接口
2. **异步支持**: 支持异步交易执行
3. **类型安全**: 利用 Rust 类型系统
4. **错误处理**: 完善的错误返回机制
5. **可扩展**: 易于添加新功能和自定义实现

## 相关 Crate

- **aurora-core**: 核心数据结构和接口定义
- **aurora-backtester**: 使用此库进行回测
- **aurora-live**: 使用此库进行实时模拟交易
- **aurora-strategy**: 策略生成交易信号

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。

## 重构内容

根据项目约定中"高内聚、低耦合"和"组件分离"的要求，进行了以下重构：

### 1. 分离投资组合管理模块

**问题**: `aurora-core/src/lib.rs` 原本包含了两种职责：
- 核心数据结构定义（Kline、MarketEvent、Signal等）
- 投资组合管理实现（Portfolio trait、BasePortfolio等）

**解决方案**: 创建了独立的 `aurora-portfolio` crate，专门负责投资组合管理功能。

### 2. 新的 aurora-portfolio 结构

```
aurora-portfolio/
├── src/
│   ├── lib.rs          # 模块导出和文档
│   ├── portfolio.rs    # 投资组合核心逻辑
│   ├── trade.rs        # 交易记录相关结构
│   └── analytics.rs    # 业绩分析功能
└── Cargo.toml
```

#### 模块职责分工：

- **portfolio.rs**: 
  - `Portfolio` trait 定义统一接口
  - `BasePortfolio` 提供标准实现
  - 买卖操作、权益计算、风险控制

- **trade.rs**:
  - `Trade` 交易记录结构
  - `TradeSide` 交易方向枚举
  - `TradeBuilder` 构建器模式支持

- **analytics.rs**:
  - `EquityPoint` 权益曲线数据点
  - `PerformanceMetrics` 业绩指标结构
  - `PortfolioAnalytics` 分析计算工具

### 3. 移除重复代码

**问题**: `aurora-backtester/src/portfolio.rs` 与 `aurora-core` 中的投资组合代码重复定义了相同的结构体。

**解决方案**: 删除重复代码，统一使用 `aurora-portfolio` crate。

### 4. 更新依赖关系

- 在根 `Cargo.toml` 中添加 `aurora-portfolio` 成员
- 更新 `aurora-backtester` 的依赖，使用新的 portfolio crate
- 修改相关导入和函数调用

## 改进效果

### 高内聚
- 每个模块专注于单一职责
- 相关功能聚集在同一模块内
- 清晰的模块边界和接口

### 低耦合
- 通过 trait 定义抽象接口
- 减少模块间的直接依赖
- 支持不同的投资组合实现策略

### 组件分离
- 核心数据结构与业务逻辑分离
- 投资组合管理独立成专门 crate
- 便于测试、维护和扩展

### 可扩展性
- 新的 Portfolio trait 支持异步操作
- TradeBuilder 提供灵活的交易记录创建
- 详细的业绩分析功能

## 使用示例

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建投资组合
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 执行交易
    let buy_trade = portfolio.execute_buy(100.0, 1640995200000).await?;
    portfolio.update_equity(1640995260000, 105.0);
    let sell_trade = portfolio.execute_sell(105.0, 1640995320000).await?;
    
    // 分析业绩
    let metrics = portfolio.calculate_performance(1.0); // 1天
    metrics.print_report();
    
    Ok(())
}
```

## 风险管理功能

### RiskManager - 风险管理器

提供投资组合级别的风险控制功能:

```rust
use aurora_portfolio::{RiskManager, RiskRules};

// 创建风险规则
let rules = RiskRules::new()
    .with_max_drawdown(15.0)           // 最大回撤15%
    .with_max_daily_loss(5.0)          // 单日最大亏损5%
    .with_max_consecutive_losses(3)    // 最多连续亏损3次
    .with_min_equity(5000.0);          // 最低权益5000

let mut risk_manager = RiskManager::new(rules, 10000.0);

// 执行风险检查
let result = risk_manager.check_risk(9500.0, 5.0, 100.0);
if result.is_pass() {
    println!("风险检查通过,可以继续交易");
} else if let Some(reason) = result.get_reason() {
    println!("风险检查未通过: {}", reason);
}

// 记录交易结果
risk_manager.record_trade_result(false); // 记录亏损
println!("连续亏损次数: {}", risk_manager.get_consecutive_losses());

// 检查是否应停止交易
if risk_manager.should_stop_trading() {
    println!("触发风控规则,停止交易!");
}
```

### 止损止盈设置

```rust
use aurora_portfolio::RiskManager;

let mut risk_manager = RiskManager::new(RiskRules::new(), 10000.0);

// 设置止损止盈(入场价100,止损2%,止盈5%)
risk_manager.set_stop_loss_take_profit(100.0, 2.0, 5.0);

// 检查是否触发
let result = risk_manager.check_risk(10000.0, 0.0, 97.0);
if !result.is_pass() {
    println!("触发止损!");
}

let result2 = risk_manager.check_risk(10000.0, 0.0, 106.0);
if !result2.is_pass() {
    println!("触发止盈!");
}
```

### Order - 订单管理

支持多种订单类型:

```rust
use aurora_portfolio::{Order, OrderType, OrderSide};

// 创建市价买入订单
let market_order = Order::new(
    OrderType::Market,
    OrderSide::Buy,
    10.0,
    1640995200000,
);

// 创建限价卖出订单
let limit_order = Order::new(
    OrderType::Limit(105.0),
    OrderSide::Sell,
    10.0,
    1640995200000,
);

// 创建止损订单
let stop_loss_order = Order::new(
    OrderType::StopLoss(95.0),
    OrderSide::Sell,
    10.0,
    1640995200000,
);

// 创建止盈订单
let take_profit_order = Order::new(
    OrderType::TakeProfit(110.0),
    OrderSide::Sell,
    10.0,
    1640995200000,
);

// 检查订单是否应触发
if market_order.should_trigger(100.0) {
    println!("市价单立即触发");
}

if stop_loss_order.should_trigger(94.0) {
    println!("价格跌破止损价,触发止损订单");
}
```

## 仓位管理功能

### PositionManager - 仓位管理器

提供多种仓位管理策略:

#### 1. 固定金额策略

```rust
use aurora_portfolio::{PositionManager, PositionSizingStrategy};

let manager = PositionManager::new(
    PositionSizingStrategy::FixedAmount(1000.0)
);

let size = manager.calculate_position_size(10000.0, 0.0)?;
println!("建议仓位: {:.2}", size); // 总是1000
```

#### 2. 固定比例策略

```rust
let manager = PositionManager::new(
    PositionSizingStrategy::FixedPercentage(0.2) // 使用20%资金
);

let size = manager.calculate_position_size(10000.0, 0.0)?;
println!("建议仓位: {:.2}", size); // 2000 (10000 * 0.2)
```

#### 3. Kelly准则策略

```rust
let manager = PositionManager::new(
    PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,           // 胜率60%
        profit_loss_ratio: 2.0,  // 盈亏比2:1
        kelly_fraction: 0.5,     // 使用半凯利(更保守)
    }
);

let size = manager.calculate_position_size(10000.0, 0.0)?;
println!("Kelly建议仓位: {:.2}", size);
```

#### 4. 金字塔加仓策略

```rust
let manager = PositionManager::new(
    PositionSizingStrategy::Pyramid {
        initial_percentage: 0.1,  // 初始10%仓位
        profit_threshold: 5.0,    // 盈利5%时加仓
        max_percentage: 0.5,      // 最大50%仓位
        increment: 0.1,           // 每次加仓10%
    }
);

// 无盈利时
let size1 = manager.calculate_position_size(10000.0, 0.0)?;
println!("初始仓位: {:.2}", size1); // 1000 (10%)

// 盈利6%时,触发一次加仓
let size2 = manager.calculate_position_size(10000.0, 6.0)?;
println!("加仓后仓位: {:.2}", size2); // 2000 (20%)
```

#### 5. 使用杠杆

```rust
let manager = PositionManager::new(
    PositionSizingStrategy::FixedPercentage(0.5)
)
.with_max_leverage(2.0)              // 2倍杠杆
.with_min_position_value(50.0);      // 最小50单位

let size = manager.calculate_position_size(10000.0, 0.0)?;
println!("含杠杆仓位: {:.2}", size); // 10000 (50% * 2倍杠杆)
```

## 完整示例:集成风控和仓位管理

```rust
use aurora_portfolio::{
    Portfolio, BasePortfolio, RiskManager, RiskRules,
    PositionManager, PositionSizingStrategy,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建投资组合
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 2. 配置风险规则
    let risk_rules = RiskRules::new()
        .with_max_drawdown(15.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(5000.0);
    
    let mut risk_manager = RiskManager::new(risk_rules, 10000.0);
    
    // 3. 配置仓位管理
    let position_manager = PositionManager::new(
        PositionSizingStrategy::FixedPercentage(0.2)
    );
    
    // 模拟交易流程
    let current_price = 100.0;
    let current_equity = portfolio.get_total_equity(current_price);
    
    // 4. 风险检查
    let risk_result = risk_manager.check_risk(
        current_equity,
        0.0, // 当前回撤
        current_price,
    );
    
    if !risk_result.is_pass() {
        println!("风险检查未通过,停止交易");
        return Ok(());
    }
    
    // 5. 计算仓位大小
    let position_size = position_manager.calculate_position_size(
        current_equity,
        0.0, // 当前盈亏
    )?;
    
    println!("建议使用资金: {:.2}", position_size);
    
    // 6. 设置止损止盈
    risk_manager.set_stop_loss_take_profit(
        current_price,
        2.0,  // 止损2%
        5.0,  // 止盈5%
    );
    
    // 7. 执行交易
    let trade = portfolio.execute_buy(current_price, 1640995200000).await?;
    println!("买入成功: 数量={:.6}", trade.quantity);
    
    // 8. 记录交易结果(示例)
    risk_manager.record_trade_result(true); // 盈利
    
    // 9. 更新权益和检查风控
    portfolio.update_equity(1640995260000, 105.0);
    let result = risk_manager.check_risk(
        portfolio.get_total_equity(105.0),
        0.0,
        105.0,
    );
    
    if !result.is_pass() {
        println!("触发止盈,卖出!");
        portfolio.execute_sell(105.0, 1640995260000).await?;
    }
    
    Ok(())
}
```

## 后续建议

1. ✅ **风险管理**: 已实现止损止盈、回撤限制、连续亏损控制等完整风控功能
2. ✅ **仓位管理**: 已实现固定金额、固定比例、Kelly准则、金字塔加仓等多种策略
3. **多资产支持**: 扩展为支持多种资产的投资组合管理
4. **实时交易**: 为实时交易环境优化异步操作
5. **更多指标**: 添加更多业绩和风险分析指标
6. **订单簿管理**: 实现完整的订单簿和订单生命周期管理