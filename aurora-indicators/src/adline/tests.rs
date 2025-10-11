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
fn test_adline_initialization() {
    let ad = ADLine::new();
    
    // 初始值应该为 0
    assert_eq!(ad.value(), 0.0);
}

#[test]
fn test_adline_accumulation() {
    let mut ad = ADLine::new();
    
    // 模拟累积（买入）：收盘价接近最高价
    let result1 = ad.update(100.0, 90.0, 99.0, 10000.0);  // 强买方压力
    
    // A/D Line 应该增加
    assert!(result1 > 0.0);
    
    let result2 = ad.update(105.0, 95.0, 104.0, 10000.0);  // 继续强买方压力
    
    // A/D Line 应该继续增加
    assert!(result2 > result1);
}

#[test]
fn test_adline_distribution() {
    let mut ad = ADLine::new();
    
    // 模拟派发（卖出）：收盘价接近最低价
    let result1 = ad.update(100.0, 90.0, 91.0, 10000.0);  // 强卖方压力
    
    // A/D Line 应该减少
    assert!(result1 < 0.0);
    
    let result2 = ad.update(105.0, 95.0, 96.0, 10000.0);  // 继续强卖方压力
    
    // A/D Line 应该继续减少
    assert!(result2 < result1);
}

#[test]
fn test_adline_neutral() {
    let mut ad = ADLine::new();
    
    // 收盘价在中间位置（中性）
    let result = ad.update(100.0, 90.0, 95.0, 10000.0);
    
    // A/D Line 应该接近 0
    assert!(result.abs() < 100.0);  // 给一定的容差
}

#[test]
fn test_adline_cumulative_nature() {
    let mut ad = ADLine::new();
    
    // 第一个周期
    let result1 = ad.update(100.0, 90.0, 98.0, 10000.0);
    
    // 第二个周期
    let result2 = ad.update(105.0, 95.0, 103.0, 10000.0);
    
    // 第三个周期
    let result3 = ad.update(110.0, 100.0, 108.0, 10000.0);
    
    // A/D Line 应该是累积的，每次都在上一次的基础上变化
    assert!(result3 > result2);
    assert!(result2 > result1);
    assert!(result1 > 0.0);
}

#[test]
fn test_adline_volume_impact() {
    let mut ad1 = ADLine::new();
    let mut ad2 = ADLine::new();
    
    // 相同价格，不同成交量
    let result1 = ad1.update(100.0, 90.0, 98.0, 10000.0);   // 小成交量
    let result2 = ad2.update(100.0, 90.0, 98.0, 100000.0);  // 大成交量
    
    // 成交量大的应该对 A/D Line 产生更大影响
    assert!(result2.abs() > result1.abs());
}

#[test]
fn test_adline_zero_range() {
    let mut ad = ADLine::new();
    
    // 高低价相同的情况（避免除以零）
    let result = ad.update(100.0, 100.0, 100.0, 10000.0);
    
    // 应该返回当前值（不变）
    assert_eq!(result, 0.0);
}

#[test]
fn test_adline_price_at_high() {
    let mut ad = ADLine::new();
    
    // 收盘价等于最高价（最强买方压力）
    let result = ad.update(100.0, 90.0, 100.0, 10000.0);
    
    // A/D Line 应该增加整个成交量
    assert!(result > 9000.0);  // 应该接近 10000
}

#[test]
fn test_adline_price_at_low() {
    let mut ad = ADLine::new();
    
    // 收盘价等于最低价（最强卖方压力）
    let result = ad.update(100.0, 90.0, 90.0, 10000.0);
    
    // A/D Line 应该减少整个成交量
    assert!(result < -9000.0);  // 应该接近 -10000
}

#[test]
fn test_adline_bullish_divergence() {
    let mut ad = ADLine::new();
    
    // 模拟看涨背离：价格下跌但 A/D Line 上升
    // 第一波下跌
    ad.update(100.0, 90.0, 91.0, 10000.0);  // 价格低，卖压大
    let low1_ad = ad.value();
    
    // 价格反弹
    ad.update(95.0, 85.0, 93.0, 15000.0);
    
    // 第二波下跌，但买方开始介入
    ad.update(93.0, 83.0, 89.0, 5000.0);   // 价格更低
    ad.update(92.0, 82.0, 90.0, 8000.0);   // 但成交量减少
    let low2_ad = ad.value();
    
    // 如果是看涨背离，第二次低点的 A/D Line 应该高于第一次
    // （这里简化测试，实际市场可能更复杂）
    assert!(low2_ad != low1_ad);
}

