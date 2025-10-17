# Aurora Backtester

Aurora 回测引擎 - 基于历史数据验证和优化交易策略

## 概述

`aurora-backtester` 是 Aurora 量化交易框架的回测引擎，用于在历史数据上测试和评估交易策略的表现。它提供了完整的回测流程，包括数据加载、信号生成、订单执行、仓位管理和绩效分析，帮助开发者在实盘交易前验证策略的有效性。

## 主要功能

### 📈 策略回测
- 支持多种交易策略（MA交叉、RSI、布林带等）
- **事件驱动回测引擎** - 逐事件处理，精确模拟真实交易
- **向量化回测引擎** - 高性能批量处理，适合参数优化
- 精确的信号触发和订单执行
- 支持做多策略（未来可扩展做空、对冲等）

### 💰 真实性增强
- **买卖价差模式 (Bid-Ask Spread)** - 更真实的交易价格模拟
  - 买入使用卖一价（Ask Price）
  - 卖出使用买一价（Bid Price）
  - 可配置价差百分比
- **收盘价模式** - 传统简化回测模式
- 手续费和滑点考虑

### 💼 仓位管理
- 自动管理现金和持仓
- 全仓买入/卖出执行
- 实时权益曲线跟踪
- 交易记录完整保存

### 📊 绩效分析
- 总收益率和年化收益率
- 最大回撤分析
- 夏普比率、索提诺比率、卡玛比率
- 交易次数和胜率统计
- 详细的回测报告

### 📈 可视化报告
- **权益曲线图** - 显示资金变化趋势
- **回撤曲线图** - 可视化风险暴露
- **交易点位图** - 标记买卖信号在价格图上
- **HTML 完整报告** - 精美的交互式报告
- 性能指标仪表板

### 🎯 命令行工具
- 简单易用的 CLI 界面
- 灵活的参数配置
- 实时进度显示
- 彩色日志输出

## 快速开始

### 安装

```bash
# 编译回测引擎
cargo build --release --package aurora-backtester

# 安装到系统
cargo install --path aurora-backtester
```

### 基本使用

```bash
# 使用默认参数运行回测
aurora-backtester --data-path btc_1h.csv

# 自定义策略参数
aurora-backtester \
  --data-path eth_4h.csv \
  --strategy-name ma-crossover \
  --short 5 \
  --long 20 \
  --initial-cash 50000.0
```

### 输出示例

```
INFO  开始回测: 数据文件=btc_1h.csv, 策略=ma-crossover, 参数=5:20
INFO  加载数据文件: btc_1h.csv
INFO  成功加载 1000 条K线数据
INFO  初始化回测引擎，策略: ma-crossover, 参数: 5:20, 初始资金: 10000.00
INFO  开始回测，数据时间范围: 1640995200000 - 1644537600000
INFO  回测进度: 10.0%, 当前权益: 10250.00
INFO  回测进度: 20.0%, 当前权益: 10580.00
...
INFO  回测进度: 100.0%, 当前权益: 12340.00
INFO  回测完成，处理了 1000 条K线数据

========================================
            回测报告
========================================
测试周期: 41.75 天
初始资金: 10000.00
最终权益: 12340.00
总收益率: 23.40%
年化收益率: 204.85%
最大回撤: -5.20%
夏普比率: 2.15
总交易次数: 15 次
盈利交易: 10 次
亏损交易: 5 次
胜率: 66.67%
========================================
```

## 核心组件

### BacktestEngine - 回测引擎

回测引擎的核心类，负责协调整个回测流程：

```rust
use aurora_backtester::BacktestEngine;
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建策略
    let strategy = MACrossoverStrategy::new(5, 10);
    
    // 创建回测引擎
    let mut engine = BacktestEngine::new(strategy, 10000.0);
    
    // 加载K线数据
    let klines = load_klines_from_csv("btc_1h.csv")?;
    
    // 运行回测
    engine.run(&klines).await?;
    
    Ok(())
}
```

### 回测流程

```
加载历史数据
    ↓
初始化引擎 (策略 + 投资组合)
    ↓
遍历每根K线
    ↓
策略生成信号 (Buy/Sell/Hold)
    ↓
执行交易 (更新仓位和现金)
    ↓
更新权益曲线
    ↓
计算绩效指标
    ↓
输出回测报告
```

## 使用库

