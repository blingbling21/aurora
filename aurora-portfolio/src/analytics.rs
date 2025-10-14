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

//! 投资组合分析和业绩指标计算模块

use crate::trade::Trade;
use serde::{Deserialize, Serialize};

/// 权益曲线数据点
///
/// 记录特定时刻的投资组合权益状态，用于绘制权益曲线和计算风险指标。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquityPoint {
    /// 时间戳（Unix毫秒）
    pub timestamp: i64,
    /// 总权益值
    pub equity: f64,
    /// 当前回撤百分比
    pub drawdown: f64,
}

/// 投资组合业绩指标
///
/// 包含投资组合的各项关键业绩和风险指标。
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    /// 总收益率（%）
    pub total_return: f64,
    /// 年化收益率（%）
    pub annualized_return: f64,
    /// 最大回撤（%）
    pub max_drawdown: f64,
    /// 夏普比率
    pub sharpe_ratio: f64,
    /// 胜率（%）
    pub win_rate: f64,
    /// 总交易次数
    pub total_trades: usize,
    /// 盈利交易次数
    pub winning_trades: usize,
    /// 亏损交易次数
    pub losing_trades: usize,
    /// 平均盈利
    pub average_win: f64,
    /// 平均亏损
    pub average_loss: f64,
    /// 盈亏比
    pub profit_loss_ratio: f64,
}

/// 投资组合分析器
///
/// 提供投资组合业绩分析的核心功能，包括各种风险和收益指标的计算。
pub struct PortfolioAnalytics;

impl PortfolioAnalytics {
    /// 计算投资组合业绩指标
    ///
    /// # 参数
    ///
    /// * `initial_equity` - 初始权益
    /// * `final_equity` - 最终权益
    /// * `equity_curve` - 权益曲线数据
    /// * `trades` - 交易记录
    /// * `time_period_days` - 投资期间天数
    ///
    /// # 返回值
    ///
    /// 返回包含各项业绩指标的 `PerformanceMetrics` 结构体
    pub fn calculate_metrics(
        initial_equity: f64,
        final_equity: f64,
        equity_curve: &[EquityPoint],
        trades: &[Trade],
        time_period_days: f64,
    ) -> PerformanceMetrics {
        // 计算总收益率
        let total_return = ((final_equity - initial_equity) / initial_equity) * 100.0;

        // 计算年化收益率
        let years = time_period_days / 365.25;
        let annualized_return = if years > 0.0 {
            ((final_equity / initial_equity).powf(1.0 / years) - 1.0) * 100.0
        } else {
            0.0
        };

        // 计算最大回撤
        let max_drawdown = Self::calculate_max_drawdown(equity_curve);

        // 计算夏普比率（简化版本，假设无风险利率为0）
        let returns = Self::calculate_daily_returns(equity_curve);
        let sharpe_ratio = Self::calculate_sharpe_ratio(&returns);

        // 分析交易记录
        let (win_rate, total_trades, winning_trades, losing_trades, avg_win, avg_loss) =
            Self::analyze_trades(trades);

        // 计算盈亏比
        let profit_loss_ratio = if avg_loss != 0.0 {
            avg_win / avg_loss.abs()
        } else {
            0.0
        };

        PerformanceMetrics {
            total_return,
            annualized_return,
            max_drawdown,
            sharpe_ratio,
            win_rate,
            total_trades,
            winning_trades,
            losing_trades,
            average_win: avg_win,
            average_loss: avg_loss,
            profit_loss_ratio,
        }
    }

    /// 计算最大回撤
    fn calculate_max_drawdown(equity_curve: &[EquityPoint]) -> f64 {
        equity_curve
            .iter()
            .map(|point| point.drawdown)
            .fold(0.0, f64::max)
    }

    /// 计算日收益率序列
    fn calculate_daily_returns(equity_curve: &[EquityPoint]) -> Vec<f64> {
        let mut returns = Vec::new();

        for window in equity_curve.windows(2) {
            if let [prev, curr] = window {
                let return_rate = (curr.equity - prev.equity) / prev.equity;
                returns.push(return_rate);
            }
        }

        returns
    }

