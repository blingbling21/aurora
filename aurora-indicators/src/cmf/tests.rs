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
fn test_cmf_initialization() {
    let mut cmf = CMF::new(20);
    
    // 前19个数据点不应产生结果
    for i in 0..19 {
        let result = cmf.update(100.0 + i as f64, 95.0 + i as f64, 98.0 + i as f64, 10000.0);
        assert!(result.is_none());
    }
    
    // 第20个数据点应该产生结果
    let result = cmf.update(119.0, 114.0, 117.0, 10000.0);
    assert!(result.is_some());
}

#[test]
fn test_cmf_buying_pressure() {
    let mut cmf = CMF::new(5);
    
    // 模拟强烈的买方压力（收盘价接近最高价）
    for _ in 0..5 {
        cmf.update(100.0, 90.0, 99.0, 10000.0);  // 收盘价接近最高价
    }
    
    let result = cmf.update(100.0, 90.0, 99.0, 10000.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 应该显示正的资金流（买方压力）
    assert!(value > 0.0);
    assert!(value <= 1.0);
}

#[test]
fn test_cmf_selling_pressure() {
    let mut cmf = CMF::new(5);
    
    // 模拟强烈的卖方压力（收盘价接近最低价）
    for _ in 0..5 {
        cmf.update(100.0, 90.0, 91.0, 10000.0);  // 收盘价接近最低价
    }
    
    let result = cmf.update(100.0, 90.0, 91.0, 10000.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 应该显示负的资金流（卖方压力）
    assert!(value < 0.0);
    assert!(value >= -1.0);
}

#[test]
fn test_cmf_neutral_market() {
    let mut cmf = CMF::new(10);
    
    // 模拟中性市场（收盘价在中间位置）
    for _ in 0..10 {
        cmf.update(100.0, 90.0, 95.0, 10000.0);  // 收盘价在中点
    }
    
    let result = cmf.update(100.0, 90.0, 95.0, 10000.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 应该接近0（市场平衡）
    assert!(value.abs() < 0.1);
}

#[test]
fn test_cmf_value_range() {
    let mut cmf = CMF::new(5);
    
    // 测试各种极端情况
    let test_cases = vec![
        (100.0, 90.0, 100.0, 10000.0),  // 收盘价 = 最高价
        (100.0, 90.0, 90.0, 10000.0),   // 收盘价 = 最低价
        (100.0, 90.0, 95.0, 10000.0),   // 收盘价在中间
    ];
    
    for (high, low, close, volume) in test_cases {
        for _ in 0..5 {
            cmf.update(high, low, close, volume);
        }
        
        let result = cmf.update(high, low, close, volume);
        if let Some(value) = result {
            // CMF 值应该在 -1 到 +1 之间
            assert!(value >= -1.0 && value <= 1.0);
        }
        
        cmf.reset();
    }
}

#[test]
fn test_cmf_volume_impact() {
    let mut cmf1 = CMF::new(5);
    let mut cmf2 = CMF::new(5);
    
    // 相同价格，不同成交量
    for _ in 0..5 {
        cmf1.update(100.0, 90.0, 98.0, 10000.0);   // 普通成交量
        cmf2.update(100.0, 90.0, 98.0, 50000.0);   // 大成交量
    }
    
    let result1 = cmf1.update(100.0, 90.0, 98.0, 10000.0);
    let result2 = cmf2.update(100.0, 90.0, 98.0, 50000.0);
    
    assert!(result1.is_some());
    assert!(result2.is_some());
    
    // 成交量不影响 CMF 的符号，但可能影响计算细节
    let value1 = result1.unwrap();
    let value2 = result2.unwrap();
    
    assert!(value1 > 0.0);
    assert!(value2 > 0.0);
}

#[test]
fn test_cmf_zero_range() {
    let mut cmf = CMF::new(5);
    
    // 测试高低价相同的情况（避免除以零）
    for _ in 0..5 {
        cmf.update(100.0, 100.0, 100.0, 10000.0);
    }
    
    let result = cmf.update(100.0, 100.0, 100.0, 10000.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 当价格无波动时，CMF 应该为 0
    assert_eq!(value, 0.0);
}

#[test]
fn test_cmf_zero_volume() {
    let mut cmf = CMF::new(5);
    
    // 测试零成交量的情况
    for _ in 0..5 {
        cmf.update(100.0, 90.0, 95.0, 0.0);
    }
    
    let result = cmf.update(100.0, 90.0, 95.0, 0.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 零成交量应该返回 0
    assert_eq!(value, 0.0);
}

#[test]
fn test_cmf_trend_confirmation() {
    let mut cmf = CMF::new(10);
    
    // 模拟上升趋势伴随买方压力
    for i in 0..15 {
        let base = 100.0 + i as f64 * 2.0;
        // 收盘价接近最高价，表示买方压力
        cmf.update(base + 5.0, base - 5.0, base + 4.0, 10000.0);
    }
    
    let result = cmf.update(135.0, 125.0, 134.0, 10000.0);
    assert!(result.is_some());
    
    let value = result.unwrap();
    
    // 在上升趋势中，CMF 应该为正
    assert!(value > 0.0);
}

#[test]
fn test_cmf_divergence_detection() {
    let mut cmf = CMF::new(5);
    
    // 价格上升但买方压力减弱（可能的顶部背离）
    let data = vec![
        (100.0, 90.0, 98.0, 10000.0),   // 强买方压力
        (105.0, 95.0, 103.0, 10000.0),  // 强买方压力
        (110.0, 100.0, 108.0, 10000.0), // 强买方压力
        (115.0, 105.0, 110.0, 10000.0), // 买方压力减弱
        (120.0, 110.0, 112.0, 10000.0), // 买方压力进一步减弱
    ];
    
    let mut cmf_values = Vec::new();
    for (high, low, close, volume) in data {
        if let Some(value) = cmf.update(high, low, close, volume) {
            cmf_values.push(value);
        }
    }
    
    // 应该能够检测到 CMF 值的变化趋势
    assert!(cmf_values.len() > 0);
}

#[test]
fn test_cmf_custom_period() {
    let mut cmf_short = CMF::new(5);
    let mut cmf_long = CMF::new(20);
    
    assert_eq!(cmf_short.period(), 5);
    assert_eq!(cmf_long.period(), 20);
    
    // 短周期应该更快产生结果
    for i in 0..10 {
        let base = 100.0 + i as f64;
        let result_short = cmf_short.update(base + 5.0, base - 5.0, base + 3.0, 10000.0);
        let result_long = cmf_long.update(base + 5.0, base - 5.0, base + 3.0, 10000.0);
        
        if i >= 4 {
            assert!(result_short.is_some());
        }
        if i < 19 {
            assert!(result_long.is_none());
        }
    }
}

#[test]
fn test_cmf_reset() {
    let mut cmf = CMF::new(10);
    
    // 添加一些数据
    for i in 0..15 {
        cmf.update(100.0 + i as f64, 95.0 + i as f64, 98.0 + i as f64, 10000.0);
    }
    
    // 重置
    cmf.reset();
    
    // 重置后应该需要重新积累数据
    let result = cmf.update(100.0, 95.0, 98.0, 10000.0);
    assert!(result.is_none());
}

#[test]
fn test_cmf_real_market_scenario() {
    let mut cmf = CMF::new(10);
    
    // 模拟真实市场场景：上升 -> 顶部 -> 下跌
    // 上升阶段：强买方压力
    for i in 0..10 {
        let base = 100.0 + i as f64 * 2.0;
        cmf.update(base + 5.0, base - 5.0, base + 4.0, 15000.0);
    }
    
    let rising_cmf = cmf.update(125.0, 115.0, 124.0, 15000.0).unwrap();
    assert!(rising_cmf > 0.0);
    
    // 顶部阶段：买方压力减弱
    for i in 0..5 {
        let base = 120.0 + i as f64;
        cmf.update(base + 5.0, base - 5.0, base, 8000.0);
    }
    
    let top_cmf = cmf.update(130.0, 120.0, 122.0, 5000.0).unwrap();
    
    // 下跌阶段：卖方压力增强
    for i in 0..5 {
        let base = 120.0 - i as f64 * 3.0;
        cmf.update(base + 5.0, base - 5.0, base - 3.0, 20000.0);
    }
    
    let falling_cmf = cmf.update(105.0, 95.0, 96.0, 25000.0).unwrap();
    assert!(falling_cmf < 0.0);
}

#[test]
#[should_panic(expected = "CMF period must be greater than 0")]
fn test_cmf_invalid_period() {
    CMF::new(0);
}
