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

//! 配置加载和验证功能
//!
//! 提供配置文件的读取、解析和验证功能

use crate::error::{ConfigError, ConfigResult};
use crate::types::Config;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

impl Config {
    /// 从TOML文件加载配置
    ///
    /// # 参数
    /// * `path` - 配置文件路径
    ///
    /// # 返回
    /// 成功返回配置对象,失败返回错误
    ///
    /// # 示例
    /// ```no_run
    /// use aurora_config::Config;
    ///
    /// let config = Config::from_file("config.toml").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigResult<Self> {
        let path = path.as_ref();
        info!("正在加载配置文件: {}", path.display());

        // 读取文件内容
        let content = fs::read_to_string(path).map_err(|e| {
            ConfigError::FileRead(std::io::Error::new(
                e.kind(),
                format!("无法读取配置文件 {}: {}", path.display(), e),
            ))
        })?;

        // 解析TOML
        let config: Config = toml::from_str(&content)?;

        debug!("配置文件解析成功");

        // 验证配置
        config.validate()?;

        info!("配置加载完成");
        Ok(config)
    }

    /// 从TOML字符串加载配置
    ///
    /// # 参数
    /// * `content` - TOML格式的配置内容
    ///
    /// # 返回
    /// 成功返回配置对象,失败返回错误
    pub fn from_str(content: &str) -> ConfigResult<Self> {
        let config: Config = toml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    /// 验证配置有效性
    ///
    /// 检查配置的各个字段是否符合要求
    fn validate(&self) -> ConfigResult<()> {
        // 验证投资组合配置
        self.validate_portfolio()?;

        // 验证策略配置
        self.validate_strategies()?;

        // 验证回测配置
        if let Some(ref backtest) = self.backtest {
            self.validate_backtest(backtest)?;
        }

        // 验证实时交易配置
        if let Some(ref live) = self.live {
            self.validate_live(live)?;
        }

        // 验证日志配置
        self.validate_logging()?;

        Ok(())
    }

    /// 验证投资组合配置
    fn validate_portfolio(&self) -> ConfigResult<()> {
        // 检查初始资金
        if self.portfolio.initial_cash <= 0.0 {
            return Err(ConfigError::InvalidValue {
                field: "portfolio.initial_cash".to_string(),
                value: self.portfolio.initial_cash.to_string(),
                reason: "初始资金必须大于0".to_string(),
            });
        }

        // 检查手续费率
        if self.portfolio.commission < 0.0 || self.portfolio.commission >= 1.0 {
            return Err(ConfigError::InvalidValue {
                field: "portfolio.commission".to_string(),
                value: self.portfolio.commission.to_string(),
                reason: "手续费率必须在[0, 1)范围内".to_string(),
            });
        }

        // 检查滑点率
        if self.portfolio.slippage < 0.0 || self.portfolio.slippage >= 1.0 {
            return Err(ConfigError::InvalidValue {
                field: "portfolio.slippage".to_string(),
                value: self.portfolio.slippage.to_string(),
                reason: "滑点率必须在[0, 1)范围内".to_string(),
            });
        }

        // 检查最大持仓金额
        if let Some(max_size) = self.portfolio.max_position_size {
            if max_size <= 0.0 {
                return Err(ConfigError::InvalidValue {
                    field: "portfolio.max_position_size".to_string(),
                    value: max_size.to_string(),
                    reason: "最大持仓金额必须大于0".to_string(),
                });
            }
        }

        // 检查最大持仓数量
        if let Some(max_pos) = self.portfolio.max_positions {
            if max_pos == 0 {
                return Err(ConfigError::InvalidValue {
                    field: "portfolio.max_positions".to_string(),
                    value: max_pos.to_string(),
                    reason: "最大持仓数量必须大于0".to_string(),
                });
            }
        }

        Ok(())
    }

    /// 验证策略配置
    fn validate_strategies(&self) -> ConfigResult<()> {
        if self.strategies.is_empty() {
            return Err(ConfigError::Validation(
                "至少需要配置一个策略".to_string(),
            ));
        }

        for (idx, strategy) in self.strategies.iter().enumerate() {
            // 检查策略名称
            if strategy.name.is_empty() {
                return Err(ConfigError::InvalidValue {
                    field: format!("strategies[{}].name", idx),
                    value: "".to_string(),
                    reason: "策略名称不能为空".to_string(),
                });
            }

            // 检查策略类型
            if strategy.strategy_type.is_empty() {
                return Err(ConfigError::InvalidValue {
                    field: format!("strategies[{}].strategy_type", idx),
                    value: "".to_string(),
                    reason: "策略类型不能为空".to_string(),
                });
            }
        }

        Ok(())
    }

    /// 验证回测配置
    fn validate_backtest(&self, backtest: &crate::types::BacktestConfig) -> ConfigResult<()> {
        // 检查数据文件路径
        if backtest.data_path.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "backtest.data_path".to_string(),
                value: "".to_string(),
                reason: "数据文件路径不能为空".to_string(),
            });
        }

        Ok(())
    }

    /// 验证实时交易配置
    fn validate_live(&self, live: &crate::types::LiveConfig) -> ConfigResult<()> {
        // 检查交易对符号
        if live.symbol.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "live.symbol".to_string(),
                value: "".to_string(),
                reason: "交易对符号不能为空".to_string(),
            });
        }

        // 检查时间间隔
        if live.interval.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "live.interval".to_string(),
                value: "".to_string(),
                reason: "时间间隔不能为空".to_string(),
            });
        }

        Ok(())
    }

    /// 验证日志配置
    fn validate_logging(&self) -> ConfigResult<()> {
        // 检查日志级别
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(ConfigError::InvalidValue {
                field: "logging.level".to_string(),
                value: self.logging.level.clone(),
                reason: format!("日志级别必须是以下之一: {:?}", valid_levels),
            });
        }

        // 检查日志格式
        let valid_formats = ["json", "pretty"];
        if !valid_formats.contains(&self.logging.format.as_str()) {
            return Err(ConfigError::InvalidValue {
                field: "logging.format".to_string(),
                value: self.logging.format.clone(),
                reason: format!("日志格式必须是以下之一: {:?}", valid_formats),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "loader/tests.rs"]
mod tests;
