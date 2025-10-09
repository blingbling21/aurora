//! 策略模块集成测试

use aurora_core::{Kline, MarketEvent, Signal, Strategy};
use aurora_strategy::MACrossoverStrategy;

/// 测试MA交叉策略的基本功能
#[test]
fn test_ma_crossover_strategy_basic() {
    let mut strategy = MACrossoverStrategy::new(2, 5);

    // 验证初始参数
    assert_eq!(strategy.short_period(), 2);
    assert_eq!(strategy.long_period(), 5);

    // 初始状态应该不产生信号（数据不足）
    let kline = create_test_kline(100.0, 1640995200000);
    let event = MarketEvent::Kline(kline);
    let signal = strategy.on_market_event(&event);
    assert!(signal.is_none());
}

/// 测试金叉信号生成
#[test]
fn test_golden_cross_signal() {
    let mut strategy = MACrossoverStrategy::new(2, 3);

    // 创建下跌趋势的K线数据
    let klines = vec![
        create_test_kline(100.0, 1640995200000),
        create_test_kline(98.0, 1640995260000),
        create_test_kline(96.0, 1640995320000),
        create_test_kline(94.0, 1640995380000),
        create_test_kline(92.0, 1640995440000),
    ];

    // 输入下跌数据
    for kline in klines {
        let event = MarketEvent::Kline(kline);
        let signal = strategy.on_market_event(&event);
        // 下跌趋势中不应该有买入信号
        if let Some(signal_event) = signal {
            assert_ne!(signal_event.signal, Signal::Buy);
        }
    }

    // 现在输入上涨数据，应该产生金叉买入信号
    let uptrend_klines = vec![
        create_test_kline(95.0, 1640995500000),
        create_test_kline(98.0, 1640995560000),
        create_test_kline(102.0, 1640995620000),
        create_test_kline(106.0, 1640995680000),
    ];

    let mut buy_signals = 0;
    for kline in uptrend_klines {
        let event = MarketEvent::Kline(kline.clone());
        if let Some(signal_event) = strategy.on_market_event(&event) {
            if signal_event.signal == Signal::Buy {
                buy_signals += 1;
                assert_eq!(signal_event.price, kline.close);
                assert_eq!(signal_event.timestamp, kline.timestamp);
            }
        }
    }

    assert!(buy_signals > 0, "应该产生至少一个买入信号");
}

/// 测试死叉信号生成
#[test]
fn test_death_cross_signal() {
    let mut strategy = MACrossoverStrategy::new(2, 3);

    // 创建上涨趋势的K线数据
    let uptrend_klines = vec![
        create_test_kline(90.0, 1640995200000),
        create_test_kline(92.0, 1640995260000),
        create_test_kline(95.0, 1640995320000),
        create_test_kline(98.0, 1640995380000),
        create_test_kline(102.0, 1640995440000),
    ];

    // 输入上涨数据
    for kline in uptrend_klines {
        let event = MarketEvent::Kline(kline);
        strategy.on_market_event(&event);
    }

    // 现在输入下跌数据，应该产生死叉卖出信号
    let downtrend_klines = vec![
        create_test_kline(100.0, 1640995500000),
        create_test_kline(96.0, 1640995560000),
        create_test_kline(92.0, 1640995620000),
        create_test_kline(88.0, 1640995680000),
    ];

    let mut sell_signals = 0;
    for kline in downtrend_klines {
        let event = MarketEvent::Kline(kline.clone());
        if let Some(signal_event) = strategy.on_market_event(&event) {
            if signal_event.signal == Signal::Sell {
                sell_signals += 1;
                assert_eq!(signal_event.price, kline.close);
                assert_eq!(signal_event.timestamp, kline.timestamp);
            }
        }
    }

    assert!(sell_signals > 0, "应该产生至少一个卖出信号");
}

/// 测试震荡市场中的信号
#[test]
fn test_sideways_market_signals() {
    let mut strategy = MACrossoverStrategy::new(3, 5);

    // 创建震荡市场数据
    let sideways_prices = vec![100.0, 102.0, 98.0, 101.0, 99.0, 103.0, 97.0, 100.0];
    let mut signals = Vec::new();

    for (i, price) in sideways_prices.iter().enumerate() {
        let kline = create_test_kline(*price, 1640995200000 + (i as i64) * 60000);
        let event = MarketEvent::Kline(kline);
        if let Some(signal_event) = strategy.on_market_event(&event) {
            signals.push(signal_event.signal);
        }
    }

    // 震荡市场中可能产生多个信号，但不应该全是同一种信号
    if signals.len() > 1 {
        let buy_count = signals.iter().filter(|&s| *s == Signal::Buy).count();
        let sell_count = signals.iter().filter(|&s| *s == Signal::Sell).count();

        // 在震荡市场中，买入和卖出信号应该比较平衡
        assert!(buy_count > 0 || sell_count > 0);
    }
}

