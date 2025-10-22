# Aurora - 模块化量化交易研究框架

<p align="center">
  <img src="https://img.shields.io/badge/Code%20Generation-AI%20Powered-critical" alt="AI Generated">
  <img src="https://img.shields.io/badge/Status-Experimental-red" alt="Experimental">
  <img src="https://img.shields.io/badge/Use%20with%20Caution-EXTREME-yellow" alt="Extreme Caution">
</p>

> **🛑 严重警告：本项目完全由 AI 生成**
>
> **这是一个实验性项目，其几乎所有内容——包括需求规格、技术方案、全部 Rust 代码、所有测试（单元、集成、文档）、以及全部注释和文档——均由大型语言模型（LLM）生成。**
>
- **人类的角色**：作者仅作为**高级指导者**，负责提出需求、编排生成流程、并进行基本的整合与调整。作者并未逐行编写或深度审查所有代码。
- **核心风险**：代码中**极有可能**包含隐蔽的逻辑错误、严重的安全漏洞、性能陷阱以及其他不可预知的缺陷。**AI 生成的测试可能无法发现其自身生成的代码中的根本性问题（“自我验证谬误”）**。
>
> **严禁在任何生产环境、回测决策或涉及真实资金的场景中使用本项目的任何部分。**
> **使用者必须将本项目视为一个关于“AI 驱动软件开发”的学术案例，而非一个可靠的工具。所有风险由使用者自行承担。**

Aurora是一个使用Rust构建的模块化、事件驱动的量化交易研究框架。该框架提供高性能、高安全性的工具，支持从数据采集、策略回测到实时模拟交易的全流程。

## 项目特色

- 🚀 **高性能**: 基于Rust构建，具备零成本抽象和内存安全保证
- 🧩 **模块化**: 7个独立crate，各司其职，可自由组合
- ⚡ **事件驱动**: 统一的市场事件处理架构，支持历史和实时数据
- 📊 **完整工具链**: 数据采集→策略开发→回测验证→实时模拟，一站式解决方案
- 🔧 **可扩展**: 通过trait系统轻松扩展新的数据源、指标和策略
- 📈 **生产就绪**: 完善的测试覆盖、错误处理和日志记录

## 项目结构

### Cargo Workspace 成员

Aurora框架由8个精心设计的crate组成，每个crate都有明确的职责和边界：

#### 📦 **`aurora-core`** (库)
> 核心抽象层 - 提供整个框架的基础数据结构和trait定义

**核心功能**：
- `Kline`数据结构：标准化的OHLCV市场数据
- `MarketEvent`事件系统：统一的事件驱动架构
- `Signal`交易信号：Buy/Sell/Hold枚举定义
- `SignalEvent`信号事件：包含价格、时间戳等完整上下文
- `DataSource` trait：异步数据源统一接口
- `Strategy` trait：策略开发统一接口
- `Portfolio` trait：投资组合管理统一接口

**依赖关系**：无外部依赖，作为最底层的基础库
**适用场景**：作为其他所有crate的基础依赖

---

#### 📡 **`aurora-data`** (可执行文件 + 库)
> 数据采集模块 - 支持历史数据下载和实时数据流接收

**核心功能**：
- **历史数据下载器**：通过REST API批量获取K线数据
  - 支持多种时间间隔（1m, 5m, 15m, 1h, 4h, 1d等）
  - 支持时间范围过滤
  - 自动数据验证和CSV存储
- **实时数据流**：通过WebSocket接收实时市场数据
  - K线数据实时更新
  - 逐笔成交数据流
  - 自动重连机制和错误恢复
- **数据加载器**：从CSV等格式加载历史数据

**支持的数据源**：
- Binance（完整支持）
- 可扩展支持其他交易所

**CLI命令**：
```bash
# 下载历史数据
aurora-data download --symbol BTCUSDT --interval 1h --output data.csv

# 实时数据流
aurora-data stream --symbol BTCUSDT --interval 1m
```

---

#### 📊 **`aurora-indicators`** (库)
> 技术指标库 - 提供20+种常用技术分析指标

