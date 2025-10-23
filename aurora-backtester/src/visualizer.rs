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

//! 回测结果可视化模块
//!
//! 提供回测结果的可视化展示功能，包括：
//! - 权益曲线图
//! - 最大回撤图
//! - 交易点位标记图
//! - 性能指标汇总
//! - HTML 报告生成

mod charts;
mod report;

pub use charts::*;
pub use report::*;

use anyhow::Result;
use aurora_portfolio::PerformanceMetrics;

/// 回测可视化器
///
/// 负责生成回测结果的各种可视化图表和报告。
///
/// # 示例
///
/// ```rust,no_run
/// use aurora_backtester::{BacktestVisualizer, BacktestData};
/// use aurora_portfolio::PerformanceMetrics;
/// # use anyhow::Result;
///
/// # fn main() -> Result<()> {
/// let visualizer = BacktestVisualizer::new();
/// 
/// // 创建示例数据
/// let data = BacktestData {
///     equity_curve: vec![(1640995200000, 10000.0)],
///     drawdown_curve: vec![(1640995200000, 0.0)],
///     price_data: vec![(1640995200000, 100.0)],
///     buy_trades: vec![],
///     sell_trades: vec![],
///     metrics: PerformanceMetrics {
///         total_return: 0.1,
///         annualized_return: 0.15,
///         max_drawdown: 0.05,
///         max_drawdown_duration: 5.0,
///         annualized_volatility: 12.5,
///         sharpe_ratio: 1.5,
///         sortino_ratio: 1.8,
///         calmar_ratio: 3.0,
///         win_rate: 0.6,
///         total_trades: 10,
///         winning_trades: 6,
///         losing_trades: 4,
///         average_win: 150.0,
///         average_loss: -50.0,
///         profit_loss_ratio: 3.0,
///         profit_factor: 2.5,
///         max_consecutive_wins: 3,
///         max_consecutive_losses: 2,
///         avg_holding_period: 24.0,
///         max_win: 300.0,
///         max_loss: -100.0,
///     },
///     initial_cash: 10000.0,
/// };
///
/// // 生成完整的 HTML 报告
/// visualizer.generate_html_report(&data, "output/report.html")?;
/// # Ok(())
/// # }
/// ```
pub struct BacktestVisualizer {
    /// 图表宽度（像素）
    pub chart_width: u32,
    /// 图表高度（像素）
    pub chart_height: u32,
}

impl Default for BacktestVisualizer {
    fn default() -> Self {
        Self {
            chart_width: 1200,
            chart_height: 600,
        }
    }
}

impl BacktestVisualizer {
    /// 创建新的可视化器
    pub fn new() -> Self {
        Self::default()
    }

    /// 使用自定义尺寸创建可视化器
    pub fn with_size(chart_width: u32, chart_height: u32) -> Self {
        Self {
            chart_width,
            chart_height,
        }
    }

    /// 生成权益曲线图
    ///
    /// # 参数
    ///
    /// * `data` - 回测数据
    /// * `output_path` - 输出文件路径
    pub fn plot_equity_curve(&self, data: &BacktestData, output_path: &str) -> Result<()> {
        charts::plot_equity_curve(data, output_path, self.chart_width, self.chart_height)
    }

    /// 生成回撤图
    ///
    /// # 参数
    ///
    /// * `data` - 回测数据
    /// * `output_path` - 输出文件路径
    pub fn plot_drawdown(&self, data: &BacktestData, output_path: &str) -> Result<()> {
        charts::plot_drawdown(data, output_path, self.chart_width, self.chart_height)
    }

    /// 生成交易点位标记图
    ///
    /// # 参数
    ///
    /// * `data` - 回测数据
    /// * `output_path` - 输出文件路径
    pub fn plot_trades(&self, data: &BacktestData, output_path: &str) -> Result<()> {
        charts::plot_trades(data, output_path, self.chart_width, self.chart_height)
    }

    /// 生成完整的 HTML 报告
    ///
    /// # 参数
    ///
    /// * `data` - 回测数据
    /// * `output_path` - 输出文件路径
    pub fn generate_html_report(&self, data: &BacktestData, output_path: &str) -> Result<()> {
        report::generate_html_report(data, output_path, self.chart_width, self.chart_height)
    }
}

/// 回测数据
///
/// 包含用于可视化的所有必要数据。
#[derive(Debug, Clone)]
pub struct BacktestData {
    /// 权益曲线数据点 (timestamp, equity)
    pub equity_curve: Vec<(i64, f64)>,
    /// 回撤数据点 (timestamp, drawdown_pct)
    pub drawdown_curve: Vec<(i64, f64)>,
    /// 价格数据点 (timestamp, price)
    pub price_data: Vec<(i64, f64)>,
    /// 买入交易 (timestamp, price)
    pub buy_trades: Vec<(i64, f64)>,
    /// 卖出交易 (timestamp, price)
    pub sell_trades: Vec<(i64, f64)>,
    /// 性能指标
    pub metrics: PerformanceMetrics,
    /// 初始资金
    pub initial_cash: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_portfolio::PerformanceMetrics;

    fn create_test_data() -> BacktestData {
        BacktestData {
            equity_curve: vec![
                (1640995200000, 10000.0),
                (1640995260000, 10500.0),
                (1640995320000, 11000.0),
            ],
            drawdown_curve: vec![
                (1640995200000, 0.0),
                (1640995260000, 0.0),
                (1640995320000, 0.0),
            ],
            price_data: vec![
                (1640995200000, 100.0),
                (1640995260000, 105.0),
                (1640995320000, 110.0),
            ],
            buy_trades: vec![(1640995200000, 100.0)],
            sell_trades: vec![(1640995320000, 110.0)],
            metrics: PerformanceMetrics {
                total_return: 0.1,
                annualized_return: 0.15,
                max_drawdown: 0.05,
                max_drawdown_duration: 5.0,
                annualized_volatility: 12.5,
                sharpe_ratio: 1.5,
                sortino_ratio: 1.8,
                calmar_ratio: 3.0,
                win_rate: 0.6,
                total_trades: 10,
                winning_trades: 6,
                losing_trades: 4,
                average_win: 150.0,
                average_loss: -50.0,
                profit_loss_ratio: 3.0,
                profit_factor: 2.5,
                max_consecutive_wins: 3,
                max_consecutive_losses: 2,
                avg_holding_period: 24.0,
                max_win: 300.0,
                max_loss: -100.0,
            },
            initial_cash: 10000.0,
        }
    }

    #[test]
    fn test_visualizer_creation() {
        let visualizer = BacktestVisualizer::new();
        assert_eq!(visualizer.chart_width, 1200);
        assert_eq!(visualizer.chart_height, 600);

        let custom_visualizer = BacktestVisualizer::with_size(800, 400);
        assert_eq!(custom_visualizer.chart_width, 800);
        assert_eq!(custom_visualizer.chart_height, 400);
    }

    #[test]
    fn test_backtest_data_creation() {
        let data = create_test_data();
        assert_eq!(data.equity_curve.len(), 3);
        assert_eq!(data.buy_trades.len(), 1);
        assert_eq!(data.sell_trades.len(), 1);
        assert_eq!(data.initial_cash, 10000.0);
    }
}
