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
use approx::assert_relative_eq;

/// 测试MA指标创建
#[test]
fn test_ma_creation() {
    let ma = MA::new(5);
    assert_eq!(ma.period(), 5);
    assert_eq!(ma.len(), 0);
    assert_eq!(ma.sum, 0.0);
    assert!(ma.is_empty());
    assert!(!ma.is_ready());
}

/// 测试周期为0时的panic行为
#[test]
#[should_panic(expected = "移动平均周期必须大于0")]
fn test_ma_zero_period_panic() {
    MA::new(0);
}

/// 测试数据不足时的行为
#[test]
fn test_ma_insufficient_data() {
    let mut ma = MA::new(3);

    assert_eq!(ma.update(10.0), None);
    assert_eq!(ma.len(), 1);
    assert!(!ma.is_empty());
    assert!(!ma.is_ready());

    assert_eq!(ma.update(20.0), None);
    assert_eq!(ma.len(), 2);
    assert!(!ma.is_ready());

    assert_eq!(ma.value(), None);
}

/// 测试有足够数据时的计算
#[test]
fn test_ma_sufficient_data() {
    let mut ma = MA::new(3);

    ma.update(10.0);
    ma.update(20.0);
    let result = ma.update(30.0);

    assert!(result.is_some());
    assert_relative_eq!(result.unwrap(), 20.0, epsilon = 1e-10);
    assert_relative_eq!(ma.value().unwrap(), 20.0, epsilon = 1e-10);
    assert!(ma.is_ready());
    assert_eq!(ma.len(), 3);
}

/// 测试滑动窗口功能
#[test]
fn test_ma_sliding_window() {
    let mut ma = MA::new(3);

    // 填充初始数据
    ma.update(10.0);
    ma.update(20.0);
    let result1 = ma.update(30.0);
    assert_relative_eq!(result1.unwrap(), 20.0, epsilon = 1e-10); // (10+20+30)/3 = 20

    // 添加新数据，应该滑动窗口
    let result2 = ma.update(40.0);
    assert_relative_eq!(result2.unwrap(), 30.0, epsilon = 1e-10); // (20+30+40)/3 = 30

    let result3 = ma.update(50.0);
    assert_relative_eq!(result3.unwrap(), 40.0, epsilon = 1e-10); // (30+40+50)/3 = 40

    // 验证窗口大小始终保持为3
    assert_eq!(ma.len(), 3);
}

/// 测试重置功能
#[test]
fn test_ma_reset() {
    let mut ma = MA::new(3);

    ma.update(10.0);
    ma.update(20.0);
    ma.update(30.0);

    assert!(ma.value().is_some());
    assert!(ma.is_ready());
    assert!(!ma.is_empty());

    ma.reset();

    assert_eq!(ma.len(), 0);
    assert_eq!(ma.sum, 0.0);
    assert!(ma.value().is_none());
    assert!(!ma.is_ready());
    assert!(ma.is_empty());
}

/// 测试单周期MA
#[test]
fn test_ma_single_period() {
    let mut ma = MA::new(1);

    let result1 = ma.update(42.0);
    assert_relative_eq!(result1.unwrap(), 42.0, epsilon = 1e-10);

    let result2 = ma.update(84.0);
    assert_relative_eq!(result2.unwrap(), 84.0, epsilon = 1e-10);

    assert_eq!(ma.len(), 1); // 单周期只保存一个值
}

/// 测试大数据量的处理
#[test]
fn test_ma_large_dataset() {
    let mut ma = MA::new(100);

    // 添加前99个数据点，不应该有返回值
    for i in 1..100 {
        assert_eq!(ma.update(i as f64), None);
    }

    assert_eq!(ma.len(), 99);
    assert!(!ma.is_ready());

    // 第100个数据点应该产生结果
    let result = ma.update(100.0);
    assert!(result.is_some());
    assert!(ma.is_ready());
    assert_eq!(ma.len(), 100);

    // 验证计算结果: (1+2+...+100)/100 = 50.5
    assert_relative_eq!(result.unwrap(), 50.5, epsilon = 1e-10);
}

/// 测试极端值处理
#[test]
fn test_ma_extreme_values() {
    let mut ma = MA::new(3);

    // 测试0值
    ma.update(0.0);
    ma.update(0.0);
    let result1 = ma.update(0.0);
    assert_relative_eq!(result1.unwrap(), 0.0, epsilon = 1e-10);

    // 测试负值
    ma.reset();
    ma.update(-10.0);
    ma.update(-20.0);
    let result2 = ma.update(-30.0);
    assert_relative_eq!(result2.unwrap(), -20.0, epsilon = 1e-10);

    // 测试很大的值
    ma.reset();
    ma.update(1e10);
    ma.update(1e10);
    let result3 = ma.update(1e10);
    assert_relative_eq!(result3.unwrap(), 1e10, epsilon = 1e-5);
}

/// 测试精度保持
#[test]
fn test_ma_precision() {
    let mut ma = MA::new(3);

    // 使用需要高精度的小数
    ma.update(1.0 / 3.0); // 0.333...
    ma.update(2.0 / 3.0); // 0.666...
    let result = ma.update(1.0);

    let expected = (1.0 / 3.0 + 2.0 / 3.0 + 1.0) / 3.0; // 2/3
    assert_relative_eq!(result.unwrap(), expected, epsilon = 1e-15);
}

/// 测试克隆功能
#[test]
fn test_ma_clone() {
    let mut ma1 = MA::new(3);
    ma1.update(10.0);
    ma1.update(20.0);

    let ma2 = ma1.clone();

    // 克隆的对象应该有相同的状态
    assert_eq!(ma1.period(), ma2.period());
    assert_eq!(ma1.len(), ma2.len());
    assert_eq!(ma1.sum, ma2.sum);
    assert_eq!(ma1.value(), ma2.value());
}

/// 测试Debug格式化
#[test]
fn test_ma_debug_format() {
    let mut ma = MA::new(5);
    ma.update(100.0);

    let debug_str = format!("{:?}", ma);
    assert!(debug_str.contains("MA"));
    assert!(debug_str.contains("period: 5"));
}

/// 测试边界条件：接近溢出的情况
#[test]
fn test_ma_overflow_resistance() {
    let mut ma = MA::new(2);

    // 使用接近f64最大值的数字
    let large_val = f64::MAX / 10.0;
    ma.update(large_val);
    let result = ma.update(large_val);

    assert!(result.is_some());
    assert_relative_eq!(result.unwrap(), large_val, epsilon = large_val * 1e-15);
}
