# aurora-config

配置管理库,为Aurora量化交易框架提供统一的配置文件支持。

## 功能特性

- 支持TOML格式配置文件
- 数据源配置(API密钥、URL等)
- 策略参数配置
- 投资组合配置(初始资金、手续费等)
- 日志配置
- 配置验证和默认值

## 使用方法

```rust
use aurora_config::Config;

// 从文件加载配置
let config = Config::from_file("config.toml")?;

// 访问配置
let initial_cash = config.portfolio.initial_cash;
let strategy_params = &config.strategies[0].parameters;
```

## 配置文件示例

参见 `examples/config_example.toml`
