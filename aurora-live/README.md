# Aurora Live

Aurora 实时交易引擎 - 基于实时数据流的模拟交易系统

## 概述

`aurora-live` 是 Aurora 量化交易框架的实时交易引擎，通过 WebSocket 连接到加密货币交易所（如 Binance），接收实时市场数据，运行交易策略，并在模拟账户中执行交易。它为策略实盘前的验证提供了真实市场环境，无需承担实际资金风险。

## 主要功能

### 📡 实时数据接收
- 通过 WebSocket 连接到 Binance 实时数据流
- 接收实时 K线数据
- 自动重连机制
- 多端点容错支持

### 🤖 自动化交易
- 实时运行交易策略
- 自动生成买卖信号
- 模拟订单执行
- 交易记录追踪

### 💼 模拟账户管理
- 虚拟资金管理
- 实时持仓跟踪
- 交易成本模拟
- 权益曲线记录

### 📊 实时监控
- 账户状态实时显示
- 交易信号通知
- 定期业绩报告
- 日志记录

### 🛡️ 风险管理
- 全仓买卖控制
- 资金不足检查
- 重复交易防护
- 异常处理机制

## 快速开始

### 安装

```bash
# 编译实时引擎
cargo build --release --package aurora-live

# 安装到系统
cargo install --path aurora-live
```

### 基本使用

```bash
# 启动实时模拟交易（使用默认参数）
aurora-live --symbol BTCUSDT

# 自定义策略参数
aurora-live \
  --symbol ETHUSDT \
  --strategy-name ma-crossover \
  --short 5 \
  --long 20 \
  --initial-cash 50000.0 \
  --interval 5m
```

### 输出示例

```
INFO  启动实时模拟交易: 交易对=BTCUSDT, 策略=ma-crossover, 参数=10:30
INFO  💰 初始化模拟账户，初始资金: 10000.00
INFO  初始化实时交易引擎，策略: ma-crossover, 参数: 10:30, 交易对: BTCUSDT
INFO  尝试连接到WebSocket端点 1: wss://stream.binance.com:9443/ws/btcusdt@kline_1m
INFO  WebSocket连接成功，开始接收实时数据
INFO  📈 模拟买入成功: 价格=45000.00, 数量=0.222222, 总价值=10000.00
INFO  📊 账户状态:
INFO    现金: 0.00
INFO    持仓: 0.222222 (价值: 10000.00)
INFO    总权益: 10000.00
INFO    交易次数: 1
INFO  📉 模拟卖出成功: 价格=46000.00, 数量=0.222222, 总价值=10222.22
INFO  📊 账户状态:
INFO    现金: 10222.22
INFO    持仓: 0.000000 (价值: 0.00)
INFO    总权益: 10222.22
INFO    交易次数: 2
```

## 核心组件

### LiveEngine - 实时交易引擎

负责连接数据流、运行策略和执行交易的核心引擎：

```rust
use aurora_live::LiveEngine;
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建策略
    let strategy = MACrossoverStrategy::new(10, 30);
    
    // 创建实时引擎
    let mut engine = LiveEngine::new(strategy, 10000.0);
    
    // 连接并开始交易
    engine.run("BTCUSDT", "1m").await?;
    
    Ok(())
}
```

**核心功能**:
- 管理 WebSocket 连接
- 解析实时 K线数据
- 调用策略生成信号
- 执行模拟交易
- 定期报告状态

### PaperTrader - 模拟交易者

封装投资组合管理，提供模拟交易功能：

```rust
use aurora_live::PaperTrader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建模拟交易者
    let mut trader = PaperTrader::new(10000.0);
    
    // 执行模拟买入
    trader.execute_paper_buy(45000.0, 1640995200000).await?;
    
    // 查询账户状态
    trader.print_status(45500.0);
    
    // 执行模拟卖出
    trader.execute_paper_sell(46000.0, 1640998800000).await?;
    
    Ok(())
}
```

**核心方法**:
- `execute_paper_buy()`: 执行模拟买入
- `execute_paper_sell()`: 执行模拟卖出
- `get_cash()`: 获取现金余额
- `get_position()`: 获取持仓数量
- `get_total_equity()`: 获取总权益
- `print_status()`: 打印账户状态

