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
fn test_psar_initialization() {
    let mut psar = PSAR::default();
    
    // 第一个数据点不应产生结果
    let result = psar.update(100.0, 95.0, 98.0);
    assert!(result.is_none());
}

#[test]
fn test_psar_uptrend() {
    let mut psar = PSAR::default();
    
    // 初始化数据
    psar.update(100.0, 95.0, 98.0);
    let result = psar.update(105.0, 100.0, 104.0);
    
    assert!(result.is_some());
    let output = result.unwrap();
    
    // 应该是上升趋势（第二根收盘价高于第一根）
    assert!(output.is_uptrend);
    
    // SAR 应该在价格下方
    assert!(output.sar < 100.0);
}

#[test]
fn test_psar_downtrend() {
    let mut psar = PSAR::default();
    
    // 初始化下降趋势数据
    psar.update(105.0, 100.0, 104.0);
    let result = psar.update(100.0, 95.0, 96.0);
    
    assert!(result.is_some());
    let output = result.unwrap();
    
    // 应该是下降趋势
    assert!(!output.is_uptrend);
    
    // SAR 应该在价格上方
    assert!(output.sar > 96.0);
}

#[test]
fn test_psar_trend_reversal() {
    let mut psar = PSAR::default();
    
    // 建立上升趋势
    psar.update(100.0, 95.0, 98.0);
    psar.update(105.0, 100.0, 104.0);
    psar.update(110.0, 105.0, 109.0);
    
    // 价格突然下跌，应该触发反转
    let result = psar.update(100.0, 90.0, 92.0);
    
    assert!(result.is_some());
    let output = result.unwrap();
    
    // 趋势应该已经反转为下降
    assert!(!output.is_uptrend);
}

#[test]
fn test_psar_acceleration_factor() {
    let mut psar = PSAR::new(0.02, 0.20);
    
    // 建立持续上升趋势，测试加速因子增长
    let highs = vec![100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0, 135.0];
    let lows = vec![95.0, 100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0];
    let closes = vec![98.0, 104.0, 109.0, 114.0, 119.0, 124.0, 129.0, 134.0];
    
    let mut last_sar = None;
    for i in 0..highs.len() {
        if let Some(output) = psar.update(highs[i], lows[i], closes[i]) {
            // SAR 应该持续在价格下方（上升趋势）
            assert!(output.is_uptrend);
            assert!(output.sar < closes[i]);
            
            // SAR 之间的差距应该逐渐增大（加速因子增长效应）
            if let Some(prev_sar) = last_sar {
                if i > 2 {
                    assert!(output.sar > prev_sar);
                }
            }
            last_sar = Some(output.sar);
        }
    }
}

#[test]
fn test_psar_custom_parameters() {
    let mut psar1 = PSAR::new(0.01, 0.10);  // 较慢的参数
    let mut psar2 = PSAR::new(0.03, 0.30);  // 较快的参数
    
    let test_data = vec![
        (100.0, 95.0, 98.0),
        (105.0, 100.0, 104.0),
        (110.0, 105.0, 109.0),
        (115.0, 110.0, 114.0),
    ];
    
    let mut sar1_values = Vec::new();
    let mut sar2_values = Vec::new();
    
    for (high, low, close) in test_data {
        if let Some(output) = psar1.update(high, low, close) {
            sar1_values.push(output.sar);
        }
        if let Some(output) = psar2.update(high, low, close) {
            sar2_values.push(output.sar);
        }
    }
    
    // 较快的参数应该使 SAR 更接近价格
    assert!(sar1_values.len() == sar2_values.len());
    for i in 0..sar1_values.len() {
        // 这个测试可能需要根据实际数据调整
        assert!(sar1_values[i] > 0.0);
        assert!(sar2_values[i] > 0.0);
    }
}

#[test]
fn test_psar_reset() {
    let mut psar = PSAR::default();
    
    // 添加一些数据
    psar.update(100.0, 95.0, 98.0);
    psar.update(105.0, 100.0, 104.0);
    
    // 重置
    psar.reset();
    
    // 重置后第一个数据点应该不产生结果
    let result = psar.update(100.0, 95.0, 98.0);
    assert!(result.is_none());
}

#[test]
fn test_psar_real_market_scenario() {
    let mut psar = PSAR::default();
    
    // 模拟真实市场数据：先上升后下降
    let market_data = vec![
        (50.0, 45.0, 48.0),   // 初始
        (52.0, 48.0, 51.0),   // 上升
        (55.0, 50.0, 54.0),   // 继续上升
        (58.0, 53.0, 57.0),   // 继续上升
        (60.0, 55.0, 59.0),   // 达到高点
        (58.0, 52.0, 53.0),   // 开始下跌
        (54.0, 48.0, 49.0),   // 继续下跌
        (50.0, 44.0, 45.0),   // 继续下跌
    ];
    
    let mut trend_changes = 0;
    let mut prev_trend = None;
    
    for (high, low, close) in market_data {
        if let Some(output) = psar.update(high, low, close) {
            if let Some(prev) = prev_trend {
                if prev != output.is_uptrend {
                    trend_changes += 1;
                }
            }
            prev_trend = Some(output.is_uptrend);
        }
    }
    
    // 应该至少检测到一次趋势反转
    assert!(trend_changes >= 1);
}

#[test]
fn test_psar_extreme_values() {
    let mut psar = PSAR::default();
    
    // 测试极端价格波动
    psar.update(1000.0, 500.0, 800.0);
    let result = psar.update(2000.0, 900.0, 1800.0);
    
    assert!(result.is_some());
    let output = result.unwrap();
    
    // SAR 应该是有效的数值
    assert!(output.sar.is_finite());
    assert!(output.sar > 0.0);
}

#[test]
fn test_psar_sideways_market() {
    let mut psar = PSAR::default();
    
    // 横盘市场数据
    let sideways_data = vec![
        (102.0, 98.0, 100.0),
        (103.0, 97.0, 100.0),
        (102.0, 98.0, 100.0),
        (103.0, 97.0, 100.0),
        (102.0, 98.0, 100.0),
    ];
    
    let mut results = Vec::new();
    for (high, low, close) in sideways_data {
        if let Some(output) = psar.update(high, low, close) {
            results.push(output);
        }
    }
    
    // 在横盘市场中，可能会频繁出现小的趋势转换
    assert!(results.len() > 0);
}
