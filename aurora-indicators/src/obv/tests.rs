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

/// 测试OBV创建
#[test]
fn test_obv_creation() {
    let obv = OBV::new();
    assert_relative_eq!(obv.value(), 0.0, epsilon = 1e-10);
    assert!(obv.is_empty());
    assert!(!obv.is_ready());
}

/// 测试默认构造
#[test]
fn test_obv_default() {
    let obv = OBV::default();
    assert_relative_eq!(obv.value(), 0.0, epsilon = 1e-10);
}

/// 测试第一根K线
#[test]
fn test_obv_first_candle() {
    let mut obv = OBV::new();
    
    let result = obv.update(100.0, 1000.0);
    
    // 第一根K线，OBV应该保持为0
    assert_relative_eq!(result, 0.0, epsilon = 1e-10);
    assert!(obv.is_ready());
}

/// 测试价格上涨
#[test]
fn test_obv_price_rises() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    let result = obv.update(105.0, 1500.0);
    
    // 价格上涨，OBV应该增加成交量
    assert_relative_eq!(result, 1500.0, epsilon = 1e-10);
}

/// 测试价格下跌
#[test]
fn test_obv_price_falls() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    let result = obv.update(95.0, 1500.0);
    
    // 价格下跌，OBV应该减少成交量
    assert_relative_eq!(result, -1500.0, epsilon = 1e-10);
}

/// 测试价格不变
#[test]
fn test_obv_price_unchanged() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    let result = obv.update(100.0, 1500.0);
    
    // 价格不变，OBV应该保持不变
    assert_relative_eq!(result, 0.0, epsilon = 1e-10);
}

/// 测试连续上涨
#[test]
fn test_obv_continuous_rises() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    obv.update(102.0, 1200.0);
    obv.update(105.0, 1500.0);
    let result = obv.update(108.0, 1800.0);
    
    // 连续上涨，OBV应该持续增加
    let expected = 1200.0 + 1500.0 + 1800.0;
    assert_relative_eq!(result, expected, epsilon = 1e-10);
    assert!(obv.is_rising(expected - 1800.0));
}

/// 测试连续下跌
#[test]
fn test_obv_continuous_falls() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    obv.update(98.0, 1200.0);
    obv.update(95.0, 1500.0);
    let result = obv.update(92.0, 1800.0);
    
    // 连续下跌，OBV应该持续减少
    let expected = -(1200.0 + 1500.0 + 1800.0);
    assert_relative_eq!(result, expected, epsilon = 1e-10);
    assert!(obv.is_falling(expected + 1800.0));
}

/// 测试价格涨跌交替
#[test]
fn test_obv_alternating() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    obv.update(105.0, 1000.0); // +1000
    obv.update(103.0, 1000.0); // -1000
    let result = obv.update(107.0, 1000.0); // +1000
    
    // 涨跌交替，最终OBV = +1000 - 1000 + 1000 = 1000
    assert_relative_eq!(result, 1000.0, epsilon = 1e-10);
}

/// 测试重置功能
#[test]
fn test_obv_reset() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    obv.update(105.0, 1500.0);
    assert!(obv.value() != 0.0);
    
    obv.reset();
    
    assert_relative_eq!(obv.value(), 0.0, epsilon = 1e-10);
    assert!(obv.is_empty());
    assert!(!obv.is_ready());
}

/// 测试大成交量
#[test]
fn test_obv_large_volume() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000000.0);
    let result = obv.update(105.0, 5000000.0);
    
    assert_relative_eq!(result, 5000000.0, epsilon = 1e-5);
}

/// 测试零成交量
#[test]
fn test_obv_zero_volume() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 1000.0);
    let result = obv.update(105.0, 0.0);
    
    // 即使价格上涨，但成交量为0，OBV不变
    assert_relative_eq!(result, 0.0, epsilon = 1e-10);
}

/// 测试克隆
#[test]
fn test_obv_clone() {
    let mut obv1 = OBV::new();
    
    obv1.update(100.0, 1000.0);
    obv1.update(105.0, 1500.0);
    
    let obv2 = obv1.clone();
    
    assert_relative_eq!(obv1.value(), obv2.value(), epsilon = 1e-10);
}

/// 测试价格背离
#[test]
fn test_obv_divergence() {
    let mut obv = OBV::new();
    
    // 模拟价格创新高但成交量萎缩的情况
    obv.update(100.0, 10000.0);
    obv.update(102.0, 8000.0);  // 价格上涨但成交量减少
    let obv1 = obv.value();
    
    obv.update(105.0, 5000.0);  // 价格继续上涨但成交量继续减少
    let obv2 = obv.value();
    
    // OBV仍然在上涨，但增速放缓
    assert!(obv2 > obv1);
    assert!((obv2 - obv1) < 8000.0); // 增量小于前一次
}

/// 测试实际场景数据
#[test]
fn test_obv_realistic_data() {
    let mut obv = OBV::new();
    
    // 模拟真实的价格和成交量数据
    let data = vec![
        (100.0, 10000.0),
        (102.0, 12000.0),
        (101.0, 8000.0),
        (103.0, 15000.0),
        (105.0, 18000.0),
    ];
    
    let mut last_obv = 0.0;
    for (close, volume) in data {
        last_obv = obv.update(close, volume);
    }
    
    // 验证OBV在合理范围内
    assert!(last_obv.abs() < 100000.0);
}

/// 测试趋势确认
#[test]
fn test_obv_trend_confirmation() {
    let mut obv = OBV::new();
    
    // 模拟强势上涨趋势
    obv.update(100.0, 10000.0);
    
    let mut prev_obv = obv.value();
    for i in 1..=5 {
        let price = 100.0 + i as f64 * 2.0;
        let volume = 10000.0 + i as f64 * 1000.0;
        obv.update(price, volume);
        
        // 每次都应该上涨
        assert!(obv.is_rising(prev_obv));
        prev_obv = obv.value();
    }
}

/// 测试振荡市场
#[test]
fn test_obv_oscillating_market() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 10000.0);
    
    // 价格在小范围内波动
    obv.update(101.0, 5000.0);  // +5000
    obv.update(100.0, 5000.0);  // -5000
    obv.update(101.0, 5000.0);  // +5000
    let result = obv.update(100.0, 5000.0);  // -5000
    
    // 振荡市场中，OBV应该接近初始值
    assert_relative_eq!(result, 0.0, epsilon = 1e-10);
}

/// 测试负OBV值
#[test]
fn test_obv_negative_values() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 10000.0);
    
    // 大幅下跌
    let result = obv.update(80.0, 50000.0);
    
    // OBV应该变为负值
    assert!(result < 0.0);
    assert_relative_eq!(result, -50000.0, epsilon = 1e-10);
}

/// 测试is_rising和is_falling
#[test]
fn test_obv_rising_falling() {
    let mut obv = OBV::new();
    
    obv.update(100.0, 10000.0);
    obv.update(105.0, 10000.0);
    
    let obv1 = obv.value();
    assert!(obv1 > 0.0);
    
    obv.update(102.0, 15000.0);
    let obv2 = obv.value();
    
    assert!(obv.is_falling(obv1));
    assert!(!obv.is_rising(obv1));
}
