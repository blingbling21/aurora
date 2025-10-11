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
fn test_ichimoku_initialization() {
    let mut ichimoku = Ichimoku::default();
    
    // 前51个数据点不应产生结果（需要52个数据点）
    for i in 0..51 {
        let result = ichimoku.update(100.0 + i as f64, 95.0 + i as f64, 98.0 + i as f64);
        assert!(result.is_none());
    }
    
    // 第52个数据点应该产生结果
    let result = ichimoku.update(151.0, 146.0, 149.0);
    assert!(result.is_some());
}

#[test]
fn test_ichimoku_basic_calculation() {
    let mut ichimoku = Ichimoku::default();
    
    // 输入52个稳定的数据点
    for _ in 0..52 {
        ichimoku.update(110.0, 90.0, 100.0);
    }
    
    let result = ichimoku.update(110.0, 90.0, 100.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 在稳定价格下，转换线和基准线应该相等
    assert_eq!(output.tenkan_sen, 100.0);  // (110 + 90) / 2
    assert_eq!(output.kijun_sen, 100.0);
    assert_eq!(output.senkou_span_a, 100.0);  // (100 + 100) / 2
    assert_eq!(output.senkou_span_b, 100.0);
    assert_eq!(output.chikou_span, 100.0);
}

#[test]
fn test_ichimoku_uptrend() {
    let mut ichimoku = Ichimoku::default();
    
    // 模拟上升趋势
    for i in 0..60 {
        let base = 100.0 + i as f64 * 0.5;
        ichimoku.update(base + 5.0, base - 5.0, base);
    }
    
    let result = ichimoku.update(130.0, 120.0, 125.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 在上升趋势中，转换线通常高于基准线
    assert!(output.tenkan_sen >= output.kijun_sen);
}

#[test]
fn test_ichimoku_downtrend() {
    let mut ichimoku = Ichimoku::default();
    
    // 先建立高位
    for _ in 0..52 {
        ichimoku.update(200.0, 180.0, 190.0);
    }
    
    // 然后模拟下降趋势
    for i in 0..20 {
        let base = 190.0 - i as f64 * 2.0;
        ichimoku.update(base + 5.0, base - 5.0, base);
    }
    
    let result = ichimoku.update(155.0, 145.0, 150.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 在下降趋势中，转换线通常低于基准线
    assert!(output.tenkan_sen <= output.kijun_sen);
}

#[test]
fn test_ichimoku_cloud_position() {
    let mut ichimoku = Ichimoku::default();
    
    // 建立基础数据
    for _ in 0..52 {
        ichimoku.update(110.0, 90.0, 100.0);
    }
    
    // 测试价格在云上
    let (position, thickness) = ichimoku.get_trend_position(120.0).unwrap();
    assert_eq!(position, 1);  // 云上
    assert!(thickness >= 0.0);
    
    // 测试价格在云下
    let (position, _) = ichimoku.get_trend_position(80.0).unwrap();
    assert_eq!(position, -1);  // 云下
    
    // 测试价格在云中
    let (position, _) = ichimoku.get_trend_position(100.0).unwrap();
    assert!(position >= -1 && position <= 1);
}

#[test]
fn test_ichimoku_custom_parameters() {
    let mut ichimoku1 = Ichimoku::new(9, 26, 52);   // 标准参数
    let mut ichimoku2 = Ichimoku::new(7, 22, 44);   // 更快的参数
    
    // 输入相同的数据
    for i in 0..60 {
        let base = 100.0 + i as f64 * 0.3;
        ichimoku1.update(base + 5.0, base - 5.0, base);
        ichimoku2.update(base + 5.0, base - 5.0, base);
    }
    
    let result1 = ichimoku1.update(118.0, 108.0, 113.0);
    let result2 = ichimoku2.update(118.0, 108.0, 113.0);
    
    assert!(result1.is_some());
    assert!(result2.is_some());
    
    // 不同参数应该产生不同的结果
    let output1 = result1.unwrap();
    let output2 = result2.unwrap();
    
    // 滞后线相同（都是收盘价）
    assert_eq!(output1.chikou_span, output2.chikou_span);
}

#[test]
fn test_ichimoku_senkou_spans() {
    let mut ichimoku = Ichimoku::default();
    
    // 建立上升趋势
    for i in 0..52 {
        let base = 100.0 + i as f64;
        ichimoku.update(base + 5.0, base - 5.0, base);
    }
    
    let result = ichimoku.update(157.0, 147.0, 152.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 先行带A是转换线和基准线的平均值
    let expected_span_a = (output.tenkan_sen + output.kijun_sen) / 2.0;
    assert!((output.senkou_span_a - expected_span_a).abs() < 0.001);
    
    // 先行带B应该是有效值
    assert!(output.senkou_span_b > 0.0);
    assert!(output.senkou_span_b.is_finite());
}

#[test]
fn test_ichimoku_chikou_span() {
    let mut ichimoku = Ichimoku::default();
    
    for i in 0..52 {
        ichimoku.update(110.0, 90.0, 100.0 + i as f64 * 0.1);
    }
    
    let close_price = 105.1;
    let result = ichimoku.update(115.0, 95.0, close_price);
    
    assert!(result.is_some());
    let output = result.unwrap();
    
    // 滞后线应该等于当前收盘价
    assert_eq!(output.chikou_span, close_price);
}

#[test]
fn test_ichimoku_reset() {
    let mut ichimoku = Ichimoku::default();
    
    // 添加足够的数据
    for i in 0..60 {
        ichimoku.update(100.0 + i as f64, 95.0 + i as f64, 98.0 + i as f64);
    }
    
    // 重置
    ichimoku.reset();
    
    // 重置后应该需要重新积累数据
    let result = ichimoku.update(100.0, 95.0, 98.0);
    assert!(result.is_none());
}

#[test]
fn test_ichimoku_volatility_expansion() {
    let mut ichimoku = Ichimoku::default();
    
    // 先是低波动率
    for _ in 0..30 {
        ichimoku.update(102.0, 98.0, 100.0);
    }
    
    let result_low_vol = ichimoku.update(102.0, 98.0, 100.0);
    let low_vol_thickness = if let Some(output) = result_low_vol {
        (output.senkou_span_a - output.senkou_span_b).abs()
    } else {
        0.0
    };
    
    // 然后波动率扩大
    for i in 0..30 {
        let volatility = (i + 1) as f64 * 2.0;
        ichimoku.update(100.0 + volatility, 100.0 - volatility, 100.0);
    }
    
    let result = ichimoku.update(160.0, 40.0, 100.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 云层厚度应该增加
    let cloud_thickness = (output.senkou_span_a - output.senkou_span_b).abs();
    // 高波动率时的云层厚度应该大于低波动率时
    assert!(cloud_thickness >= low_vol_thickness);
    assert!(cloud_thickness >= 0.0);
}

#[test]
fn test_ichimoku_real_market_scenario() {
    let mut ichimoku = Ichimoku::default();
    
    // 模拟真实市场：盘整 -> 突破 -> 上升
    // 盘整期
    for _ in 0..26 {
        ichimoku.update(105.0, 95.0, 100.0);
    }
    
    // 突破期
    for i in 0..13 {
        let base = 100.0 + i as f64 * 2.0;
        ichimoku.update(base + 5.0, base - 5.0, base);
    }
    
    // 持续上升
    for i in 0..20 {
        let base = 126.0 + i as f64 * 1.0;
        ichimoku.update(base + 5.0, base - 5.0, base);
    }
    
    let result = ichimoku.update(151.0, 141.0, 146.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 在上升趋势后期，转换线应该在基准线上方
    assert!(output.tenkan_sen > output.kijun_sen);
    
    // 价格应该在云层上方
    let (position, _) = ichimoku.get_trend_position(146.0).unwrap();
    assert!(position >= 0);
}

#[test]
fn test_ichimoku_extreme_values() {
    let mut ichimoku = Ichimoku::default();
    
    // 测试极端价格
    for i in 0..52 {
        ichimoku.update(1000.0 * (i + 1) as f64, 500.0 * (i + 1) as f64, 750.0 * (i + 1) as f64);
    }
    
    let result = ichimoku.update(53000.0, 26500.0, 39750.0);
    assert!(result.is_some());
    
    let output = result.unwrap();
    
    // 所有值应该是有限的正数
    assert!(output.tenkan_sen.is_finite() && output.tenkan_sen > 0.0);
    assert!(output.kijun_sen.is_finite() && output.kijun_sen > 0.0);
    assert!(output.senkou_span_a.is_finite() && output.senkou_span_a > 0.0);
    assert!(output.senkou_span_b.is_finite() && output.senkou_span_b > 0.0);
    assert!(output.chikou_span.is_finite() && output.chikou_span > 0.0);
}
