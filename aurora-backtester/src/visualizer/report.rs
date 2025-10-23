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

//! HTML 报告生成模块
//!
//! 生成包含图表和性能指标的完整 HTML 回测报告。

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::BacktestData;
use crate::visualizer::charts;

/// 生成完整的 HTML 报告
///
/// # 参数
///
/// * `data` - 回测数据
/// * `output_path` - 输出文件路径
/// * `width` - 图表宽度
/// * `height` - 图表高度
pub fn generate_html_report(
    data: &BacktestData,
    output_path: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let output_dir = Path::new(output_path)
        .parent()
        .context("无效的输出路径")?;

    // 创建输出目录
    fs::create_dir_all(output_dir).context("无法创建输出目录")?;

    // 生成图表
    let equity_chart_path = output_dir.join("equity_curve.png");
    let drawdown_chart_path = output_dir.join("drawdown.png");
    let trades_chart_path = output_dir.join("trades.png");

    charts::plot_equity_curve(data, equity_chart_path.to_str().unwrap(), width, height)?;
    charts::plot_drawdown(data, drawdown_chart_path.to_str().unwrap(), width, height)?;
    charts::plot_trades(data, trades_chart_path.to_str().unwrap(), width, height)?;

    // 生成 HTML 内容
    let html_content = generate_html_content(data);

    // 写入 HTML 文件
    fs::write(output_path, html_content).context("无法写入 HTML 文件")?;

    Ok(())
}

