# Aurora Data

Aurora 数据采集和处理库 - 为量化交易系统提供历史数据和实时数据流

## 概述

`aurora-data` 是 Aurora 量化交易框架的数据层组件，负责从各种数据源获取、验证和存储市场数据。它支持从交易所 REST API 获取历史数据，通过 WebSocket 接收实时数据流，并提供统一的数据加载接口。

## 主要功能

### 🔽 历史数据获取
- 从 Binance REST API 下载 K线历史数据
- 支持多种时间周期（1m, 5m, 1h, 1d 等）
- 批量获取大量历史数据
- 自动处理 API 请求限制

### 📊 实时数据流
- 通过 WebSocket 连接获取实时市场数据
- 支持 K线流和逐笔成交流
- 自动重连机制
- 事件驱动的数据处理

### 💾 数据存储
- 将数据保存为 CSV 格式
- 自动创建目录结构
- 数据完整性验证

### 📁 数据加载
- 从 CSV 文件加载历史数据
- 数据验证和清洗
- 自动排序和格式化

## 架构组织

```
aurora-data/
├── historical/          # 历史数据模块
│   ├── downloader.rs   # Binance 下载器实现
│   ├── types.rs        # 数据类型定义
│   └── utils.rs        # 工具函数
├── live/               # 实时数据模块
│   ├── stream.rs       # WebSocket 流实现
│   └── utils.rs        # 工具函数
└── loader.rs           # 数据加载器
```

## 核心组件

### BinanceHistoricalDownloader - 历史数据下载器

从 Binance 交易所获取历史 K线数据：

```rust
use aurora_data::BinanceHistoricalDownloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = BinanceHistoricalDownloader::new();
    
    // 下载 BTCUSDT 的 1小时 K线数据
    downloader.download_klines(
        "BTCUSDT",           // 交易对
        "1h",                // 时间周期
        1640995200000,       // 开始时间戳
        1641081600000,       // 结束时间戳
        "btc_1h.csv"         // 输出文件
    ).await?;
    
    Ok(())
}
```

**支持的时间周期**: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M

### BinanceLiveStream - 实时数据流

通过 WebSocket 接收实时市场数据：

```rust
use aurora_data::BinanceLiveStream;
use aurora_core::DataSource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = BinanceLiveStream::new();
    
    // 连接到实时数据流
    stream.connect(&["BTCUSDT", "ETHUSDT"]).await?;
    
    // 接收数据
    let mut receiver = stream.start().await?;
    
    while let Some(event) = receiver.recv().await {
        match event {
            MarketEvent::Kline(kline) => {
                println!("收到K线: {} - 收盘价: {}", 
                    kline.timestamp, kline.close);
            }
        }
    }
    
    Ok(())
}
```

### CsvDataLoader - CSV 数据加载器

从 CSV 文件加载历史数据：

```rust
use aurora_data::CsvDataLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = CsvDataLoader::new();
    
    // 加载 CSV 数据
    let klines = loader.load_from_csv("btc_1h.csv")?;
    
    println!("成功加载 {} 条K线数据", klines.len());
    
    // 数据已按时间戳排序
    for kline in klines.iter().take(5) {
        println!("时间: {}, 收盘价: {}", kline.timestamp, kline.close);
    }
    
    Ok(())
}
```

## 命令行工具

`aurora-data` 提供了一个功能完整的命令行工具：

### 安装

```bash
cargo install --path aurora-data
```

### 下载历史数据

```bash
# 基本用法
aurora-data download --symbol BTCUSDT --interval 1h --output btc_1h.csv

# 指定时间范围
aurora-data download \
  --symbol ETHUSDT \
  --interval 4h \
  --start-time "2024-01-01" \
  --end-time "2024-12-31" \
  --output eth_4h.csv

# 下载日线数据
aurora-data download --symbol BNBUSDT --interval 1d --output bnb_daily.csv
```

### 接收实时数据流

```bash
# K线数据流
aurora-data stream --symbol BTCUSDT --stream-type kline --interval 1m

# 多个交易对
aurora-data stream --symbol BTCUSDT --interval 5m
aurora-data stream --symbol ETHUSDT --interval 5m
```

### 命令行参数

#### download 命令

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--symbol` | `-s` | 交易对符号 (如 BTCUSDT) | 必需 |
| `--interval` | `-i` | 时间间隔 (如 1m, 1h, 1d) | 1h |
| `--start-time` | | 开始时间 (如 2024-01-01) | 可选 |
| `--end-time` | | 结束时间 (如 2024-12-31) | 可选 |
| `--output` | `-o` | 输出文件路径 | 可选 |

#### stream 命令

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--symbol` | `-s` | 交易对符号 | 必需 |
| `--stream-type` | | 流类型 (kline/trade) | kline |
| `--interval` | `-i` | 时间间隔 (仅kline) | 1m |

## 配置选项

### DataSourceConfig - 数据源配置

自定义数据源的行为：

```rust
use aurora_data::DataSourceConfig;

// 使用默认配置
let config = DataSourceConfig::default();

// 自定义配置
let config = DataSourceConfig::new("https://api.binance.com")
    .with_timeout(60)          // 60秒超时
    .with_max_retries(5);      // 最多重试5次

let downloader = BinanceHistoricalDownloader::with_config(config);
```

**配置项**:
- `base_url`: API 基础 URL
- `timeout_secs`: 请求超时时间（秒）
- `max_retries`: 最大重试次数

## 错误处理

