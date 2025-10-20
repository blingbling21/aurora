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

//! 配置数据结构定义
//!
//! 本模块定义了Aurora框架的所有配置数据结构,包括:
//! - 数据源配置
//! - 策略配置
//! - 投资组合配置
//! - 日志配置
//! - 回测配置
//! - 实时交易配置

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 根配置结构
///
/// 包含所有子配置项,是配置文件的顶层结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 数据源配置
    #[serde(default)]
    pub data_source: DataSourceConfig,

    /// 策略列表
    #[serde(default)]
    pub strategies: Vec<StrategyConfig>,

    /// 投资组合配置
    #[serde(default)]
    pub portfolio: PortfolioConfig,

    /// 日志配置
    #[serde(default)]
    pub logging: LogConfig,

    /// 回测配置(可选)
    #[serde(default)]
    pub backtest: Option<BacktestConfig>,

    /// 实时交易配置(可选)
    #[serde(default)]
    pub live: Option<LiveConfig>,
}

/// 数据源配置
///
/// 配置数据获取相关的参数,如API密钥、URL等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    /// 数据提供商(如: binance, okx)
    #[serde(default = "default_provider")]
    pub provider: String,

    /// API密钥(可选,用于私有API)
    #[serde(default)]
    pub api_key: Option<String>,

    /// API密钥(可选,用于私有API)
    #[serde(default)]
    pub api_secret: Option<String>,

    /// 基础URL(可选,用于自定义API端点)
    #[serde(default)]
    pub base_url: Option<String>,

    /// WebSocket URL(可选,用于实时数据流)
    #[serde(default)]
    pub ws_url: Option<String>,

    /// 连接超时(秒)
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// 最大重试次数
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

/// 策略配置
///
/// 定义单个交易策略的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    /// 策略名称
    pub name: String,

    /// 策略类型(如: ma-crossover, rsi-oversold)
    pub strategy_type: String,

    /// 策略参数(key-value格式)
    #[serde(default)]
    pub parameters: HashMap<String, StrategyParameter>,

    /// 是否启用该策略
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

/// 策略参数值
///
/// 支持多种类型的参数值
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrategyParameter {
    /// 整数参数
    Integer(i64),
    /// 浮点数参数
    Float(f64),
    /// 字符串参数
    String(String),
    /// 布尔值参数
    Bool(bool),
}

/// 投资组合配置
///
/// 配置投资组合管理相关参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConfig {
    /// 初始资金
    #[serde(default = "default_initial_cash")]
    pub initial_cash: f64,

    /// 手续费率(如0.001表示0.1%)
    #[serde(default = "default_commission")]
    pub commission: f64,

    /// 滑点率(如0.0005表示0.05%)
    #[serde(default)]
    pub slippage: f64,

    /// 单笔最大交易金额(可选)
    #[serde(default)]
    pub max_position_size: Option<f64>,

    /// 最大持仓数量(可选)
    #[serde(default)]
    pub max_positions: Option<usize>,

    /// 风险管理规则(可选)
    #[serde(default)]
    pub risk_rules: Option<RiskRulesConfig>,

    /// 仓位管理策略(可选)
    #[serde(default)]
    pub position_sizing: Option<PositionSizingConfig>,
}

/// 风险管理规则配置
///
/// 对应 aurora-portfolio 中的 RiskRules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRulesConfig {
    /// 最大回撤限制(百分比),例如15.0表示15%
    #[serde(default)]
    pub max_drawdown_pct: Option<f64>,

    /// 单日最大亏损限制(百分比)
    #[serde(default)]
    pub max_daily_loss_pct: Option<f64>,

    /// 连续亏损次数限制
    #[serde(default)]
    pub max_consecutive_losses: Option<u32>,

    /// 单笔交易最大亏损限制(百分比)
    #[serde(default)]
    pub max_single_trade_loss_pct: Option<f64>,

    /// 账户最低权益要求
    #[serde(default)]
    pub min_equity: Option<f64>,

    /// 止损百分比(相对于入场价)
    #[serde(default)]
    pub stop_loss_pct: Option<f64>,

    /// 止盈百分比(相对于入场价)
    #[serde(default)]
    pub take_profit_pct: Option<f64>,
}

/// 仓位管理策略配置
///
/// 对应 aurora-portfolio 中的 PositionSizingStrategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "strategy_type", rename_all = "snake_case")]
pub enum PositionSizingConfig {
    /// 固定金额策略
    FixedAmount {
        /// 每次交易的固定金额
        amount: f64,
    },

    /// 固定比例策略
    FixedPercentage {
        /// 使用账户权益的百分比(0.0-1.0)
        percentage: f64,
    },

    /// Kelly准则策略
    KellyCriterion {
        /// 历史胜率(0.0-1.0)
        win_rate: f64,
        /// 平均盈亏比
        profit_loss_ratio: f64,
        /// Kelly系数调整因子(0.0-1.0),如0.5表示半凯利
        kelly_fraction: f64,
    },

