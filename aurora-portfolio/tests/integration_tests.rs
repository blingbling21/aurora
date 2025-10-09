//! 集成测试 - 测试整个投资组合管理流程

use aurora_portfolio::{
    BasePortfolio, PerformanceMetrics, Portfolio, PortfolioAnalytics, Trade, TradeSide,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// 测试完整的交易流程
#[tokio::test]
async fn test_complete_trading_flow() {
    let mut portfolio = BasePortfolio::new(10000.0);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    // 初始状态验证
    assert_eq!(portfolio.get_cash(), 10000.0);
    assert_eq!(portfolio.get_position(), 0.0);
    assert_eq!(portfolio.get_total_equity(100.0), 10000.0);
    assert_eq!(portfolio.get_trades().len(), 0);

    // 执行买入
    let buy_trade = portfolio.execute_buy(100.0, timestamp).await.unwrap();
    assert_eq!(buy_trade.side, TradeSide::Buy);
    assert_eq!(buy_trade.price, 100.0);
    assert_eq!(buy_trade.quantity, 100.0); // 10000 / 100
    assert_eq!(buy_trade.value, 10000.0);

    // 买入后状态验证
    assert_eq!(portfolio.get_cash(), 0.0);
    assert_eq!(portfolio.get_position(), 100.0);
    assert_eq!(portfolio.get_total_equity(100.0), 10000.0);
    assert_eq!(portfolio.get_trades().len(), 1);

    // 价格上涨，更新权益
    portfolio.update_equity(timestamp + 60000, 110.0);
    assert_eq!(portfolio.get_total_equity(110.0), 11000.0);

    // 执行卖出
    let sell_trade = portfolio
        .execute_sell(110.0, timestamp + 120000)
        .await
        .unwrap();
    assert_eq!(sell_trade.side, TradeSide::Sell);
    assert_eq!(sell_trade.price, 110.0);
    assert_eq!(sell_trade.quantity, 100.0);
    assert_eq!(sell_trade.value, 11000.0);

    // 卖出后状态验证
    assert_eq!(portfolio.get_cash(), 11000.0);
    assert_eq!(portfolio.get_position(), 0.0);
    assert_eq!(portfolio.get_total_equity(110.0), 11000.0);
    assert_eq!(portfolio.get_trades().len(), 2);

    // 权益曲线验证
    let equity_curve = portfolio.get_equity_curve();
    assert_eq!(equity_curve.len(), 1);
    assert_eq!(equity_curve[0].equity, 11000.0);
}

/// 测试错误场景
#[tokio::test]
async fn test_error_scenarios() {
    let mut portfolio = BasePortfolio::new(100.0);
    let timestamp = 1640995200000;

    // 无持仓时卖出应该失败
    let result = portfolio.execute_sell(100.0, timestamp).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("无持仓"));

    // 执行买入
    portfolio.execute_buy(100.0, timestamp).await.unwrap();

    // 现金不足时再次买入应该失败
    let result = portfolio.execute_buy(100.0, timestamp + 1000).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("现金不足"));

    // 无效价格
    let result = portfolio.execute_sell(-100.0, timestamp).await;
    assert!(result.is_err());

    // 无效时间戳
    let result = portfolio.execute_sell(100.0, -1).await;
    assert!(result.is_err());
}

/// 测试业绩计算
#[tokio::test]
async fn test_performance_calculation() {
    let mut portfolio = BasePortfolio::new(10000.0);
    let start_time = 1640995200000; // 2022-01-01

    // 执行一系列交易
    portfolio.execute_buy(100.0, start_time).await.unwrap();
    portfolio.update_equity(start_time + 3600000, 105.0); // 1小时后
    portfolio
        .execute_sell(105.0, start_time + 7200000)
        .await
        .unwrap(); // 2小时后

    portfolio
        .execute_buy(105.0, start_time + 10800000)
        .await
        .unwrap(); // 3小时后
    portfolio.update_equity(start_time + 14400000, 110.0); // 4小时后
    portfolio
        .execute_sell(110.0, start_time + 18000000)
        .await
        .unwrap(); // 5小时后

    // 计算业绩
    let metrics = portfolio.calculate_performance(1.0); // 1天

    assert!(metrics.total_return > 0.0);
    assert_eq!(metrics.total_trades, 2);
    assert_eq!(metrics.winning_trades, 2);
    assert_eq!(metrics.losing_trades, 0);
    assert_eq!(metrics.win_rate, 100.0);
}

