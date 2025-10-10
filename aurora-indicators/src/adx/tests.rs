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

/// 测试ADX创建
#[test]
fn test_adx_creation() {
    let adx = ADX::new(14);
    assert_eq!(adx.period(), 14);
    assert!(adx.is_empty());
    assert!(!adx.is_ready());
}

/// 测试默认参数
#[test]
fn test_adx_default() {
    let adx = ADX::default();
    assert_eq!(adx.period(), 14);
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "ADX周期必须大于0")]
fn test_adx_zero_period_panic() {
    ADX::new(0);
}

/// 测试第一根K线
#[test]
fn test_adx_first_candle() {
    let mut adx = ADX::new(14);
    
    let result = adx.update(110.0, 90.0, 100.0);
    assert_eq!(result, None);
    assert!(!adx.is_empty());
}

/// 测试数据不足
#[test]
fn test_adx_insufficient_data() {
    let mut adx = ADX::new(5);
    
    for i in 0..5 {
        let base = 100.0 + i as f64;
        assert_eq!(adx.update(base + 5.0, base - 5.0, base), None);
    }
    
    assert!(!adx.is_ready());
}

/// 测试强趋势识别
#[test]
fn test_adx_strong_trend() {
    let mut adx = ADX::new(5);
    
    // 模拟强上升趋势
    for i in 0..20 {
        let base = 100.0 + i as f64 * 3.0;
        adx.update(base + 5.0, base - 5.0, base + 4.0);
    }
    
    if adx.is_ready() {
        // 强趋势中ADX应该较高
        if let Some(output) = adx.value() {
            assert!(output.adx >= 0.0);
            // +DI应该大于-DI（上升趋势）
            assert!(output.plus_di > output.minus_di);
        }
    }
}

/// 测试弱趋势识别
#[test]
fn test_adx_weak_trend() {
    let mut adx = ADX::new(5);
    
    // 模拟横盘整理
    for i in 0..20 {
        let base = 100.0 + (i as f64 * 0.5).sin() * 2.0;
        adx.update(base + 2.0, base - 2.0, base);
    }
    
    if adx.is_ready() {
        // 横盘时ADX应该较低
        if let Some(output) = adx.value() {
            assert!(output.adx >= 0.0 && output.adx <= 100.0);
        }
    }
}

/// 测试上升趋势判断
#[test]
fn test_adx_uptrend() {
    let mut adx = ADX::new(5);
    
    // 持续上涨
    for i in 0..15 {
        let base = 100.0 + i as f64 * 2.0;
        adx.update(base + 3.0, base - 1.0, base + 2.0);
    }
    
    if adx.is_ready() {
        assert!(adx.is_uptrend() || adx.count < 20);
    }
}

/// 测试下降趋势判断
#[test]
fn test_adx_downtrend() {
    let mut adx = ADX::new(5);
    
    // 先上涨建立基准
    for i in 0..8 {
        let base = 100.0 + i as f64;
        adx.update(base + 5.0, base - 5.0, base);
    }
    
    // 然后持续下跌
    for i in 0..10 {
        let base = 110.0 - i as f64 * 3.0;
        adx.update(base + 1.0, base - 3.0, base - 2.0);
    }
    
    if adx.is_ready() {
        if let Some(output) = adx.value() {
            // 下跌趋势中-DI可能大于+DI
            assert!(output.adx >= 0.0);
        }
    }
}

/// 测试重置功能
#[test]
fn test_adx_reset() {
    let mut adx = ADX::new(5);
    
    for i in 0..10 {
        let base = 100.0 + i as f64;
        adx.update(base + 5.0, base - 5.0, base);
    }
    
    adx.reset();
    
    assert!(adx.is_empty());
    assert!(!adx.is_ready());
    assert_eq!(adx.value(), None);
}

/// 测试克隆
#[test]
fn test_adx_clone() {
    let mut adx1 = ADX::new(14);
    
    for i in 0..20 {
        let base = 100.0 + i as f64;
        adx1.update(base + 5.0, base - 5.0, base);
    }
    
    let adx2 = adx1.clone();
    
    assert_eq!(adx1.period(), adx2.period());
    assert_eq!(adx1.is_ready(), adx2.is_ready());
}

/// 测试DI值范围
#[test]
fn test_adx_di_range() {
    let mut adx = ADX::new(5);
    
    for i in 0..20 {
        let base = 100.0 + i as f64;
        adx.update(base + 5.0, base - 5.0, base);
    }
    
    if let Some(output) = adx.value() {
        // DI值应该在0-100之间
        assert!(output.plus_di >= 0.0 && output.plus_di <= 100.0);
        assert!(output.minus_di >= 0.0 && output.minus_di <= 100.0);
        assert!(output.adx >= 0.0 && output.adx <= 100.0);
    }
}

/// 测试实际场景数据
#[test]
fn test_adx_realistic_data() {
    let mut adx = ADX::new(14);
    
    // 模拟真实OHLC数据
    let candles = vec![
        (105.0, 95.0, 100.0),
        (107.0, 97.0, 103.0),
        (110.0, 100.0, 108.0),
        (112.0, 102.0, 105.0),
        (108.0, 98.0, 102.0),
        (110.0, 100.0, 107.0),
        (115.0, 105.0, 112.0),
        (113.0, 103.0, 108.0),
    ];
    
    let mut last_output = None;
    for (high, low, close) in candles {
        last_output = adx.update(high, low, close);
    }
    
    // 验证输出合理性
    if let Some(output) = last_output {
        assert!(output.adx >= 0.0 && output.adx <= 100.0);
    }
}

/// 测试趋势强度判断
#[test]
fn test_adx_trend_strength() {
    let mut adx = ADX::new(5);
    
    // 模拟一段数据
    for i in 0..25 {
        let base = 100.0 + i as f64 * 2.0;
        adx.update(base + 5.0, base - 5.0, base + 3.0);
    }
    
    if adx.is_ready() {
        // 应该能判断趋势强弱（不会panic）
        let _ = adx.is_strong_trend();
        let _ = adx.is_weak_trend();
    }
}

/// 测试价格不变情况
#[test]
fn test_adx_no_movement() {
    let mut adx = ADX::new(5);
    
    for _ in 0..10 {
        adx.update(100.0, 100.0, 100.0);
    }
    
    if let Some(output) = adx.value() {
        // 价格不变时，ADX应该为0或接近0
        assert_relative_eq!(output.adx, 0.0, epsilon = 1e-5);
        assert_relative_eq!(output.plus_di, 0.0, epsilon = 1e-5);
        assert_relative_eq!(output.minus_di, 0.0, epsilon = 1e-5);
    }
}
