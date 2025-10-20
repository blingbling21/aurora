# Aurora Config

Aurora 配置管理库 - 为量化交易框架提供统一的 TOML 配置文件支持

## 概述

`aurora-config` 是 Aurora 量化交易框架的配置管理组件，提供类型安全、易于使用的配置文件解析和验证功能。它支持回测和实时交易的所有配置需求，包括数据源、策略参数、投资组合设置、风险管理规则等。

## 主要功能

### ⚙️ 配置类型

- **数据源配置** (`DataSourceConfig`)
  - 交易所选择（Binance, OKX等）
  - API密钥和密钥
  - REST API和WebSocket URL
  - 超时和重试设置
  
- **策略配置** (`StrategyConfig`)
  - 策略类型和名称
  - 策略参数（支持多种参数类型）
  - 启用/禁用开关
  - 支持多策略配置
  
- **投资组合配置** (`PortfolioConfig`)
  - 初始资金
  - 手续费率
  - 滑点设置
  - 最大持仓数量和大小
  
- **风险管理配置** (`RiskRulesConfig`)
  - 最大回撤限制
  - 单日最大亏损
  - 连续亏损次数
  - 账户最低权益
  
- **仓位管理配置** (`PositionSizingConfig`)
  - 固定金额策略
  - 固定比例策略
  - Kelly准则
  - 金字塔加仓
  
- **日志配置** (`LogConfig`)
  - 日志级别
  - 输出格式
  - 文件路径
  
- **回测配置** (`BacktestConfig`)
  - 数据文件路径
  - 交易对和时间周期
  - 时间范围过滤（可选）
  - 定价模式配置（收盘价或买卖价差）
  
- **实时交易配置** (`LiveConfig`)
  - 交易对和时间周期
  - 是否为模拟交易

### 🔧 功能特性

- ✅ TOML 格式配置文件
- ✅ 类型安全的配置结构
- ✅ 配置验证和错误提示
- ✅ 默认值支持
- ✅ 嵌套配置结构
- ✅ 可选和必选参数
- ✅ 多策略支持
- ✅ 环境特定配置（开发/生产）
- ✅ 定价模式配置（收盘价/买卖价差）
- ✅ 动态止损止盈配置
- ✅ 灵活的仓位管理策略

## 快速开始

### 基本使用

```rust
use aurora_config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从文件加载配置
    let config = Config::from_file("config.toml")?;
    
    // 访问数据源配置
    println!("数据源: {}", config.data_source.provider);
    println!("超时: {}秒", config.data_source.timeout);
    
    // 访问策略配置
    for strategy in &config.strategies {
        if strategy.enabled {
            println!("启用策略: {}", strategy.name);
            println!("策略类型: {}", strategy.strategy_type);
        }
    }
    
    // 访问投资组合配置
    println!("初始资金: {}", config.portfolio.initial_cash);
    println!("手续费率: {}", config.portfolio.commission);
    
    Ok(())
}
```

### 配置文件示例

#### 基本配置文件

```toml
# config.toml

# 数据源配置
[data_source]
provider = "binance"
timeout = 30

# 策略配置
[[strategies]]
name = "MA交叉策略"
strategy_type = "ma-crossover"
enabled = true

[strategies.parameters]
short = 10
long = 30

# 投资组合配置
[portfolio]
initial_cash = 10000.0
commission = 0.001
slippage = 0.0005

# 日志配置
[logging]
level = "info"
format = "pretty"

# 回测配置
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"

# 定价模式配置
[backtest.pricing_mode]
mode = "close"          # 或 "bid_ask"
# spread_pct = 0.001    # 仅在 bid_ask 模式下需要
```

#### 完整配置示例

参见 `examples/complete_config.toml` 查看所有可用配置选项。

### 在回测中使用

```rust
use aurora_config::Config;
use aurora_backtester::run_backtest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::from_file("backtest_config.toml")?;
    
    // 获取回测配置
    let backtest_config = config.backtest.as_ref()
        .ok_or("缺少回测配置")?;
    
    // 获取第一个启用的策略
    let strategy_config = config.strategies.iter()
        .find(|s| s.enabled)
        .ok_or("没有启用的策略")?;
    
    // 执行回测
    run_backtest(
        &backtest_config.data_path,
        &strategy_config.strategy_type,
        strategy_config.parameters.get("short")?,
        strategy_config.parameters.get("long")?,
        &config.portfolio,
    ).await?;
    
    Ok(())
}
```

### 在实时交易中使用

```rust
use aurora_config::Config;
use aurora_live::LiveEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::from_file("live_config.toml")?;
    
    // 获取实时交易配置
    let live_config = config.live.as_ref()
        .ok_or("缺少实时交易配置")?;
    
    // 创建并运行实时引擎
    let mut engine = LiveEngine::from_config(&config)?;
    engine.run(&live_config.symbol).await?;
    
    Ok(())
}
```

## 配置结构

### DataSourceConfig

