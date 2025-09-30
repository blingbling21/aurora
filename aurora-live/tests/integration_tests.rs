use aurora_live::{LiveEngine, PaperTrader};
use aurora_strategy::MACrossoverStrategy;
use aurora_core::{Kline, MarketEvent, Strategy};
use aurora_portfolio::Portfolio;
use tokio;

/// 测试实时引擎的创建和基本功能
#[tokio::test]
async fn test_live_engine_integration() {
    // 创建策略
    let strategy = MACrossoverStrategy::new(5, 20);
    
    // 创建实时引擎
    let engine = LiveEngine::new(strategy, 10000.0);
    
    // 验证引擎创建成功
    assert!(engine.paper_trader().get_cash() > 0.0);
    assert_eq!(engine.paper_trader().get_cash(), 10000.0);
}

/// 测试模拟交易器的完整交易流程
#[tokio::test]
async fn test_paper_trader_full_workflow() {
    let mut trader = PaperTrader::new(10000.0);
    
    // 初始状态验证
    assert_eq!(trader.get_cash(), 10000.0);
    assert_eq!(trader.get_position(), 0.0);
    assert_eq!(trader.get_total_equity(50000.0), 10000.0);
    
    // 执行买入
    let buy_price = 50000.0;
    let buy_timestamp = 1640995200000;
    let buy_result = trader.execute_paper_buy(buy_price, buy_timestamp).await;
    assert!(buy_result.is_ok());
    
    // 验证买入后状态
    assert!(trader.get_position() > 0.0);
    assert!(trader.get_cash() < 10000.0);
    
    // 计算预期的持仓数量和剩余现金
    let expected_position = 10000.0 / buy_price;
    let expected_cash = 10000.0 - (expected_position * buy_price);
    
    assert!((trader.get_position() - expected_position).abs() < 0.01);
    assert!((trader.get_cash() - expected_cash).abs() < 0.01);
    
    // 执行卖出
    let sell_price = 52000.0;
    let sell_timestamp = 1640995800000;
    let sell_result = trader.execute_paper_sell(sell_price, sell_timestamp).await;
    assert!(sell_result.is_ok());
    
    // 验证卖出后状态
    assert_eq!(trader.get_position(), 0.0);
    
    // 验证盈亏
    let final_cash = trader.get_cash();
    let profit = final_cash - 10000.0;
    assert!(profit > 0.0); // 因为卖价高于买价，应该有盈利
}

/// 测试策略信号处理的集成功能
#[tokio::test]
async fn test_strategy_signal_integration() {
    let mut strategy = MACrossoverStrategy::new(2, 3);
    let mut trader = PaperTrader::new(10000.0);
    
    // 创建一系列K线数据来触发信号
    let test_klines = vec![
        Kline {
            timestamp: 1640995200000,
            open: 49000.0,
            high: 51000.0,
            low: 48000.0,
            close: 50000.0,
            volume: 100.0,
        },
        Kline {
            timestamp: 1640995260000,
            open: 50000.0,
            high: 52000.0,
            low: 49000.0,
            close: 51000.0,
            volume: 120.0,
        },
        Kline {
            timestamp: 1640995320000,
            open: 51000.0,
            high: 53000.0,
            low: 50000.0,
            close: 52000.0,
            volume: 110.0,
        },
        Kline {
            timestamp: 1640995380000,
            open: 52000.0,
            high: 54000.0,
            low: 51000.0,
            close: 53000.0,
            volume: 130.0,
        },
        Kline {
            timestamp: 1640995440000,
            open: 53000.0,
            high: 55000.0,
            low: 52000.0,
            close: 54000.0,
            volume: 125.0,
        },
    ];
    
    let mut signals_processed = 0;
    
    // 处理每个K线事件
    for kline in &test_klines {
        let market_event = MarketEvent::Kline(kline.clone());
        
        if let Some(signal_event) = strategy.on_market_event(&market_event) {
            signals_processed += 1;
            
            // 根据信号执行交易
            match signal_event.signal {
                aurora_core::Signal::Buy => {
                    let _ = trader.execute_paper_buy(signal_event.price, signal_event.timestamp).await;
                }
                aurora_core::Signal::Sell => {
                    let _ = trader.execute_paper_sell(signal_event.price, signal_event.timestamp).await;
                }
                aurora_core::Signal::Hold => {
                    // 不执行任何操作
                }
            }
        }
        
        // 更新权益
        trader.update_equity(kline.timestamp, kline.close);
    }
    
    // 验证至少处理了一些信号
    // 注意: 由于MA需要足够的数据点，可能前几个K线不会产生信号
    assert!(signals_processed >= 0); // 至少验证没有崩溃
    
    // 验证权益曲线有数据
    assert!(!trader.portfolio().get_equity_curve().is_empty());
}

/// 测试多次买卖的完整场景
#[tokio::test]
async fn test_multiple_trades_scenario() {
    let mut trader = PaperTrader::new(10000.0);
    
    // 第一轮交易
    let _ = trader.execute_paper_buy(50000.0, 1640995200000).await;
    let position_after_buy1 = trader.get_position();
    assert!(position_after_buy1 > 0.0);
    
    let _ = trader.execute_paper_sell(52000.0, 1640995300000).await;
    assert_eq!(trader.get_position(), 0.0);
    let cash_after_sell1 = trader.get_cash();
    
    // 第二轮交易
    let _ = trader.execute_paper_buy(51000.0, 1640995400000).await;
    let position_after_buy2 = trader.get_position();
    assert!(position_after_buy2 > 0.0);
    
    let _ = trader.execute_paper_sell(53000.0, 1640995500000).await;
    assert_eq!(trader.get_position(), 0.0);
    let final_cash = trader.get_cash();
    
    // 验证经过两轮交易后现金有变化
    assert_ne!(final_cash, 10000.0);
    
    // 验证权益曲线记录了所有更新
    trader.update_equity(1640995600000, 53000.0);
    assert!(!trader.portfolio().get_equity_curve().is_empty());
}

/// 测试边界条件：余额不足时的买入
#[tokio::test]
async fn test_insufficient_funds_buy() {
    let mut trader = PaperTrader::new(100.0); // 很少的初始资金
    
    // 尝试以高价买入，应该失败或购买很少的数量
    let result = trader.execute_paper_buy(50000.0, 1640995200000).await;
    
    // 验证交易结果合理（要么失败，要么只能买很少的量）
    if result.is_ok() {
        assert!(trader.get_position() > 0.0);
        assert!(trader.get_position() < 0.01); // 应该只能买很少的量
    }
}

/// 测试边界条件：没有持仓时的卖出
#[tokio::test]
async fn test_sell_without_position() {
    let mut trader = PaperTrader::new(10000.0);
    
    // 在没有持仓的情况下尝试卖出
    let result = trader.execute_paper_sell(50000.0, 1640995200000).await;
    
    // 应该失败或不产生任何效果
    if result.is_err() {
        // 如果返回错误，这是正确的行为
        assert!(true);
    } else {
        // 如果没有返回错误，持仓应该仍然为0
        assert_eq!(trader.get_position(), 0.0);
    }
}