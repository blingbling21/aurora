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

/// 测试MACD创建
#[test]
fn test_macd_creation() {
    let macd = MACD::new(12, 26, 9);
    assert_eq!(macd.fast_period(), 12);
    assert_eq!(macd.slow_period(), 26);
    assert_eq!(macd.signal_period(), 9);
    assert!(macd.is_empty());
    assert!(!macd.is_ready());
}

/// 测试默认参数
#[test]
fn test_macd_default() {
    let macd = MACD::default();
    assert_eq!(macd.fast_period(), 12);
    assert_eq!(macd.slow_period(), 26);
    assert_eq!(macd.signal_period(), 9);
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "快线周期必须大于0")]
fn test_macd_zero_fast_period_panic() {
    MACD::new(0, 26, 9);
}

/// 测试快线大于慢线时的panic
#[test]
#[should_panic(expected = "快线周期必须小于慢线周期")]
fn test_macd_invalid_periods_panic() {
    MACD::new(26, 12, 9);
}

/// 测试基本更新
#[test]
fn test_macd_update() {
    let mut macd = MACD::new(3, 6, 3);
    
    let output = macd.update(100.0);
    
    // 第一个值，MACD应该是0（快慢EMA相同）
    assert_relative_eq!(output.macd, 0.0, epsilon = 1e-10);
    assert_relative_eq!(output.signal, 0.0, epsilon = 1e-10);
    assert_relative_eq!(output.histogram, 0.0, epsilon = 1e-10);
}

/// 测试上涨趋势
#[test]
fn test_macd_uptrend() {
    let mut macd = MACD::new(3, 6, 3);
    
    // 模拟上涨趋势
    for i in 1..=20 {
        macd.update(100.0 + i as f64);
    }
    
    let output = macd.value().unwrap();
    
    // 上涨趋势中，MACD应该为正（快线在慢线上方）
    assert!(output.macd > 0.0);
}

/// 测试下跌趋势
#[test]
fn test_macd_downtrend() {
    let mut macd = MACD::new(3, 6, 3);
    
    // 先上涨建立基准
    for i in 1..=10 {
        macd.update(100.0 + i as f64);
    }
    
    // 然后下跌
    for i in 1..=10 {
        macd.update(110.0 - i as f64 * 2.0);
    }
    
    let output = macd.value().unwrap();
    
    // 下跌趋势中，MACD应该为负或histogram为负
    assert!(output.histogram < 0.0 || output.macd < 0.0);
}

/// 测试金叉
#[test]
fn test_macd_bullish_crossover() {
    let mut macd = MACD::new(3, 6, 3);
    
    // 先下跌使histogram为负
    for i in (1..=10).rev() {
        macd.update(100.0 + i as f64);
    }
    
    let prev = macd.value().unwrap();
    
    // 然后上涨
    for _ in 1..5 {
        macd.update(120.0);
    }
    
    let current = macd.value().unwrap();
    
    // 检查是否可能出现金叉
    if prev.histogram < 0.0 && current.histogram > 0.0 {
        assert!(macd.is_bullish_crossover(&prev, &current));
    }
}

/// 测试死叉
#[test]
fn test_macd_bearish_crossover() {
    let mut macd = MACD::new(3, 6, 3);
    
    // 先上涨使histogram为正
    for i in 1..=10 {
        macd.update(100.0 + i as f64);
    }
    
    let prev = macd.value().unwrap();
    
    // 然后下跌
    for i in 1..5 {
        macd.update(110.0 - i as f64 * 5.0);
    }
    
    let current = macd.value().unwrap();
    
    // 检查是否可能出现死叉
    if prev.histogram > 0.0 && current.histogram < 0.0 {
        assert!(macd.is_bearish_crossover(&prev, &current));
    }
}

/// 测试重置功能
#[test]
fn test_macd_reset() {
    let mut macd = MACD::new(3, 6, 3);
    
    for i in 1..=10 {
        macd.update(100.0 + i as f64);
    }
    
    assert!(macd.value().is_some());
    
    macd.reset();
    
    assert!(macd.is_empty());
    assert!(!macd.is_ready());
}

/// 测试横盘整理
#[test]
fn test_macd_sideways() {
    let mut macd = MACD::new(5, 10, 5);
    
    // 价格在小范围内波动
    let prices = vec![100.0, 101.0, 100.0, 101.0, 100.0, 101.0, 100.0];
    
    for price in prices {
        macd.update(price);
    }
    
    let output = macd.value().unwrap();
    
    // 横盘时，MACD应该接近0
    assert!(output.macd.abs() < 2.0);
}

/// 测试克隆
#[test]
fn test_macd_clone() {
    let mut macd1 = MACD::new(12, 26, 9);
    
    for i in 1..=10 {
        macd1.update(100.0 + i as f64);
    }
    
    let macd2 = macd1.clone();
    
    assert_eq!(macd1.fast_period(), macd2.fast_period());
    assert_eq!(macd1.slow_period(), macd2.slow_period());
    assert_eq!(macd1.signal_period(), macd2.signal_period());
}

/// 测试柱状图的符号变化
#[test]
fn test_macd_histogram_changes() {
    let mut macd = MACD::new(3, 6, 3);
    
    let mut prev_histogram = 0.0;
    let mut sign_changes = 0;
    
    // 模拟价格波动
    for i in 1..=30 {
        let price = 100.0 + (i as f64 * 0.5).sin() * 10.0;
        let output = macd.update(price);
        
        if prev_histogram * output.histogram < 0.0 {
            sign_changes += 1;
        }
        
        prev_histogram = output.histogram;
    }
    
    // 在波动的价格中应该有符号变化
    assert!(sign_changes > 0);
}

/// 测试实际场景数据
#[test]
fn test_macd_realistic_data() {
    let mut macd = MACD::default();
    
    // 模拟真实的价格数据
    let prices = vec![
        100.0, 102.0, 104.0, 103.0, 105.0, 107.0, 106.0, 108.0,
        110.0, 109.0, 111.0, 113.0, 112.0, 114.0, 116.0, 115.0,
    ];
    
    let mut outputs = Vec::new();
    for price in prices {
        outputs.push(macd.update(price));
    }
    
    // 验证MACD值在合理范围内
    for output in outputs {
        assert!(output.macd.abs() < 50.0);
        assert!(output.signal.abs() < 50.0);
        assert!(output.histogram.abs() < 50.0);
    }
}
