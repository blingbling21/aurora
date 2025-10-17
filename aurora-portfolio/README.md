# Aurora Portfolio

Aurora 投资组合管理库 - 为量化交易系统提供专业的资金管理、风险控制和业绩分析

## 概述

`aurora-portfolio` 是 Aurora 量化交易框架的投资组合管理组件，提供完整的交易执行、资金管理、持仓跟踪、风险控制和业绩分析功能。它采用统一的接口设计，同时支持回测和实时交易环境，是构建量化交易系统的核心模块之一。

## 架构设计

```
aurora-portfolio/
├── portfolio.rs         # 投资组合核心接口和实现
├── trade.rs            # 交易记录数据结构
├── analytics.rs        # 业绩分析和指标计算
├── broker.rs           # 经纪商统一接口
├── paper_broker.rs     # 模拟交易经纪商实现
├── order.rs            # 订单类型和状态管理
├── order_book.rs       # 订单簿和撮合引擎
├── risk_manager.rs     # 风险管理和风控规则
├── position_manager.rs # 仓位管理和资金分配
└── fees.rs             # 手续费和滑点模型
```

## 主要功能

### 💼 投资组合管理 (Portfolio)

提供统一的投资组合管理接口，支持异步操作。

**核心功能**:
- 现金余额管理
- 持仓数量跟踪
- 总权益实时计算
- 交易记录保存
- 权益曲线跟踪

**实现类**:
- `Portfolio` trait - 定义标准接口
- `BasePortfolio` - 基础实现（全仓模式）

### 📈 订单管理 (Order)

支持多种订单类型和完整的订单生命周期管理。

**订单类型 (OrderType)**:
- `Market` - 市价单，立即以市场价成交
- `Limit(price)` - 限价单，指定价格触发
- `StopLoss(price)` - 止损单，价格跌破时触发卖出
- `TakeProfit(price)` - 止盈单，价格涨至时触发卖出

**订单状态 (OrderStatus)**:
- `Pending` - 待执行
- `Triggered` - 已触发
- `Executed` - 已执行
- `Cancelled` - 已取消
- `Expired` - 已过期

**订单方向 (OrderSide)**:
- `Buy` - 买入
- `Sell` - 卖出

### 🏦 经纪商抽象 (Broker)

提供统一的经纪商接口，隔离模拟交易和实盘交易的实现细节。

**Broker trait 主要方法**:
- `submit_order()` - 提交订单
- `cancel_order()` - 取消订单
- `get_order_status()` - 查询订单状态
- `get_balance()` - 查询余额
- `get_position()` - 查询持仓
- `update_market_price()` - 更新市场价格（触发订单）

**PaperBroker - 模拟交易经纪商**:
- 完整的订单簿模拟
- 自动撮合和执行
- 手续费和滑点计算
- 多交易对支持
- 余额和持仓管理

### 📚 订单簿和撮合引擎 (OrderBook)

完整的订单簿实现和自动撮合机制。

**OrderBook - 订单簿**:
- 买单簿和卖单簿分离管理
- 按价格和时间优先排序
- 止损单独立列表
- 订单索引快速查询

**MatchingEngine - 撮合引擎**:
- 多交易对订单簿管理
- 自动撮合限价单
- 价格触发机制
- 市价单即时执行
- 完整的成交记录生成

### 🛡️ 风险管理 (RiskManager)

多层次的风险控制体系。

**风险规则 (RiskRules)**:

*投资组合级别*:
- `max_drawdown_pct` - 最大回撤限制（如 15.0 表示 15%）
- `max_daily_loss_pct` - 单日最大亏损限制
- `max_consecutive_losses` - 连续亏损次数限制
- `max_single_trade_loss_pct` - 单笔最大亏损限制
- `min_equity` - 账户最低权益要求

*持仓级别*:
- `stop_loss_price` - 止损价格
- `take_profit_price` - 止盈价格

**风险检查 (RiskManager)**:
- `check_risk()` - 执行风险检查
- `record_trade_result()` - 记录交易结果
- `should_stop_trading()` - 判断是否停止交易
- `set_stop_loss_take_profit()` - 设置止损止盈

**风险检查结果 (RiskCheckResult)**:
- `Pass` - 通过检查
- `StopLoss` - 触发止损
- `TakeProfit` - 触发止盈
- `MaxDrawdownReached` - 达到最大回撤
- `MaxDailyLossReached` - 达到单日最大亏损
- `MaxConsecutiveLossesReached` - 达到连续亏损限制
- `MinEquityBreached` - 低于最低权益

