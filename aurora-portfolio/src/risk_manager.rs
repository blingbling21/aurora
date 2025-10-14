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

//! 风险管理模块
//!
//! 提供投资组合级别的风险控制功能,包括最大回撤限制、
//! 连续亏损限制等,用于保护账户资金安全。

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 风险控制规则
///
/// 定义投资组合级别的风险限制,当触发任一规则时,
/// 系统应停止交易以保护资金。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskRules {
    /// 最大回撤限制(百分比),例如15.0表示15%
    ///
    /// 当回撤达到此值时停止交易
    pub max_drawdown_pct: Option<f64>,

    /// 单日最大亏损限制(百分比)
    ///
    /// 防止单日损失过大
    pub max_daily_loss_pct: Option<f64>,

    /// 连续亏损次数限制
    ///
    /// 连续亏损达到此次数时停止交易,避免在不利市场中持续损失
    pub max_consecutive_losses: Option<u32>,

    /// 单笔交易最大亏损限制(百分比)
    ///
    /// 限制单笔交易的风险敞口
    pub max_single_trade_loss_pct: Option<f64>,

    /// 账户最低权益要求
    ///
    /// 当账户权益低于此值时停止交易
    pub min_equity: Option<f64>,

    /// 止损价格(用于单个持仓)
    ///
    /// 价格跌破此值时触发止损
    pub stop_loss_price: Option<f64>,

    /// 止盈价格(用于单个持仓)
    ///
    /// 价格涨至此值时触发止盈
    pub take_profit_price: Option<f64>,
}

impl RiskRules {
    /// 创建默认风险规则(无限制)
    pub fn new() -> Self {
        Self {
            max_drawdown_pct: None,
            max_daily_loss_pct: None,
            max_consecutive_losses: None,
            max_single_trade_loss_pct: None,
            min_equity: None,
            stop_loss_price: None,
            take_profit_price: None,
        }
    }

    /// 设置最大回撤限制
    pub fn with_max_drawdown(mut self, pct: f64) -> Self {
        self.max_drawdown_pct = Some(pct);
        self
    }

    /// 设置单日最大亏损限制
    pub fn with_max_daily_loss(mut self, pct: f64) -> Self {
        self.max_daily_loss_pct = Some(pct);
        self
    }

    /// 设置连续亏损次数限制
    pub fn with_max_consecutive_losses(mut self, count: u32) -> Self {
        self.max_consecutive_losses = Some(count);
        self
    }

    /// 设置单笔交易最大亏损限制
    pub fn with_max_single_trade_loss(mut self, pct: f64) -> Self {
        self.max_single_trade_loss_pct = Some(pct);
        self
    }

    /// 设置最低权益要求
    pub fn with_min_equity(mut self, equity: f64) -> Self {
        self.min_equity = Some(equity);
        self
    }

    /// 设置止损价格
    pub fn with_stop_loss_price(mut self, price: f64) -> Self {
        self.stop_loss_price = Some(price);
        self
    }

    /// 设置止盈价格
    pub fn with_take_profit_price(mut self, price: f64) -> Self {
        self.take_profit_price = Some(price);
        self
    }
}

impl Default for RiskRules {
    fn default() -> Self {
        Self::new()
    }
}

/// 风险检查结果
///
/// 描述风险检查的结果和触发的规则。
#[derive(Debug, Clone, PartialEq)]
pub enum RiskCheckResult {
    /// 通过所有风险检查
    Pass,

    /// 触发止损
    StopLoss(String),

    /// 触发止盈
    TakeProfit(String),

    /// 达到最大回撤
    MaxDrawdownReached(String),

    /// 达到单日最大亏损
    MaxDailyLossReached(String),

    /// 达到连续亏损次数限制
    MaxConsecutiveLossesReached(String),

    /// 账户权益过低
    MinEquityBreached(String),
}

impl RiskCheckResult {
    /// 检查是否通过风控
    pub fn is_pass(&self) -> bool {
        matches!(self, RiskCheckResult::Pass)
    }

    /// 检查是否需要停止交易
    pub fn should_stop_trading(&self) -> bool {
        !self.is_pass()
    }

