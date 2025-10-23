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

//! 回测结果数据结构

use aurora_portfolio::{EquityPoint, PerformanceMetrics, Trade};
use serde::{Deserialize, Serialize};

/// 回测结果
///
/// 包含回测的完整结果数据，包括业绩指标、权益曲线和交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// 业绩指标
    pub metrics: SerializableMetrics,
    /// 权益曲线数据
    pub equity_curve: Vec<EquityPoint>,
    /// 交易记录
    pub trades: Vec<SerializableTrade>,
    /// 回测时间范围（天数）
    pub time_period_days: f64,
    /// 初始权益
    pub initial_equity: f64,
    /// 最终权益
    pub final_equity: f64,
    /// 数据文件路径
    /// 用于前端加载K线数据进行可视化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path: Option<String>,
    /// 基准策略权益曲线（Buy & Hold）
    /// 如果未计算基准，则为 None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_equity_curve: Option<Vec<EquityPoint>>,
    /// 相对于基准的超额收益（Alpha，%）
    /// 如果未计算基准，则为 None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
    /// 年化 Alpha（%）
    /// 如果未计算基准,则为 None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualized_alpha: Option<f64>,
}

/// 可序列化的业绩指标
///
/// 将 PerformanceMetrics 转换为可序列化的格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMetrics {
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

impl From<PerformanceMetrics> for SerializableMetrics {
    fn from(metrics: PerformanceMetrics) -> Self {
        Self {
            total_return: metrics.total_return,
            annualized_return: metrics.annualized_return,
            max_drawdown: metrics.max_drawdown,
            max_drawdown_duration: metrics.max_drawdown_duration,
            annualized_volatility: metrics.annualized_volatility,
            sharpe_ratio: metrics.sharpe_ratio,
            sortino_ratio: metrics.sortino_ratio,
            calmar_ratio: metrics.calmar_ratio,
            win_rate: metrics.win_rate,
            total_trades: metrics.total_trades,
            winning_trades: metrics.winning_trades,
            losing_trades: metrics.losing_trades,
            average_win: metrics.average_win,
            average_loss: metrics.average_loss,
            profit_loss_ratio: metrics.profit_loss_ratio,
            profit_factor: metrics.profit_factor,
            max_consecutive_wins: metrics.max_consecutive_wins,
            max_consecutive_losses: metrics.max_consecutive_losses,
            avg_holding_period: metrics.avg_holding_period,
            max_win: metrics.max_win,
            max_loss: metrics.max_loss,
        }
    }
}

/// 可序列化的交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTrade {
    /// 交易价格
    pub price: f64,
    /// 交易数量
    pub quantity: f64,
    /// 时间戳（Unix毫秒）
    pub timestamp: i64,
    /// 交易方向：true为买入，false为卖出
    pub is_buy: bool,
}

impl From<Trade> for SerializableTrade {
    fn from(trade: Trade) -> Self {
        Self {
            price: trade.price,
            quantity: trade.quantity,
            timestamp: trade.timestamp,
            is_buy: trade.is_buy(),
        }
    }
}

impl BacktestResult {
    /// 创建新的回测结果
    pub fn new(
        metrics: PerformanceMetrics,
        equity_curve: Vec<EquityPoint>,
        trades: Vec<Trade>,
        time_period_days: f64,
        initial_equity: f64,
        final_equity: f64,
        data_path: Option<String>,
    ) -> Self {
        Self {
            metrics: metrics.into(),
            equity_curve,
            trades: trades.into_iter().map(|t| t.into()).collect(),
            time_period_days,
            initial_equity,
            final_equity,
            data_path,
            benchmark_equity_curve: None,
            alpha: None,
            annualized_alpha: None,
        }
    }

