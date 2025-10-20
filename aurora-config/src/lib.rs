// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Aurora配置管理库
//!
//! 本库为Aurora量化交易框架提供统一的配置文件支持。
//!
//! # 功能特性
//!
//! - 支持TOML格式配置文件
//! - 数据源配置(API密钥、URL等)
//! - 策略参数配置
//! - 投资组合配置(初始资金、手续费等)
//! - 风险管理配置(止损止盈、回撤限制等)
//! - 仓位管理配置(多种策略支持)
//! - 日志配置
//! - 配置验证和默认值
//!
//! # 使用示例
//!
//! ```no_run
//! use aurora_config::Config;
//!
//! // 从文件加载配置
//! let config = Config::from_file("config.toml").unwrap();
//!
//! // 访问配置
//! let initial_cash = config.portfolio.initial_cash;
//! let strategy_name = &config.strategies[0].name;
//! ```
//!
//! # 配置文件格式
//!
//! ```toml
//! [data_source]
//! provider = "binance"
//! timeout = 30
//!
//! [[strategies]]
//! name = "MA交叉策略"
//! strategy_type = "ma-crossover"
//! enabled = true
//!
//! [strategies.parameters]
//! short = { Integer = 10 }
//! long = { Integer = 30 }
//!
//! [portfolio]
//! initial_cash = 10000.0
//! commission = 0.001
//!
//! [logging]
//! level = "info"
//! format = "pretty"
//! ```

mod error;
mod loader;
mod types;

// 重新导出公共API
pub use error::{ConfigError, ConfigResult};
pub use types::{
    BacktestConfig, Config, DataSourceConfig, LiveConfig, LogConfig, PortfolioConfig,
    PositionSizingConfig, PricingModeConfig, RiskRulesConfig, StrategyConfig, StrategyParameter,
};