/// 生成 HTML 内容
fn generate_html_content(data: &BacktestData) -> String {
    let metrics = &data.metrics;

    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aurora 回测报告</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            color: #333;
        }}

        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 10px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
            overflow: hidden;
        }}

        header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}

        h1 {{
            font-size: 2.5em;
            margin-bottom: 10px;
        }}

        .subtitle {{
            font-size: 1.1em;
            opacity: 0.9;
        }}

        .metrics-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            padding: 30px;
            background: #f8f9fa;
        }}

        .metric-card {{
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }}

        .metric-card:hover {{
            transform: translateY(-5px);
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
        }}

        .metric-label {{
            font-size: 0.9em;
            color: #666;
            margin-bottom: 8px;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}

        .metric-value {{
            font-size: 1.8em;
            font-weight: bold;
            color: #333;
        }}

        .metric-value.positive {{
            color: #28a745;
        }}

        .metric-value.negative {{
            color: #dc3545;
        }}

        .charts-section {{
            padding: 30px;
        }}

        .chart-container {{
            margin-bottom: 40px;
        }}

        .chart-title {{
            font-size: 1.5em;
            margin-bottom: 15px;
            color: #333;
            border-left: 4px solid #667eea;
            padding-left: 15px;
        }}

        .chart-image {{
            width: 100%;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }}

        .summary-section {{
            padding: 30px;
            background: #f8f9fa;
        }}

        .summary-title {{
            font-size: 1.8em;
            margin-bottom: 20px;
            color: #333;
        }}

        .summary-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
        }}

        .summary-item {{
            background: white;
            padding: 15px 20px;
            border-radius: 8px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
        }}

        .summary-label {{
            font-weight: 500;
            color: #555;
        }}

        .summary-value {{
            font-weight: bold;
            color: #333;
        }}

        footer {{
            background: #2c3e50;
            color: white;
            text-align: center;
            padding: 20px;
            font-size: 0.9em;
        }}

        @media print {{
            body {{
                background: white;
            }}

            .container {{
                box-shadow: none;
            }}

            .metric-card:hover {{
                transform: none;
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🌟 Aurora 回测报告</h1>
            <p class="subtitle">专业量化交易策略回测分析</p>
        </header>

        <div class="metrics-grid">
            <div class="metric-card">
                <div class="metric-label">总收益率</div>
                <div class="metric-value {}">{:.2}%</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">年化收益率</div>
                <div class="metric-value {}">{:.2}%</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">最大回撤</div>
                <div class="metric-value negative">{:.2}%</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">回撤持续时间</div>
                <div class="metric-value">{:.1} 天</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">年化波动率</div>
                <div class="metric-value">{:.2}%</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">夏普比率</div>
                <div class="metric-value">{:.2}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">索提诺比率</div>
                <div class="metric-value">{:.2}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">卡玛比率</div>
                <div class="metric-value">{:.2}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">胜率</div>
                <div class="metric-value">{:.2}%</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">盈亏比</div>
                <div class="metric-value">{:.2}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">利润因子</div>
                <div class="metric-value">{:.2}</div>
            </div>
            <div class="metric-card">
                <div class="metric-label">平均持仓时间</div>
                <div class="metric-value">{:.1}h</div>
            </div>
        </div>

        <div class="charts-section">
            <div class="chart-container">
                <h2 class="chart-title">📈 权益曲线</h2>
                <img src="equity_curve.png" alt="权益曲线" class="chart-image">
            </div>

            <div class="chart-container">
                <h2 class="chart-title">📉 回撤分析</h2>
                <img src="drawdown.png" alt="回撤曲线" class="chart-image">
            </div>

            <div class="chart-container">
                <h2 class="chart-title">🎯 交易点位</h2>
                <img src="trades.png" alt="交易点位" class="chart-image">
            </div>
        </div>

        <div class="summary-section">
            <h2 class="summary-title">📊 交易统计</h2>
            <div class="summary-grid">
                <div class="summary-item">
                    <span class="summary-label">初始资金</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">最终权益</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">总交易次数</span>
                    <span class="summary-value">{}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">盈利次数</span>
                    <span class="summary-value">{}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">亏损次数</span>
                    <span class="summary-value">{}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">平均盈利</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">平均亏损</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">盈亏比</span>
                    <span class="summary-value">{:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">利润因子</span>
                    <span class="summary-value">{:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">最大单笔盈利</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">最大单笔亏损</span>
                    <span class="summary-value">${:.2}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">最大连续盈利</span>
                    <span class="summary-value">{}</span>
                </div>
                <div class="summary-item">
                    <span class="summary-label">最大连续亏损</span>
                    <span class="summary-value">{}</span>
                </div>
            </div>
        </div>

        <footer>
            <p>由 Aurora 量化交易系统生成 | Powered by Rust & Plotters</p>
        </footer>
    </div>
</body>
</html>"#,
        if metrics.total_return >= 0.0 { "positive" } else { "negative" },
        metrics.total_return * 100.0,
        if metrics.annualized_return >= 0.0 { "positive" } else { "negative" },
        metrics.annualized_return * 100.0,
        metrics.max_drawdown * 100.0,
        metrics.max_drawdown_duration,
        metrics.annualized_volatility,
        metrics.sharpe_ratio,
        metrics.sortino_ratio,
        metrics.calmar_ratio,
        metrics.win_rate * 100.0,
        metrics.profit_loss_ratio,
        metrics.profit_factor,
        metrics.avg_holding_period,
        data.initial_cash,
        data.initial_cash * (1.0 + metrics.total_return),
        metrics.total_trades,
        metrics.winning_trades,
        metrics.losing_trades,
        metrics.average_win,
        metrics.average_loss,
        metrics.profit_loss_ratio,
        metrics.profit_factor,
        metrics.max_win,
        metrics.max_loss,
        metrics.max_consecutive_wins,
        metrics.max_consecutive_losses,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_portfolio::PerformanceMetrics;
    use tempfile::tempdir;

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
    fn test_generate_html_content() {
        let data = create_test_data();
        let html = generate_html_content(&data);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Aurora 回测报告"));
        assert!(html.contains("总收益率"));
        assert!(html.contains("夏普比率"));
        assert!(html.contains("equity_curve.png"));
    }

    #[test]
    fn test_generate_html_report() {
        let data = create_test_data();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("report.html");

        let result = generate_html_report(&data, output_path.to_str().unwrap(), 800, 600);
        assert!(result.is_ok());
        assert!(output_path.exists());

        // 检查图表文件是否生成
        assert!(dir.path().join("equity_curve.png").exists());
        assert!(dir.path().join("drawdown.png").exists());
        assert!(dir.path().join("trades.png").exists());
    }
}