    /// 创建包含基准数据的回测结果
    ///
    /// # 参数
    ///
    /// * `metrics` - 策略业绩指标
    /// * `equity_curve` - 策略权益曲线
    /// * `trades` - 交易记录
    /// * `time_period_days` - 回测时间范围（天数）
    /// * `initial_equity` - 初始权益
    /// * `final_equity` - 最终权益
    /// * `data_path` - 数据文件路径（可选）
    /// * `benchmark_equity_curve` - 基准策略权益曲线
    /// * `benchmark_return` - 基准总收益率（%）
    pub fn new_with_benchmark(
        metrics: PerformanceMetrics,
        equity_curve: Vec<EquityPoint>,
        trades: Vec<Trade>,
        time_period_days: f64,
        initial_equity: f64,
        final_equity: f64,
        data_path: Option<String>,
        benchmark_equity_curve: Vec<EquityPoint>,
        benchmark_return: f64,
    ) -> Self {
        use aurora_portfolio::PortfolioAnalytics;
        
        let alpha = PortfolioAnalytics::calculate_alpha(metrics.total_return, benchmark_return);
        let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(
            metrics.total_return,
            benchmark_return,
            time_period_days,
        );

        Self {
            metrics: metrics.into(),
            equity_curve,
            trades: trades.into_iter().map(|t| t.into()).collect(),
            time_period_days,
            initial_equity,
            final_equity,
            data_path,
            benchmark_equity_curve: Some(benchmark_equity_curve),
            alpha: Some(alpha),
            annualized_alpha: Some(annualized_alpha),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_portfolio::Trade as PortfolioTrade;

    #[test]
    fn test_serializable_trade_conversion() {
        let trade = PortfolioTrade::new_buy(100.0, 10.0, 1640995200000);
        let serializable: SerializableTrade = trade.into();

        assert_eq!(serializable.price, 100.0);
        assert_eq!(serializable.quantity, 10.0);
        assert_eq!(serializable.timestamp, 1640995200000);
        assert!(serializable.is_buy);
    }

    #[test]
    fn test_backtest_result_serialization() {
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
        ];

        let trades = vec![
            PortfolioTrade::new_buy(100.0, 10.0, 0),
            PortfolioTrade::new_sell(105.0, 10.0, 1),
        ];

        let metrics = aurora_portfolio::PortfolioAnalytics::calculate_metrics(
            10000.0,
            10500.0,
            &equity_curve,
            &trades,
            1.0,
        );

        let result = BacktestResult::new(
            metrics,
            equity_curve.clone(),
            trades,
            1.0,
            10000.0,
            10500.0,
            None, // data_path
        );

        // 测试序列化
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("total_return"));
        assert!(json.contains("equity_curve"));
        assert!(json.contains("trades"));

        // 测试反序列化
        let deserialized: BacktestResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.initial_equity, 10000.0);
        assert_eq!(deserialized.final_equity, 10500.0);
        assert_eq!(deserialized.equity_curve.len(), 2);
        assert_eq!(deserialized.trades.len(), 2);
        assert!(deserialized.benchmark_equity_curve.is_none());
        assert!(deserialized.alpha.is_none());
        assert!(deserialized.annualized_alpha.is_none());
    }

    #[test]
    fn test_backtest_result_with_benchmark() {
        let equity_curve = vec![
            EquityPoint {
                timestamp: 0,
                equity: 10000.0,
                drawdown: 0.0,
            },
            EquityPoint {
                timestamp: 365 * 24 * 60 * 60 * 1000, // 1年后
                equity: 12000.0, // 20% 收益
                drawdown: 0.0,
            },
        ];

        let benchmark_curve = vec![
            EquityPoint {
                timestamp: 0,
                equity: 10000.0,
                drawdown: 0.0,
            },
            EquityPoint {
                timestamp: 365 * 24 * 60 * 60 * 1000, // 1年后
                equity: 11000.0, // 10% 收益
                drawdown: 0.0,
            },
        ];

        let trades = vec![
            PortfolioTrade::new_buy(100.0, 100.0, 0),
            PortfolioTrade::new_sell(120.0, 100.0, 365 * 24 * 60 * 60 * 1000),
        ];

        let metrics = aurora_portfolio::PortfolioAnalytics::calculate_metrics(
            10000.0,
            12000.0,
            &equity_curve,
            &trades,
            365.0,
        );

        let result = BacktestResult::new_with_benchmark(
            metrics,
            equity_curve.clone(),
            trades,
            365.0,
            10000.0,
            12000.0,
            None, // data_path
            benchmark_curve.clone(),
            10.0, // 基准收益率 10%
        );

        // 测试基准数据存在
        assert!(result.benchmark_equity_curve.is_some());
        assert_eq!(result.benchmark_equity_curve.as_ref().unwrap().len(), 2);

        // 测试 Alpha 计算
        assert!(result.alpha.is_some());
        let alpha = result.alpha.unwrap();
        // 策略收益 20% - 基准收益 10% = Alpha 10%
        assert!((alpha - 10.0).abs() < 0.1);

        // 测试年化 Alpha（一年期，应该等于 Alpha）
        assert!(result.annualized_alpha.is_some());
        let ann_alpha = result.annualized_alpha.unwrap();
        assert!((ann_alpha - 10.0).abs() < 0.1);

        // 测试序列化
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("benchmark_equity_curve"));
        assert!(json.contains("alpha"));
        assert!(json.contains("annualized_alpha"));

        // 测试反序列化
        let deserialized: BacktestResult = serde_json::from_str(&json).unwrap();
        assert!(deserialized.benchmark_equity_curve.is_some());
        assert!(deserialized.alpha.is_some());
        assert!(deserialized.annualized_alpha.is_some());
    }
}