## 使用库

### 作为库使用

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
aurora-live = { path = "../aurora-live" }
aurora-core = { path = "../aurora-core" }
aurora-strategy = { path = "../aurora-strategy" }
tokio = { version = "1.0", features = ["full"] }
```

### 代码示例

#### 基本实时交易

```rust
use aurora_live::run_live_trading;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 运行实时交易
    run_live_trading(
        "BTCUSDT",       // 交易对
        "1m",            // K线周期
        "ma-crossover",  // 策略名称
        10,              // 短期周期
        30,              // 长期周期
        10000.0          // 初始资金
    ).await?;
    
    Ok(())
}
```

#### 自定义引擎

```rust
use aurora_live::{LiveEngine, PaperTrader};
use aurora_strategy::MACrossoverStrategy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建自定义策略
    let strategy = MACrossoverStrategy::new(5, 20);
    
    // 创建引擎
    let mut engine = LiveEngine::new(strategy, 50000.0);
    
    // 连接到ETH交易对，使用5分钟K线
    engine.run("ETHUSDT", "5m").await?;
    
    Ok(())
}
```

#### 手动控制交易

```rust
use aurora_live::PaperTrader;
use aurora_core::{Strategy, MarketEvent, Signal, Kline};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut trader = PaperTrader::new(10000.0);
    let mut strategy = MyCustomStrategy::new();
    
    // 假设从某处获取实时K线
    loop {
        let kline = receive_kline().await?;
        
        // 创建市场事件
        let event = MarketEvent::Kline(kline.clone());
        
        // 策略处理
        if let Some(signal_event) = strategy.on_market_event(&event) {
            match signal_event.signal {
                Signal::Buy => {
                    trader.execute_paper_buy(
                        signal_event.price,
                        signal_event.timestamp
                    ).await?;
                }
                Signal::Sell => {
                    trader.execute_paper_sell(
                        signal_event.price,
                        signal_event.timestamp
                    ).await?;
                }
                Signal::Hold => {
                    // 不操作
                }
            }
        }
        
        // 更新权益
        trader.update_equity(kline.timestamp, kline.close);
        
        // 定期打印状态
        trader.print_status(kline.close);
    }
}
```

#### 多交易对监控

```rust
use aurora_live::LiveEngine;
use aurora_strategy::MACrossoverStrategy;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 为每个交易对创建独立的引擎
    let symbols = vec!["BTCUSDT", "ETHUSDT", "BNBUSDT"];
    
    let mut handles = vec![];
    
    for symbol in symbols {
        let handle = tokio::spawn(async move {
            let strategy = MACrossoverStrategy::new(10, 30);
            let mut engine = LiveEngine::new(strategy, 10000.0);
            
            if let Err(e) = engine.run(symbol, "1m").await {
                eprintln!("引擎 {} 错误: {}", symbol, e);
            }
        });
        
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }
    
    Ok(())
}
```

## 命令行接口

### 命令格式

```bash
aurora-live [OPTIONS] --symbol <SYMBOL>
```

### 参数说明

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--symbol` | `-s` | 交易对符号（如 BTCUSDT） | 必需 |
| `--strategy-name` | | 策略名称 | ma-crossover |
| `--short` | | 短期MA周期 | 10 |
| `--long` | | 长期MA周期 | 30 |
| `--initial-cash` | | 初始模拟资金 | 10000.0 |
| `--interval` | `-i` | K线时间间隔 | 1m |

### 支持的时间间隔

- **秒级**: 1s (Binance不支持)
- **分钟级**: 1m, 3m, 5m, 15m, 30m
- **小时级**: 1h, 2h, 4h, 6h, 8h, 12h
- **天级**: 1d, 3d
- **周级**: 1w
- **月级**: 1M

### 支持的策略

- `ma-crossover`: 移动平均线交叉策略（默认）
- 更多策略正在开发中...

### 使用示例