### 作为库使用

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
aurora-backtester = { path = "../aurora-backtester" }
aurora-core = { path = "../aurora-core" }
aurora-strategy = { path = "../aurora-strategy" }
```

### 代码示例

#### 基本回测

```rust
use aurora_backtester::{BacktestEngine, run_backtest};
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 方法1: 使用便捷函数
    run_backtest(
        "btc_1h.csv",      // 数据文件
        "ma-crossover",     // 策略名称
        5,                  // 短期周期
        20,                 // 长期周期
        10000.0            // 初始资金
    ).await?;
    
    Ok(())
}
```

#### 自定义回测

```rust
use aurora_backtester::BacktestEngine;
use aurora_strategy::MACrossoverStrategy;
use aurora_core::Kline;
use csv::Reader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载数据
    let mut klines = Vec::new();
    let mut reader = Reader::from_path("btc_1h.csv")?;
    
    for result in reader.deserialize() {
        let kline: Kline = result?;
        klines.push(kline);
    }
    
    // 2. 创建策略
    let strategy = MACrossoverStrategy::new(10, 30);
    
    // 3. 创建回测引擎
    let mut engine = BacktestEngine::new(strategy, 50000.0);
    
    // 4. 运行回测
    engine.run(&klines).await?;
    
    // 5. 访问回测结果
    let portfolio = engine.portfolio();
    println!("最终现金: {:.2}", portfolio.get_cash());
    println!("持仓数量: {:.6}", portfolio.get_position());
    
    Ok(())
}
```

#### 批量参数优化

```rust
use aurora_backtester::BacktestEngine;
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let klines = load_klines("btc_1h.csv")?;
    let initial_cash = 10000.0;
    
    let mut best_params = (0, 0);
    let mut best_return = 0.0;
    
    // 遍历不同的参数组合
    for short in (5..=20).step_by(5) {
        for long in (20..=60).step_by(10) {
            if short >= long {
                continue;
            }
            
            let strategy = MACrossoverStrategy::new(short, long);
            let mut engine = BacktestEngine::new(strategy, initial_cash);
            
            engine.run(&klines).await?;
            
            let portfolio = engine.portfolio();
            let final_equity = portfolio.get_total_equity(
                klines.last().unwrap().close
            );
            let return_rate = (final_equity - initial_cash) / initial_cash;
            
            if return_rate > best_return {
                best_return = return_rate;
                best_params = (short, long);
            }
            
            println!("参数 {}:{} - 收益率: {:.2}%", 
                short, long, return_rate * 100.0);
        }
    }
    
    println!("\n最佳参数: {}:{}, 收益率: {:.2}%", 
        best_params.0, best_params.1, best_return * 100.0);
    
    Ok(())
}
```

## 新功能详解

### 🎯 买卖价差模式 (Bid-Ask Spread)

更真实的回测模式，模拟实际交易中的买卖价差：

```rust
use aurora_backtester::{BacktestEngine, PricingMode};
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let klines = load_klines("btc_1h.csv")?;
    let strategy = MACrossoverStrategy::new(10, 30);
    
    // 使用 0.1% 的买卖价差
    let pricing_mode = PricingMode::BidAsk { spread_pct: 0.001 };
    
    let mut engine = BacktestEngine::with_pricing_mode(
        strategy,
        10000.0,
        pricing_mode
    );
    
    engine.run(&klines).await?;
    
    Ok(())
}
```

**定价模式对比**:

- **Close 模式**: 买入和卖出都使用收盘价（简化模式）
- **BidAsk 模式**: 买入使用 Ask价格，卖出使用 Bid价格（真实模式）

```rust
// 收盘价模式
let close_mode = PricingMode::Close;

