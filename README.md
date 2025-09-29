# Aurora - 模块化量化交易研究框架

Aurora是一个使用Rust构建的模块化、事件驱动的量化交易研究框架。该框架旨在提供高性能、高安全性的工具，支持从数据采集、策略回测到实时模拟交易的全流程。

## 项目结构

### Cargo Workspace 成员

- **`aurora-core`** (库) - 核心数据结构和trait定义
- **`aurora-data`** (二进制) - 数据采集工具
- **`aurora-indicators`** (库) - 技术指标计算库
- **`aurora-strategy`** (库) - 交易策略实现
- **`aurora-backtester`** (二进制) - 历史回测引擎
- **`aurora-live`** (二进制) - 实时模拟交易引擎

### 架构特点

- **模块化设计**: 各功能组件高度解耦，易于独立开发和测试
- **事件驱动**: 基于市场事件的统一处理架构
- **策略模式**: 通过`Strategy` trait实现可插拔的交易策略
- **异步支持**: 基于tokio的异步运行时，支持高并发数据处理

## 快速开始

### 编译项目
```bash
cargo build
```

### 运行测试
```bash
cargo test --lib
```

### 使用工具

#### 1. 数据采集 (aurora-data)
```bash
# 下载历史数据
cargo run -p aurora-data -- download --symbol BTCUSDT --interval 1h --output btc_1h.csv

# 接收实时数据流
cargo run -p aurora-data -- stream --symbol BTCUSDT --interval 1m
```

#### 2. 历史回测 (aurora-backtester)
```bash
# 运行MA交叉策略回测
cargo run -p aurora-backtester -- --data-path btc_1h.csv --short 10 --long 30 --initial-cash 10000
```

#### 3. 实时模拟交易 (aurora-live)
```bash
# 启动实时模拟交易
cargo run -p aurora-live -- --symbol BTCUSDT --short 10 --long 30 --initial-cash 10000
```

## 核心概念

### 数据结构
- `Kline`: K线数据结构，包含OHLCV和时间戳
- `MarketEvent`: 市场事件枚举，统一不同类型的市场数据
- `Signal`: 交易信号枚举 (Buy/Sell/Hold)
- `SignalEvent`: 包含信号和元数据的事件结构

### Trait设计
- `DataSource`: 异步数据源统一接口
- `Strategy`: 策略统一接口，支持事件驱动的信号生成

### 已实现组件
- 移动平均线(MA)指标
- MA交叉策略
- Binance数据源集成
- 投资组合管理
- 回测引擎
- 模拟交易引擎

## 依赖项

主要依赖包括：
- `tokio`: 异步运行时
- `serde`: 序列化/反序列化
- `reqwest`: HTTP客户端
- `tokio-tungstenite`: WebSocket客户端
- `csv`: CSV文件处理
- `clap`: 命令行接口
- `tracing`: 结构化日志

## 开发计划

框架按照以下阶段开发：

- **阶段零**: ✅ 项目结构和基础工具
- **阶段一**: ✅ 数据核心 (数据采集和核心结构)
- **阶段二**: ✅ 策略大脑 (指标和策略实现)
- **阶段三**: ✅ 历史回测引擎
- **阶段四**: ✅ 实时模拟交易引擎

## 注意事项

- 所有实时交易功能均为**模拟交易**，不会执行真实订单
- 当前支持的数据源为Binance公开API
- 建议在生产环境使用前进行充分的回测验证

## 文档

详细的技术文档请参阅：
- [软件需求规格说明书](docs/软件需求规格说明书.md)
- [技术方案说明书](docs/技术方案说明书.md)
- [项目大纲](docs/大纲.md)

## 许可证

本项目采用MIT许可证。详情请参阅LICENSE文件。