**核心功能**：
- **趋势指标**（6种）：
  - `MA`：简单移动平均线
  - `EMA`：指数移动平均线
  - `MACD`：移动平均收敛散度
  - `ADX`：平均动向指数
  - `PSAR`：抛物线转向指标
  - `Ichimoku`：一目均衡表
  
- **动量指标**（5种）：
  - `RSI`：相对强弱指数
  - `Stochastic`：随机震荡指标
  - `ROC`：变动率指标
  - `CCI`：商品通道指数
  - `Williams %R`：威廉指标
  
- **波动率指标**（4种）：
  - `Bollinger Bands`：布林带
  - `ATR`：平均真实波幅
  - `StdDev`：标准差
  - `Keltner Channels`：肯特纳通道
  
- **成交量指标**（5种）：
  - `OBV`：能量潮
  - `MFI`：资金流量指数
  - `VWAP`：成交量加权平均价
  - `CMF`：佳庆资金流
  - `ADLine`：累积/派发线

**设计特点**：
- 滑动窗口算法，内存高效
- 状态管理，支持流式数据处理
- 统一的`update()`接口设计
- 完整的单元测试覆盖

---

#### 🎯 **`aurora-strategy`** (库)
> 策略开发框架 - 提供统一的策略接口和经典策略实现

**核心功能**：
- `Strategy` trait定义：标准化的策略开发接口
- `MACrossoverStrategy`：移动平均线交叉策略
  - 金叉（短期MA上穿长期MA）产生买入信号
  - 死叉（短期MA下穿长期MA）产生卖出信号
  - 支持自定义短期和长期周期
  - 状态维护，避免重复信号

**扩展开发**：
```rust
impl Strategy for MyStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        // 实现自定义策略逻辑
    }
}
```

---

#### 💼 **`aurora-portfolio`** (库)
> 投资组合管理 - 交易执行、风险控制、业绩分析的完整解决方案

**核心功能**：
- **基础投资组合管理**（`BasePortfolio`）：
  - 现金和持仓跟踪
  - 买卖操作执行
  - 权益曲线记录
  - 手续费和滑点模拟
  
- **风险管理器**（`RiskManager`）：
  - 最大回撤限制
  - 连续亏损保护
  - 最低权益要求
  - 单次交易风险控制
  
- **仓位管理器**（`PositionManager`）：
  - 固定金额策略
  - 固定比例策略
  - Kelly准则
  - 金字塔加仓策略
  
- **业绩分析器**（`PortfolioAnalytics`）：
  - 总收益率
  - 年化收益率
  - 最大回撤
  - 夏普比率
  - 胜率统计
  - 盈亏比

**订单类型支持**：
- 市价单
- 限价单
- 止损单
- 止盈单

---

#### 🔄 **`aurora-backtester`** (可执行文件 + 库)
> 历史回测引擎 - 基于历史数据验证策略有效性

**核心功能**：
- **事件驱动回测引擎**：逐事件处理，精确模拟真实交易
- **定价模式支持**：
  - Close 模式：使用收盘价（简化模式）
  - BidAsk 模式：模拟买卖价差（真实模式）
  - 可通过配置文件灵活切换
- **动态止损止盈**：
  - 基于入场价格的百分比止损止盈
  - 买入时自动设置，卖出时自动清除
  - 止损价 = 入场价 × (1 - stop_loss_pct/100)
  - 止盈价 = 入场价 × (1 + take_profit_pct/100)
- **投资组合状态跟踪**：实时权益曲线和持仓管理
- **详细的业绩报告生成**：多维度绩效指标分析

**回测报告包含**：
- 总收益率和年化收益率
- 最大回撤和回撤持续时间
- 夏普比率和索提诺比率
- 交易次数和胜率
- 平均盈利和平均亏损
- 盈亏比和最大连续盈亏
- 止损止盈配置信息
- 定价模式信息

**CLI命令**：
```bash
# 使用配置文件（推荐）
aurora-backtester --config backtest_config.toml

# 使用命令行参数
aurora-backtester --data-path btc_1h.csv --short 10 --long 30
```

**特性**：
- 严格避免未来函数
- 支持手续费和滑点模拟
- 支持多种风险管理规则
- 支持多种仓位管理策略
- 支持定价模式配置
- 支持动态止损止盈

---

