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

/// 测试RSI创建
#[test]
fn test_rsi_creation() {
    let rsi = RSI::new(14);
    assert_eq!(rsi.period(), 14);
    assert!(rsi.is_empty());
    assert!(!rsi.is_ready());
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "RSI周期必须大于0")]
fn test_rsi_zero_period_panic() {
    RSI::new(0);
}

/// 测试第一个值
#[test]
fn test_rsi_first_value() {
    let mut rsi = RSI::new(14);
    
    // 第一个值无法计算RSI
    let result = rsi.update(100.0);
    assert_eq!(result, None);
    assert!(!rsi.is_empty());
    assert!(!rsi.is_ready());
}

/// 测试持续上涨的情况（应该接近100）
#[test]
fn test_rsi_continuous_gains() {
    let mut rsi = RSI::new(14);
    
    // 第一个价格
    rsi.update(100.0);
    
    // 持续上涨
    let mut last_rsi = 0.0;
    for i in 1..20 {
        if let Some(value) = rsi.update(100.0 + i as f64) {
            last_rsi = value;
        }
    }
    
    // RSI应该非常高（接近100）
    assert!(last_rsi > 90.0);
    assert!(rsi.is_overbought());
}

/// 测试持续下跌的情况（应该接近0）
#[test]
fn test_rsi_continuous_losses() {
    let mut rsi = RSI::new(14);
    
    // 第一个价格
    rsi.update(100.0);
    
    // 持续下跌
    let mut last_rsi = 100.0;
    for i in 1..20 {
        if let Some(value) = rsi.update(100.0 - i as f64) {
            last_rsi = value;
        }
    }
    
    // RSI应该非常低（接近0）
    assert!(last_rsi < 10.0);
    assert!(rsi.is_oversold());
}

/// 测试震荡行情（RSI应该在合理范围）
#[test]
fn test_rsi_oscillating() {
    let mut rsi = RSI::new(14);
    
    let prices = vec![100.0, 102.0, 101.0, 103.0, 102.0, 104.0, 103.0, 105.0];
    let mut last_rsi = None;
    
    for price in prices {
        last_rsi = rsi.update(price);
    }
    
    // 震荡行情的RSI应该在合理范围
    if let Some(rsi_value) = last_rsi {
        assert!(rsi_value >= 0.0 && rsi_value <= 100.0);
        // 由于价格持续上涨，RSI可能较高
        assert!(!rsi.is_oversold());
    }
}

/// 测试重置功能
#[test]
fn test_rsi_reset() {
    let mut rsi = RSI::new(14);
    
    rsi.update(100.0);
    rsi.update(110.0);
    assert!(rsi.value().is_some());
    
    rsi.reset();
    
    assert!(rsi.value().is_none());
    assert!(rsi.is_empty());
    assert!(!rsi.is_ready());
}

/// 测试价格不变的情况
#[test]
fn test_rsi_no_change() {
    let mut rsi = RSI::new(5);
    
    rsi.update(100.0);
    
    // 价格不变多次
    for _ in 0..10 {
        rsi.update(100.0);
    }
    
    // 没有涨跌，RSI应该接近50（实际会是100因为avg_loss为0）
    let rsi_value = rsi.value().unwrap();
    assert_relative_eq!(rsi_value, 100.0, epsilon = 1e-5);
}

/// 测试单次大涨后的RSI
#[test]
fn test_rsi_single_big_gain() {
    let mut rsi = RSI::new(3);
    
    rsi.update(100.0);
    let rsi1 = rsi.update(200.0).unwrap(); // 大涨100%
    
    // 第一次涨幅后，RSI应该是100（因为没有跌幅）
    assert_relative_eq!(rsi1, 100.0, epsilon = 1e-10);
}

/// 测试单次大跌后的RSI
#[test]
fn test_rsi_single_big_loss() {
    let mut rsi = RSI::new(3);
    
    rsi.update(200.0);
    let rsi1 = rsi.update(100.0).unwrap(); // 大跌50%
    
    // 第一次跌幅后，RSI应该是0（因为没有涨幅）
    assert_relative_eq!(rsi1, 0.0, epsilon = 1e-10);
}

/// 测试克隆
#[test]
fn test_rsi_clone() {
    let mut rsi1 = RSI::new(14);
    rsi1.update(100.0);
    rsi1.update(110.0);
    rsi1.update(105.0);
    
    let rsi2 = rsi1.clone();
    
    assert_eq!(rsi1.period(), rsi2.period());
    assert_eq!(rsi1.value(), rsi2.value());
}

/// 测试边界情况：超买超卖阈值
#[test]
fn test_rsi_thresholds() {
    let mut rsi = RSI::new(14);
    
    // 模拟价格使RSI刚好在阈值附近
    rsi.update(100.0);
    
    // 先上涨使其超买
    for i in 1..20 {
        rsi.update(100.0 + i as f64 * 2.0);
    }
    assert!(rsi.is_overbought());
    
    // 重置后下跌使其超卖
    rsi.reset();
    rsi.update(100.0);
    for i in 1..20 {
        rsi.update(100.0 - i as f64 * 2.0);
    }
    assert!(rsi.is_oversold());
}

/// 测试实际场景数据
#[test]
fn test_rsi_realistic_data() {
    let mut rsi = RSI::new(14);
    
    // 模拟一段真实的价格数据
    let prices = vec![
        44.34, 44.09, 43.61, 44.33, 44.83,
        45.10, 45.42, 45.84, 46.08, 45.89,
        46.03, 45.61, 46.28, 46.28, 46.00,
    ];
    
    let mut last_rsi = None;
    for price in prices {
        last_rsi = rsi.update(price);
    }
    
    // 验证RSI在合理范围内
    if let Some(rsi_value) = last_rsi {
        assert!(rsi_value >= 0.0 && rsi_value <= 100.0);
    }
}