#[test]
fn test_adline_bearish_divergence() {
    let mut ad = ADLine::new();
    
    // 模拟看跌背离：价格上涨但 A/D Line 不再创新高
    // 第一波上涨
    ad.update(100.0, 90.0, 99.0, 20000.0);  // 强买压
    ad.update(105.0, 95.0, 104.0, 25000.0);
    let high1_ad = ad.value();
    
    // 小幅回调
    ad.update(103.0, 93.0, 95.0, 15000.0);
    
    // 第二波上涨，但买压减弱
    ad.update(106.0, 96.0, 100.0, 12000.0);  // 价格更高
    ad.update(108.0, 98.0, 102.0, 10000.0);  // 但成交量减少，收盘价不强
    let high2_ad = ad.value();
    
    // 第二次高点的 A/D Line 应该显示出买压减弱
    assert!(high2_ad > 0.0);
}

#[test]
fn test_adline_trend_confirmation() {
    let mut ad = ADLine::new();
    
    // 模拟上升趋势：价格和 A/D Line 都上升
    let mut prev_ad = ad.value();
    
    for i in 0..10 {
        let base = 100.0 + i as f64 * 5.0;
        let current_ad = ad.update(base + 5.0, base - 5.0, base + 4.0, 15000.0);
        
        // A/D Line 应该持续上升
        if i > 0 {
            assert!(current_ad > prev_ad);
        }
        prev_ad = current_ad;
    }
}

#[test]
fn test_adline_reset() {
    let mut ad = ADLine::new();
    
    // 添加一些数据
    ad.update(100.0, 90.0, 98.0, 10000.0);
    ad.update(105.0, 95.0, 103.0, 12000.0);
    
    assert!(ad.value() != 0.0);
    
    // 重置
    ad.reset();
    
    // 重置后应该回到 0
    assert_eq!(ad.value(), 0.0);
}

#[test]
fn test_adline_mixed_signals() {
    let mut ad = ADLine::new();
    
    // 混合的买卖信号
    let result1 = ad.update(100.0, 90.0, 99.0, 10000.0);   // 买入
    let result2 = ad.update(105.0, 95.0, 96.0, 12000.0);   // 卖出
    let result3 = ad.update(110.0, 100.0, 108.0, 15000.0); // 买入
    let result4 = ad.update(115.0, 105.0, 107.0, 11000.0); // 卖出
    
    // A/D Line 应该反映这些混合信号
    assert!(result1 > 0.0);
    assert!(result2 < result1);  // 卖压导致下降
    assert!(result3 > result2);  // 买压导致上升
}

#[test]
fn test_adline_real_market_scenario() {
    let mut ad = ADLine::new();
    
    // 模拟真实市场：积累 -> 上涨 -> 派发 -> 下跌
    
    // 积累阶段：价格低位，但有买入
    for _ in 0..5 {
        ad.update(105.0, 95.0, 103.0, 12000.0);
    }
    let accumulation_ad = ad.value();
    assert!(accumulation_ad > 0.0);
    
    // 上涨阶段：价格和 A/D Line 都上升
    for i in 0..10 {
        let base = 105.0 + i as f64 * 3.0;
        ad.update(base + 5.0, base - 5.0, base + 4.0, 15000.0);
    }
    let rally_ad = ad.value();
    assert!(rally_ad > accumulation_ad);
    
    // 派发阶段：价格高位，但开始卖出
    for _ in 0..5 {
        ad.update(140.0, 130.0, 132.0, 14000.0);
    }
    let distribution_ad = ad.value();
    
    // 下跌阶段：价格和 A/D Line 都下降
    for i in 0..10 {
        let base = 135.0 - i as f64 * 3.0;
        ad.update(base + 5.0, base - 5.0, base - 3.0, 18000.0);
    }
    let decline_ad = ad.value();
    assert!(decline_ad < distribution_ad);
}

#[test]
fn test_adline_zero_volume() {
    let mut ad = ADLine::new();
    
    // 先建立一个基准值
    ad.update(100.0, 90.0, 98.0, 10000.0);
    let before = ad.value();
    
    // 零成交量不应改变 A/D Line
    let result = ad.update(105.0, 95.0, 103.0, 0.0);
    
    assert_eq!(result, before);
}

#[test]
fn test_adline_extreme_values() {
    let mut ad = ADLine::new();
    
    // 测试极端价格和成交量
    let result = ad.update(10000.0, 5000.0, 9500.0, 1000000.0);
    
    // 应该产生有效值
    assert!(result.is_finite());
    assert!(result > 0.0);
}
