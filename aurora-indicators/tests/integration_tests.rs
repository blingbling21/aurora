//! 技术指标模块集成测试

use aurora_indicators::MA;

/// 测试移动平均线基本功能
#[test]
fn test_ma_basic_functionality() {
    let mut ma = MA::new(5);

    // 验证初始参数
    assert_eq!(ma.period(), 5);
    assert_eq!(ma.value(), None);

    // 前4个数据点不应该产生结果
    assert_eq!(ma.update(100.0), None);
    assert_eq!(ma.update(102.0), None);
    assert_eq!(ma.update(98.0), None);
    assert_eq!(ma.update(105.0), None);

    // 第5个数据点应该产生移动平均值
    let avg = ma.update(95.0).unwrap();
    let expected = (100.0 + 102.0 + 98.0 + 105.0 + 95.0) / 5.0;
    assert_eq!(avg, expected);
    assert_eq!(ma.value(), Some(expected));
}

/// 测试滑动窗口功能
#[test]
fn test_ma_sliding_window() {
    let mut ma = MA::new(3);

    // 填充初始数据
    ma.update(10.0);
    ma.update(20.0);
    let first_avg = ma.update(30.0).unwrap();
    assert_eq!(first_avg, 20.0); // (10+20+30)/3

    // 添加新数据，应该滑动窗口
    let second_avg = ma.update(40.0).unwrap();
    assert_eq!(second_avg, 30.0); // (20+30+40)/3

    let third_avg = ma.update(50.0).unwrap();
    assert_eq!(third_avg, 40.0); // (30+40+50)/3
}

/// 测试不同周期的移动平均线
#[test]
fn test_different_periods() {
    let test_cases = vec![1, 2, 5, 10, 20, 50, 100];
    let test_data = (1..=100).map(|i| i as f64).collect::<Vec<_>>();

    for period in test_cases {
        let mut ma = MA::new(period);

        for (i, &value) in test_data.iter().enumerate() {
            let result = ma.update(value);

            if i + 1 >= period {
                // 应该有结果
                assert!(result.is_some());
                let avg = result.unwrap();

                // 验证平均值计算正确
                let start_idx = i + 1 - period;
                let expected: f64 = test_data[start_idx..=i].iter().sum::<f64>() / period as f64;
                assert!((avg - expected).abs() < 1e-10);
            } else {
                // 不应该有结果
                assert!(result.is_none());
            }
        }
    }
}

/// 测试极端数值
#[test]
fn test_extreme_values() {
    let mut ma = MA::new(3);

    let extreme_values = vec![
        0.0,
        -1000.0,
        1000000.0,
        f64::MIN / 1e10,
        f64::MAX / 1e10,
        1e-10,
        -1e-10,
    ];

    for value in extreme_values {
        ma.update(value);
        // 应该能处理极端值而不崩溃
        if let Some(avg) = ma.value() {
            assert!(!avg.is_nan());
            assert!(!avg.is_infinite());
        }
    }
}

/// 测试零和负数
#[test]
fn test_zero_and_negative_values() {
    let mut ma = MA::new(4);

    let values = vec![-10.0, 0.0, 10.0, -5.0];
    let mut results = Vec::new();

    for value in values {
        if let Some(avg) = ma.update(value) {
            results.push(avg);
        }
    }

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], -1.25); // (-10+0+10-5)/4 = -1.25
}

/// 测试大量数据的性能
#[test]
fn test_performance_with_large_dataset() {
    let mut ma = MA::new(50);
    let start_time = std::time::Instant::now();

    // 处理大量数据
    for i in 0..100000 {
        ma.update(i as f64);
    }

    let duration = start_time.elapsed();
    println!("处理100000个数据点耗时: {:?}", duration);

    // 性能要求：100000个数据点应该在100ms内处理完成
    assert!(duration.as_millis() < 100);
}

