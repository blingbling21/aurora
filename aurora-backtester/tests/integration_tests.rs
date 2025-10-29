//! 回测引擎集成测试

use anyhow::Result;
use aurora_backtester::engine::{BacktestEngine, run_backtest};
use aurora_config::PortfolioConfig;
use aurora_core::Kline;
use aurora_portfolio::Portfolio;
use aurora_strategy::MACrossoverStrategy;
use std::fs::File;
use std::io::Write;
use tempfile::{TempDir, tempdir};

/// 创建默认的测试用 PortfolioConfig
fn create_test_portfolio_config(initial_cash: f64) -> PortfolioConfig {
    PortfolioConfig {
        initial_cash,
        commission: 0.001,
        slippage: 0.0005,
        max_position_size: None,
        max_positions: None,
        risk_rules: None,
        position_sizing: None,
    }
}

/// 创建测试用的CSV数据文件
fn create_test_csv_file() -> Result<(String, TempDir)> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test_data.csv");
    let mut file = File::create(&file_path)?;

    // 写入CSV头部
    writeln!(file, "timestamp,open,high,low,close,volume")?;

    // 写入测试数据 - 创建一个先下跌后上涨的趋势
    let test_data = vec![
        (1640995200000i64, 50000.0, 50500.0, 49500.0, 50000.0, 100.0),
        (1640995260000, 50000.0, 50200.0, 49800.0, 49900.0, 120.0),
        (1640995320000, 49900.0, 50100.0, 49600.0, 49800.0, 110.0),
        (1640995380000, 49800.0, 50000.0, 49500.0, 49700.0, 105.0),
        (1640995440000, 49700.0, 49900.0, 49400.0, 49600.0, 95.0),
        // 开始上涨趋势
        (1640995500000, 49600.0, 49900.0, 49500.0, 49800.0, 115.0),
        (1640995560000, 49800.0, 50200.0, 49700.0, 50100.0, 125.0),
        (1640995620000, 50100.0, 50600.0, 50000.0, 50500.0, 140.0),
        (1640995680000, 50500.0, 51000.0, 50300.0, 50900.0, 160.0),
        (1640995740000, 50900.0, 51500.0, 50700.0, 51300.0, 180.0),
    ];

    for (timestamp, open, high, low, close, volume) in test_data {
        writeln!(
            file,
            "{},{},{},{},{},{}",
            timestamp, open, high, low, close, volume
        )?;
    }

    Ok((file_path.to_string_lossy().to_string(), dir))
}

/// 测试回测引擎的基本功能
#[tokio::test]
async fn test_backtest_engine_basic() -> Result<()> {
    let strategy = MACrossoverStrategy::new(2, 3);
    let portfolio_config = create_test_portfolio_config(10000.0);
    let engine = BacktestEngine::new(strategy, &portfolio_config)?;

    // 验证初始状态
    assert_eq!(engine.portfolio().get_cash(), 10000.0);
    assert_eq!(engine.portfolio().get_position(), 0.0);

    Ok(())
}

/// 测试完整的回测流程
#[tokio::test]
async fn test_complete_backtest_flow() -> Result<()> {
    let (csv_file, _temp_dir) = create_test_csv_file()?;

    // 运行回测
    let portfolio_config = create_test_portfolio_config(10000.0);
    let result = run_backtest(&csv_file, "ma-crossover", 2, 3, &portfolio_config, None).await;
    assert!(result.is_ok());

    Ok(())
}

/// 测试回测引擎的K线处理
#[tokio::test]
async fn test_kline_processing() -> Result<()> {
    let strategy = MACrossoverStrategy::new(2, 3);
    let portfolio_config = create_test_portfolio_config(10000.0);
    let mut engine = BacktestEngine::new(strategy, &portfolio_config)?;

    // 创建测试K线数据
    let klines = vec![
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        },
        Kline {
            timestamp: 1640995260000,
            open: 102.0,
            high: 108.0,
            low: 98.0,
            close: 106.0,
            volume: 1200.0,
        },
        Kline {
            timestamp: 1640995320000,
            open: 106.0,
            high: 112.0,
            low: 102.0,
            close: 110.0,
            volume: 1100.0,
        },
    ];

    let result = engine.run(&klines, None).await;
    assert!(result.is_ok());

    // 验证权益曲线已更新
    assert!(engine.portfolio().get_equity_curve().len() > 0);

    Ok(())
}