### 💰 仓位管理 (PositionManager)

多种仓位管理策略，科学分配交易资金。

**仓位策略 (PositionSizingStrategy)**:

1. **固定金额 (FixedAmount)**
   ```rust
   PositionSizingStrategy::FixedAmount(1000.0)
   ```
   每次交易使用固定金额，适合初学者。

2. **固定比例 (FixedPercentage)**
   ```rust
   PositionSizingStrategy::FixedPercentage(0.2) // 20%
   ```
   按账户权益的固定比例分配，随账户增长。

3. **Kelly 准则 (KellyCriterion)**
   ```rust
   PositionSizingStrategy::KellyCriterion {
       win_rate: 0.6,              // 胜率 60%
       profit_loss_ratio: 2.0,     // 盈亏比 2:1
       kelly_fraction: 0.5,        // 半凯利系数
   }
   ```
   根据胜率和盈亏比科学计算最优仓位，推荐使用半凯利降低风险。

4. **金字塔加仓 (Pyramid)**
   ```rust
   PositionSizingStrategy::Pyramid {
       initial_percentage: 0.1,    // 初始 10%
       profit_threshold: 5.0,      // 盈利 5% 触发加仓
       max_percentage: 0.5,        // 最大 50%
       increment: 0.1,             // 每次加 10%
   }
   ```
   在盈利时逐步增加仓位，顺势而为。

5. **全仓 (AllIn)**
   ```rust
   PositionSizingStrategy::AllIn
   ```
   使用全部资金，风险极高，不推荐。

**PositionManager 配置**:
- `with_min_position_value()` - 设置最小交易金额
- `with_max_leverage()` - 设置最大杠杆倍数
- `calculate_position_size()` - 计算建议仓位

### 💸 交易成本模拟 (Fees & Slippage)

真实模拟交易成本，提高回测准确性。

**手续费模型 (FeeModel)**:

1. **固定金额**
   ```rust
   FeeModel::Fixed(5.0) // 每笔交易固定 5 元
   ```

2. **百分比**
   ```rust
   FeeModel::Percentage(0.1) // 0.1% 手续费
   ```

3. **分层费率**
   ```rust
   FeeModel::Tiered(vec![
       (1000.0, 0.1),   // <1000: 0.1%
       (10000.0, 0.08), // 1000-10000: 0.08%
       (f64::MAX, 0.05) // >10000: 0.05%
   ])
   ```

4. **Maker-Taker**
   ```rust
   FeeModel::MakerTaker {
       maker_fee: 0.05,  // Maker 0.05%
       taker_fee: 0.1,   // Taker 0.1%
   }
   ```

5. **无手续费**
   ```rust
   FeeModel::None
   ```

**滑点模型 (SlippageModel)**:

1. **固定滑点**
   ```rust
   SlippageModel::Fixed(0.5) // 固定滑点 0.5
   ```

2. **百分比滑点**
   ```rust
   SlippageModel::Percentage(0.05) // 0.05% 滑点
   ```

3. **基于成交量**
   ```rust
   SlippageModel::VolumeBased {
       base_slippage: 0.05,        // 基础滑点
       volume_coefficient: 0.5,     // 成交量系数
       reference_volume: 1000.0,    // 参考成交量
   }
   ```
   交易量越大，滑点越大。

4. **基于波动率**
   ```rust
   SlippageModel::VolatilityBased {
       base_slippage: 0.05,              // 基础滑点
       volatility_coefficient: 2.0,      // 波动率系数
   }
   ```
   市场波动越大，滑点越大。

5. **综合动态**
   ```rust
   SlippageModel::Dynamic {
       base_slippage: 0.05,
       volume_coefficient: 0.5,
       reference_volume: 1000.0,
       volatility_coefficient: 2.0,
   }
   ```
   同时考虑成交量和波动率。

6. **无滑点**
   ```rust
   SlippageModel::None
   ```

**TradeCostCalculator - 成本计算器**:
- `calculate_buy_cost()` - 计算买入成本
- `calculate_sell_proceeds()` - 计算卖出收益
- 自动应用手续费和滑点

### 📊 业绩分析 (Analytics)

全面的投资组合业绩评估。

**业绩指标 (PerformanceMetrics)**:

*收益指标*:
- `total_return` - 总收益率（%）
- `annualized_return` - 年化收益率（%）

*风险指标*:
- `max_drawdown` - 最大回撤（%）
- `sharpe_ratio` - 夏普比率

