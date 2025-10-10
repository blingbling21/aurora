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

use super::*;

#[test]
fn test_stddev_new() {
    let stddev = StdDev::new(20);
    assert_eq!(stddev.period, 20);
    assert_eq!(stddev.count(), 0);
    assert!(!stddev.is_ready());
}

#[test]
#[should_panic(expected = "标准差周期必须大于0")]
fn test_stddev_new_zero_period() {
    StdDev::new(0);
}

#[test]
fn test_stddev_insufficient_data() {
    let mut stddev = StdDev::new(20);

    // 前19个数据点应该返回None
    for i in 0..19 {
        assert_eq!(stddev.update(100.0), None);
        assert_eq!(stddev.count(), i + 1);
    }

    // 第20个数据点应该有结果
    assert!(stddev.update(100.0).is_some());
}

#[test]
fn test_stddev_constant_values() {
    let mut stddev = StdDev::new(10);

    // 所有价格相同时,标准差应该为0
    for _ in 0..10 {
        stddev.update(100.0);
    }

    let result = stddev.update(100.0);
    assert!(result.is_some());
    assert!((result.unwrap() - 0.0).abs() < 1e-10);
}

#[test]
fn test_stddev_simple_case() {
    let mut stddev = StdDev::new(5);

    // 价格序列: 2, 4, 4, 4, 5, 5, 7, 9
    // 取最后5个: 4, 5, 5, 7, 9
    // 平均值: (4 + 5 + 5 + 7 + 9) / 5 = 6
    // 方差: ((4-6)² + (5-6)² + (5-6)² + (7-6)² + (9-6)²) / 5
    //     = (4 + 1 + 1 + 1 + 9) / 5 = 16 / 5 = 3.2
    // 标准差: √3.2 ≈ 1.7889
    let prices = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut result = None;

    for price in prices {
        result = stddev.update(price);
    }

    assert!(result.is_some());
    let std_value = result.unwrap();
    assert!((std_value - 1.7889).abs() < 0.001);
}

#[test]
fn test_stddev_mean() {
    let mut stddev = StdDev::new(5);

    // 添加数据
    let prices = vec![100.0, 102.0, 98.0, 101.0, 99.0];
    for price in prices {
        stddev.update(price);
    }

    // 平均值应该是 (100+102+98+101+99)/5 = 100
    let mean = stddev.mean();
    assert!(mean.is_some());
    assert!((mean.unwrap() - 100.0).abs() < 1e-10);
}

#[test]
fn test_stddev_high_volatility() {
    let mut stddev = StdDev::new(5);

    // 高波动性数据
    let prices = vec![100.0, 120.0, 80.0, 130.0, 70.0];
    for price in prices {
        stddev.update(price);
    }

    let result = stddev.update(90.0);
    assert!(result.is_some());

    // 高波动性应该产生较大的标准差
    let std_value = result.unwrap();
    assert!(std_value > 15.0, "高波动性标准差应该 > 15,实际值: {}", std_value);
}

#[test]
fn test_stddev_low_volatility() {
    let mut stddev = StdDev::new(5);

    // 低波动性数据
    let prices = vec![100.0, 101.0, 99.0, 100.5, 99.5];
    for price in prices {
        stddev.update(price);
    }

    let result = stddev.update(100.0);
    assert!(result.is_some());

    // 低波动性应该产生较小的标准差
    let std_value = result.unwrap();
    assert!(std_value < 1.0, "低波动性标准差应该 < 1,实际值: {}", std_value);
}

#[test]
fn test_stddev_reset() {
    let mut stddev = StdDev::new(10);

    for _ in 0..10 {
        stddev.update(100.0);
    }

    assert!(stddev.is_ready());
    assert_eq!(stddev.count(), 10);

    stddev.reset();

    assert!(!stddev.is_ready());
    assert_eq!(stddev.count(), 0);
    assert_eq!(stddev.sum, 0.0);
    assert_eq!(stddev.mean(), None);
}

#[test]
fn test_stddev_sliding_window() {
    let mut stddev = StdDev::new(5);

    // 填满窗口
    for i in 0..5 {
        stddev.update(100.0 + i as f64);
    }

    assert_eq!(stddev.count(), 5);

    // 继续添加数据
    stddev.update(110.0);

    // 窗口应该保持大小
    assert_eq!(stddev.count(), 5);
}

#[test]
fn test_stddev_increasing_trend() {
    let mut stddev = StdDev::new(10);

    // 持续上涨的趋势
    for i in 0..10 {
        stddev.update(100.0 + i as f64 * 2.0);
    }

    let result = stddev.update(120.0);
    assert!(result.is_some());

    // 线性趋势应该有中等程度的标准差
    let std_value = result.unwrap();
    assert!(std_value > 5.0 && std_value < 10.0);
}

#[test]
fn test_stddev_decreasing_trend() {
    let mut stddev = StdDev::new(10);

    // 持续下跌的趋势
    for i in 0..10 {
        stddev.update(120.0 - i as f64 * 2.0);
    }

    let result = stddev.update(100.0);
    assert!(result.is_some());

    // 线性趋势应该有中等程度的标准差
    let std_value = result.unwrap();
    assert!(std_value > 5.0 && std_value < 10.0);
}

#[test]
fn test_stddev_oscillation() {
    let mut stddev = StdDev::new(10);

    // 在范围内振荡
    for i in 0..10 {
        let price = if i % 2 == 0 { 95.0 } else { 105.0 };
        stddev.update(price);
    }

    let result = stddev.update(100.0);
    assert!(result.is_some());

    // 振荡应该产生一定的标准差
    let std_value = result.unwrap();
    assert!(std_value > 3.0);
}

#[test]
fn test_stddev_single_outlier() {
    let mut stddev = StdDev::new(5);

    // 大部分数据稳定,有一个异常值
    stddev.update(100.0);
    stddev.update(101.0);
    stddev.update(99.0);
    stddev.update(100.5);

    // 添加异常值之前
    let result_before = stddev.update(99.5);
    assert!(result_before.is_some());
    let std_before = result_before.unwrap();

    // 添加异常值
    let result_after = stddev.update(120.0);
    assert!(result_after.is_some());
    let std_after = result_after.unwrap();

    // 异常值应该增加标准差
    assert!(std_after > std_before);
}

#[test]
fn test_stddev_negative_prices() {
    let mut stddev = StdDev::new(5);

    // 测试负数价格(虽然现实中不常见,但数学上应该正确)
    let prices = vec![-10.0, -5.0, 0.0, 5.0, 10.0];
    for price in prices {
        stddev.update(price);
    }

    let result = stddev.update(15.0);
    assert!(result.is_some());
    assert!(result.unwrap() > 0.0);
}

#[test]
fn test_stddev_precision() {
    let mut stddev = StdDev::new(3);

    // 测试计算精度
    stddev.update(1.0);
    stddev.update(2.0);
    let result = stddev.update(3.0);

    assert!(result.is_some());
    
    // 均值 = 2, 方差 = ((1-2)² + (2-2)² + (3-2)²) / 3 = 2/3
    // 标准差 = √(2/3) ≈ 0.8165
    let expected = (2.0_f64 / 3.0).sqrt();
    assert!((result.unwrap() - expected).abs() < 1e-10);
}
