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

/// 测试EMA创建
#[test]
fn test_ema_creation() {
    let ema = EMA::new(10);
    assert_eq!(ema.period(), 10);
    assert_eq!(ema.count(), 0);
    assert!(ema.is_empty());
    assert!(!ema.is_ready());

    // 验证alpha计算正确: α = 2 / (10 + 1) = 2/11
    assert_relative_eq!(ema.alpha(), 2.0 / 11.0, epsilon = 1e-10);
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "EMA周期必须大于0")]
fn test_ema_zero_period_panic() {
    EMA::new(0);
}

/// 测试第一个值的处理
#[test]
fn test_ema_first_value() {
    let mut ema = EMA::new(5);

    let result = ema.update(100.0);
    assert_relative_eq!(result, 100.0, epsilon = 1e-10);
    assert_relative_eq!(ema.value().unwrap(), 100.0, epsilon = 1e-10);
    assert_eq!(ema.count(), 1);
    assert!(ema.is_ready());
    assert!(!ema.is_empty());
}

/// 测试EMA计算
#[test]
fn test_ema_calculation() {
    let mut ema = EMA::new(3); // α = 2/(3+1) = 0.5

    // 第一个值
    let ema1 = ema.update(10.0);
    assert_relative_eq!(ema1, 10.0, epsilon = 1e-10);

    // 第二个值: EMA = 0.5 * 20 + 0.5 * 10 = 15
    let ema2 = ema.update(20.0);
    assert_relative_eq!(ema2, 15.0, epsilon = 1e-10);

    // 第三个值: EMA = 0.5 * 30 + 0.5 * 15 = 22.5
    let ema3 = ema.update(30.0);
    assert_relative_eq!(ema3, 22.5, epsilon = 1e-10);
}

/// 测试EMA响应性
#[test]
fn test_ema_responsiveness() {
    // 比较不同周期的EMA对价格变化的响应
    let mut ema_short = EMA::new(5);  // 短周期，响应快
    let mut ema_long = EMA::new(20);  // 长周期，响应慢

    // 初始价格
    ema_short.update(100.0);
    ema_long.update(100.0);

    // 价格突然上涨到120
    let short_result = ema_short.update(120.0);
    let long_result = ema_long.update(120.0);

    // 短周期EMA应该更接近新价格
    assert!(short_result > long_result);
}

/// 测试重置功能
#[test]
fn test_ema_reset() {
    let mut ema = EMA::new(5);

    ema.update(100.0);
    ema.update(110.0);
    assert!(ema.value().is_some());
    assert_eq!(ema.count(), 2);

    ema.reset();

    assert!(ema.value().is_none());
    assert_eq!(ema.count(), 0);
    assert!(ema.is_empty());
    assert!(!ema.is_ready());
}

/// 测试连续更新
#[test]
fn test_ema_continuous_updates() {
    let mut ema = EMA::new(10);

    for i in 1..=100 {
        let result = ema.update(i as f64);
        assert!(result > 0.0);
    }

    assert_eq!(ema.count(), 100);
    // EMA应该接近但小于最后的值
    let final_ema = ema.value().unwrap();
    assert!(final_ema < 100.0 && final_ema > 90.0);
}

/// 测试极端值
#[test]
fn test_ema_extreme_values() {
    let mut ema = EMA::new(5);

    // 零值
    let result1 = ema.update(0.0);
    assert_relative_eq!(result1, 0.0, epsilon = 1e-10);

    // 负值
    ema.reset();
    ema.update(-100.0);
    let result2 = ema.update(-200.0);
    assert!(result2 < -100.0);

    // 大值
    ema.reset();
    ema.update(1e10);
    let result3 = ema.update(1e10);
    assert_relative_eq!(result3, 1e10, epsilon = 1e5);
}

/// 测试克隆
#[test]
fn test_ema_clone() {
    let mut ema1 = EMA::new(10);
    ema1.update(100.0);
    ema1.update(110.0);

    let ema2 = ema1.clone();

    assert_eq!(ema1.period(), ema2.period());
    assert_eq!(ema1.count(), ema2.count());
    assert_relative_eq!(ema1.alpha(), ema2.alpha(), epsilon = 1e-10);
    assert_eq!(ema1.value(), ema2.value());
}

/// 测试与理论值的对比
#[test]
fn test_ema_theoretical_values() {
    let mut ema = EMA::new(9); // α = 2/10 = 0.2
    
    // 使用一系列已知的价格
    let prices = vec![22.0, 22.5, 23.0, 23.5, 24.0];
    let mut results = Vec::new();
    
    for price in prices {
        results.push(ema.update(price));
    }
    
    // 验证第一个值
    assert_relative_eq!(results[0], 22.0, epsilon = 1e-10);
    
    // 手工计算验证第二个值: 0.2 * 22.5 + 0.8 * 22.0 = 22.1
    assert_relative_eq!(results[1], 22.1, epsilon = 1e-10);
}

/// 测试单周期EMA
#[test]
fn test_ema_single_period() {
    let mut ema = EMA::new(1); // α = 2/2 = 1.0
    
    // alpha为1时，EMA应该等于当前价格
    let result1 = ema.update(100.0);
    assert_relative_eq!(result1, 100.0, epsilon = 1e-10);
    
    let result2 = ema.update(200.0);
    assert_relative_eq!(result2, 200.0, epsilon = 1e-10);
}
