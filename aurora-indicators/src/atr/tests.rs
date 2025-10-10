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

/// 测试ATR创建
#[test]
fn test_atr_creation() {
    let atr = ATR::new(14);
    assert_eq!(atr.period(), 14);
    assert!(atr.is_empty());
    assert!(!atr.is_ready());
}

/// 测试默认参数
#[test]
fn test_atr_default() {
    let atr = ATR::default();
    assert_eq!(atr.period(), 14);
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "ATR周期必须大于0")]
fn test_atr_zero_period_panic() {
    ATR::new(0);
}

/// 测试第一根K线
#[test]
fn test_atr_first_candle() {
    let mut atr = ATR::new(5);
    
    // 第一根K线：high=110, low=90, close=100
    let result = atr.update(110.0, 90.0, 100.0);
    
    assert!(result.is_some());
    // 第一根K线的TR应该是 high - low = 20
    assert_relative_eq!(result.unwrap(), 20.0, epsilon = 1e-10);
}

/// 测试连续K线
#[test]
fn test_atr_multiple_candles() {
    let mut atr = ATR::new(3);
    
    // 第一根：high=110, low=90, close=100
    atr.update(110.0, 90.0, 100.0);
    
    // 第二根：high=105, low=95, close=102
    // TR = max(105-95, |105-100|, |95-100|) = max(10, 5, 5) = 10
    let result2 = atr.update(105.0, 95.0, 102.0);
    assert!(result2.is_some());
}

/// 测试跳空高开
#[test]
fn test_atr_gap_up() {
    let mut atr = ATR::new(3);
    
    // 第一根K线
    atr.update(100.0, 95.0, 98.0);
    
    // 跳空高开：最低价高于前收盘价
    // high=110, low=105, close=108, prev_close=98
    // TR = max(110-105, |110-98|, |105-98|) = max(5, 12, 7) = 12
    let result = atr.update(110.0, 105.0, 108.0);
    
    assert!(result.is_some());
    // ATR应该反映出跳空导致的大波动
    assert!(result.unwrap() > 5.0);
}

/// 测试跳空低开
#[test]
fn test_atr_gap_down() {
    let mut atr = ATR::new(3);
    
    // 第一根K线
    atr.update(100.0, 95.0, 98.0);
    
    // 跳空低开：最高价低于前收盘价
    // high=90, low=85, close=88, prev_close=98
    // TR = max(90-85, |90-98|, |85-98|) = max(5, 8, 13) = 13
    let result = atr.update(90.0, 85.0, 88.0);
    
    assert!(result.is_some());
    // ATR应该反映出跳空导致的大波动
    assert!(result.unwrap() > 5.0);
}

/// 测试低波动性
#[test]
fn test_atr_low_volatility() {
    let mut atr = ATR::new(5);
    
    // 模拟低波动的市场
    for i in 0..10 {
        let base = 100.0 + i as f64 * 0.1;
        atr.update(base + 0.5, base - 0.5, base);
    }
    
    let atr_value = atr.value().unwrap();
    
    // 低波动时，ATR应该很小
    assert!(atr_value < 2.0);
}

/// 测试高波动性
#[test]
fn test_atr_high_volatility() {
    let mut atr = ATR::new(5);
    
    // 模拟高波动的市场
    for i in 0..10 {
        let base = 100.0 + (i as f64 * 10.0).sin() * 20.0;
        atr.update(base + 10.0, base - 10.0, base);
    }
    
    let atr_value = atr.value().unwrap();
    
    // 高波动时，ATR应该较大
    assert!(atr_value > 10.0);
}

/// 测试重置功能
#[test]
fn test_atr_reset() {
    let mut atr = ATR::new(5);
    
    for i in 0..10 {
        let price = 100.0 + i as f64;
        atr.update(price + 5.0, price - 5.0, price);
    }
    
    assert!(atr.value().is_some());
    assert!(atr.is_ready());
    
    atr.reset();
    
    assert!(atr.is_empty());
    assert!(!atr.is_ready());
    assert_eq!(atr.value(), None);
}

/// 测试止损价格计算（多头）
#[test]
fn test_atr_stop_loss_long() {
    let mut atr = ATR::new(3);
    
    // 建立ATR值
    atr.update(105.0, 95.0, 100.0);
    atr.update(110.0, 100.0, 105.0);
    atr.update(112.0, 102.0, 110.0);
    
    // 入场价100，使用2倍ATR止损
    let stop = atr.stop_loss(100.0, 2.0, true);
    
    assert!(stop.is_some());
    // 多头止损应该在入场价下方
    assert!(stop.unwrap() < 100.0);
}

/// 测试止损价格计算（空头）
#[test]
fn test_atr_stop_loss_short() {
    let mut atr = ATR::new(3);
    
    // 建立ATR值
    atr.update(105.0, 95.0, 100.0);
    atr.update(110.0, 100.0, 105.0);
    atr.update(112.0, 102.0, 110.0);
    
    // 入场价100，使用2倍ATR止损
    let stop = atr.stop_loss(100.0, 2.0, false);
    
    assert!(stop.is_some());
    // 空头止损应该在入场价上方
    assert!(stop.unwrap() > 100.0);
}

/// 测试克隆
#[test]
fn test_atr_clone() {
    let mut atr1 = ATR::new(14);
    
    for i in 0..5 {
        let price = 100.0 + i as f64;
        atr1.update(price + 5.0, price - 5.0, price);
    }
    
    let atr2 = atr1.clone();
    
    assert_eq!(atr1.period(), atr2.period());
    assert_eq!(atr1.value(), atr2.value());
}

/// 测试异常K线数据
#[test]
fn test_atr_invalid_candle() {
    let mut atr = ATR::new(3);
    
    // high < low 的异常情况（实际中不应该出现，但测试容错性）
    // 注意：在真实场景中应该在调用前验证数据
    atr.update(95.0, 100.0, 98.0);
    
    // 程序应该不会崩溃
    assert!(atr.value().is_some());
}

/// 测试相同价格
#[test]
fn test_atr_same_prices() {
    let mut atr = ATR::new(3);
    
    // 所有价格相同（无波动）
    for _ in 0..5 {
        atr.update(100.0, 100.0, 100.0);
    }
    
    let atr_value = atr.value().unwrap();
    
    // 无波动时，ATR应该为0或接近0
    assert_relative_eq!(atr_value, 0.0, epsilon = 1e-10);
}

/// 测试实际场景数据
#[test]
fn test_atr_realistic_data() {
    let mut atr = ATR::new(14);
    
    // 模拟真实的OHLC数据
    let candles = vec![
        (102.0, 98.0, 100.0),
        (104.0, 100.0, 103.0),
        (105.0, 101.0, 102.0),
        (103.0, 99.0, 100.0),
        (101.0, 97.0, 99.0),
    ];
    
    let mut last_atr = None;
    for (high, low, close) in candles {
        last_atr = atr.update(high, low, close);
    }
    
    assert!(last_atr.is_some());
    let atr_value = last_atr.unwrap();
    
    // 验证ATR在合理范围内
    assert!(atr_value > 0.0 && atr_value < 20.0);
}

/// 测试ATR的平滑效果
#[test]
fn test_atr_smoothing() {
    let mut atr = ATR::new(5);
    
    // 一根大波动的K线
    atr.update(120.0, 80.0, 100.0);
    let atr1 = atr.value().unwrap();
    
    // 后续多根小波动的K线
    for _ in 0..10 {
        atr.update(102.0, 98.0, 100.0);
    }
    let atr2 = atr.value().unwrap();
    
    // ATR应该被平滑，从大值逐渐降低
    assert!(atr2 < atr1);
}