```bash
# 最简单的用法 - BTC 1分钟
aurora-live --symbol BTCUSDT

# ETH 5分钟K线
aurora-live --symbol ETHUSDT --interval 5m

# 自定义策略参数
aurora-live \
  --symbol BNBUSDT \
  --short 5 \
  --long 20 \
  --interval 15m

# 大额初始资金
aurora-live \
  --symbol BTCUSDT \
  --initial-cash 100000.0 \
  --interval 1h

# 快速测试（1分钟K线，敏感参数）
aurora-live \
  --symbol BTCUSDT \
  --short 3 \
  --long 7 \
  --interval 1m
```

## 实时交易流程

```
启动引擎
    ↓
连接 WebSocket
    ↓
接收实时K线数据
    ↓
策略分析生成信号
    ↓
    ├─ Buy → 执行模拟买入
    ├─ Sell → 执行模拟卖出
    └─ Hold → 不操作
    ↓
更新权益曲线
    ↓
定期报告状态（每5分钟）
    ↓
继续接收数据（循环）
```

## WebSocket 连接

### 连接特性

- **自动重连**: 连接断开后自动重试
- **多端点支持**: 主端点失败时切换备用端点
- **心跳处理**: 自动响应 Ping/Pong 消息
- **错误处理**: 完善的错误捕获和恢复机制

### 端点配置

默认使用以下 Binance WebSocket 端点：

1. `wss://stream.binance.com:9443`
2. `wss://stream.binance.com:443`

连续失败3次后自动切换到下一个端点。

### 数据格式

接收的 K线数据格式：

```json
{
  "e": "kline",
  "E": 1640995200000,
  "s": "BTCUSDT",
  "k": {
    "t": 1640995200000,
    "T": 1640995259999,
    "s": "BTCUSDT",
    "i": "1m",
    "f": 100,
    "L": 200,
    "o": "46000.00",
    "c": "46500.00",
    "h": "47000.00",
    "l": "45500.00",
    "v": "100.0",
    "n": 100,
    "x": true,
    "q": "4650000.00",
    "V": "50.0",
    "Q": "2325000.00"
  }
}
```

引擎只处理已完成的K线（`x: true`）。

## 模拟交易规则

### 买入规则

1. 检查现金余额是否充足
2. 检查是否已有持仓（当前只支持全仓）
3. 使用全部现金买入
4. 计算可买数量 = 现金 / 当前价格
5. 记录交易并清零现金

### 卖出规则

1. 检查是否有持仓
2. 卖出全部持仓
3. 计算所得现金 = 持仓数量 × 当前价格
4. 记录交易并清零持仓

### 交易限制

- **全仓模式**: 每次买卖都使用全部资金/持仓
- **单向持仓**: 不支持做空，只能先买后卖
- **无手续费**: 当前版本不模拟交易手续费
- **即时成交**: 按当前价格立即成交，不考虑滑点

## 账户状态监控

### 实时信息

- **现金余额**: 可用于交易的资金
- **持仓数量**: 当前持有的数量
- **持仓价值**: 持仓按当前价格计算的价值
- **总权益**: 现金 + 持仓价值
- **交易次数**: 累计交易次数

### 定期报告

每5分钟自动打印一次账户状态：

```
INFO  📊 定期状态报告
INFO    时间: 2025-10-11 12:05:00
INFO    当前价格: 45500.00
INFO    现金: 0.00
INFO    持仓: 0.222222 (价值: 10111.11)
INFO    总权益: 10111.11
INFO    总交易次数: 5
INFO    盈利交易: 3
INFO    亏损交易: 2
```

### 交易通知

每次交易都会显示详细信息：

```
INFO  📈 模拟买入成功: 价格=45000.00, 数量=0.222222, 总价值=10000.00
INFO  📉 模拟卖出成功: 价格=46000.00, 数量=0.222222, 总价值=10222.22
```

## 日志配置

使用 `RUST_LOG` 环境变量控制日志级别：

```bash
# 只显示重要信息
RUST_LOG=aurora_live=info cargo run --bin aurora-live -- --symbol BTCUSDT

# 显示详细日志（包括每个K线）
RUST_LOG=aurora_live=debug cargo run --bin aurora-live -- --symbol BTCUSDT

# 显示所有模块的详细日志
RUST_LOG=debug cargo run --bin aurora-live -- --symbol BTCUSDT

# Windows PowerShell
$env:RUST_LOG="aurora_live=info"
cargo run --bin aurora-live -- --symbol BTCUSDT
```

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
aurora-strategy = { path = "../aurora-strategy" }
aurora-portfolio = { path = "../aurora-portfolio" }
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
futures-util = "0.3"
serde_json = "1.0"
```

## 测试

```bash
# 运行所有测试
cargo test --package aurora-live