/// 测试CSV文件加载错误处理
#[tokio::test]
async fn test_csv_loading_errors() {
    let portfolio_config = create_test_portfolio_config(10000.0);
    
    // 测试不存在的文件
    let result = run_backtest("nonexistent_file.csv", "ma-crossover", 5, 20, &portfolio_config, None).await;
    assert!(result.is_err());

    // 测试无效的策略名称
    let (csv_file, _temp_dir) = create_test_csv_file().unwrap();
    let result = run_backtest(&csv_file, "invalid-strategy", 5, 20, &portfolio_config, None).await;
    assert!(result.is_err());
}

/// 测试空数据文件处理
#[tokio::test]
async fn test_empty_data_file() -> Result<()> {
    // 创建空的CSV文件
    let dir = tempdir()?;
    let file_path = dir.path().join("empty_data.csv");
    let mut file = File::create(&file_path)?;
    writeln!(file, "timestamp,open,high,low,close,volume")?; // 只有头部

    let portfolio_config = create_test_portfolio_config(10000.0);
    let result = run_backtest(&file_path.to_string_lossy(), "ma-crossover", 5, 20, &portfolio_config, None).await;

    assert!(result.is_err());

    Ok(())
}

/// 测试无效CSV数据处理
#[tokio::test]
async fn test_invalid_csv_data() -> Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("invalid_data.csv");
    let mut file = File::create(&file_path)?;

    // 写入无效数据
    writeln!(file, "timestamp,open,high,low,close,volume")?;
    writeln!(file, "invalid,data,here,bad,format,wrong")?;
    writeln!(file, "1640995200000,50000.0,50500.0,49500.0,50000.0,100.0")?; // 一行有效数据

    let portfolio_config = create_test_portfolio_config(10000.0);
    let result = run_backtest(&file_path.to_string_lossy(), "ma-crossover", 2, 3, &portfolio_config, None).await;

    // 应该能处理部分无效数据，只要有一些有效数据
    assert!(result.is_ok());

    Ok(())
}

/// 测试不同策略参数的回测
#[tokio::test]
async fn test_different_strategy_parameters() -> Result<()> {
    let (csv_file, _temp_dir) = create_test_csv_file()?;
    let portfolio_config = create_test_portfolio_config(10000.0);

    let test_cases = vec![(5, 10), (10, 20), (2, 5)];

    for (short, long) in test_cases {
        let result = run_backtest(&csv_file, "ma-crossover", short, long, &portfolio_config, None).await;
        assert!(result.is_ok(), "策略参数 {}:{} 回测失败", short, long);
    }

    Ok(())
}

/// 测试不同初始资金的回测
#[tokio::test]
async fn test_different_initial_cash() -> Result<()> {
    let (csv_file, _temp_dir) = create_test_csv_file()?;

    let cash_amounts = vec![1000.0, 10000.0, 100000.0];

    for cash in cash_amounts {
        let portfolio_config = create_test_portfolio_config(cash);
        let result = run_backtest(&csv_file, "ma-crossover", 5, 10, &portfolio_config, None).await;
        assert!(result.is_ok(), "初始资金 {} 回测失败", cash);
    }

    Ok(())
}

/// 测试大数据集回测性能
#[tokio::test]
async fn test_large_dataset_performance() -> Result<()> {
    // 创建大数据集
    let dir = tempdir()?;
    let file_path = dir.path().join("large_data.csv");
    let mut file = File::create(&file_path)?;

    writeln!(file, "timestamp,open,high,low,close,volume")?;

    let start_time = std::time::Instant::now();

    // 生成1000条K线数据
    for i in 0..1000 {
        let timestamp = 1640995200000i64 + (i as i64 * 60000); // 每分钟一条
        let base_price = 50000.0 + (i as f64 * 10.0); // 缓慢上涨
        let open = base_price;
        let high = base_price + 100.0;
        let low = base_price - 100.0;
        let close = base_price + 50.0;
        let volume = 100.0 + (i as f64);

        writeln!(
            file,
            "{},{},{},{},{},{}",
            timestamp, open, high, low, close, volume
        )?;
    }

    let generation_time = start_time.elapsed();
    println!("生成1000条K线数据耗时: {:?}", generation_time);

    // 运行回测
    let backtest_start = std::time::Instant::now();
    let portfolio_config = create_test_portfolio_config(100000.0);
    let result = run_backtest(
        &file_path.to_string_lossy(),
        "ma-crossover",
        10,
        30,
        &portfolio_config,
        None,
    )
    .await;

    let backtest_time = backtest_start.elapsed();
    println!("回测1000条K线数据耗时: {:?}", backtest_time);

    assert!(result.is_ok());

    // 性能要求：1000条数据的回测应该在5秒内完成
    assert!(backtest_time.as_secs() < 5);

    Ok(())
}