    /// 金字塔加仓策略
    Pyramid {
        /// 初始仓位比例(0.0-1.0)
        initial_percentage: f64,
        /// 盈利达到此百分比时加仓
        profit_threshold: f64,
        /// 最大仓位比例(0.0-1.0)
        max_percentage: f64,
        /// 每次加仓的比例增量
        increment: f64,
    },

    /// 全仓策略
    AllIn,
}

/// 日志配置
///
/// 配置日志记录相关参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// 日志级别(trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// 日志格式(json, pretty)
    #[serde(default = "default_log_format")]
    pub format: String,

    /// 日志输出路径(可选,不设置则输出到stdout)
    #[serde(default)]
    pub output: Option<String>,
}

/// 回测配置
///
/// 配置回测引擎特定的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    /// 数据文件路径
    pub data_path: String,

    /// 交易对符号(可选,用于标识)
    #[serde(default)]
    pub symbol: Option<String>,

    /// 时间间隔(可选,用于标识)
    #[serde(default)]
    pub interval: Option<String>,

    /// 开始时间(可选)
    #[serde(default)]
    pub start_time: Option<String>,

    /// 结束时间(可选)
    #[serde(default)]
    pub end_time: Option<String>,

    /// 定价模式配置(可选)
    #[serde(default)]
    pub pricing_mode: Option<PricingModeConfig>,
}

/// 定价模式配置
///
/// 用于控制回测中交易价格的计算方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum PricingModeConfig {
    /// 使用收盘价执行交易(简单模式)
    Close,

    /// 使用买一卖一价执行交易(更真实的模式)
    BidAsk {
        /// 买卖价差百分比(例如 0.001 表示 0.1% 的价差)
        spread_pct: f64,
    },
}

/// 实时交易配置
///
/// 配置实时交易引擎特定的参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveConfig {
    /// 交易对符号
    pub symbol: String,

    /// 时间间隔
    #[serde(default = "default_interval")]
    pub interval: String,

    /// 是否为模拟交易
    #[serde(default = "default_paper_trading")]
    pub paper_trading: bool,
}

// === 默认值函数 ===

fn default_provider() -> String {
    "binance".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_max_retries() -> u32 {
    3
}

fn default_enabled() -> bool {
    true
}

fn default_initial_cash() -> f64 {
    10000.0
}

fn default_commission() -> f64 {
    0.001
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

fn default_interval() -> String {
    "1m".to_string()
}

fn default_paper_trading() -> bool {
    true
}

impl Default for DataSourceConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            api_key: None,
            api_secret: None,
            base_url: None,
            ws_url: None,
            timeout: default_timeout(),
            max_retries: default_max_retries(),
        }
    }
}

impl Default for PortfolioConfig {
    fn default() -> Self {
        Self {
            initial_cash: default_initial_cash(),
            commission: default_commission(),
            slippage: 0.0,
            max_position_size: None,
            max_positions: None,
            risk_rules: None,
            position_sizing: None,
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            output: None,
        }
    }
}

impl StrategyParameter {
    /// 尝试转换为整数
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            StrategyParameter::Integer(v) => Some(*v),
            _ => None,
        }
    }

    /// 尝试转换为usize
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            StrategyParameter::Integer(v) if *v >= 0 => Some(*v as usize),
            _ => None,
        }
    }

    /// 尝试转换为浮点数
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            StrategyParameter::Float(v) => Some(*v),
            StrategyParameter::Integer(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// 尝试转换为字符串
    pub fn as_str(&self) -> Option<&str> {
        match self {
            StrategyParameter::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    /// 尝试转换为布尔值
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StrategyParameter::Bool(v) => Some(*v),
            _ => None,
        }
    }
}

impl RiskRulesConfig {
    /// 转换为 aurora-portfolio 的 RiskRules 类型
    ///
    /// 注意: 需要在使用此函数的 crate 中依赖 aurora-portfolio
    #[cfg(feature = "portfolio-integration")]
    pub fn to_risk_rules(&self) -> aurora_portfolio::RiskRules {
        let mut rules = aurora_portfolio::RiskRules::new();

        if let Some(max_dd) = self.max_drawdown_pct {
            rules = rules.with_max_drawdown(max_dd);
        }
        if let Some(max_daily_loss) = self.max_daily_loss_pct {
            rules = rules.with_max_daily_loss(max_daily_loss);
        }
        if let Some(max_losses) = self.max_consecutive_losses {
            rules = rules.with_max_consecutive_losses(max_losses);
        }
        if let Some(max_single_loss) = self.max_single_trade_loss_pct {
            rules = rules.with_max_single_trade_loss(max_single_loss);
        }
        if let Some(min_eq) = self.min_equity {
            rules = rules.with_min_equity(min_eq);
        }

        rules
    }