#### 🚀 **`aurora-live`** (可执行文件 + 库)
> 实时模拟交易引擎 - 7x24小时模拟交易系统

**核心功能**：
- 实时数据接收和处理
- 策略实时执行
- 模拟订单执行（纸上交易）
- 实时权益监控
- 定期业绩报告

**PaperTrader（模拟交易器）**：
- 无风险的模拟交易环境
- 实时价格执行
- 完整的交易记录
- 与回测引擎一致的业绩计算

**CLI命令**：
```bash
# 使用命令行参数
aurora-live --symbol BTCUSDT --short 10 --long 30

# 使用配置文件
aurora-live --config live_config.toml
```

**监控特性**：
- 每分钟输出当前状态
- 自动重连机制
- 错误日志记录
- Graceful shutdown支持

---

#### ⚙️ **`aurora-config`** (库)
> 配置管理 - 统一的TOML配置文件支持

**核心功能**：
- TOML格式配置文件解析
- 配置验证和默认值处理
- 多策略配置支持
- 类型安全的配置结构

**配置类型**：
- `DataSourceConfig`：数据源配置（API密钥、URL、超时等）
- `StrategyConfig`：策略配置（类型、参数、启用状态）
- `PortfolioConfig`：投资组合配置（初始资金、手续费、滑点）
- `RiskRulesConfig`：风险规则配置（回撤、止损止盈等）
- `PositionSizingConfig`：仓位管理配置（固定金额、固定比例、Kelly、金字塔等）
- `LogConfig`：日志配置
- `BacktestConfig`：回测专用配置（数据路径、定价模式等）
- `LiveConfig`：实时交易专用配置
- `PricingModeConfig`：定价模式配置（Close/BidAsk）

**使用示例**：
```rust
use aurora_config::Config;

let config = Config::from_file("config.toml")?;
let initial_cash = config.portfolio.initial_cash;
```

**新增功能**：
- ✅ 定价模式配置（Close/BidAsk）
- ✅ 动态止损止盈配置（stop_loss_pct/take_profit_pct）
- ✅ 灵活的仓位管理策略配置

---

#### 🌐 **`aurora-web`** (可执行文件 + 库) ⭐ **NEW**
> Web管理界面 - 图形化配置、回测和结果分析

**核心功能**：
- **配置管理**：创建、编辑、验证配置文件
- **数据管理**：查看和管理历史数据文件
- **回测执行**：通过Web界面启动和监控回测任务
- **实时监控**：WebSocket实时推送回测进度
- **结果分析**：图形化展示回测结果和性能指标
- **历史记录**：查看所有回测任务历史

**技术栈**：
- 后端：Actix-Web 4.9 + Tokio (异步Rust Web框架)
- 前端：HTML5 + CSS3 + Vanilla JS (轻量级单页应用)
- 可视化：Chart.js 4.4 (权益曲线、回撤曲线等)
- 通信：RESTful API + WebSocket (实时双向通信)

**API端点**：
- `/api/config` - 配置管理 (7个端点)
- `/api/backtest` - 回测执行 (5个端点)
- `/api/data` - 数据管理 (3个端点)
- `/ws/backtest/{id}` - WebSocket进度推送

**启动命令**：
```bash
# 启动Web服务器
cargo run -p aurora-web

# 或使用启动脚本
.\start-web.ps1

# 访问: http://localhost:8080
```

**界面功能**：
- 📊 仪表盘：任务概览和统计
- ⚙️ 配置管理：TOML配置文件编辑器
- 📁 数据管理：CSV文件列表和信息
- 🚀 回测执行：表单配置和实时进度
- 📜 历史记录：任务列表和结果查看

**特点**：
- 无需命令行，纯图形化操作
- 实时进度显示，无需等待
- 配置验证，避免参数错误
- 结果可视化，直观分析性能
- 完整文档，5分钟快速上手

**快速开始**：
```bash
# 1. 准备数据和配置
mkdir data configs
cp examples/backtest_config.toml configs/

# 2. 启动服务器
cargo run -p aurora-web

# 3. 打开浏览器访问 http://localhost:8080
```