    /// 计算夏普比率
    fn calculate_sharpe_ratio(returns: &[f64]) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;

        if returns.len() < 2 {
            return 0.0;
        }

        let variance = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / (returns.len() - 1) as f64;

        let std_dev = variance.sqrt();

        if std_dev == 0.0 {
            0.0
        } else {
            mean_return / std_dev * (252.0_f64).sqrt() // 年化，假设252个交易日
        }
    }

    /// 分析交易记录
    fn analyze_trades(trades: &[Trade]) -> (f64, usize, usize, usize, f64, f64) {
        if trades.len() < 2 {
            return (0.0, 0, 0, 0, 0.0, 0.0);
        }

        let mut profits = Vec::new();
        let mut i = 0;

        // 配对买卖交易计算盈亏
        while i + 1 < trades.len() {
            if trades[i].is_buy() && trades[i + 1].is_sell() {
                let profit = (trades[i + 1].price - trades[i].price) * trades[i].quantity;
                profits.push(profit);
                i += 2;
            } else {
                i += 1;
            }
        }

        let total_trades = profits.len();
        if total_trades == 0 {
            return (0.0, 0, 0, 0, 0.0, 0.0);
        }

        let winning_trades = profits.iter().filter(|&&p| p > 0.0).count();
        let losing_trades = profits.iter().filter(|&&p| p < 0.0).count();

        let win_rate = (winning_trades as f64 / total_trades as f64) * 100.0;

        let total_wins: f64 = profits.iter().filter(|&&p| p > 0.0).sum();
        let total_losses: f64 = profits.iter().filter(|&&p| p < 0.0).sum();

        let avg_win = if winning_trades > 0 {
            total_wins / winning_trades as f64
        } else {
            0.0
        };

        let avg_loss = if losing_trades > 0 {
            total_losses / losing_trades as f64
        } else {
            0.0
        };

        (
            win_rate,
            total_trades,
            winning_trades,
            losing_trades,
            avg_win,
            avg_loss,
        )
    }
}

impl PerformanceMetrics {
    /// 打印业绩报告
    pub fn print_report(&self) {
        println!("=== 投资组合业绩报告 ===");
        println!("总收益率: {:.2}%", self.total_return);
        println!("年化收益率: {:.2}%", self.annualized_return);
        println!("最大回撤: {:.2}%", self.max_drawdown);
        println!("夏普比率: {:.3}", self.sharpe_ratio);
        println!("胜率: {:.2}%", self.win_rate);
        println!("总交易次数: {}", self.total_trades);
        println!("盈利交易: {}", self.winning_trades);
        println!("亏损交易: {}", self.losing_trades);
        println!("平均盈利: {:.2}", self.average_win);
        println!("平均亏损: {:.2}", self.average_loss);
        println!("盈亏比: {:.2}", self.profit_loss_ratio);
        println!("========================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trade::{Trade, TradeSide};

    #[test]
    fn test_equity_point_creation() {
        let point = EquityPoint {
            timestamp: 1640995200000,
            equity: 10500.0,
            drawdown: 2.5,
        };

        assert_eq!(point.equity, 10500.0);
        assert_eq!(point.drawdown, 2.5);
    }

    #[test]
    fn test_performance_metrics_calculation() {
        let equity_curve = vec![
            EquityPoint {
                timestamp: 0,
                equity: 10000.0,
                drawdown: 0.0,
            },
            EquityPoint {
                timestamp: 1,
                equity: 10500.0,
                drawdown: 0.0,
            },
            EquityPoint {
                timestamp: 2,
                equity: 11000.0,
                drawdown: 0.0,
            },
        ];

        let trades = vec![
            Trade::new_buy(100.0, 100.0, 0),
            Trade::new_sell(110.0, 100.0, 1),
        ];

        let metrics =
            PortfolioAnalytics::calculate_metrics(10000.0, 11000.0, &equity_curve, &trades, 365.0);

        assert_eq!(metrics.total_return, 10.0);
        assert_eq!(metrics.total_trades, 1);
        assert_eq!(metrics.winning_trades, 1);
        assert_eq!(metrics.win_rate, 100.0);
    }
}
