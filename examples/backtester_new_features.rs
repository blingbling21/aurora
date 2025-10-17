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

//! 新功能使用示例
//!
//! 展示如何使用 aurora-backtester 的新功能：
//! 1. 买卖价差模式
//! 2. 可视化报告生成
//! 3. 向量化回测引擎

use anyhow::Result;
use aurora_backtester::{
    BacktestData, BacktestEngine, BacktestVisualizer, PricingMode, VectorizedBacktestEngine,
};
use aurora_config::PortfolioConfig;
use aurora_core::Kline;
use aurora_portfolio::PerformanceMetrics;
use aurora_strategy::MACrossoverStrategy;

/// 示例1: 使用买卖价差模式进行回测
pub async fn example_bid_ask_spread() -> Result<()> {
    println!("=== 示例1: 买卖价差模式 ===\n");

    // 创建测试数据
    let klines = create_sample_klines();

    let strategy = MACrossoverStrategy::new(2, 3);
    let portfolio_config = create_test_config();

    // 使用 0.1% 的买卖价差
    let pricing_mode = PricingMode::BidAsk { spread_pct: 0.001 };

    let mut engine =
        BacktestEngine::with_pricing_mode(strategy, &portfolio_config, pricing_mode)?;

    println!("定价模式: Bid-Ask Spread (0.1%)");
    println!("初始资金: {:.2}\n", portfolio_config.initial_cash);

    engine.run(&klines).await?;

    println!("\n✓ 回测完成！");
    println!("注意: 使用买卖价差模式会略微降低收益，但更接近真实交易情况。\n");

    Ok(())
}

/// 示例2: 生成可视化报告
pub async fn example_visualization() -> Result<()> {
    println!("=== 示例2: 可视化报告生成 ===\n");

    // 创建测试数据
    let data = create_sample_backtest_data();

    // 创建可视化器
    let visualizer = BacktestVisualizer::new();

    println!("生成图表中...");

    // 创建输出目录
    std::fs::create_dir_all("output")?;

    // 生成单独的图表
    visualizer.plot_equity_curve(&data, "output/equity_curve.png")?;
    println!("✓ 权益曲线图: output/equity_curve.png");

    visualizer.plot_drawdown(&data, "output/drawdown.png")?;
    println!("✓ 回撤曲线图: output/drawdown.png");

    visualizer.plot_trades(&data, "output/trades.png")?;
    println!("✓ 交易点位图: output/trades.png");

    // 生成完整 HTML 报告
    visualizer.generate_html_report(&data, "output/report.html")?;
    println!("✓ HTML 报告: output/report.html");

    println!("\n所有图表已生成！");
    println!("请用浏览器打开 output/report.html 查看完整报告。\n");

    Ok(())
}

/// 示例3: 向量化回测引擎
pub fn example_vectorized_backtest() -> Result<()> {
    println!("=== 示例3: 向量化回测引擎 ===\n");

    let klines = create_sample_klines();

    // 创建向量化引擎
    let engine = VectorizedBacktestEngine::new(10000.0);

    println!("测试参数组合:");
    println!("短期周期: 2-5");
    println!("长期周期: 3-10\n");

    let mut best_return = f64::NEG_INFINITY;
    let mut best_params = (0, 0);

    // 快速测试多个参数组合
    for short in 2..=5 {
        for long in 3..=10 {
            if short >= long {
                continue;
            }

            let signals = engine.calculate_ma_crossover_signals(&klines, short, long);
            let result = engine.run(&klines, &signals)?;

            println!(
                "参数 [{:2}, {:2}] - 收益: {:>7.2}%, 回撤: {:>6.2}%, 交易: {:2}次, 胜率: {:>5.1}%",
                short,
                long,
                result.total_return * 100.0,
                result.max_drawdown * 100.0,
                result.total_trades,
                result.win_rate * 100.0
            );

            if result.total_return > best_return {
                best_return = result.total_return;
                best_params = (short, long);
            }
        }
    }

    println!("\n最佳参数:");
    println!("  短期周期: {}", best_params.0);
    println!("  长期周期: {}", best_params.1);
    println!("  收益率: {:.2}%", best_return * 100.0);

    println!("\n✓ 向量化回测完成！");
    println!("注意: 向量化回测速度快，但不考虑路径依赖因素。\n");

    Ok(())
}