📖 **详细文档**：
- [快速开始指南](aurora-web/docs/QUICK_START.md)
- [完整用户指南](aurora-web/docs/USER_GUIDE.md)
- [项目实现总结](aurora-web/docs/PROJECT_SUMMARY.md)

---

### 架构特点

- **模块化设计**: 各功能组件高度解耦，易于独立开发和测试
- **事件驱动**: 基于市场事件的统一处理架构，支持多种数据源
- **策略模式**: 通过`Strategy` trait实现可插拔的交易策略
- **异步支持**: 基于tokio的异步运行时，支持高并发数据处理
- **类型安全**: 利用Rust类型系统在编译期确保数据安全和逻辑正确性

## 快速开始

### 环境要求

- Rust 1.70+ (推荐使用最新稳定版)
- 网络连接 (用于获取市场数据)

### 编译项目
```bash
cargo build --release
```

### 运行测试
```bash
# 运行所有库测试
cargo test --lib

# 运行集成测试
cargo test
```

### 使用工具

#### 1. 数据采集 (aurora-data)
```bash
# 下载历史数据 - 支持多种时间间隔
cargo run -p aurora-data -- download --symbol BTCUSDT --interval 1h --output btc_1h.csv

# 指定时间范围下载
cargo run -p aurora-data -- download --symbol ETHUSDT --interval 4h --start-time 2024-01-01 --end-time 2024-12-31 --output eth_4h.csv

# 接收实时数据流 - 支持K线和逐笔成交
cargo run -p aurora-data -- stream --symbol BTCUSDT --interval 1m
cargo run -p aurora-data -- stream --symbol BTCUSDT --stream-type trade
```

#### 2. 历史回测 (aurora-backtester)
```bash
# 方式1: 使用配置文件 (推荐)
cargo run -p aurora-backtester -- --config examples/backtest_config.toml

# 使用 BidAsk 定价模式
cargo run -p aurora-backtester -- --config examples/backtest_bidask_config.toml

# 方式2: 使用命令行参数 (传统方式)
cargo run -p aurora-backtester -- --data-path btc_1h.csv --short 10 --long 30 --initial-cash 10000

# 查看详细回测报告
cargo run -p aurora-backtester -- --data-path btc_1h.csv --short 5 --long 20 --initial-cash 50000
```

#### 3. 实时模拟交易 (aurora-live)
```bash
# 方式1: 使用命令行参数 (传统方式)
cargo run -p aurora-live -- --symbol BTCUSDT --short 10 --long 30 --initial-cash 10000

# 方式2: 使用配置文件 (推荐)
cargo run -p aurora-live -- --config examples/live_config.toml

# 使用不同策略参数
cargo run -p aurora-live -- --symbol ETHUSDT --short 5 --long 20 --initial-cash 20000
```

### 配置文件使用

Aurora支持通过TOML配置文件管理所有参数,这对于复杂策略和重复运行非常方便。

#### 创建配置文件
```toml
# my_strategy.toml

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

# 日志配置
[logging]
level = "info"
format = "pretty"

# 回测配置
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"
```

#### 使用配置文件
```bash
# 回测
aurora-backtester --config my_strategy.toml

# 实时交易
aurora-live --config my_strategy.toml
```

更多配置示例请参考 `examples/` 目录:
- `backtest_config.toml` - 回测配置示例（包含定价模式和止损止盈配置）
- `backtest_bidask_config.toml` - BidAsk 定价模式回测示例
- `live_config.toml` - 实时交易配置示例
- `complete_config.toml` - 完整配置选项参考
- `strict_risk_config.toml` - 严格风控配置示例

## 核心概念

### 数据结构
- **`Kline`**: K线数据结构，包含OHLCV和时间戳，支持序列化/反序列化
- **`MarketEvent`**: 市场事件枚举，统一不同类型的市场数据
- **`Signal`**: 交易信号枚举 (Buy/Sell/Hold)
- **`SignalEvent`**: 包含信号和元数据的事件结构
- **`Trade`**: 交易记录结构，支持多种交易类型

### Trait设计
- **`DataSource`**: 异步数据源统一接口，支持历史和实时数据
- **`Strategy`**: 策略统一接口，支持事件驱动的信号生成
- **`Portfolio`**: 投资组合管理接口，支持权益跟踪和风险控制