*交易统计*:
- `total_trades` - 总交易次数
- `winning_trades` - 盈利交易次数
- `losing_trades` - 亏损交易次数
- `win_rate` - 胜率（%）
- `average_win` - 平均盈利
- `average_loss` - 平均亏损
- `profit_loss_ratio` - 盈亏比

**权益曲线 (EquityPoint)**:
- `timestamp` - 时间戳
- `equity` - 权益值
- `drawdown` - 回撤百分比

**PortfolioAnalytics - 分析工具**:
- `calculate_metrics()` - 计算业绩指标
- `calculate_max_drawdown()` - 计算最大回撤
- `calculate_sharpe_ratio()` - 计算夏普比率
- 支持批量交易分析

### 📝 交易记录 (Trade)

完整的交易信息记录。

**Trade 结构**:
- `timestamp` - 交易时间戳
- `side` - 交易方向 (Buy/Sell)
- `price` - 成交价格
- `quantity` - 交易数量
- `value` - 交易总价值
- `fee` - 手续费（可选）
- `note` - 备注（可选）

**TradeSide 枚举**:
- `Buy` - 买入
- `Sell` - 卖出

**TradeBuilder - 构建器**:
```rust
TradeBuilder::new(TradeSide::Buy, 100.0, 10.0, 1640995200000)
    .with_fee(5.0)
    .with_note("开仓买入".to_string())
    .build()
```

## 使用示例

### 1. 基础投资组合使用

```rust
use aurora_portfolio::{Portfolio, BasePortfolio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建投资组合，初始资金 10000
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 执行买入操作
    let trade = portfolio.execute_buy(100.0, 1640995200000).await?;
    println!("买入: 价格={}, 数量={}", trade.price, trade.quantity);
    
    // 查询账户状态
    println!("现金: {}", portfolio.get_cash());
    println!("持仓: {}", portfolio.get_position());
    println!("总权益: {}", portfolio.get_total_equity(105.0));
    
    // 更新权益曲线
    portfolio.update_equity(1640995260000, 105.0);
    
    // 执行卖出操作
    let trade = portfolio.execute_sell(105.0, 1640995320000).await?;
    println!("卖出: 价格={}, 数量={}", trade.price, trade.quantity);
    
    // 计算业绩
    let metrics = portfolio.calculate_performance(1.0);
    metrics.print_report();
    
    Ok(())
}
```

### 2. 使用模拟经纪商 (PaperBroker)

```rust
use aurora_portfolio::{PaperBroker, Broker, Order, OrderType, OrderSide};
use aurora_portfolio::{FeeModel, SlippageModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建模拟经纪商
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .with_fee_model(FeeModel::Percentage(0.1))
        .with_slippage_model(SlippageModel::Percentage(0.05));
    
    // 设置市场价格
    broker.update_market_price("BTC/USDT", 50000.0, 1640995200000).await?;
    
    // 提交限价买单
    let order = Order::new(
        OrderType::Limit(49000.0),
        OrderSide::Buy,
        0.1,
        1640995200000,
    );
    let order_id = broker.submit_order("BTC/USDT", order).await?;
    println!("订单已提交: {}", order_id);
    
    // 价格下跌，触发订单
    let trades = broker.update_market_price("BTC/USDT", 49000.0, 1640995260000).await?;
    if !trades.is_empty() {
        println!("订单已成交: {} 笔", trades.len());
    }
    
    // 查询账户状态
    let balance = broker.get_balance("USDT").await?;
    let position = broker.get_position("BTC/USDT").await?;
    println!("USDT 余额: {}", balance);
    println!("BTC 持仓: {}", position);
    
    Ok(())
}
```

### 3. 风险管理

```rust
use aurora_portfolio::{RiskManager, RiskRules};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建风险规则
    let rules = RiskRules::new()
        .with_max_drawdown(15.0)           // 最大回撤 15%
        .with_max_consecutive_losses(3)    // 最多连续亏损 3 次
        .with_min_equity(5000.0);          // 最低权益 5000
    
    let mut risk_manager = RiskManager::new(rules, 10000.0);
    
    // 设置止损止盈
    risk_manager.set_stop_loss_take_profit(
        100.0,  // 入场价
        2.0,    // 止损 2%
        5.0,    // 止盈 5%
    );
    
    // 执行风险检查
    let result = risk_manager.check_risk(9500.0, 5.0, 97.0);
    
    if result.is_pass() {
        println!("风险检查通过，可以继续交易");
    } else if let Some(reason) = result.get_reason() {
        println!("风险检查未通过: {}", reason);
    }
    
    // 记录交易结果
    risk_manager.record_trade_result(true); // 盈利
    risk_manager.record_trade_result(false); // 亏损
    
    // 检查是否应停止交易
    if risk_manager.should_stop_trading() {
        println!("触发风控规则，停止交易！");
    }
    
    Ok(())
}
```

