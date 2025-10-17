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

//! 图表绘制模块
//!
//! 使用 plotters 库绘制各种回测分析图表。

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use plotters::prelude::*;

use super::BacktestData;

/// 绘制权益曲线图
///
/// # 参数
///
/// * `data` - 回测数据
/// * `output_path` - 输出文件路径
/// * `width` - 图表宽度
/// * `height` - 图表高度
pub fn plot_equity_curve(
    data: &BacktestData,
    output_path: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)
        .context("无法填充背景")?;

    if data.equity_curve.is_empty() {
        return Ok(());
    }

    // 计算数据范围
    let min_equity = data
        .equity_curve
        .iter()
        .map(|(_, e)| *e)
        .fold(f64::INFINITY, f64::min);
    let max_equity = data
        .equity_curve
        .iter()
        .map(|(_, e)| *e)
        .fold(f64::NEG_INFINITY, f64::max);

    let start_time = data.equity_curve.first().unwrap().0;
    let end_time = data.equity_curve.last().unwrap().0;

    // 添加一些边距
    let equity_margin = (max_equity - min_equity) * 0.1;
    let y_min = (min_equity - equity_margin).max(0.0);
    let y_max = max_equity + equity_margin;

    // 创建图表
    let mut chart = ChartBuilder::on(&root)
        .caption("权益曲线", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(start_time..end_time, y_min..y_max)
        .context("无法构建图表")?;

    chart
        .configure_mesh()
        .x_desc("时间")
        .y_desc("权益")
        .x_label_formatter(&|x| format_timestamp(*x))
        .y_label_formatter(&|y| format!("{:.2}", y))
        .draw()
        .context("无法绘制网格")?;

    // 绘制权益曲线
    chart
        .draw_series(LineSeries::new(
            data.equity_curve.iter().map(|(t, e)| (*t, *e)),
            &BLUE,
        ))
        .context("无法绘制权益曲线")?
        .label("权益")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 绘制初始资金基准线
    chart
        .draw_series(LineSeries::new(
            vec![(start_time, data.initial_cash), (end_time, data.initial_cash)],
            &RED.mix(0.5),
        ))
        .context("无法绘制基准线")?
        .label("初始资金")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .context("无法绘制图例")?;

    root.present().context("无法保存图表")?;

    Ok(())
}

/// 绘制回撤图
///
/// # 参数
///
/// * `data` - 回测数据
/// * `output_path` - 输出文件路径
/// * `width` - 图表宽度
/// * `height` - 图表高度
pub fn plot_drawdown(
    data: &BacktestData,
    output_path: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)
        .context("无法填充背景")?;

    if data.drawdown_curve.is_empty() {
        return Ok(());
    }

    // 计算数据范围
    let max_drawdown = data
        .drawdown_curve
        .iter()
        .map(|(_, d)| *d)
        .fold(f64::NEG_INFINITY, f64::max);

    let start_time = data.drawdown_curve.first().unwrap().0;
    let end_time = data.drawdown_curve.last().unwrap().0;

    // 添加边距
    let y_max = (max_drawdown * 1.1).min(0.0);

    // 创建图表
    let mut chart = ChartBuilder::on(&root)
        .caption("回撤曲线", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(start_time..end_time, y_max..0.0)
        .context("无法构建图表")?;

    chart
        .configure_mesh()
        .x_desc("时间")
        .y_desc("回撤 (%)")
        .x_label_formatter(&|x| format_timestamp(*x))
        .y_label_formatter(&|y| format!("{:.2}%", y * 100.0))
        .draw()
        .context("无法绘制网格")?;

    // 绘制回撤曲线（填充区域）
    chart
        .draw_series(AreaSeries::new(
            data.drawdown_curve.iter().map(|(t, d)| (*t, *d)),
            0.0,
            &RED.mix(0.3),
        ))
        .context("无法绘制回撤区域")?
        .label("回撤")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 20, y + 5)], &RED.mix(0.3)));

    // 绘制回撤线条
    chart
        .draw_series(LineSeries::new(
            data.drawdown_curve.iter().map(|(t, d)| (*t, *d)),
            &RED,
        ))
        .context("无法绘制回撤线")?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .context("无法绘制图例")?;

    root.present().context("无法保存图表")?;

    Ok(())
}

