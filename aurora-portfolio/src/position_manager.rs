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

//! 仓位管理模块
//!
//! 提供多种仓位管理策略,用于计算每次交易应该使用多少资金。
//! 支持固定金额、固定比例、Kelly准则、金字塔加仓等策略。

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// 仓位管理策略
///
/// 定义了不同的资金分配方式,用于控制每次交易的风险敞口。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PositionSizingStrategy {
    /// 固定金额策略 - 每次交易使用固定金额
    ///
    /// 参数: 固定交易金额
    FixedAmount(f64),

    /// 固定比例策略 - 每次交易使用账户总权益的固定比例
    ///
    /// 参数: 比例(0.0-1.0),例如0.1表示使用10%的资金
    FixedPercentage(f64),

    /// Kelly准则 - 根据胜率和盈亏比动态调整仓位
    ///
    /// 参数: (胜率, 盈亏比), Kelly比例 = (胜率 * 盈亏比 - (1 - 胜率)) / 盈亏比
    /// 
    /// 胜率应在0-1之间,盈亏比应大于0
    KellyCriterion {
        /// 历史胜率(0.0-1.0)
        win_rate: f64,
        /// 平均盈亏比(平均盈利/平均亏损)
        profit_loss_ratio: f64,
        /// Kelly系数调整因子(0.0-1.0),用于降低风险,如0.5表示使用半凯利
        kelly_fraction: f64,
    },

    /// 金字塔加仓策略 - 在盈利时逐步增加仓位
    ///
    /// 参数: (初始比例, 加仓阈值, 最大比例)
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

    /// 全仓策略 - 使用所有可用资金(不推荐,风险极高)
    AllIn,
}

/// 仓位管理器
///
/// 根据选定的策略计算每次交易应使用的资金量。
#[derive(Debug, Clone)]
pub struct PositionManager {
    /// 当前使用的策略
    strategy: PositionSizingStrategy,
    
    /// 最小交易金额(避免交易金额过小)
    min_position_value: f64,
    
    /// 最大杠杆倍数(1.0表示无杠杆)
    max_leverage: f64,
}

impl PositionManager {
    /// 创建新的仓位管理器
    ///
    /// # 参数
    ///
    /// * `strategy` - 仓位管理策略
    ///
    /// # 返回值
    ///
    /// 返回新的仓位管理器实例
    pub fn new(strategy: PositionSizingStrategy) -> Self {
        Self {
            strategy,
            min_position_value: 10.0, // 默认最小10单位货币
            max_leverage: 1.0,         // 默认无杠杆
        }
    }

    /// 设置最小交易金额
    pub fn with_min_position_value(mut self, min_value: f64) -> Self {
        self.min_position_value = min_value;
        self
    }

    /// 设置最大杠杆倍数
    pub fn with_max_leverage(mut self, leverage: f64) -> Self {
        self.max_leverage = leverage.max(1.0);
        self
    }

    /// 计算交易仓位大小
    ///
    /// # 参数
    ///
    /// * `current_equity` - 当前账户总权益
    /// * `current_profit` - 当前未实现盈亏百分比(用于金字塔策略)
    ///
    /// # 返回值
    ///
    /// 返回建议的交易金额
    pub fn calculate_position_size(
        &self,
        current_equity: f64,
        current_profit: f64,
    ) -> Result<f64> {
        if current_equity <= 0.0 {
            return Err(anyhow!("账户权益必须大于0"));
        }

        let position_size = match &self.strategy {
            PositionSizingStrategy::FixedAmount(amount) => *amount,

            PositionSizingStrategy::FixedPercentage(percentage) => {
                self.validate_percentage(*percentage)?;
                current_equity * percentage
            }

            PositionSizingStrategy::KellyCriterion {
                win_rate,
                profit_loss_ratio,
                kelly_fraction,
            } => {
                self.validate_percentage(*win_rate)?;
                self.validate_kelly_params(*profit_loss_ratio, *kelly_fraction)?;
                self.calculate_kelly_size(
                    current_equity,
                    *win_rate,
                    *profit_loss_ratio,
                    *kelly_fraction,
                )
            }

            PositionSizingStrategy::Pyramid {
                initial_percentage,
                profit_threshold,
                max_percentage,
                increment,
            } => {
                self.validate_percentage(*initial_percentage)?;
                self.validate_percentage(*max_percentage)?;
                self.calculate_pyramid_size(
                    current_equity,
                    current_profit,
                    *initial_percentage,
                    *profit_threshold,
                    *max_percentage,
                    *increment,
                )
            }

            PositionSizingStrategy::AllIn => current_equity,
        };

        // 应用杠杆
        let leveraged_size = position_size * self.max_leverage;

        // 确保不低于最小交易金额
        let final_size = leveraged_size.max(self.min_position_value);

        // 确保不超过账户权益(考虑杠杆后)
        let max_size = current_equity * self.max_leverage;
        Ok(final_size.min(max_size))
    }

    /// 计算Kelly仓位大小
    fn calculate_kelly_size(
        &self,
        equity: f64,
        win_rate: f64,
        pl_ratio: f64,
        kelly_fraction: f64,
    ) -> f64 {
        // Kelly公式: f = (p * b - q) / b
        // 其中: f = 建议仓位比例, p = 胜率, q = 败率(1-p), b = 盈亏比
        let kelly_percentage = (win_rate * pl_ratio - (1.0 - win_rate)) / pl_ratio;
        
        // Kelly比例可能为负(表示不应该交易),取0和计算值的最大值
        let adjusted_kelly = (kelly_percentage * kelly_fraction).max(0.0);
        
        // 限制最大比例不超过100%
        let final_percentage = adjusted_kelly.min(1.0);
        
        equity * final_percentage
    }

    /// 计算金字塔仓位大小
    fn calculate_pyramid_size(
        &self,
        equity: f64,
        current_profit: f64,
        initial_pct: f64,
        profit_threshold: f64,
        max_pct: f64,
        increment: f64,
    ) -> f64 {
        let mut percentage = initial_pct;

        // 根据当前盈利百分比决定加仓次数
        if current_profit > 0.0 {
            let add_times = (current_profit / profit_threshold).floor() as i32;
            for _ in 0..add_times {
                percentage += increment;
                if percentage >= max_pct {
                    percentage = max_pct;
                    break;
                }
            }
        }

        equity * percentage
    }

    /// 验证百分比参数
    fn validate_percentage(&self, percentage: f64) -> Result<()> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(anyhow!("百分比必须在0到1之间,当前值: {}", percentage));
        }
        Ok(())
    }

    /// 验证Kelly参数
    fn validate_kelly_params(&self, pl_ratio: f64, kelly_fraction: f64) -> Result<()> {
        if pl_ratio <= 0.0 {
            return Err(anyhow!("盈亏比必须大于0,当前值: {}", pl_ratio));
        }
        if kelly_fraction < 0.0 || kelly_fraction > 1.0 {
            return Err(anyhow!(
                "Kelly系数必须在0到1之间,当前值: {}",
                kelly_fraction
            ));
        }
        Ok(())
    }

    /// 获取当前策略
    pub fn get_strategy(&self) -> &PositionSizingStrategy {
        &self.strategy
    }

    /// 更新策略
    pub fn set_strategy(&mut self, strategy: PositionSizingStrategy) {
        self.strategy = strategy;
    }
}

#[cfg(test)]
mod tests;