/// 测试内存使用效率
#[test]
fn test_memory_efficiency() {
    // 创建不同周期的MA，验证内存使用
    let periods = vec![10, 100, 1000];

    for period in periods {
        let mut ma = MA::new(period);

        // 输入大量数据
        for i in 0..period * 10 {
            ma.update(i as f64);
        }

        // MA应该只保存period个数据点，而不是所有历史数据
        // 这是通过内部实现验证的，实际测试中我们验证功能正确性
        let final_value = ma.value().unwrap();

        // 验证最终值是正确的
        let expected_start = (period * 10 - period) as f64;
        let expected_end = (period * 10 - 1) as f64;
        let expected = (expected_start + expected_end) * period as f64 / (2.0 * period as f64);

        // 由于是连续数字的平均，可以用算术级数公式验证
        let actual_expected = (expected_start + expected_end) / 2.0;
        assert!((final_value - actual_expected).abs() < 1e-10);
    }
}

/// 测试边界条件
#[test]
fn test_boundary_conditions() {
    // 测试周期为1的情况
    let mut ma1 = MA::new(1);
    assert_eq!(ma1.update(42.0), Some(42.0));
    assert_eq!(ma1.update(84.0), Some(84.0));

    // 测试重复值
    let mut ma3 = MA::new(3);
    ma3.update(100.0);
    ma3.update(100.0);
    let result = ma3.update(100.0);
    assert_eq!(result, Some(100.0));
}

/// 测试MA的重置功能
#[test]
fn test_ma_reset() {
    let mut ma = MA::new(3);

    // 添加一些数据
    ma.update(10.0);
    ma.update(20.0);
    ma.update(30.0);
    assert!(ma.value().is_some());

    // 重置
    ma.reset();
    assert_eq!(ma.value(), None);

    // 重置后应该重新开始计算
    ma.update(100.0);
    ma.update(200.0);
    assert_eq!(ma.value(), None); // 还没有足够的数据

    let result = ma.update(300.0);
    assert_eq!(result, Some(200.0)); // (100+200+300)/3
}

/// 测试MA的克隆
#[test]
fn test_ma_clone() {
    let mut ma1 = MA::new(3);
    ma1.update(10.0);
    ma1.update(20.0);

    let mut ma2 = ma1.clone();

    // 两个MA应该产生相同的结果
    let result1 = ma1.update(30.0);
    let result2 = ma2.update(30.0);

    assert_eq!(result1, result2);
    assert_eq!(result1, Some(20.0));
}

/// 测试MA的调试输出
#[test]
fn test_ma_debug() {
    let ma = MA::new(5);
    let debug_str = format!("{:?}", ma);

    // 调试输出应该包含周期信息
    assert!(debug_str.contains("MA"));
    assert!(debug_str.contains("5"));
}

/// 测试无效周期参数
#[test]
#[should_panic(expected = "周期必须大于0")]
fn test_invalid_period_zero() {
    MA::new(0);
}

/// 测试浮点数精度问题
#[test]
fn test_floating_point_precision() {
    let mut ma = MA::new(3);

    // 使用可能导致精度问题的数值
    let values = vec![0.1, 0.2, 0.3];
    let mut result = None;

    for value in values {
        result = ma.update(value);
    }

    let avg = result.unwrap();
    let expected = 0.2;

    // 考虑浮点数精度误差
    assert!((avg - expected).abs() < 1e-15);
}

/// 测试NaN和无穷大处理
#[test]
fn test_nan_and_infinity_handling() {
    let mut ma = MA::new(3);

    // 正常值
    ma.update(10.0);
    ma.update(20.0);

    // NaN值应该被正确处理或者拒绝
    let result_with_nan = ma.update(f64::NAN);
    if let Some(avg) = result_with_nan {
        // 如果接受NaN，结果应该是NaN
        assert!(avg.is_nan());
    }

    // 重新创建MA测试无穷大
    let mut ma_inf = MA::new(2);
    ma_inf.update(10.0);
    let result_with_inf = ma_inf.update(f64::INFINITY);

    if let Some(avg) = result_with_inf {
        // 如果接受无穷大，结果应该是无穷大
        assert!(avg.is_infinite());
    }
}