/// 测试回撤计算
#[tokio::test]
async fn test_drawdown_calculation() {
    let mut portfolio = BasePortfolio::new(10000.0);
    let timestamp = 1640995200000;

    // 买入
    portfolio.execute_buy(100.0, timestamp).await.unwrap();

    // 模拟价格波动
    portfolio.update_equity(timestamp + 1000, 110.0); // 上涨10%
    portfolio.update_equity(timestamp + 2000, 95.0); // 下跌到95，产生回撤
    portfolio.update_equity(timestamp + 3000, 120.0); // 恢复并创新高

    let equity_curve = portfolio.get_equity_curve();
    assert_eq!(equity_curve.len(), 3);

    // 验证回撤计算
    let max_drawdown = PortfolioAnalytics::calculate_metrics(
        10000.0,
        12000.0,
        equity_curve,
        portfolio.get_trades(),
        1.0,
    )
    .max_drawdown;

    assert!(max_drawdown > 0.0); // 应该有回撤
}

/// 测试并发安全性
#[tokio::test]
async fn test_concurrent_operations() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let portfolio = Arc::new(Mutex::new(BasePortfolio::new(10000.0)));
    let timestamp = 1640995200000;

    // 启动多个并发任务
    let portfolio1 = Arc::clone(&portfolio);
    let task1 = tokio::spawn(async move {
        let mut p = portfolio1.lock().await;
        p.execute_buy(100.0, timestamp).await
    });

    let portfolio2 = Arc::clone(&portfolio);
    let task2 = tokio::spawn(async move {
        let mut p = portfolio2.lock().await;
        p.update_equity(timestamp + 1000, 105.0);
        Ok(())
    });

    // 等待所有任务完成
    let result1 = task1.await.unwrap();
    let _result2: Result<(), anyhow::Error> = task2.await.unwrap();

    // 验证第一个任务成功，第二个任务正常执行
    assert!(result1.is_ok());

    let final_portfolio = portfolio.lock().await;
    assert_eq!(final_portfolio.get_position(), 100.0);
}

/// 测试极端市场条件
#[tokio::test]
async fn test_extreme_market_conditions() {
    let mut portfolio = BasePortfolio::new(10000.0);
    let timestamp = 1640995200000;

    // 买入
    portfolio.execute_buy(100.0, timestamp).await.unwrap();

    // 极端价格变化
    portfolio.update_equity(timestamp + 1000, 0.01); // 价格暴跌99.99%
    let extreme_equity = portfolio.get_total_equity(0.01);
    assert!(extreme_equity < 10.0); // 极小的权益值

    portfolio.update_equity(timestamp + 2000, 1000000.0); // 价格暴涨
    let extreme_high = portfolio.get_total_equity(1000000.0);
    assert!(extreme_high > 10000000.0); // 极大的权益值

    // 验证权益曲线正确记录了这些极端变化
    let equity_curve = portfolio.get_equity_curve();
    assert_eq!(equity_curve.len(), 2);
}

/// 测试大量交易的性能
#[tokio::test]
async fn test_high_frequency_trading_performance() {
    let mut portfolio = BasePortfolio::new(1000000.0); // 更大的初始资金
    let start_time = std::time::Instant::now();
    let timestamp = 1640995200000;

    // 执行大量买卖操作
    for i in 0..100 {
        let price = 100.0 + (i as f64);

        if i % 2 == 0 {
            // 买入
            if portfolio.get_cash() > price {
                let _ = portfolio
                    .execute_buy(price, timestamp + (i as i64) * 1000)
                    .await;
            }
        } else {
            // 卖出
            if portfolio.get_position() > 0.0 {
                let _ = portfolio
                    .execute_sell(price, timestamp + (i as i64) * 1000)
                    .await;
            }
        }

        // 更新权益
        portfolio.update_equity(timestamp + (i as i64) * 1000, price);
    }

    let duration = start_time.elapsed();
    println!("100次交易操作耗时: {:?}", duration);

    // 验证性能要求（100次操作应该在1秒内完成）
    assert!(duration.as_secs() < 1);

    // 验证交易记录
    assert!(portfolio.get_trades().len() > 0);

    // 验证权益曲线
    assert_eq!(portfolio.get_equity_curve().len(), 100);
}