    /// 获取风控触发原因
    pub fn get_reason(&self) -> Option<&str> {
        match self {
            RiskCheckResult::Pass => None,
            RiskCheckResult::StopLoss(msg) => Some(msg),
            RiskCheckResult::TakeProfit(msg) => Some(msg),
            RiskCheckResult::MaxDrawdownReached(msg) => Some(msg),
            RiskCheckResult::MaxDailyLossReached(msg) => Some(msg),
            RiskCheckResult::MaxConsecutiveLossesReached(msg) => Some(msg),
            RiskCheckResult::MinEquityBreached(msg) => Some(msg),
        }
    }
}

/// 风险管理器
///
/// 负责执行风险检查,跟踪交易状态,并根据规则判断是否应停止交易。
#[derive(Debug, Clone)]
pub struct RiskManager {
    /// 风险控制规则
    rules: RiskRules,

    /// 连续亏损计数器
    consecutive_losses: u32,

    /// 连续盈利计数器
    consecutive_wins: u32,

    /// 今日起始权益
    daily_start_equity: f64,

    /// 是否已触发交易停止
    trading_stopped: bool,

    /// 停止交易的原因
    stop_reason: Option<String>,

    /// 入场价格(用于计算止损止盈)
    entry_price: Option<f64>,
}

impl RiskManager {
    /// 创建新的风险管理器
    ///
    /// # 参数
    ///
    /// * `rules` - 风险控制规则
    /// * `initial_equity` - 初始权益(用于计算单日亏损)
    pub fn new(rules: RiskRules, initial_equity: f64) -> Self {
        Self {
            rules,
            consecutive_losses: 0,
            consecutive_wins: 0,
            daily_start_equity: initial_equity,
            trading_stopped: false,
            stop_reason: None,
            entry_price: None,
        }
    }

    /// 执行完整的风险检查
    ///
    /// # 参数
    ///
    /// * `current_equity` - 当前账户权益
    /// * `current_drawdown` - 当前回撤百分比
    /// * `current_price` - 当前市场价格
    ///
    /// # 返回值
    ///
    /// 返回风险检查结果
    pub fn check_risk(
        &mut self,
        current_equity: f64,
        current_drawdown: f64,
        current_price: f64,
    ) -> RiskCheckResult {
        // 如果已经停止交易,直接返回
        if self.trading_stopped {
            return RiskCheckResult::MaxDrawdownReached(
                self.stop_reason.clone().unwrap_or_default(),
            );
        }

        // 检查最大回撤
        if let Some(max_dd) = self.rules.max_drawdown_pct {
            if current_drawdown >= max_dd {
                let msg = format!(
                    "触发最大回撤限制: 当前回撤{:.2}% >= 限制{:.2}%",
                    current_drawdown, max_dd
                );
                error!("{}", msg);
                self.stop_trading(msg.clone());
                return RiskCheckResult::MaxDrawdownReached(msg);
            }
        }

        // 检查单日最大亏损
        if let Some(max_daily_loss) = self.rules.max_daily_loss_pct {
            let daily_loss_pct =
                ((self.daily_start_equity - current_equity) / self.daily_start_equity) * 100.0;
            if daily_loss_pct >= max_daily_loss {
                let msg = format!(
                    "触发单日最大亏损限制: 当前亏损{:.2}% >= 限制{:.2}%",
                    daily_loss_pct, max_daily_loss
                );
                error!("{}", msg);
                self.stop_trading(msg.clone());
                return RiskCheckResult::MaxDailyLossReached(msg);
            }
        }

        // 检查连续亏损次数
        if let Some(max_losses) = self.rules.max_consecutive_losses {
            if self.consecutive_losses >= max_losses {
                let msg = format!(
                    "触发连续亏损限制: 连续亏损{}次 >= 限制{}次",
                    self.consecutive_losses, max_losses
                );
                error!("{}", msg);
                self.stop_trading(msg.clone());
                return RiskCheckResult::MaxConsecutiveLossesReached(msg);
            }
        }

        // 检查最低权益
        if let Some(min_eq) = self.rules.min_equity {
            if current_equity <= min_eq {
                let msg = format!(
                    "触发最低权益限制: 当前权益{:.2} <= 限制{:.2}",
                    current_equity, min_eq
                );
                error!("{}", msg);
                self.stop_trading(msg.clone());
                return RiskCheckResult::MinEquityBreached(msg);
            }
        }

        // 检查止损价格
        if let Some(stop_price) = self.rules.stop_loss_price {
            if current_price <= stop_price {
                let msg = format!(
                    "触发止损: 当前价格{:.2} <= 止损价{:.2}",
                    current_price, stop_price
                );
                warn!("{}", msg);
                return RiskCheckResult::StopLoss(msg);
            }
        }

        // 检查止盈价格
        if let Some(take_profit) = self.rules.take_profit_price {
            if current_price >= take_profit {
                let msg = format!(
                    "触发止盈: 当前价格{:.2} >= 止盈价{:.2}",
                    current_price, take_profit
                );
                info!("{}", msg);
                return RiskCheckResult::TakeProfit(msg);
            }
        }

        RiskCheckResult::Pass
    }