/// 绘制交易点位标记图
///
/// # 参数
///
/// * `data` - 回测数据
/// * `output_path` - 输出文件路径
/// * `width` - 图表宽度
/// * `height` - 图表高度
pub fn plot_trades(
    data: &BacktestData,
    output_path: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    root.fill(&WHITE)
        .context("无法填充背景")?;

    if data.price_data.is_empty() {
        return Ok(());
    }

    // 计算价格范围
    let min_price = data
        .price_data
        .iter()
        .map(|(_, p)| *p)
        .fold(f64::INFINITY, f64::min);
    let max_price = data
        .price_data
        .iter()
        .map(|(_, p)| *p)
        .fold(f64::NEG_INFINITY, f64::max);

    let start_time = data.price_data.first().unwrap().0;
    let end_time = data.price_data.last().unwrap().0;

    // 添加边距
    let price_margin = (max_price - min_price) * 0.1;
    let y_min = min_price - price_margin;
    let y_max = max_price + price_margin;

    // 创建图表
    let mut chart = ChartBuilder::on(&root)
        .caption("价格与交易点位", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(start_time..end_time, y_min..y_max)
        .context("无法构建图表")?;

    chart
        .configure_mesh()
        .x_desc("时间")
        .y_desc("价格")
        .x_label_formatter(&|x| format_timestamp(*x))
        .y_label_formatter(&|y| format!("{:.2}", y))
        .draw()
        .context("无法绘制网格")?;

    // 绘制价格曲线
    chart
        .draw_series(LineSeries::new(
            data.price_data.iter().map(|(t, p)| (*t, *p)),
            &BLUE,
        ))
        .context("无法绘制价格曲线")?
        .label("价格")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 绘制买入点
    chart
        .draw_series(data.buy_trades.iter().map(|(t, p)| {
            Circle::new((*t, *p), 5, GREEN.filled())
        }))
        .context("无法绘制买入点")?
        .label("买入")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, GREEN.filled()));

    // 绘制卖出点
    chart
        .draw_series(data.sell_trades.iter().map(|(t, p)| {
            Circle::new((*t, *p), 5, RED.filled())
        }))
        .context("无法绘制卖出点")?
        .label("卖出")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, RED.filled()));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .context("无法绘制图例")?;

    root.present().context("无法保存图表")?;

    Ok(())
}

/// 格式化时间戳
fn format_timestamp(timestamp: i64) -> String {
    let dt = Utc.timestamp_millis_opt(timestamp).unwrap();
    dt.format("%m-%d %H:%M").to_string()
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
                (1640995380000, 10800.0),
                (1640995440000, 11200.0),
            ],
            drawdown_curve: vec![
                (1640995200000, 0.0),
                (1640995260000, 0.0),
                (1640995320000, 0.0),
                (1640995380000, -0.018),
                (1640995440000, 0.0),
            ],
            price_data: vec![
                (1640995200000, 100.0),
                (1640995260000, 105.0),
                (1640995320000, 110.0),
                (1640995380000, 108.0),
                (1640995440000, 112.0),
            ],
            buy_trades: vec![(1640995200000, 100.0), (1640995380000, 108.0)],
            sell_trades: vec![(1640995320000, 110.0)],
            metrics: PerformanceMetrics {
                total_return: 0.12,
                annualized_return: 0.15,
                max_drawdown: 0.018,
                sharpe_ratio: 1.5,
                win_rate: 0.5,
                total_trades: 2,
                winning_trades: 1,
                losing_trades: 1,
                average_win: 100.0,
                average_loss: -50.0,
                profit_loss_ratio: 2.0,
            },
            initial_cash: 10000.0,
        }
    }

    #[test]
    fn test_plot_equity_curve() {
        let data = create_test_data();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("equity.png");

        let result = plot_equity_curve(&data, output_path.to_str().unwrap(), 800, 600);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_plot_drawdown() {
        let data = create_test_data();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("drawdown.png");

        let result = plot_drawdown(&data, output_path.to_str().unwrap(), 800, 600);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_plot_trades() {
        let data = create_test_data();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("trades.png");

        let result = plot_trades(&data, output_path.to_str().unwrap(), 800, 600);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = 1640995200000; // 2022-01-01 00:00:00 UTC
        let formatted = format_timestamp(timestamp);
        assert!(formatted.contains("-"));
        assert!(formatted.contains(":"));
    }
}
