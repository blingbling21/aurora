//! 集成测试 - 测试整个投资组合管理流程

use aurora_portfolio::{
    BasePortfolio, Portfolio, PortfolioAnalytics, TradeSide,
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

/// 测试风险管理功能
#[tokio::test]
async fn test_risk_management() {
    use aurora_portfolio::{RiskManager, RiskRules};

    // 创建投资组合
    let _portfolio = BasePortfolio::new(10000.0);
    
    // 配置风险规则
    let rules = RiskRules::new()
        .with_max_drawdown(15.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(5000.0);
    
    let mut risk_manager = RiskManager::new(rules, 10000.0);
    
    // 测试正常情况 - 风险检查应通过
    let result = risk_manager.check_risk(9500.0, 5.0, 100.0);
    assert!(result.is_pass());
    
    // 模拟连续亏损
    risk_manager.record_trade_result(false);
    risk_manager.record_trade_result(false);
    risk_manager.record_trade_result(false);
    
    // 应触发连续亏损限制
    let result = risk_manager.check_risk(9000.0, 10.0, 95.0);
    assert!(!result.is_pass());
    assert!(risk_manager.should_stop_trading());
    
    // 测试止损触发
    let rules2 = RiskRules::new().with_stop_loss_price(95.0);
    let mut risk_manager2 = RiskManager::new(rules2, 10000.0);
    
    let result = risk_manager2.check_risk(10000.0, 0.0, 94.0);
    assert!(!result.is_pass());
    
    // 测试止盈触发
    let rules3 = RiskRules::new().with_take_profit_price(110.0);
    let mut risk_manager3 = RiskManager::new(rules3, 10000.0);
    
    let result = risk_manager3.check_risk(11000.0, 0.0, 112.0);
    assert!(!result.is_pass());
}

/// 测试仓位管理功能
#[test]
fn test_position_sizing() {
    use aurora_portfolio::{PositionManager, PositionSizingStrategy};
    
    // 测试固定金额策略
    let manager1 = PositionManager::new(PositionSizingStrategy::FixedAmount(1000.0));
    let size = manager1.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 1000.0);
    
    // 测试固定比例策略
    let manager2 = PositionManager::new(PositionSizingStrategy::FixedPercentage(0.2));
    let size = manager2.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 2000.0);
    
    // 测试Kelly准则
    let manager3 = PositionManager::new(PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,
        profit_loss_ratio: 2.0,
        kelly_fraction: 0.5,
    });
    let size = manager3.calculate_position_size(10000.0, 0.0).unwrap();
    assert!((size - 2000.0).abs() < 0.01); // Kelly = 40%, 半凯利 = 20%
    
    // 测试金字塔策略
    let manager4 = PositionManager::new(PositionSizingStrategy::Pyramid {
        initial_percentage: 0.1,
        profit_threshold: 5.0,
        max_percentage: 0.5,
        increment: 0.1,
    });
    
    // 无盈利
    let size1 = manager4.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size1, 1000.0);
    
    // 盈利6%, 触发一次加仓
    let size2 = manager4.calculate_position_size(10000.0, 6.0).unwrap();
    assert_eq!(size2, 2000.0);
}

/// 测试订单管理功能
#[test]
fn test_order_management() {
    use aurora_portfolio::{Order, OrderType, OrderSide, OrderStatus};
    
    // 创建市价单
    let mut market_order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );
    
    assert!(market_order.should_trigger(100.0));
    assert!(market_order.is_pending());
    
    market_order.trigger();
    assert_eq!(market_order.status, OrderStatus::Triggered);
    
    market_order.execute(100.5, 1640995260000);
    assert!(market_order.is_executed());
    assert_eq!(market_order.executed_price, Some(100.5));
    
    // 创建止损单
    let stop_loss_order = Order::new(
        OrderType::StopLoss(95.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );
    
    assert!(!stop_loss_order.should_trigger(100.0)); // 未触发
    assert!(stop_loss_order.should_trigger(94.0));   // 触发
    
    // 创建止盈单
    let take_profit_order = Order::new(
        OrderType::TakeProfit(110.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );
    
    assert!(!take_profit_order.should_trigger(105.0)); // 未触发
    assert!(take_profit_order.should_trigger(112.0));  // 触发
}

/// 完整场景测试: 集成风控和仓位管理
#[tokio::test]
async fn test_complete_risk_management_flow() {
    use aurora_portfolio::{
        RiskManager, RiskRules, PositionManager, PositionSizingStrategy,
    };
    
    // 1. 创建投资组合
    let mut portfolio = BasePortfolio::new(10000.0);
    
    // 2. 配置风险规则
    let risk_rules = RiskRules::new()
        .with_max_drawdown(15.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(5000.0);
    
    let mut risk_manager = RiskManager::new(risk_rules, 10000.0);
    
    // 3. 配置仓位管理 (使用20%固定比例)
    let position_manager = PositionManager::new(
        PositionSizingStrategy::FixedPercentage(0.2)
    );
    
    let current_price = 100.0;
    let timestamp = 1640995200000;
    
    // 4. 执行风险检查
    let current_equity = portfolio.get_total_equity(current_price);
    let risk_result = risk_manager.check_risk(current_equity, 0.0, current_price);
    assert!(risk_result.is_pass());
    
    // 5. 计算仓位大小
    let position_size = position_manager
        .calculate_position_size(current_equity, 0.0)
        .unwrap();
    assert_eq!(position_size, 2000.0); // 20% of 10000
    
    // 6. 设置止损止盈
    risk_manager.set_stop_loss_take_profit(current_price, 2.0, 5.0);
    assert_eq!(risk_manager.get_rules().stop_loss_price, Some(98.0));
    assert_eq!(risk_manager.get_rules().take_profit_price, Some(105.0));
    
    // 7. 执行买入 (这里简化为全仓买入)
    let trade = portfolio.execute_buy(current_price, timestamp).await.unwrap();
    assert!(trade.value > 0.0);
    
    // 8. 模拟价格上涨触发止盈
    portfolio.update_equity(timestamp + 60000, 106.0);
    let result = risk_manager.check_risk(
        portfolio.get_total_equity(106.0),
        0.0,
        106.0,
    );
    assert!(!result.is_pass()); // 应触发止盈
    
    // 9. 执行卖出
    let sell_trade = portfolio.execute_sell(106.0, timestamp + 60000).await.unwrap();
    assert_eq!(sell_trade.price, 106.0);
    
    // 10. 记录盈利交易
    risk_manager.record_trade_result(true);
    assert_eq!(risk_manager.get_consecutive_wins(), 1);
    assert_eq!(risk_manager.get_consecutive_losses(), 0);
    
    // 验证最终权益增加
    let final_equity = portfolio.get_total_equity(106.0);
    assert!(final_equity > 10000.0);
}