# 运行集成测试
cargo test --package aurora-live --test integration_tests

# 运行特定测试
cargo test --package aurora-live test_paper_trader
```

## 性能考虑

### 网络延迟

- WebSocket 连接延迟通常在 50-200ms
- 数据处理延迟约 1-5ms
- 总延迟约 100-300ms

### CPU 使用

- 空闲时几乎不占用 CPU
- 收到数据时短暂峰值
- 策略计算复杂度决定 CPU 使用

### 内存使用

- 基础内存约 10-20MB
- 每个指标占用 < 1KB
- 权益曲线记录随时间增长

## 风险提示

### ⚠️ 重要声明

1. **模拟环境**: 这是模拟交易系统，不涉及真实资金
2. **不保证盈利**: 过去的表现不代表未来结果
3. **实盘差异**: 
   - 无手续费和滑点
   - 即时成交假设
   - 网络延迟影响
4. **数据延迟**: WebSocket 数据可能有延迟
5. **策略风险**: 任何策略都可能亏损

### 🛡️ 安全建议

- 充分测试策略后再考虑实盘
- 理解策略逻辑和风险
- 监控异常情况
- 保持合理预期
- 不要将模拟结果直接用于实盘决策

## 常见问题

### Q: 为什么连接失败？

A: 可能的原因：
- 网络问题
- Binance API 维护
- 防火墙拦截

解决方法：检查网络连接，等待几分钟后重试。

### Q: 为什么没有交易信号？

A: 可能的原因：
- 策略参数不敏感
- 市场处于震荡期
- K线周期太长

解决方法：调整策略参数或使用更短的 K线周期。

### Q: 如何停止运行？

A: 按 `Ctrl+C` 停止程序。

### Q: 可以同时运行多个实例吗？

A: 可以，每个实例独立运行，互不影响。

### Q: 如何保存交易记录？

A: 当前版本交易记录存储在内存中。可以通过修改代码将记录保存到文件或数据库。

### Q: 支持实盘交易吗？

A: 当前版本只支持模拟交易。实盘交易需要额外的风控和订单管理功能。

## 扩展功能

### 计划中的功能

- [ ] 多交易对同时监控
- [ ] 交易记录持久化
- [ ] 更多交易策略
- [ ] 手续费模拟
- [ ] 滑点模拟
- [ ] 分仓管理
- [ ] 止损止盈
- [ ] 实盘交易接口
- [ ] Web 监控面板

### 自定义开发

可以基于现有代码扩展功能：

```rust
// 自定义通知系统
impl PaperTrader {
    fn send_notification(&self, message: &str) {
        // 发送邮件
        // 发送Telegram消息
        // 发送钉钉通知
        // 等等...
    }
}

// 自定义风控
impl LiveEngine {
    async fn check_risk(&self) -> bool {
        // 实现风控逻辑
        // 检查最大回撤
        // 检查连续亏损
        // 检查持仓时间
        true
    }
}
```

## 与回测引擎的区别

| 特性 | 回测引擎 | 实时引擎 |
|------|---------|---------|
| 数据源 | 历史CSV文件 | WebSocket实时流 |
| 执行速度 | 快速（可调整） | 实时（受市场限制） |
| 测试周期 | 任意历史时期 | 当前时刻开始 |
| 资金 | 虚拟初始资金 | 虚拟初始资金 |
| 网络延迟 | 无 | 有（50-200ms） |
| 适用场景 | 策略验证、参数优化 | 实盘前验证、实时监控 |

## 相关 Crate

- **aurora-core**: 核心数据结构和接口
- **aurora-data**: 数据获取（历史和实时）
- **aurora-strategy**: 交易策略实现
- **aurora-portfolio**: 投资组合管理
- **aurora-backtester**: 历史数据回测

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 技术支持

如有问题，请在 GitHub 上提交 Issue。