```rust
pub struct DataSourceConfig {
    pub provider: String,           // 数据提供商
    pub api_key: Option<String>,    // API密钥
    pub api_secret: Option<String>, // API密钥
    pub base_url: Option<String>,   // REST API URL
    pub ws_url: Option<String>,     // WebSocket URL
    pub timeout: u64,               // 超时时间(秒)
    pub max_retries: usize,         // 最大重试次数
}
```

### StrategyConfig

```rust
pub struct StrategyConfig {
    pub name: String,                          // 策略名称
    pub strategy_type: String,                 // 策略类型
    pub enabled: bool,                         // 是否启用
    pub parameters: HashMap<String, StrategyParameter>, // 参数
}

pub enum StrategyParameter {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}
```

### PortfolioConfig

```rust
pub struct PortfolioConfig {
    pub initial_cash: f64,                // 初始资金
    pub commission: f64,                  // 手续费率
    pub slippage: f64,                    // 滑点
    pub max_position_size: Option<f64>,   // 最大持仓大小
    pub max_positions: Option<usize>,     // 最大持仓数
    pub risk_rules: Option<RiskRulesConfig>,       // 风险规则
    pub position_sizing: Option<PositionSizingConfig>, // 仓位管理
}
```

### RiskRulesConfig

```rust
pub struct RiskRulesConfig {
    pub max_drawdown_pct: Option<f64>,           // 最大回撤(%)
    pub max_daily_loss_pct: Option<f64>,         // 单日最大亏损(%)
    pub max_consecutive_losses: Option<usize>,   // 最大连续亏损次数
    pub max_single_trade_loss_pct: Option<f64>,  // 单笔最大亏损(%)
    pub min_equity: Option<f64>,                 // 最低权益
    pub stop_loss_pct: Option<f64>,              // 止损百分比(相对入场价)
    pub take_profit_pct: Option<f64>,            // 止盈百分比(相对入场价)
}
```

**动态止损止盈说明**：
- `stop_loss_pct`: 相对于入场价格的止损百分比，触发价格 = 入场价 × (1 - stop_loss_pct/100)
- `take_profit_pct`: 相对于入场价格的止盈百分比，触发价格 = 入场价 × (1 + take_profit_pct/100)
- 止损止盈在买入时自动设置，卖出时自动清除
- 基于每次交易的实际入场价格动态计算

### PositionSizingConfig

```rust
pub enum PositionSizingConfig {
    // 全仓策略
    AllIn,
    
    // 固定金额策略
    FixedAmount {
        amount: f64,
    },
    
    // 固定比例策略
    FixedPercentage {
        percentage: f64,  // 0.0-1.0
    },
    
    // Kelly准则策略
    KellyCriterion {
        win_rate: f64,
        profit_loss_ratio: f64,
        kelly_fraction: f64,
    },
    
    // 金字塔加仓策略
    Pyramid {
        initial_percentage: f64,
        profit_threshold: f64,
        max_percentage: f64,
        increment: f64,
    },
}
```

## 配置验证

配置文件加载时会自动进行验证：

```rust
use aurora_config::{Config, ConfigError};

match Config::from_file("config.toml") {
    Ok(config) => {
        println!("配置加载成功!");
    }
    Err(ConfigError::IoError(e)) => {
        eprintln!("无法读取配置文件: {}", e);
    }
    Err(ConfigError::ParseError(e)) => {
        eprintln!("配置文件格式错误: {}", e);
    }
    Err(ConfigError::ValidationError(e)) => {
        eprintln!("配置验证失败: {}", e);
    }
    Err(e) => {
        eprintln!("其他错误: {}", e);
    }
}
```

### PricingModeConfig

```rust
pub enum PricingModeConfig {
    // 使用收盘价执行交易(简单模式)
    Close,
    
    // 使用买一卖一价执行交易(更真实的模式)
    BidAsk {
        spread_pct: f64,  // 买卖价差百分比
    },
}
```

**定价模式说明**：
- **Close 模式**: 买入和卖出都使用 K 线的收盘价，适合快速回测
- **BidAsk 模式**: 买入使用卖一价，卖出使用买一价，更接近真实交易
  - Ask Price = Close × (1 + spread_pct/2)
  - Bid Price = Close × (1 - spread_pct/2)
  - 建议 spread_pct 设置为 0.001（0.1%价差）

## 配置示例文件

项目提供了多个配置示例文件：

- `examples/backtest_config.toml` - 回测配置示例（包含定价模式、止损止盈等完整配置）
- `examples/backtest_bidask_config.toml` - BidAsk 定价模式回测示例
- `examples/live_config.toml` - 实时交易配置示例
- `examples/complete_config.toml` - 完整配置选项参考
- `examples/strict_risk_config.toml` - 严格风控配置示例

## API 文档

生成完整的 API 文档：

```bash
cargo doc -p aurora-config --open
```

## 测试

运行配置管理测试：

```bash
cargo test -p aurora-config
```

## 相关 Crate

- **[aurora-core](../aurora-core)**: 核心数据结构和接口
- **[aurora-backtester](../aurora-backtester)**: 使用配置运行回测
- **[aurora-live](../aurora-live)**: 使用配置运行实时交易

## 版本

当前版本: **0.1.0**

## 许可证

Apache License 2.0 - 详见根目录 LICENSE 文件