// 买卖价差模式（0.1% 价差）
let bid_ask_mode = PricingMode::BidAsk { spread_pct: 0.001 };
```

### 📊 可视化报告生成

生成专业的图表和 HTML 报告：

```rust
use aurora_backtester::{BacktestVisualizer, BacktestData};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... 运行回测 ...
    
    // 准备可视化数据
    let data = BacktestData {
        equity_curve: vec![(timestamp1, equity1), (timestamp2, equity2), ...],
        drawdown_curve: vec![(timestamp1, drawdown1), ...],
        price_data: vec![(timestamp1, price1), ...],
        buy_trades: vec![(timestamp1, buy_price1), ...],
        sell_trades: vec![(timestamp1, sell_price1), ...],
        metrics: performance_metrics,
        initial_cash: 10000.0,
    };
    
    // 创建可视化器
    let visualizer = BacktestVisualizer::new();
    
    // 生成单独的图表
    visualizer.plot_equity_curve(&data, "output/equity.png")?;
    visualizer.plot_drawdown(&data, "output/drawdown.png")?;
    visualizer.plot_trades(&data, "output/trades.png")?;
    
    // 生成完整 HTML 报告
    visualizer.generate_html_report(&data, "output/report.html")?;
    
    Ok(())
}
```

**生成的图表包括**:
- 📈 **权益曲线**: 显示资金随时间的变化
- 📉 **回撤曲线**: 可视化最大回撤和风险
- 🎯 **交易点位图**: 在价格图上标记买卖点

**HTML 报告特点**:
- 响应式设计，支持打印
- 精美的性能指标仪表板
- 交互式图表展示
- 完整的交易统计信息

### ⚡ 向量化回测引擎

高性能批量回测，适合参数优化：

```rust
use aurora_backtester::VectorizedBacktestEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let klines = load_klines("btc_1h.csv")?;
    
    // 创建向量化引擎
    let engine = VectorizedBacktestEngine::new(10000.0);
    
    // 计算均线交叉信号
    let signals = engine.calculate_ma_crossover_signals(&klines, 10, 30);
    
    // 运行回测（非常快！）
    let result = engine.run(&klines, &signals)?;
    
    result.print_summary();
    
    Ok(())
}
```

**参数网格搜索**:

```rust
use aurora_backtester::VectorizedBacktestEngine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let klines = load_klines("btc_1h.csv")?;
    let engine = VectorizedBacktestEngine::new(10000.0);
    
    let mut best_return = 0.0;
    let mut best_params = (0, 0);
    
    // 快速测试大量参数组合
    for short in (5..=20).step_by(1) {
        for long in (20..=100).step_by(5) {
            if short >= long { continue; }
            
            let signals = engine.calculate_ma_crossover_signals(&klines, short, long);
            let result = engine.run(&klines, &signals)?;
            
            if result.total_return > best_return {
                best_return = result.total_return;
                best_params = (short, long);
            }
        }
    }
    
    println!("最佳参数: {:?}, 收益: {:.2}%", best_params, best_return * 100.0);
    
    Ok(())
}
```

**性能对比**:
- **事件驱动引擎**: 精确模拟，适合最终验证
- **向量化引擎**: 快100-1000倍，适合参数筛选

**使用场景**:
- ✅ 均线策略参数优化
- ✅ 技术指标阈值搜索
- ✅ 快速策略筛选
- ❌ 复杂仓位管理策略
- ❌ 需要精确订单模拟

### 🔧 自定义图表尺寸

```rust
let visualizer = BacktestVisualizer::with_size(1920, 1080);
visualizer.generate_html_report(&data, "report_hd.html")?;
```

## 命令行接口

### 命令格式

```bash
aurora-backtester [OPTIONS] --data-path <DATA_PATH>
```

### 参数说明

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--data-path` | `-d` | CSV数据文件路径 | 必需 |
| `--strategy-name` | `-s` | 策略名称 | ma-crossover |
| `--short` | | 短期MA周期 | 10 |
| `--long` | | 长期MA周期 | 30 |
| `--initial-cash` | | 初始资金 | 10000.0 |

### 支持的策略

- `ma-crossover`: 移动平均线交叉策略

### 使用示例

```bash
# 最简单的用法
aurora-backtester --data-path btc_1h.csv

# 指定策略参数
aurora-backtester \
  --data-path eth_4h.csv \
  --short 5 \
  --long 20

# 设置初始资金
aurora-backtester \
  --data-path btc_1h.csv \
  --initial-cash 100000.0

# 完整参数示例
aurora-backtester \
  --data-path data/btc_1h.csv \
  --strategy-name ma-crossover \
  --short 8 \
  --long 21 \
  --initial-cash 25000.0
```

## 回测报告

### 报告内容

回测完成后会输出详细的绩效报告：

```
========================================
            回测报告
========================================
测试周期: 41.75 天
初始资金: 10000.00
最终权益: 12340.00
总收益率: 23.40%
年化收益率: 204.85%
最大回撤: -5.20%
夏普比率: 2.15
总交易次数: 15 次
盈利交易: 10 次
亏损交易: 5 次
胜率: 66.67%
========================================
```

### 指标说明

- **测试周期**: 回测数据的时间跨度
- **初始资金**: 回测开始时的资金量
- **最终权益**: 回测结束时的总权益（现金 + 持仓市值）
- **总收益率**: (最终权益 - 初始资金) / 初始资金
- **年化收益率**: 总收益率折算为年化收益
- **最大回撤**: 权益曲线的最大回撤幅度
- **夏普比率**: 衡量风险调整后收益的指标
- **交易次数**: 总的买入和卖出次数
- **胜率**: 盈利交易占总交易的比例

