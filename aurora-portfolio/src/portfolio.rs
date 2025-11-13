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

//! 投资组合管理核心模块

use anyhow::Result;
use async_trait::async_trait;
use tracing::{debug, info, warn};

use crate::analytics::{EquityPoint, PerformanceMetrics, PortfolioAnalytics};
use crate::position_manager::PositionManager;
use crate::risk_manager::{RiskCheckResult, RiskManager};
use crate::trade::Trade;

/// 投资组合管理统一接口
///
/// 定义了投资组合管理的标准行为，适用于回测和实时交易环境。
/// 支持异步操作以适应实时交易的需求。
#[async_trait]
pub trait Portfolio: Send + Sync {
    /// 执行买入操作
    ///
    /// # 参数
    ///
    /// * `price` - 买入价格
    /// * `timestamp` - 交易时间戳
    ///
    /// # 返回值
    ///
    /// 成功时返回交易记录，失败时返回错误信息
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade>;

    /// 执行卖出操作
    ///
    /// # 参数
    ///
    /// * `price` - 卖出价格
    /// * `timestamp` - 交易时间戳
    ///
    /// # 返回值
    ///
    /// 成功时返回交易记录，失败时返回错误信息
    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade>;

    /// 获取总权益
    ///
    /// # 参数
    ///
    /// * `current_price` - 当前市场价格
    ///
    /// # 返回值
    ///
    /// 返回当前总权益（现金 + 持仓价值）
    fn get_total_equity(&self, current_price: f64) -> f64;

    /// 获取现金余额
    fn get_cash(&self) -> f64;

    /// 获取持仓数量
    fn get_position(&self) -> f64;

    /// 获取交易记录
    fn get_trades(&self) -> &[Trade];

    /// 更新权益曲线
    ///
    /// # 参数
    ///
    /// * `timestamp` - 时间戳
    /// * `current_price` - 当前价格
    fn update_equity(&mut self, timestamp: i64, current_price: f64);

    /// 获取权益曲线
    fn get_equity_curve(&self) -> &[EquityPoint];

    /// 计算业绩指标
    fn calculate_performance(&self, time_period_days: f64) -> PerformanceMetrics;
}

/// 基础投资组合实现
///
/// 提供投资组合管理的标准实现，适用于大多数场景。
/// 支持可选的风险管理和仓位管理功能。
#[derive(Debug, Clone)]
pub struct BasePortfolio {
    /// 现金余额
    cash: f64,
    /// 持仓数量
    position: f64,
    /// 初始权益
    initial_equity: f64,
    /// 交易记录
    trades: Vec<Trade>,
    /// 权益曲线
    equity_curve: Vec<EquityPoint>,
    /// 历史最高权益（用于计算回撤）
    max_equity: f64,
    /// 风险管理器（可选）
    risk_manager: Option<RiskManager>,
    /// 仓位管理器（可选）
    position_manager: Option<PositionManager>,
    /// 当前持仓的入场价格（用于止损止盈计算）
    entry_price: Option<f64>,
    /// 上次警告的回撤值（用于限制日志输出频率）
    last_warned_drawdown: f64,
}

