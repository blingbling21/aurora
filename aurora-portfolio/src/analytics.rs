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
    /// 最大回撤持续时间（天）
    pub max_drawdown_duration: f64,
    /// 年化波动率（%）
    pub annualized_volatility: f64,
    /// 夏普比率
    pub sharpe_ratio: f64,
    /// 索提诺比率
    pub sortino_ratio: f64,
    /// 卡玛比率
    pub calmar_ratio: f64,
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
    /// 利润因子
    pub profit_factor: f64,
    /// 最大连续盈利次数
    pub max_consecutive_wins: usize,
    /// 最大连续亏损次数
    pub max_consecutive_losses: usize,
    /// 平均持仓时间（小时）
    pub avg_holding_period: f64,
    /// 最大单笔盈利
    pub max_win: f64,
    /// 最大单笔亏损
    pub max_loss: f64,
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

        // 计算索提诺比率
        let sortino_ratio = Self::calculate_sortino_ratio(&returns);

        // 计算卡玛比率
        let calmar_ratio = Self::calculate_calmar_ratio(annualized_return, max_drawdown);

        // 计算年化波动率
        let annualized_volatility = Self::calculate_annualized_volatility(&returns);

        // 计算最大回撤持续时间
        let max_drawdown_duration = Self::calculate_max_drawdown_duration(equity_curve);

        // 分析交易记录
        let (win_rate, total_trades, winning_trades, losing_trades, avg_win, avg_loss) =
            Self::analyze_trades(trades);

        // 计算盈亏比
        let profit_loss_ratio = if avg_loss != 0.0 {
            avg_win / avg_loss.abs()
        } else {
            0.0
        };

        // 计算利润因子
        let profit_factor = Self::calculate_profit_factor(trades);

        // 计算连续盈亏统计
        let (max_consecutive_wins, max_consecutive_losses) = Self::calculate_consecutive_stats(trades);

        // 计算平均持仓时间
        let avg_holding_period = Self::calculate_holding_period(trades);

        // 计算最大单笔盈亏
        let (max_win, max_loss) = Self::calculate_max_profit_loss(trades);

        PerformanceMetrics {
            total_return,
            annualized_return,
            max_drawdown,
            max_drawdown_duration,
            annualized_volatility,
            sharpe_ratio,
            sortino_ratio,
            calmar_ratio,
            win_rate,
            total_trades,
            winning_trades,
            losing_trades,
            average_win: avg_win,
            average_loss: avg_loss,
            profit_loss_ratio,
            profit_factor,
            max_consecutive_wins,
            max_consecutive_losses,
            avg_holding_period,
            max_win,
            max_loss,
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

    /// 计算索提诺比率（Sortino Ratio）
    ///
    /// Sortino Ratio 只考虑下行波动率，更适合评估下行风险。
    /// 公式：年化收益率 / (下行标准差 × √252)
    ///
    /// # 参数
    ///
    /// * `returns` - 日收益率序列
    ///
    /// # 返回值
    ///
    /// 返回 Sortino Ratio，值越高表示风险调整后收益越好
    pub fn calculate_sortino_ratio(returns: &[f64]) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;

        if returns.len() < 2 {
            return 0.0;
        }

        // 只考虑负收益的标准差（下行波动）
        let negative_returns: Vec<f64> = returns.iter().filter(|&&r| r < 0.0).copied().collect();

        if negative_returns.is_empty() {
            // 如果没有负收益，返回一个很高的值
            return 999.0;
        }

        let downside_variance = negative_returns
            .iter()
            .map(|r| r.powi(2))
            .sum::<f64>()
            / negative_returns.len() as f64;

        let downside_std = downside_variance.sqrt();

        if downside_std == 0.0 {
            0.0
        } else {
            mean_return / downside_std * (252.0_f64).sqrt() // 年化
        }
    }

    /// 计算卡玛比率（Calmar Ratio）
    ///
    /// Calmar Ratio = 年化收益率 / 最大回撤
    /// 衡量承担单位回撤风险所获得的收益
    ///
    /// # 参数
    ///
    /// * `annualized_return` - 年化收益率（百分比）
    /// * `max_drawdown` - 最大回撤（百分比）
    ///
    /// # 返回值
    ///
    /// 返回 Calmar Ratio，值越高表示风险调整后收益越好
    pub fn calculate_calmar_ratio(annualized_return: f64, max_drawdown: f64) -> f64 {
        if max_drawdown == 0.0 {
            return 0.0;
        }
        annualized_return / max_drawdown
    }

    /// 计算年化波动率
    ///
    /// 基于日收益率计算年化标准差
    ///
    /// # 参数
    ///
    /// * `returns` - 日收益率序列
    ///
    /// # 返回值
    ///
    /// 返回年化波动率（百分比）
    pub fn calculate_annualized_volatility(returns: &[f64]) -> f64 {
        if returns.len() < 2 {
            return 0.0;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;

        let variance = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / (returns.len() - 1) as f64;

        let std_dev = variance.sqrt();

        std_dev * (252.0_f64).sqrt() * 100.0 // 年化并转换为百分比
    }

    /// 计算最大回撤持续时间
    ///
    /// 计算从最高点到恢复至该点所需的时间（天数）
    ///
    /// # 参数
    ///
    /// * `equity_curve` - 权益曲线数据
    ///
    /// # 返回值
    ///
    /// 返回最大回撤持续时间（天数）
    pub fn calculate_max_drawdown_duration(equity_curve: &[EquityPoint]) -> f64 {
        if equity_curve.len() < 2 {
            return 0.0;
        }

        let mut max_duration: f64 = 0.0;
        let mut current_duration: f64;
        let mut in_drawdown = false;
        let mut drawdown_start = 0i64;

        for i in 1..equity_curve.len() {
            if equity_curve[i].drawdown > 0.0 {
                if !in_drawdown {
                    // 开始新的回撤期
                    in_drawdown = true;
                    drawdown_start = equity_curve[i].timestamp;
                }
            } else if in_drawdown {
                // 回撤结束
                current_duration = (equity_curve[i].timestamp - drawdown_start) as f64 / (1000.0 * 60.0 * 60.0 * 24.0);
                max_duration = f64::max(max_duration, current_duration);
                in_drawdown = false;
            }
        }

        // 如果在回撤中结束，计算到最后一个点的时间
        if in_drawdown {
            current_duration = (equity_curve.last().unwrap().timestamp - drawdown_start) as f64 / (1000.0 * 60.0 * 60.0 * 24.0);
            max_duration = f64::max(max_duration, current_duration);
        }

        max_duration
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

    /// 计算利润因子（Profit Factor）
    ///
    /// Profit Factor = 总盈利 / 总亏损（取绝对值）
    /// 值大于1表示盈利，大于2通常被认为是优秀的策略
    ///
    /// # 参数
    ///
    /// * `trades` - 交易记录列表
    ///
    /// # 返回值
    ///
    /// 返回利润因子
    pub fn calculate_profit_factor(trades: &[Trade]) -> f64 {
        if trades.len() < 2 {
            return 0.0;
        }

        let mut total_wins = 0.0;
        let mut total_losses = 0.0;
        let mut i = 0;

        while i + 1 < trades.len() {
            if trades[i].is_buy() && trades[i + 1].is_sell() {
                let profit = (trades[i + 1].price - trades[i].price) * trades[i].quantity;
                if profit > 0.0 {
                    total_wins += profit;
                } else {
                    total_losses += profit.abs();
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        if total_losses == 0.0 {
            if total_wins > 0.0 {
                return 999.0; // 无亏损的情况
            }
            return 0.0;
        }

        total_wins / total_losses
    }

    /// 计算连续盈亏统计
    ///
    /// 返回 (最大连续盈利次数, 最大连续亏损次数)
    ///
    /// # 参数
    ///
    /// * `trades` - 交易记录列表
    ///
    /// # 返回值
    ///
    /// 返回元组 (最大连续盈利次数, 最大连续亏损次数)
    pub fn calculate_consecutive_stats(trades: &[Trade]) -> (usize, usize) {
        if trades.len() < 2 {
            return (0, 0);
        }

        let mut profits = Vec::new();
        let mut i = 0;

        while i + 1 < trades.len() {
            if trades[i].is_buy() && trades[i + 1].is_sell() {
                let profit = (trades[i + 1].price - trades[i].price) * trades[i].quantity;
                profits.push(profit);
                i += 2;
            } else {
                i += 1;
            }
        }

        let mut max_consecutive_wins = 0;
        let mut max_consecutive_losses = 0;
        let mut current_wins = 0;
        let mut current_losses = 0;

        for profit in profits {
            if profit > 0.0 {
                current_wins += 1;
                max_consecutive_wins = max_consecutive_wins.max(current_wins);
                current_losses = 0;
            } else if profit < 0.0 {
                current_losses += 1;
                max_consecutive_losses = max_consecutive_losses.max(current_losses);
                current_wins = 0;
            }
        }

        (max_consecutive_wins, max_consecutive_losses)
    }

    /// 计算平均持仓时间
    ///
    /// 计算所有完成交易的平均持仓时间（小时）
    ///
    /// # 参数
    ///
    /// * `trades` - 交易记录列表
    ///
    /// # 返回值
    ///
    /// 返回平均持仓时间（小时）
    pub fn calculate_holding_period(trades: &[Trade]) -> f64 {
        if trades.len() < 2 {
            return 0.0;
        }

        let mut total_holding_time = 0.0;
        let mut trade_count = 0;
        let mut i = 0;

        while i + 1 < trades.len() {
            if trades[i].is_buy() && trades[i + 1].is_sell() {
                let holding_time = (trades[i + 1].timestamp - trades[i].timestamp) as f64 / (1000.0 * 60.0 * 60.0);
                total_holding_time += holding_time;
                trade_count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        if trade_count > 0 {
            total_holding_time / trade_count as f64
        } else {
            0.0
        }
    }

    /// 计算最大单笔盈利和亏损
    ///
    /// 返回 (最大单笔盈利, 最大单笔亏损)
    ///
    /// # 参数
    ///
    /// * `trades` - 交易记录列表
    ///
    /// # 返回值
    ///
    /// 返回元组 (最大单笔盈利, 最大单笔亏损)
    pub fn calculate_max_profit_loss(trades: &[Trade]) -> (f64, f64) {
        if trades.len() < 2 {
            return (0.0, 0.0);
        }

        let mut max_win: f64 = 0.0;
        let mut max_loss: f64 = 0.0;
        let mut i = 0;

        while i + 1 < trades.len() {
            if trades[i].is_buy() && trades[i + 1].is_sell() {
                let profit = (trades[i + 1].price - trades[i].price) * trades[i].quantity;
                if profit > 0.0 {
                    max_win = f64::max(max_win, profit);
                } else {
                    max_loss = f64::min(max_loss, profit);
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        (max_win, max_loss)
    }

    /// 计算相对收益（Alpha）
    ///
    /// Alpha 是策略收益相对于基准收益的超额收益，用于衡量策略的主动管理能力。
    /// Alpha > 0 表示策略跑赢基准，Alpha < 0 表示策略跑输基准。
    ///
    /// # 参数
    ///
    /// * `strategy_return` - 策略的总收益率（百分比）
    /// * `benchmark_return` - 基准的总收益率（百分比）
    ///
    /// # 返回值
    ///
    /// 返回策略相对于基准的超额收益率（百分比）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::PortfolioAnalytics;
    ///
    /// // 策略收益率 15%，基准收益率 10%
    /// let alpha = PortfolioAnalytics::calculate_alpha(15.0, 10.0);
    /// assert_eq!(alpha, 5.0); // Alpha = 5%
    ///
    /// // 策略收益率 8%，基准收益率 12%
    /// let alpha = PortfolioAnalytics::calculate_alpha(8.0, 12.0);
    /// assert_eq!(alpha, -4.0); // Alpha = -4%
    /// ```
    ///
    /// # 注意事项
    ///
    /// - Alpha 是绝对差值，不是相对差值
    /// - 正的 Alpha 值越大，表示策略表现越好
    /// - 负的 Alpha 值越小，表示策略表现越差
    pub fn calculate_alpha(strategy_return: f64, benchmark_return: f64) -> f64 {
        strategy_return - benchmark_return
    }

    /// 计算年化 Alpha
    ///
    /// 将 Alpha 转换为年化形式，便于与其他年化指标对比。
    ///
    /// # 参数
    ///
    /// * `strategy_return` - 策略的总收益率（百分比）
    /// * `benchmark_return` - 基准的总收益率（百分比）
    /// * `time_period_days` - 投资期间天数
    ///
    /// # 返回值
    ///
    /// 返回年化的相对收益率（百分比）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::PortfolioAnalytics;
    ///
    /// // 半年（180天）策略收益率 15%，基准收益率 10%
    /// let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(15.0, 10.0, 180.0);
    /// // 年化 Alpha ≈ (15-10) * (365/180) ≈ 10.14%
    /// assert!((annualized_alpha - 10.14).abs() < 0.1);
    /// ```
    ///
    /// # 注意事项
    ///
    /// - 如果投资期间为0天，返回0
    /// - 年化因子为 365 / time_period_days
    pub fn calculate_annualized_alpha(
        strategy_return: f64,
        benchmark_return: f64,
        time_period_days: f64,
    ) -> f64 {
        if time_period_days <= 0.0 {
            return 0.0;
        }
        let alpha = Self::calculate_alpha(strategy_return, benchmark_return);
        alpha * (365.0 / time_period_days)
    }
}

impl PerformanceMetrics {
    /// 打印业绩报告
    pub fn print_report(&self) {
        println!("=== 投资组合业绩报告 ===");
        println!("\n--- 收益指标 ---");
        println!("总收益率: {:.2}%", self.total_return);
        println!("年化收益率: {:.2}%", self.annualized_return);
        
        println!("\n--- 风险指标 ---");
        println!("最大回撤: {:.2}%", self.max_drawdown);
        println!("最大回撤持续时间: {:.2} 天", self.max_drawdown_duration);
        println!("年化波动率: {:.2}%", self.annualized_volatility);
        
        println!("\n--- 风险调整收益指标 ---");
        println!("夏普比率: {:.3}", self.sharpe_ratio);
        println!("索提诺比率: {:.3}", self.sortino_ratio);
        println!("卡玛比率: {:.3}", self.calmar_ratio);
        
        println!("\n--- 交易统计 ---");
        println!("总交易次数: {}", self.total_trades);
        println!("盈利交易: {}", self.winning_trades);
        println!("亏损交易: {}", self.losing_trades);
        println!("胜率: {:.2}%", self.win_rate);
        
        println!("\n--- 盈亏分析 ---");
        println!("平均盈利: {:.2}", self.average_win);
        println!("平均亏损: {:.2}", self.average_loss);
        println!("盈亏比: {:.2}", self.profit_loss_ratio);
        println!("利润因子: {:.2}", self.profit_factor);
        println!("最大单笔盈利: {:.2}", self.max_win);
        println!("最大单笔亏损: {:.2}", self.max_loss);
        
        println!("\n--- 交易行为 ---");
        println!("最大连续盈利次数: {}", self.max_consecutive_wins);
        println!("最大连续亏损次数: {}", self.max_consecutive_losses);
        println!("平均持仓时间: {:.2} 小时", self.avg_holding_period);
        
        println!("========================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trade::Trade;

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

    #[test]
    fn test_sortino_ratio_calculation() {
        // 测试数据：包含正负收益
        let returns = vec![0.05, -0.02, 0.03, -0.01, 0.04, 0.02, -0.03];
        let sortino = PortfolioAnalytics::calculate_sortino_ratio(&returns);
        
        // Sortino应该大于0（因为平均收益为正）
        assert!(sortino > 0.0);
        
        // 测试无负收益的情况
        let all_positive = vec![0.01, 0.02, 0.03, 0.04];
        let sortino_positive = PortfolioAnalytics::calculate_sortino_ratio(&all_positive);
        assert_eq!(sortino_positive, 999.0);
        
        // 测试空数据
        let empty: Vec<f64> = vec![];
        assert_eq!(PortfolioAnalytics::calculate_sortino_ratio(&empty), 0.0);
    }

    #[test]
    fn test_calmar_ratio_calculation() {
        // 正常情况
        let calmar = PortfolioAnalytics::calculate_calmar_ratio(15.0, 10.0);
        assert_eq!(calmar, 1.5);
        
        // 零回撤的情况
        let calmar_zero = PortfolioAnalytics::calculate_calmar_ratio(15.0, 0.0);
        assert_eq!(calmar_zero, 0.0);
        
        // 负收益的情况
        let calmar_negative = PortfolioAnalytics::calculate_calmar_ratio(-5.0, 10.0);
        assert_eq!(calmar_negative, -0.5);
    }

    #[test]
    fn test_annualized_volatility_calculation() {
        let returns = vec![0.01, -0.02, 0.015, -0.01, 0.02];
        let volatility = PortfolioAnalytics::calculate_annualized_volatility(&returns);
        
        // 波动率应该大于0
        assert!(volatility > 0.0);
        
        // 测试空数据和单个数据
        let empty: Vec<f64> = vec![];
        assert_eq!(PortfolioAnalytics::calculate_annualized_volatility(&empty), 0.0);
        
        let single = vec![0.01];
        assert_eq!(PortfolioAnalytics::calculate_annualized_volatility(&single), 0.0);
    }

    #[test]
    fn test_max_drawdown_duration_calculation() {
        // 创建一个有明显回撤期的权益曲线
        let equity_curve = vec![
            EquityPoint { timestamp: 0, equity: 10000.0, drawdown: 0.0 },
            EquityPoint { timestamp: 86400000, equity: 9500.0, drawdown: 5.0 },  // 1天后
            EquityPoint { timestamp: 172800000, equity: 9000.0, drawdown: 10.0 }, // 2天后
            EquityPoint { timestamp: 259200000, equity: 10000.0, drawdown: 0.0 }, // 3天后恢复
            EquityPoint { timestamp: 345600000, equity: 10500.0, drawdown: 0.0 }, // 4天后
        ];
        
        let duration = PortfolioAnalytics::calculate_max_drawdown_duration(&equity_curve);
        // 回撤持续了大约2天
        assert!(duration >= 1.0 && duration <= 3.0);
    }

    #[test]
    fn test_profit_factor_calculation() {
        let trades = vec![
            Trade::new_buy(100.0, 10.0, 0),
            Trade::new_sell(110.0, 10.0, 1),  // 盈利 100
            Trade::new_buy(110.0, 10.0, 2),
            Trade::new_sell(105.0, 10.0, 3),  // 亏损 50
        ];
        
        let profit_factor = PortfolioAnalytics::calculate_profit_factor(&trades);
        // Profit Factor = 100 / 50 = 2.0
        assert_eq!(profit_factor, 2.0);
        
        // 测试无亏损的情况
        let winning_trades = vec![
            Trade::new_buy(100.0, 10.0, 0),
            Trade::new_sell(110.0, 10.0, 1),
        ];
        let pf_no_loss = PortfolioAnalytics::calculate_profit_factor(&winning_trades);
        assert_eq!(pf_no_loss, 999.0);
    }

    #[test]
    fn test_consecutive_stats_calculation() {
        let trades = vec![
            Trade::new_buy(100.0, 10.0, 0),
            Trade::new_sell(110.0, 10.0, 1),  // 盈利
            Trade::new_buy(110.0, 10.0, 2),
            Trade::new_sell(120.0, 10.0, 3),  // 盈利
            Trade::new_buy(120.0, 10.0, 4),
            Trade::new_sell(125.0, 10.0, 5),  // 盈利
            Trade::new_buy(125.0, 10.0, 6),
            Trade::new_sell(120.0, 10.0, 7),  // 亏损
            Trade::new_buy(120.0, 10.0, 8),
            Trade::new_sell(115.0, 10.0, 9),  // 亏损
        ];
        
        let (max_wins, max_losses) = PortfolioAnalytics::calculate_consecutive_stats(&trades);
        assert_eq!(max_wins, 3);
        assert_eq!(max_losses, 2);
    }

    #[test]
    fn test_holding_period_calculation() {
        let hour_in_ms = 1000 * 60 * 60;
        let trades = vec![
            Trade::new_buy(100.0, 10.0, 0),
            Trade::new_sell(110.0, 10.0, hour_in_ms * 24),  // 持有24小时
            Trade::new_buy(110.0, 10.0, hour_in_ms * 25),
            Trade::new_sell(115.0, 10.0, hour_in_ms * 37),  // 持有12小时
        ];
        
        let avg_period = PortfolioAnalytics::calculate_holding_period(&trades);
        // 平均持仓时间应该是 (24 + 12) / 2 = 18小时
        assert!((avg_period - 18.0).abs() < 0.1);
    }

    #[test]
    fn test_max_profit_loss_calculation() {
        let trades = vec![
            Trade::new_buy(100.0, 10.0, 0),
            Trade::new_sell(110.0, 10.0, 1),  // 盈利 100
            Trade::new_buy(110.0, 10.0, 2),
            Trade::new_sell(105.0, 10.0, 3),  // 亏损 -50
            Trade::new_buy(105.0, 10.0, 4),
            Trade::new_sell(120.0, 10.0, 5),  // 盈利 150
        ];
        
        let (max_win, max_loss) = PortfolioAnalytics::calculate_max_profit_loss(&trades);
        assert_eq!(max_win, 150.0);
        assert_eq!(max_loss, -50.0);
    }

    #[test]
    fn test_calculate_alpha() {
        // 测试策略跑赢基准的情况
        let alpha = PortfolioAnalytics::calculate_alpha(15.0, 10.0);
        assert_eq!(alpha, 5.0);

        // 测试策略跑输基准的情况
        let alpha = PortfolioAnalytics::calculate_alpha(8.0, 12.0);
        assert_eq!(alpha, -4.0);

        // 测试策略与基准持平的情况
        let alpha = PortfolioAnalytics::calculate_alpha(10.0, 10.0);
        assert_eq!(alpha, 0.0);

        // 测试负收益的情况
        let alpha = PortfolioAnalytics::calculate_alpha(-5.0, -3.0);
        assert_eq!(alpha, -2.0);

        // 测试策略负收益但跑赢基准的情况
        let alpha = PortfolioAnalytics::calculate_alpha(-2.0, -5.0);
        assert_eq!(alpha, 3.0);
    }

    #[test]
    fn test_calculate_annualized_alpha() {
        // 测试半年（180天）的年化 Alpha
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(15.0, 10.0, 180.0);
        // Alpha = 5%, 年化因子 = 365/180 ≈ 2.028
        // 年化 Alpha ≈ 5 * 2.028 ≈ 10.14
        assert!((annualized_alpha - 10.14).abs() < 0.1);

        // 测试一年（365天）的年化 Alpha
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(20.0, 15.0, 365.0);
        // Alpha = 5%, 年化因子 = 1
        assert_eq!(annualized_alpha, 5.0);

        // 测试一个月（30天）的年化 Alpha
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(2.0, 1.0, 30.0);
        // Alpha = 1%, 年化因子 = 365/30 ≈ 12.17
        // 年化 Alpha ≈ 1 * 12.17 ≈ 12.17
        assert!((annualized_alpha - 12.17).abs() < 0.1);

        // 测试负 Alpha 的年化
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(8.0, 12.0, 180.0);
        // Alpha = -4%, 年化因子 = 365/180 ≈ 2.028
        // 年化 Alpha ≈ -4 * 2.028 ≈ -8.11
        assert!((annualized_alpha - (-8.11)).abs() < 0.1);

        // 测试时间为0的边界情况
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(15.0, 10.0, 0.0);
        assert_eq!(annualized_alpha, 0.0);

        // 测试时间为负数的边界情况
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(15.0, 10.0, -30.0);
        assert_eq!(annualized_alpha, 0.0);
    }
}