### 4. 仓位管理

```rust
use aurora_portfolio::{PositionManager, PositionSizingStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 Kelly 准则
    let manager = PositionManager::new(
        PositionSizingStrategy::KellyCriterion {
            win_rate: 0.6,
            profit_loss_ratio: 2.0,
            kelly_fraction: 0.5,
        }
    )
    .with_max_leverage(2.0)
    .with_min_position_value(50.0);
    
    // 计算建议仓位
    let current_equity = 10000.0;
    let current_profit = 0.0;
    let position_size = manager.calculate_position_size(current_equity, current_profit)?;
    
    println!("建议仓位: {:.2}", position_size);
    
    Ok(())
}
```

### 5. 手续费和滑点

```rust
use aurora_portfolio::{TradeCostCalculator, FeeModel, SlippageModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建成本计算器
    let calculator = TradeCostCalculator::new(
        FeeModel::Percentage(0.1),
        SlippageModel::VolumeBased {
            base_slippage: 0.05,
            volume_coefficient: 0.5,
            reference_volume: 1000.0,
        }
    );
    
    // 计算买入成本
    let cost = calculator.calculate_buy_cost(
        100.0,    // 价格
        10.0,     // 数量
        10.0,     // 成交量
        0.02,     // 波动率
        false,    // 是否 Maker
    );
    
    println!("原始价格: {:.2}", cost.original_price);
    println!("滑点: {:.2}", cost.slippage);
    println!("实际成交价: {:.2}", cost.executed_price);
    println!("手续费: {:.2}", cost.fee);
    println!("总成本: {:.2}", cost.total_cost);
    
    Ok(())
}
```

### 6. 完整示例：集成所有功能

```rust
use aurora_portfolio::{
    PaperBroker, Broker, Order, OrderType, OrderSide,
    RiskManager, RiskRules, PositionManager, PositionSizingStrategy,
    FeeModel, SlippageModel,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建模拟经纪商
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .with_fee_model(FeeModel::Percentage(0.1))
        .with_slippage_model(SlippageModel::Percentage(0.05));
    
    // 2. 配置风险管理
    let risk_rules = RiskRules::new()
        .with_max_drawdown(15.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(5000.0);
    let mut risk_manager = RiskManager::new(risk_rules, 10000.0);
    
    // 3. 配置仓位管理
    let position_manager = PositionManager::new(
        PositionSizingStrategy::FixedPercentage(0.2)
    );
    
    // 4. 设置市场价格
    broker.update_market_price("BTC/USDT", 50000.0, 1640995200000).await?;
    
    // 5. 计算仓位大小
    let current_equity = 10000.0;
    let position_size = position_manager.calculate_position_size(current_equity, 0.0)?;
    println!("建议使用资金: {:.2}", position_size);
    
    // 6. 风险检查
    let risk_result = risk_manager.check_risk(current_equity, 0.0, 50000.0);
    if !risk_result.is_pass() {
        println!("风险检查未通过，停止交易");
        return Ok(());
    }
    
    // 7. 设置止损止盈
    risk_manager.set_stop_loss_take_profit(50000.0, 2.0, 5.0);
    
    // 8. 提交订单
    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        position_size / 50000.0,
        1640995200000,
    );
    let order_id = broker.submit_order("BTC/USDT", order).await?;
    println!("订单已提交: {}", order_id);
    
    // 9. 模拟价格变化
    broker.update_market_price("BTC/USDT", 52000.0, 1640995260000).await?;
    
    // 10. 检查止盈
    let check = risk_manager.check_risk(10400.0, 0.0, 52000.0);
    if !check.is_pass() {
        println!("触发止盈，卖出！");
        let sell_order = Order::new(
            OrderType::Market,
            OrderSide::Sell,
            position_size / 50000.0,
            1640995260000,
        );
        broker.submit_order("BTC/USDT", sell_order).await?;
    }
    
    Ok(())
}
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

### Broker Trait

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `submit_order(symbol, order)` | 提交订单 | `Result<String>` |
| `cancel_order(symbol, order_id)` | 取消订单 | `Result<()>` |
| `get_order_status(symbol, order_id)` | 查询订单状态 | `Result<OrderStatus>` |
| `get_balance(asset)` | 查询余额 | `Result<f64>` |
| `get_position(symbol)` | 查询持仓 | `Result<f64>` |
| `update_market_price(symbol, price, time)` | 更新市场价格 | `Result<Vec<Trade>>` |

### RiskManager

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `new(rules, initial_equity)` | 创建风险管理器 | `Self` |
| `check_risk(equity, drawdown, price)` | 执行风险检查 | `RiskCheckResult` |
| `record_trade_result(is_win)` | 记录交易结果 | `()` |
| `should_stop_trading()` | 判断是否停止交易 | `bool` |
| `set_stop_loss_take_profit(entry, sl%, tp%)` | 设置止损止盈 | `()` |

### PositionManager

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `new(strategy)` | 创建仓位管理器 | `Self` |
| `with_min_position_value(min)` | 设置最小仓位 | `Self` |
| `with_max_leverage(leverage)` | 设置最大杠杆 | `Self` |
| `calculate_position_size(equity, profit)` | 计算仓位大小 | `Result<f64>` |

### TradeCostCalculator

| 方法 | 说明 | 返回值 |
|------|------|--------|
| `new(fee_model, slippage_model)` | 创建成本计算器 | `Self` |
| `calculate_buy_cost(...)` | 计算买入成本 | `TradeCost` |
| `calculate_sell_proceeds(...)` | 计算卖出收益 | `TradeCost` |

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
serde = { version = "1.0", features = ["derive"] }
async-trait = "0.1"
tracing = "0.1"
anyhow = "1.0"
uuid = { version = "1.0", features = ["v4"] }

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

## 测试

```bash
# 运行所有测试
cargo test --package aurora-portfolio