    /// 检查配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if let Some(max_dd) = self.max_drawdown_pct {
            if max_dd < 0.0 || max_dd > 100.0 {
                return Err(format!("最大回撤必须在0-100之间,当前值: {}", max_dd));
            }
        }
        if let Some(max_daily_loss) = self.max_daily_loss_pct {
            if max_daily_loss < 0.0 || max_daily_loss > 100.0 {
                return Err(format!(
                    "单日最大亏损必须在0-100之间,当前值: {}",
                    max_daily_loss
                ));
            }
        }
        if let Some(stop_loss) = self.stop_loss_pct {
            if stop_loss < 0.0 || stop_loss > 100.0 {
                return Err(format!("止损百分比必须在0-100之间,当前值: {}", stop_loss));
            }
        }
        if let Some(take_profit) = self.take_profit_pct {
            if take_profit < 0.0 {
                return Err(format!("止盈百分比必须大于0,当前值: {}", take_profit));
            }
        }
        Ok(())
    }
}

impl PositionSizingConfig {
    /// 转换为 aurora-portfolio 的 PositionSizingStrategy 类型
    ///
    /// 注意: 需要在使用此函数的 crate 中依赖 aurora-portfolio
    #[cfg(feature = "portfolio-integration")]
    pub fn to_position_sizing_strategy(&self) -> aurora_portfolio::PositionSizingStrategy {
        match self {
            PositionSizingConfig::FixedAmount { amount } => {
                aurora_portfolio::PositionSizingStrategy::FixedAmount(*amount)
            }
            PositionSizingConfig::FixedPercentage { percentage } => {
                aurora_portfolio::PositionSizingStrategy::FixedPercentage(*percentage)
            }
            PositionSizingConfig::KellyCriterion {
                win_rate,
                profit_loss_ratio,
                kelly_fraction,
            } => aurora_portfolio::PositionSizingStrategy::KellyCriterion {
                win_rate: *win_rate,
                profit_loss_ratio: *profit_loss_ratio,
                kelly_fraction: *kelly_fraction,
            },
            PositionSizingConfig::Pyramid {
                initial_percentage,
                profit_threshold,
                max_percentage,
                increment,
            } => aurora_portfolio::PositionSizingStrategy::Pyramid {
                initial_percentage: *initial_percentage,
                profit_threshold: *profit_threshold,
                max_percentage: *max_percentage,
                increment: *increment,
            },
            PositionSizingConfig::AllIn => aurora_portfolio::PositionSizingStrategy::AllIn,
        }
    }

    /// 检查配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        match self {
            PositionSizingConfig::FixedAmount { amount } => {
                if *amount <= 0.0 {
                    return Err(format!("固定金额必须大于0,当前值: {}", amount));
                }
            }
            PositionSizingConfig::FixedPercentage { percentage } => {
                if *percentage <= 0.0 || *percentage > 1.0 {
                    return Err(format!(
                        "固定比例必须在0-1之间,当前值: {}",
                        percentage
                    ));
                }
            }
            PositionSizingConfig::KellyCriterion {
                win_rate,
                profit_loss_ratio,
                kelly_fraction,
            } => {
                if *win_rate < 0.0 || *win_rate > 1.0 {
                    return Err(format!("胜率必须在0-1之间,当前值: {}", win_rate));
                }
                if *profit_loss_ratio <= 0.0 {
                    return Err(format!("盈亏比必须大于0,当前值: {}", profit_loss_ratio));
                }
                if *kelly_fraction < 0.0 || *kelly_fraction > 1.0 {
                    return Err(format!(
                        "Kelly系数必须在0-1之间,当前值: {}",
                        kelly_fraction
                    ));
                }
            }
            PositionSizingConfig::Pyramid {
                initial_percentage,
                profit_threshold,
                max_percentage,
                increment,
            } => {
                if *initial_percentage <= 0.0 || *initial_percentage > 1.0 {
                    return Err(format!(
                        "初始比例必须在0-1之间,当前值: {}",
                        initial_percentage
                    ));
                }
                if *max_percentage <= 0.0 || *max_percentage > 1.0 {
                    return Err(format!(
                        "最大比例必须在0-1之间,当前值: {}",
                        max_percentage
                    ));
                }
                if *profit_threshold <= 0.0 {
                    return Err(format!("盈利阈值必须大于0,当前值: {}", profit_threshold));
                }
                if *increment <= 0.0 {
                    return Err(format!("加仓增量必须大于0,当前值: {}", increment));
                }
                if initial_percentage > max_percentage {
                    return Err("初始比例不能大于最大比例".to_string());
                }
            }
            PositionSizingConfig::AllIn => {}
        }
        Ok(())
    }
}

impl PricingModeConfig {
    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        match self {
            PricingModeConfig::Close => Ok(()),
            PricingModeConfig::BidAsk { spread_pct } => {
                if *spread_pct < 0.0 || *spread_pct > 0.1 {
                    return Err(format!(
                        "价差百分比必须在0-0.1之间(0-10%),当前值: {}",
                        spread_pct
                    ));
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
#[path = "types/tests.rs"]
mod tests;