impl BasePortfolio {
    /// 创建新的投资组合
    ///
    /// # 参数
    ///
    /// * `initial_cash` - 初始现金金额
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::{BasePortfolio, Portfolio};
    ///
    /// let portfolio = BasePortfolio::new(10000.0);
    /// assert_eq!(portfolio.get_cash(), 10000.0);
    /// assert_eq!(portfolio.get_position(), 0.0);
    /// ```
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash: initial_cash,
            position: 0.0,
            initial_equity: initial_cash,
            trades: Vec::new(),
            equity_curve: Vec::new(),
            max_equity: initial_cash,
            risk_manager: None,
            position_manager: None,
            entry_price: None,
            last_warned_drawdown: 0.0,
        }
    }

    /// 设置风险管理器
    ///
    /// # 参数
    ///
    /// * `risk_manager` - 风险管理器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::{BasePortfolio, RiskManager, RiskRules};
    ///
    /// let rules = RiskRules::new().with_max_drawdown(15.0);
    /// let risk_manager = RiskManager::new(rules, 10000.0);
    /// let portfolio = BasePortfolio::new(10000.0)
    ///     .with_risk_manager(risk_manager);
    /// ```
    pub fn with_risk_manager(mut self, risk_manager: RiskManager) -> Self {
        self.risk_manager = Some(risk_manager);
        self
    }

    /// 设置仓位管理器
    ///
    /// # 参数
    ///
    /// * `position_manager` - 仓位管理器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::{BasePortfolio, PositionManager, PositionSizingStrategy};
    ///
    /// let strategy = PositionSizingStrategy::FixedPercentage(0.2);
    /// let position_manager = PositionManager::new(strategy);
    /// let portfolio = BasePortfolio::new(10000.0)
    ///     .with_position_manager(position_manager);
    /// ```
    pub fn with_position_manager(mut self, position_manager: PositionManager) -> Self {
        self.position_manager = Some(position_manager);
        self
    }

    /// 获取风险管理器的可变引用（如果存在）
    ///
    /// # 返回值
    ///
    /// 返回风险管理器的可变引用，如果没有则返回 None
    pub fn get_risk_manager_mut(&mut self) -> Option<&mut RiskManager> {
        self.risk_manager.as_mut()
    }

    /// 检查是否可以买入
    ///
    /// # 参数
    ///
    /// * `price` - 买入价格
    ///
    /// # 返回值
    ///
    /// 如果现金足够买入至少最小单位，返回true
    fn can_buy(&self, price: f64) -> bool {
        self.cash > price * 0.001 // 最小买入单位
    }

    /// 检查是否可以卖出
    fn can_sell(&self) -> bool {
        self.position > 0.0
    }

    /// 计算买入数量
    ///
    /// 根据仓位管理器计算应使用的资金量，然后计算买入数量
    fn calculate_buy_quantity(&self, price: f64) -> f64 {
        let total_equity = self.get_total_equity(price);
        
        // 计算当前盈亏百分比（用于金字塔策略）
        let current_profit = if self.initial_equity > 0.0 {
            ((total_equity - self.initial_equity) / self.initial_equity) * 100.0
        } else {
            0.0
        };
        
        // 如果有仓位管理器，使用它计算应使用的资金
        let position_value = if let Some(ref pm) = self.position_manager {
            pm.calculate_position_size(total_equity, current_profit)
                .unwrap_or(self.cash)
        } else {
            // 默认全仓
            self.cash
        };
        
        // 确保不超过可用现金
        let position_value = position_value.min(self.cash);
        position_value / price
    }

    /// 计算卖出数量
    ///
    /// 默认卖出全部持仓
    fn calculate_sell_quantity(&self) -> f64 {
        self.position
    }

    /// 验证交易参数
    fn validate_trade_params(&self, price: f64, timestamp: i64) -> Result<()> {
        if price <= 0.0 {
            return Err(anyhow::anyhow!("价格必须大于0"));
        }
        if timestamp < 0 {
            return Err(anyhow::anyhow!("时间戳不能为负数"));
        }
        Ok(())
    }
}

#[async_trait]
impl Portfolio for BasePortfolio {
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        self.validate_trade_params(price, timestamp)?;

        if !self.can_buy(price) {
            return Err(anyhow::anyhow!("现金不足，无法买入"));
        }

        // 风险检查
        let current_equity = self.get_total_equity(price);
        let drawdown = if self.max_equity > 0.0 {
            ((self.max_equity - current_equity) / self.max_equity) * 100.0
        } else {
            0.0
        };
        
        if let Some(ref mut risk_mgr) = self.risk_manager {
            let risk_check = risk_mgr.check_risk(current_equity, drawdown, price);
            if !risk_check.is_pass() {
                warn!("风控拒绝买入: {:?}", risk_check.get_reason());
                return Err(anyhow::anyhow!(
                    "风控拒绝: {}",
                    risk_check.get_reason().unwrap_or("未知原因")
                ));
            }
        }

        let quantity = self.calculate_buy_quantity(price);
        let value = quantity * price;

        // 更新持仓和现金
        self.position += quantity;
        self.cash -= value;
        
        // 记录入场价格（用于止损止盈）
        self.entry_price = Some(price);

        // 创建交易记录
        let trade = Trade::new_buy(price, quantity, timestamp);
        self.trades.push(trade.clone());