    /// 记录交易结果
    ///
    /// # 参数
    ///
    /// * `is_profitable` - 交易是否盈利
    pub fn record_trade_result(&mut self, is_profitable: bool) {
        if is_profitable {
            self.consecutive_wins += 1;
            self.consecutive_losses = 0;
        } else {
            self.consecutive_losses += 1;
            self.consecutive_wins = 0;
        }
    }

    /// 重置单日统计(在每日开始时调用)
    ///
    /// # 参数
    ///
    /// * `current_equity` - 当日起始权益
    pub fn reset_daily_stats(&mut self, current_equity: f64) {
        self.daily_start_equity = current_equity;
    }

    /// 设置入场价格
    pub fn set_entry_price(&mut self, price: f64) {
        self.entry_price = Some(price);
    }

    /// 根据入场价格和风险百分比计算止损价格
    ///
    /// # 参数
    ///
    /// * `entry_price` - 入场价格
    /// * `risk_pct` - 风险百分比,例如2.0表示2%
    ///
    /// # 返回值
    ///
    /// 返回计算出的止损价格
    pub fn calculate_stop_loss(&self, entry_price: f64, risk_pct: f64) -> f64 {
        entry_price * (1.0 - risk_pct / 100.0)
    }

    /// 根据入场价格和收益百分比计算止盈价格
    ///
    /// # 参数
    ///
    /// * `entry_price` - 入场价格
    /// * `profit_pct` - 收益百分比,例如5.0表示5%
    ///
    /// # 返回值
    ///
    /// 返回计算出的止盈价格
    pub fn calculate_take_profit(&self, entry_price: f64, profit_pct: f64) -> f64 {
        entry_price * (1.0 + profit_pct / 100.0)
    }

    /// 设置止损止盈价格
    ///
    /// # 参数
    ///
    /// * `entry_price` - 入场价格
    /// * `stop_loss_pct` - 止损百分比
    /// * `take_profit_pct` - 止盈百分比
    pub fn set_stop_loss_take_profit(
        &mut self,
        entry_price: f64,
        stop_loss_pct: f64,
        take_profit_pct: f64,
    ) {
        self.entry_price = Some(entry_price);
        self.rules.stop_loss_price = Some(self.calculate_stop_loss(entry_price, stop_loss_pct));
        self.rules.take_profit_price =
            Some(self.calculate_take_profit(entry_price, take_profit_pct));
    }

    /// 清除止损止盈价格
    pub fn clear_stop_loss_take_profit(&mut self) {
        self.rules.stop_loss_price = None;
        self.rules.take_profit_price = None;
        self.entry_price = None;
    }

    /// 停止交易
    fn stop_trading(&mut self, reason: String) {
        self.trading_stopped = true;
        self.stop_reason = Some(reason);
    }

    /// 恢复交易(谨慎使用)
    pub fn resume_trading(&mut self) {
        self.trading_stopped = false;
        self.stop_reason = None;
        self.consecutive_losses = 0;
        warn!("风险管理器已恢复交易,请确认风险已解除");
    }

    /// 检查是否应停止交易
    pub fn should_stop_trading(&self) -> bool {
        self.trading_stopped
    }

    /// 获取停止原因
    pub fn get_stop_reason(&self) -> Option<&str> {
        self.stop_reason.as_deref()
    }

    /// 获取连续亏损次数
    pub fn get_consecutive_losses(&self) -> u32 {
        self.consecutive_losses
    }

    /// 获取连续盈利次数
    pub fn get_consecutive_wins(&self) -> u32 {
        self.consecutive_wins
    }

    /// 获取风险规则
    pub fn get_rules(&self) -> &RiskRules {
        &self.rules
    }

    /// 更新风险规则
    pub fn update_rules(&mut self, rules: RiskRules) {
        self.rules = rules;
    }
}

#[cfg(test)]
mod tests;