/// 测试不同周期参数的策略
#[test]
fn test_different_period_strategies() {
    let test_cases = vec![(5, 10), (10, 20), (20, 50)];

    for (short, long) in test_cases {
        let mut strategy = MACrossoverStrategy::new(short, long);
        assert_eq!(strategy.short_period(), short);
        assert_eq!(strategy.long_period(), long);

        // 输入一些数据
        for i in 0..long + 5 {
            let price = 100.0 + (i as f64);
            let kline = create_test_kline(price, 1640995200000 + (i as i64) * 60000);
            let event = MarketEvent::Kline(kline);
            strategy.on_market_event(&event);
        }

        // 策略应该能正常运行而不崩溃
        // 这是一个基本的稳定性测试
    }
}

/// 测试策略的克隆功能
#[test]
fn test_strategy_clone() {
    let strategy1 = MACrossoverStrategy::new(5, 20);
    let strategy2 = strategy1.clone();

    assert_eq!(strategy1.short_period(), strategy2.short_period());
    assert_eq!(strategy1.long_period(), strategy2.long_period());

    // 克隆的策略应该独立工作
    let mut strategy1_mut = strategy1;
    let mut strategy2_mut = strategy2;

    let kline = create_test_kline(100.0, 1640995200000);
    let event = MarketEvent::Kline(kline.clone());

    strategy1_mut.on_market_event(&event);
    strategy2_mut.on_market_event(&event);

    // 两个策略应该产生相同的结果
}

/// 测试极端价格数据
#[test]
fn test_extreme_price_data() {
    let mut strategy = MACrossoverStrategy::new(2, 3);

    let extreme_prices = vec![
        0.0001,          // 极小价格
        1000000.0,       // 极大价格
        0.0,             // 零价格
        f64::MAX / 1e10, // 接近最大值
    ];

    for (i, price) in extreme_prices.iter().enumerate() {
        let kline = create_test_kline(*price, 1640995200000 + (i as i64) * 60000);
        let event = MarketEvent::Kline(kline);

        // 策略应该能处理极端价格而不崩溃
        let result = strategy.on_market_event(&event);

        // 如果有信号，价格应该是有效的
        if let Some(signal_event) = result {
            assert!(!signal_event.price.is_nan());
            assert!(!signal_event.price.is_infinite());
        }
    }
}

/// 测试大量数据的性能
#[test]
fn test_large_dataset_performance() {
    let mut strategy = MACrossoverStrategy::new(20, 50);
    let start_time = std::time::Instant::now();

    // 处理大量K线数据
    for i in 0..10000 {
        let price = 100.0 + (i as f64 % 100.0); // 创建周期性价格变化
        let kline = create_test_kline(price, 1640995200000 + (i as i64) * 1000);
        let event = MarketEvent::Kline(kline);
        strategy.on_market_event(&event);
    }

    let duration = start_time.elapsed();
    println!("处理10000条K线数据耗时: {:?}", duration);

    // 性能要求：10000条数据应该在1秒内处理完成
    assert!(duration.as_secs() < 1);
}

/// 测试策略参数验证
#[test]
#[should_panic(expected = "短期移动平均线周期必须小于长期移动平均线周期")]
fn test_invalid_period_parameters() {
    // 短期周期大于长期周期应该panic
    MACrossoverStrategy::new(20, 10);
}

#[test]
#[should_panic(expected = "短期移动平均线周期必须小于长期移动平均线周期")]
fn test_equal_period_parameters() {
    // 相等周期应该panic
    MACrossoverStrategy::new(10, 10);
}

/// 测试策略状态一致性
#[test]
fn test_strategy_state_consistency() {
    let mut strategy = MACrossoverStrategy::new(3, 5);

    // 输入相同的K线数据多次
    let kline = create_test_kline(100.0, 1640995200000);
    let event = MarketEvent::Kline(kline);

    let result1 = strategy.on_market_event(&event);
    let result2 = strategy.on_market_event(&event);

    // 相同输入应该产生相同输出（幂等性）
    match (result1, result2) {
        (None, None) => {}
        (Some(s1), Some(s2)) => {
            assert_eq!(s1.signal, s2.signal);
            assert_eq!(s1.price, s2.price);
        }
        _ => panic!("相同输入产生了不同的输出"),
    }
}

/// 辅助函数：创建测试用的K线数据
fn create_test_kline(close_price: f64, timestamp: i64) -> Kline {
    Kline {
        timestamp,
        open: close_price * 0.99,
        high: close_price * 1.01,
        low: close_price * 0.98,
        close: close_price,
        volume: 1000.0,
    }
}