        info!(
            "执行买入: 价格={:.2}, 数量={:.6}, 总价值={:.2}",
            price, quantity, value
        );
        debug!(
            "买入后状态: 持仓={:.6}, 现金={:.2}",
            self.position, self.cash
        );

        Ok(trade)
    }

    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        self.validate_trade_params(price, timestamp)?;

        if !self.can_sell() {
            return Err(anyhow::anyhow!("无持仓，无法卖出"));
        }

        // 风险检查（止损止盈）
        let current_equity = self.get_total_equity(price);
        let drawdown = if self.max_equity > 0.0 {
            ((self.max_equity - current_equity) / self.max_equity) * 100.0
        } else {
            0.0
        };
        
        if let Some(ref mut risk_mgr) = self.risk_manager {
            let risk_check = risk_mgr.check_risk(current_equity, drawdown, price);
            
            // 对于卖出操作，如果触发止损或止盈，应该执行而不是拒绝
            match risk_check {
                RiskCheckResult::StopLoss(ref reason) => {
                    info!("触发止损: {}", reason);
                }
                RiskCheckResult::TakeProfit(ref reason) => {
                    info!("触发止盈: {}", reason);
                }
                _ => {
                    // 其他风控条件在卖出时不阻止
                }
            }
        }

        let quantity = self.calculate_sell_quantity();
        let value = quantity * price;
        
        // 计算本次交易的盈亏（用于风险管理器记录）
        let is_profitable = if let Some(entry) = self.entry_price {
            price > entry
        } else {
            false
        };

        // 更新持仓和现金
        self.cash += value;
        self.position = 0.0; // 全部卖出
        
        // 清除入场价格
        self.entry_price = None;
        
        // 记录交易结果到风险管理器
        if let Some(ref mut risk_mgr) = self.risk_manager {
            risk_mgr.record_trade_result(is_profitable);
        }

        // 创建交易记录
        let trade = Trade::new_sell(price, quantity, timestamp);
        self.trades.push(trade.clone());

        info!(
            "执行卖出: 价格={:.2}, 数量={:.6}, 总价值={:.2}, 盈亏={}",
            price, quantity, value, if is_profitable { "盈利" } else { "亏损" }
        );
        debug!(
            "卖出后状态: 持仓={:.6}, 现金={:.2}",
            self.position, self.cash
        );

        Ok(trade)
    }

    fn get_total_equity(&self, current_price: f64) -> f64 {
        self.cash + (self.position * current_price)
    }

    fn get_cash(&self) -> f64 {
        self.cash
    }

    fn get_position(&self) -> f64 {
        self.position
    }

    fn get_trades(&self) -> &[Trade] {
        &self.trades
    }

    fn update_equity(&mut self, timestamp: i64, current_price: f64) {
        let equity = self.get_total_equity(current_price);

        // 更新历史最高权益
        if equity > self.max_equity {
            self.max_equity = equity;
        }

        // 计算当前回撤
        let drawdown = if self.max_equity > 0.0 {
            ((self.max_equity - equity) / self.max_equity) * 100.0
        } else {
            0.0
        };

        // 创建权益点
        let equity_point = EquityPoint {
            timestamp,
            equity,
            drawdown,
        };

        self.equity_curve.push(equity_point);

        // 如果回撤超过警告阈值，记录日志
        // 为避免大量日志输出影响性能，只有在回撤变化超过1%时才输出警告
        if drawdown > 10.0 && (drawdown - self.last_warned_drawdown).abs() > 1.0 {
            warn!("当前回撤较大: {:.2}%", drawdown);
            self.last_warned_drawdown = drawdown;
        }
    }

    fn get_equity_curve(&self) -> &[EquityPoint] {
        &self.equity_curve
    }

    fn calculate_performance(&self, time_period_days: f64) -> PerformanceMetrics {
        let final_equity = if let Some(last_point) = self.equity_curve.last() {
            last_point.equity
        } else {
            self.initial_equity
        };

        PortfolioAnalytics::calculate_metrics(
            self.initial_equity,
            final_equity,
            &self.equity_curve,
            &self.trades,
            time_period_days,
        )
    }
}

#[cfg(test)]
mod tests;