/// 示例4: 对比不同定价模式
pub async fn example_pricing_mode_comparison() -> Result<()> {
    println!("=== 示例4: 定价模式对比 ===\n");

    let klines = create_sample_klines();
    let strategy1 = MACrossoverStrategy::new(2, 3);
    let strategy2 = MACrossoverStrategy::new(2, 3);
    let portfolio_config = create_test_config();

    // 收盘价模式
    println!("1. 收盘价模式回测:");
    let mut engine1 =
        BacktestEngine::with_pricing_mode(strategy1, &portfolio_config, PricingMode::Close)?;
    engine1.run(&klines).await?;
    let final_equity1 = engine1
        .portfolio()
        .get_total_equity(klines.last().unwrap().close);

    println!("\n2. 买卖价差模式回测 (0.1% 价差):");
    let mut engine2 = BacktestEngine::with_pricing_mode(
        strategy2,
        &portfolio_config,
        PricingMode::BidAsk { spread_pct: 0.001 },
    )?;
    engine2.run(&klines).await?;
    let final_equity2 = engine2
        .portfolio()
        .get_total_equity(klines.last().unwrap().close);

    println!("\n对比结果:");
    println!("  收盘价模式最终权益: {:.2}", final_equity1);
    println!("  买卖价差模式最终权益: {:.2}", final_equity2);
    println!(
        "  差异: {:.2} ({:.2}%)",
        final_equity1 - final_equity2,
        (final_equity1 - final_equity2) / final_equity1 * 100.0
    );

    println!("\n结论: 买卖价差模式会略微降低收益，但更接近真实交易。\n");

    Ok(())
}

// ==================== 辅助函数 ====================

fn create_sample_klines() -> Vec<Kline> {
    vec![
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 100.0,
            volume: 1000.0,
        },
        Kline {
            timestamp: 1640995260000,
            open: 100.0,
            high: 110.0,
            low: 98.0,
            close: 105.0,
            volume: 1200.0,
        },
        Kline {
            timestamp: 1640995320000,
            open: 105.0,
            high: 112.0,
            low: 103.0,
            close: 110.0,
            volume: 1100.0,
        },
        Kline {
            timestamp: 1640995380000,
            open: 110.0,
            high: 115.0,
            low: 108.0,
            close: 108.0,
            volume: 1300.0,
        },
        Kline {
            timestamp: 1640995440000,
            open: 108.0,
            high: 113.0,
            low: 105.0,
            close: 112.0,
            volume: 1250.0,
        },
        Kline {
            timestamp: 1640995500000,
            open: 112.0,
            high: 118.0,
            low: 110.0,
            close: 115.0,
            volume: 1400.0,
        },
        Kline {
            timestamp: 1640995560000,
            open: 115.0,
            high: 120.0,
            low: 113.0,
            close: 118.0,
            volume: 1350.0,
        },
        Kline {
            timestamp: 1640995620000,
            open: 118.0,
            high: 122.0,
            low: 116.0,
            close: 120.0,
            volume: 1500.0,
        },
    ]
}

fn create_sample_backtest_data() -> BacktestData {
    BacktestData {
        equity_curve: vec![
            (1640995200000, 10000.0),
            (1640995260000, 10500.0),
            (1640995320000, 11000.0),
            (1640995380000, 10800.0),
            (1640995440000, 11200.0),
            (1640995500000, 11500.0),
            (1640995560000, 11800.0),
            (1640995620000, 12000.0),
        ],
        drawdown_curve: vec![
            (1640995200000, 0.0),
            (1640995260000, 0.0),
            (1640995320000, 0.0),
            (1640995380000, -0.018),
            (1640995440000, 0.0),
            (1640995500000, 0.0),
            (1640995560000, 0.0),
            (1640995620000, 0.0),
        ],
        price_data: vec![
            (1640995200000, 100.0),
            (1640995260000, 105.0),
            (1640995320000, 110.0),
            (1640995380000, 108.0),
            (1640995440000, 112.0),
            (1640995500000, 115.0),
            (1640995560000, 118.0),
            (1640995620000, 120.0),
        ],
        buy_trades: vec![(1640995200000, 100.0), (1640995380000, 108.0)],
        sell_trades: vec![(1640995320000, 110.0)],
        metrics: PerformanceMetrics {
            total_return: 0.20,
            annualized_return: 0.25,
            max_drawdown: 0.018,
            sharpe_ratio: 2.5,
            sortino_ratio: 3.0,
            calmar_ratio: 13.89,
            win_rate: 0.5,
            profit_factor: 3.0,
            total_trades: 2,
            winning_trades: 1,
            losing_trades: 1,
            avg_win: 200.0,
            avg_loss: -50.0,
            largest_win: 200.0,
            largest_loss: -50.0,
        },
        initial_cash: 10000.0,
    }
}

fn create_test_config() -> PortfolioConfig {
    PortfolioConfig {
        initial_cash: 10000.0,
        commission: 0.001,
        slippage: 0.0005,
        max_position_size: None,
        max_positions: None,
        risk_rules: None,
        position_sizing: None,
    }
}

// ==================== 主函数 ====================

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n");
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Aurora Backtester - 新功能示例程序        ║");
    println!("╚══════════════════════════════════════════════╝");
    println!("\n");

    // 运行所有示例
    example_bid_ask_spread().await?;
    println!("{}\n", "=".repeat(50));

    example_visualization().await?;
    println!("{}\n", "=".repeat(50));

    example_vectorized_backtest()?;
    println!("{}\n", "=".repeat(50));

    example_pricing_mode_comparison().await?;

    println!("\n所有示例运行完成！");
    println!("请查看 output 目录中的可视化报告。\n");

    Ok(())
}