## Portfolio 接口

`BacktestEngine` 使用 `aurora-portfolio` 提供的投资组合管理功能：

```rust
// 获取投资组合引用
let portfolio = engine.portfolio();

// 查询状态
let cash = portfolio.get_cash();           // 获取现金余额
let position = portfolio.get_position();   // 获取持仓数量
let equity = portfolio.get_total_equity(current_price);  // 总权益

// 查询交易历史
let trades = portfolio.get_trades();       // 获取所有交易记录
```

## 数据格式

### CSV 数据格式

回测引擎接受标准的 CSV 格式 K线数据：

```csv
timestamp,open,high,low,close,volume
1640995200000,46000.0,47000.0,45500.0,46500.0,123.45
1640998800000,46500.0,46800.0,46200.0,46600.0,98.76
...
```

**字段说明**:
- `timestamp`: Unix时间戳（毫秒）
- `open`: 开盘价
- `high`: 最高价
- `low`: 最低价
- `close`: 收盘价
- `volume`: 成交量

### 数据获取

使用 `aurora-data` 工具下载历史数据：

```bash
# 下载BTC 1小时数据
aurora-data download --symbol BTCUSDT --interval 1h --output btc_1h.csv

# 下载ETH 4小时数据
aurora-data download --symbol ETHUSDT --interval 4h --output eth_4h.csv
```

## 日志配置

使用环境变量 `RUST_LOG` 控制日志级别：

```bash
# 显示 info 级别日志（默认）
RUST_LOG=aurora_backtester=info cargo run --bin aurora-backtester -- --data-path btc_1h.csv

# 显示 debug 级别日志（包括每笔交易详情）
RUST_LOG=aurora_backtester=debug cargo run --bin aurora-backtester -- --data-path btc_1h.csv

# 显示所有模块的 debug 日志
RUST_LOG=debug cargo run --bin aurora-backtester -- --data-path btc_1h.csv
```

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
aurora-strategy = { path = "../aurora-strategy" }
aurora-portfolio = { path = "../aurora-portfolio" }
csv = "1.3"
clap = { version = "4.4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

## 测试

```bash
# 运行所有测试
cargo test --package aurora-backtester

# 运行集成测试
cargo test --package aurora-backtester --test integration_tests

# 运行特定测试
cargo test --package aurora-backtester test_backtest_engine
```

## 性能优化建议

1. **数据预加载**: 一次性加载所有数据到内存，避免重复 I/O
2. **批量回测**: 使用多线程并行测试不同参数组合
3. **采样测试**: 对于长周期数据，可以先用采样数据快速验证
4. **缓存计算**: 缓存指标计算结果，避免重复计算

## 扩展性

### 添加新策略

实现 `Strategy` trait 即可添加新策略：

```rust
use aurora_core::{Strategy, MarketEvent, SignalEvent, Signal};

pub struct MyCustomStrategy {
    // 策略状态
}

impl Strategy for MyCustomStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        // 实现策略逻辑
        match event {
            MarketEvent::Kline(kline) => {
                // 分析K线，生成信号
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

### 自定义绩效指标

可以访问 `Portfolio` 获取交易历史和权益曲线，计算自定义指标：

```rust
let portfolio = engine.portfolio();
let trades = portfolio.get_trades();
let equity_curve = portfolio.get_equity_curve();

// 计算自定义指标
let avg_trade_duration = calculate_avg_duration(&trades);
let profit_factor = calculate_profit_factor(&trades);
```

## 常见问题

### Q: 如何处理数据文件不存在的错误？

A: 确保数据文件路径正确，并且文件格式符合要求。使用 `aurora-data` 工具下载标准格式的数据。

### Q: 回测结果与实盘差异大？

A: 回测不考虑滑点、手续费等实盘因素。建议：
- 在策略中预留滑点空间
- 考虑手续费对收益的影响
- 在实盘前进行模拟盘测试

### Q: 如何加速回测？

A: 
- 减少日志输出级别
- 使用 `--release` 模式编译
- 对于参数优化，使用并行计算

### Q: 支持做空吗？

A: 当前版本仅支持做多策略。做空、对冲等功能将在后续版本中添加。

## 相关 Crate

- **aurora-core**: 核心数据结构和接口
- **aurora-data**: 历史数据获取和加载
- **aurora-strategy**: 交易策略实现
- **aurora-portfolio**: 投资组合管理
- **aurora-live**: 实盘交易引擎

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。