### 已实现组件

#### 技术指标 (aurora-indicators)
提供完整的技术分析指标库，包含20+种常用指标：

- **趋势指标**: MA、EMA、MACD、ADX、PSAR、Ichimoku
- **动量指标**: RSI、Stochastic、ROC、CCI、Williams %R
- **波动率指标**: Bollinger Bands、ATR、StdDev、Keltner Channels
- **成交量指标**: OBV、MFI、VWAP、CMF、ADLine

每个指标都具备：
  - 滑动窗口机制，内存高效
  - 状态管理，支持流式数据处理
  - 完整测试覆盖，包括边界条件

#### 交易策略 (aurora-strategy)
- **MA交叉策略**: 双均线交叉买卖信号
  - 金叉买入，死叉卖出
  - 状态维护，避免重复信号
  - 支持自定义短期和长期周期

#### 数据源集成 (aurora-data)
- **Binance数据源**: 完整的REST API和WebSocket支持
  - 历史数据下载，支持多种时间间隔
  - 实时数据流，自动重连机制
  - 数据验证和错误处理

#### 投资组合管理 (aurora-portfolio)
- **权益跟踪**: 实时计算总权益、持仓和现金
- **交易执行**: 统一的买卖操作接口，支持异步执行
- **业绩分析**: 收益率、最大回撤、夏普比率等多维度指标
- **权益曲线**: 记录和追踪投资组合的权益变化
- **交易记录**: 完整的交易历史管理，支持TradeBuilder构建

#### 回测引擎 (aurora-backtester)
- **历史数据回放**: 按时间顺序模拟市场环境
- **策略执行**: 支持任意策略实现
- **业绩报告**: 详细的回测结果和统计指标
- **无未来函数**: 严格的时间序列处理

#### 实时引擎 (aurora-live)
- **7x24小时运行**: 稳定的实时数据处理
- **模拟交易**: 安全的纸上交易，不涉及真实资金
- **实时监控**: 定期输出账户状态和业绩
- **错误恢复**: 网络断线自动重连

## 技术栈与依赖

### 核心依赖
- **`tokio`**: 异步运行时，支持高并发和非阻塞I/O
- **`serde`**: 序列化/反序列化框架，支持JSON和CSV格式
- **`anyhow`**: 错误处理，提供统一的错误类型
- **`tracing`**: 结构化日志记录，支持多级别日志

### 网络与数据
- **`reqwest`**: HTTP客户端，用于REST API调用
- **`tokio-tungstenite`**: WebSocket客户端，支持实时数据流
- **`csv`**: CSV文件处理，用于历史数据存储
- **`url`**: URL处理和验证

### 命令行与工具
- **`clap`**: 命令行接口框架，支持子命令和参数验证
- **`chrono`**: 时间处理库，支持时间戳转换
- **`async-trait`**: 异步trait支持

### 测试与开发
- **完整单元测试**: 119+ 测试用例，覆盖核心功能
- **集成测试**: 模块间协作验证
- **文档测试**: 代码示例自动验证
- **性能测试**: 大数据量处理验证

## 项目统计

- **模块数量**: 8个crate (5个库 + 3个二进制)
- **代码行数**: 15,000+ 行Rust代码
- **测试覆盖**: 615+ 测试用例（全部通过）
  - 单元测试：539+
  - 集成测试：69+
  - 文档测试：7+
- **技术指标**: 20+ 种技术分析指标
- **文档完整性**: 所有公共API都有详细文档和示例
- **依赖管理**: 统一的workspace依赖版本管理

## 开发计划与完成状态

框架按照以下阶段开发，**所有阶段均已完成**：

- **阶段零**: ✅ **已完成** - 项目结构和基础工具
  - Cargo workspace配置
  - 统一的依赖管理
  - 开发环境和工具链设置

- **阶段一**: ✅ **已完成** - 数据核心 (数据采集和核心结构)
  - `aurora-core`: 完整的数据结构和trait定义
  - `aurora-data`: 历史数据下载和实时数据流功能
  - Binance API集成和WebSocket连接