# 运行集成测试
cargo test --package aurora-portfolio --test integration_tests

# 运行特定模块测试
cargo test --package aurora-portfolio portfolio::
cargo test --package aurora-portfolio broker::
cargo test --package aurora-portfolio risk_manager::
```

## 设计原则

1. **接口抽象** - 通过 trait 定义标准接口，支持多种实现
2. **异步支持** - 完整的异步操作支持，适应实时交易需求
3. **类型安全** - 充分利用 Rust 类型系统保证安全性
4. **错误处理** - 使用 `anyhow` 提供清晰的错误信息
5. **可扩展性** - 模块化设计，易于添加新功能
6. **高内聚低耦合** - 每个模块职责单一，模块间松耦合

## 相关 Crate

- **aurora-core** - 核心数据结构和接口定义
- **aurora-backtester** - 使用此库进行历史数据回测
- **aurora-live** - 使用此库进行实时模拟交易
- **aurora-strategy** - 策略生成交易信号
- **aurora-indicators** - 技术指标计算

## 版本

当前版本: **0.1.0**

## 许可证

本项目采用 Apache License 2.0 许可证。详见 [LICENSE](../LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 常见问题

### Q: Portfolio 和 Broker 有什么区别？

A: `Portfolio` 专注于投资组合状态管理（现金、持仓、权益），而 `Broker` 负责订单执行和与交易系统交互。在实际使用中，`Portfolio` 可以使用 `Broker` 来执行交易。

### Q: 为什么需要 PaperBroker？

A: `PaperBroker` 提供完整的模拟交易环境，包括订单簿、撮合引擎、手续费和滑点模拟。它让你能在不连接真实交易所的情况下，准确测试策略表现。

### Q: 如何选择仓位管理策略？

A: 
- 初学者：使用 `FixedAmount` 或 `FixedPercentage`
- 有历史数据：使用 `KellyCriterion`（推荐半凯利）
- 趋势策略：使用 `Pyramid` 加仓
- 避免使用：`AllIn`（风险过高）

### Q: 风险管理器如何与 Portfolio 配合？

A: 在执行交易前调用 `RiskManager::check_risk()`，根据返回结果决定是否继续交易。同时定期记录交易结果，让风险管理器跟踪连续亏损等指标。

### Q: 如何模拟真实的交易成本？

A: 使用 `PaperBroker` 时配置合适的 `FeeModel` 和 `SlippageModel`。可以参考目标交易所的实际费率和市场深度来设置参数。

### Q: 支持多币种同时交易吗？

A: `PaperBroker` 天然支持多交易对，可以同时管理多个币种的订单和持仓。但 `BasePortfolio` 目前只支持单资产。

### Q: 如何扩展自定义功能？

A: 实现相应的 trait：
- 自定义投资组合：实现 `Portfolio` trait
- 自定义经纪商：实现 `Broker` trait
- 自定义手续费模型：扩展 `FeeModel` 枚举
- 自定义仓位策略：扩展 `PositionSizingStrategy` 枚举