/// 测试回测结果的正确性
#[tokio::test]
async fn test_backtest_result_correctness() -> Result<()> {
    let strategy = MACrossoverStrategy::new(2, 3);
    let portfolio_config = create_test_portfolio_config(10000.0);
    let mut engine = BacktestEngine::new(strategy, &portfolio_config)?;

    // 创建明确的趋势数据，应该产生交易信号
    let klines = vec![
        // 下跌趋势
        Kline {
            timestamp: 1000,
            open: 100.0,
            high: 101.0,
            low: 99.0,
            close: 100.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 2000,
            open: 100.0,
            high: 100.5,
            low: 98.0,
            close: 99.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 3000,
            open: 99.0,
            high: 99.5,
            low: 97.0,
            close: 98.0,
            volume: 100.0,
        },
        // 上涨趋势，应该触发买入信号
        Kline {
            timestamp: 4000,
            open: 98.0,
            high: 100.0,
            low: 97.5,
            close: 99.5,
            volume: 100.0,
        },
        Kline {
            timestamp: 5000,
            open: 99.5,
            high: 102.0,
            low: 99.0,
            close: 101.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 6000,
            open: 101.0,
            high: 104.0,
            low: 100.5,
            close: 103.0,
            volume: 100.0,
        },
        // 下跌趋势，应该触发卖出信号
        Kline {
            timestamp: 7000,
            open: 103.0,
            high: 103.5,
            low: 101.0,
            close: 102.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 8000,
            open: 102.0,
            high: 102.5,
            low: 100.0,
            close: 101.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 9000,
            open: 101.0,
            high: 101.5,
            low: 99.0,
            close: 100.0,
            volume: 100.0,
        },
    ];

    let result = engine.run(&klines, None).await;
    assert!(result.is_ok());

    // 验证有交易发生
    let trades = engine.portfolio().get_trades();
    assert!(trades.len() > 0, "应该有交易发生");

    // 验证权益曲线
    let equity_curve = engine.portfolio().get_equity_curve();
    assert_eq!(
        equity_curve.len(),
        klines.len(),
        "权益曲线应该有每个K线对应的点"
    );

    Ok(())
}

/// 测试回测引擎的状态管理
#[tokio::test]
async fn test_backtest_engine_state_management() -> Result<()> {
    let strategy = MACrossoverStrategy::new(5, 10);
    let portfolio_config = create_test_portfolio_config(50000.0);
    let engine = BacktestEngine::new(strategy, &portfolio_config)?;

    // 验证初始状态
    assert_eq!(engine.portfolio().get_cash(), 50000.0);
    assert_eq!(engine.portfolio().get_position(), 0.0);
    assert_eq!(engine.portfolio().get_trades().len(), 0);
    assert_eq!(engine.portfolio().get_equity_curve().len(), 0);

    Ok(())
}

/// 测试并发回测（如果需要）
#[tokio::test]
async fn test_concurrent_backtests() -> Result<()> {
    let (csv_file, _temp_dir) = create_test_csv_file()?;

    // 启动多个并发回测任务
    let mut tasks = Vec::new();

    for i in 0..3 {
        let file_clone = csv_file.clone();
        let task = tokio::spawn(async move {
            let portfolio_config = create_test_portfolio_config(10000.0 + (i as f64 * 1000.0));
            run_backtest(
                &file_clone,
                "ma-crossover",
                5,
                10,
                &portfolio_config,
                None,
            )
            .await
        });
        tasks.push(task);
    }

    // 等待所有任务完成
    for task in tasks {
        let result = task.await?;
        assert!(result.is_ok());
    }

    Ok(())
}