- **阶段二**: ✅ **已完成** - 策略大脑 (指标和策略实现)
  - `aurora-indicators`: 移动平均线等技术指标
  - `aurora-strategy`: MA交叉等经典策略
  - 可扩展的策略框架设计

- **阶段三**: ✅ **已完成** - 历史回测引擎
  - `aurora-backtester`: 完整的回测引擎
  - `aurora-portfolio`: 投资组合管理
  - 业绩分析和风险指标计算

- **阶段四**: ✅ **已完成** - 实时模拟交易引擎
  - `aurora-live`: 7x24小时实时引擎
  - 模拟交易执行和状态监控
  - 错误处理和自动恢复机制

**最新更新 (2025年10月20日)**:
- ✅ 完成定价模式配置功能（Close/BidAsk）
- ✅ 实现动态止损止盈功能（基于入场价百分比）
- ✅ 增强配置文件系统，支持更灵活的参数配置
- ✅ 所有测试通过（615+ 测试用例）
- ✅ 更新完整文档和配置示例

## 生产使用指南

### 风险提示

本项目本质上是一个关于 AI 在复杂软件工程中能力的探索性实验。因此，存在以下重大风险：

1.  **代码来源与可靠性**:
    - **完全由 AI 生成**：本项目的全部代码、测试和文档均由 AI 自动生成。人类作者并未参与逐行编码，也无法保证对每一处实现细节的完全理解和控制。
    - **逻辑缺陷风险**：AI 可能产生看似正确但包含细微、致命逻辑错误的算法。对于量化交易这种对精度和逻辑严密性要求极高的领域，此类风险是不可接受的。

2.  **安全漏洞**:
    - **未经审计**：代码库未经任何形式的人工安全审计。AI 可能从其训练数据中学习并复现了不安全的编码模式，从而引入严重的安全漏洞（如：不当的API密钥处理、数据泄露风险等）。

3.  **测试的局限性**:
    - **“自我验证谬误”**：所有测试用例同样由 AI 生成，它们很可能只能覆盖 AI“预想”到的正常路径和简单边界，而无法发现其自身思维模型之外的未知缺陷。测试通过**绝不代表**代码的正确性或健壮性。

4.  **免责声明**:
    - **本项目严格限定于学术研究和技术探讨目的。**
    - **严禁**将本项目的任何部分用于生产系统、投资决策辅助、或任何涉及真实金融资产的活动。
    - 任何因使用、修改或分发本项目而导致的直接或间接损失（包括但不限于资金损失、数据丢失、系统崩溃），**作者概不负责**。

### 性能特征

- **低延迟**: 基于Rust的零成本抽象，毫秒级响应
- **高吞吐**: 异步设计支持数千并发连接
- **内存安全**: 编译期保证，运行时零crash
- **资源效率**: 低内存占用，适合长期运行

### 扩展开发

- **新增指标**: 实现标准trait即可集成
- **自定义策略**: 继承Strategy trait开发新策略
- **多数据源**: 通过DataSource trait接入新的交易所
- **风控模块**: 在Portfolio层添加风险管理逻辑

## 文档与支持

### 完整文档
- [软件需求规格说明书](docs/软件需求规格说明书.md) - 详细的功能需求和验收标准
- [技术方案说明书](docs/技术方案说明书.md) - 系统架构和技术实现细节
- [项目大纲](docs/大纲.md) - 开发阶段和里程碑规划
- [项目约定](docs/项目约定.md) - 开发规范和测试策略

### API文档
```bash
# 生成本地文档
cargo doc --open

# 查看特定模块文档
cargo doc -p aurora-core --open
cargo doc -p aurora-strategy --open
```

### 示例代码
每个模块都包含详细的使用示例和文档测试，可作为学习参考。

### 贡献指南
- 遵循Rust社区编码规范
- 所有新功能必须包含测试
- 提交前运行 `cargo test` 和 `cargo clippy`
- 使用 `cargo fmt` 格式化代码

## 版本信息

- **当前版本**: 0.1.0
- **Rust版本要求**: 1.70+
- **最后更新**: 2025年10月13日

## 许可证

本项目采用Apache 2.0许可证。详情请参阅[LICENSE](LICENSE)文件。

---

**Aurora** - 让量化交易研究更加高效、安全、可靠 🚀