`aurora-data` 定义了详细的错误类型：

```rust
pub enum DataError {
    NetworkError(String),      // 网络连接错误
    ApiError(String),          // API 响应错误
    ParseError(String),        // 数据解析错误
    ValidationError(String),   // 数据验证错误
    FileNotFound(String),      // 文件不存在
    IoError(String),           // IO 错误
    WebSocketError(String),    // WebSocket 错误
}
```

**错误处理示例**:

```rust
use aurora_data::{BinanceHistoricalDownloader, DataError};

async fn download_with_error_handling() {
    let downloader = BinanceHistoricalDownloader::new();
    
    match downloader.download_klines("BTCUSDT", "1h", None, None, Some(1000)).await {
        Ok(klines) => println!("成功下载 {} 条数据", klines.len()),
        Err(e) => match e.downcast_ref::<DataError>() {
            Some(DataError::NetworkError(msg)) => {
                eprintln!("网络错误: {}", msg);
            }
            Some(DataError::ApiError(msg)) => {
                eprintln!("API错误: {}", msg);
            }
            _ => eprintln!("其他错误: {}", e),
        }
    }
}
```

## 数据验证

所有加载的数据都会经过自动验证：

- ✅ 价格数据合理性检查 (high >= low, open/close 在 high/low 范围内)
- ✅ 成交量非负检查
- ✅ 时间戳有效性检查
- ✅ 数据完整性检查
- ✅ 自动排序（按时间戳）

## 功能特性

### 自动重试机制

网络请求失败时自动重试：

```rust
// 配置重试次数
let config = DataSourceConfig::default()
    .with_max_retries(3);  // 失败后重试3次
```

### 请求频率限制

自动处理 API 请求频率限制，避免被限流：

```rust
// 下载器会自动在请求之间添加适当的延迟
downloader.download_klines("BTCUSDT", "1h", start, end, "output.csv").await?;
```

### WebSocket 自动重连

实时数据流断连时自动重连：

```rust
// 连接失败或断开时会自动尝试重连
let mut stream = BinanceLiveStream::new();
stream.connect(&["BTCUSDT"]).await?;
```

## 性能优化

- 🚀 异步 I/O 操作，高并发性能
- 📦 批量数据获取，减少 API 调用次数
- 💨 流式数据处理，低内存占用
- ⚡ 高效的 CSV 解析和序列化

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.3"
clap = { version = "4.4", features = ["derive"] }
tracing = "0.1"
anyhow = "1.0"
```

## 使用示例

### 完整的数据下载流程

```rust
use aurora_data::{BinanceHistoricalDownloader, DataSourceConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 创建下载器
    let config = DataSourceConfig::default();
    let downloader = BinanceHistoricalDownloader::with_config(config);
    
    info!("开始下载BTC历史数据...");
    
    // 下载最近1000条1小时K线
    let klines = downloader.download_klines(
        "BTCUSDT",
        "1h",
        None,
        None,
        Some(1000)
    ).await?;
    
    info!("下载完成，共 {} 条数据", klines.len());
    
    // 保存到CSV
    downloader.save_to_csv(&klines, "btc_latest.csv")?;
    
    Ok(())
}
```

### 实时数据流处理

```rust
use aurora_data::BinanceLiveStream;
use aurora_core::{DataSource, MarketEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = BinanceLiveStream::new();
    
    // 连接多个交易对
    stream.connect(&["BTCUSDT", "ETHUSDT", "BNBUSDT"]).await?;
    
    // 启动数据流
    let mut receiver = stream.start().await?;
    
    // 处理实时数据
    let mut count = 0;
    while let Some(event) = receiver.recv().await {
        if let MarketEvent::Kline(kline) = event {
            count += 1;
            println!("[{}] 时间: {}, 价格: {:.2}", 
                count, kline.timestamp, kline.close);
            
            // 处理100条后退出
            if count >= 100 {
                break;
            }
        }
    }
    
    Ok(())
}
```

## 测试

运行测试套件：

```bash
# 运行所有测试
cargo test --package aurora-data

# 运行集成测试
cargo test --package aurora-data --test integration_tests

# 运行特定模块测试
cargo test --package aurora-data historical::
cargo test --package aurora-data live::
```

## 日志配置

使用 `RUST_LOG` 环境变量控制日志级别：

```bash
# 显示所有日志
RUST_LOG=debug cargo run --bin aurora-data

# 只显示 aurora-data 的 info 级别日志
RUST_LOG=aurora_data=info cargo run --bin aurora-data

# 显示详细的追踪信息
RUST_LOG=aurora_data=trace cargo run --bin aurora-data
```

## 扩展性

### 添加新的数据源

实现 `DataSource` trait 即可添加新的数据源：

```rust
use aurora_core::{DataSource, MarketEvent};
use async_trait::async_trait;

struct CustomDataSource;

#[async_trait]
impl DataSource for CustomDataSource {
    async fn start(&mut self) -> Result<UnboundedReceiver<MarketEvent>> {
        // 实现自定义数据源逻辑
        todo!()
    }
}
```

### 支持新的交易所

参考 `BinanceHistoricalDownloader` 的实现，创建新的下载器类。

## 相关 Crate

- **aurora-core**: 提供核心数据结构和接口定义
- **aurora-backtester**: 使用历史数据进行策略回测
- **aurora-live**: 使用实时数据进行实盘交易

## 版本

当前版本: **0.1.0**

## 许可证

本项目的许可证信息请参考根目录的 LICENSE 文件。
