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
fn test_vwap_new() {
    let vwap = VWAP::new(20);
    assert_eq!(vwap.period, 20);
    assert_eq!(vwap.count(), 0);
    assert!(!vwap.is_ready());
}

#[test]
fn test_vwap_cumulative_mode() {
    let vwap = VWAP::new(0);
    assert_eq!(vwap.period, 0);
}

#[test]
fn test_vwap_first_update() {
    let mut vwap = VWAP::new(20);

    // 第一个数据点就应该有结果
    let result = vwap.update(110.0, 90.0, 100.0, 1000.0);
    assert!(result.is_some());

    // 典型价格 = (110 + 90 + 100) / 3 = 100
    // VWAP = 100 × 1000 / 1000 = 100
    let vwap_value = result.unwrap();
    assert!((vwap_value - 100.0).abs() < 1e-10);
}

#[test]
fn test_vwap_calculation() {
    let mut vwap = VWAP::new(0); // 累积模式

    // 数据1: TP=100, Volume=1000, PV=100000
    vwap.update(110.0, 90.0, 100.0, 1000.0);

    // 数据2: TP=105, Volume=2000, PV=210000
    // VWAP = (100000 + 210000) / (1000 + 2000) = 310000 / 3000 = 103.33...
    let result = vwap.update(115.0, 95.0, 105.0, 2000.0);
    assert!(result.is_some());

    let vwap_value = result.unwrap();
    let expected = 310000.0 / 3000.0;
    assert!((vwap_value - expected).abs() < 1e-10);
}

#[test]
fn test_vwap_rolling_window() {
    let mut vwap = VWAP::new(3);

    // 添加3个数据点
    vwap.update(110.0, 90.0, 100.0, 1000.0);  // TP=100
    vwap.update(115.0, 95.0, 105.0, 1000.0);  // TP=105
    vwap.update(120.0, 100.0, 110.0, 1000.0); // TP=110

    assert_eq!(vwap.count(), 3);

    // 添加第4个数据点,第1个应该被移除
    vwap.update(125.0, 105.0, 115.0, 1000.0); // TP=115

    // 窗口大小应该保持为3
    assert_eq!(vwap.count(), 3);
}

#[test]
fn test_vwap_volume_weighted() {
    let mut vwap = VWAP::new(0);

    // 价格100,大成交量
    vwap.update(110.0, 90.0, 100.0, 10000.0);

    // 价格120,小成交量
    // VWAP应该更接近100而不是110(120和100的中点)
    let result = vwap.update(130.0, 110.0, 120.0, 1000.0);
    assert!(result.is_some());

    let vwap_value = result.unwrap();
    
    // VWAP = (100×10000 + 120×1000) / (10000+1000)
    //      = 1120000 / 11000 = 101.818...
    let expected = 1120000.0 / 11000.0;
    assert!((vwap_value - expected).abs() < 1e-10);
    
    // 应该更接近100
    assert!(vwap_value < 110.0);
}

#[test]
fn test_vwap_zero_volume() {
    let mut vwap = VWAP::new(0);

    // 只有零成交量
    let result = vwap.update(110.0, 90.0, 100.0, 0.0);
    
    // 成交量为0时应该返回None
    assert_eq!(result, None);
}

#[test]
fn test_vwap_mixed_volumes() {
    let mut vwap = VWAP::new(0);

    vwap.update(110.0, 90.0, 100.0, 1000.0);
    vwap.update(115.0, 95.0, 105.0, 0.0); // 零成交量
    
    let result = vwap.update(120.0, 100.0, 110.0, 2000.0);
    assert!(result.is_some());

    // 应该忽略零成交量的数据点
    // VWAP = (100×1000 + 110×2000) / (1000+2000)
    let expected = (100.0 * 1000.0 + 110.0 * 2000.0) / 3000.0;
    let vwap_value = result.unwrap();
    assert!((vwap_value - expected).abs() < 1e-10);
}

#[test]
fn test_vwap_reset() {
    let mut vwap = VWAP::new(20);

    vwap.update(110.0, 90.0, 100.0, 1000.0);
    assert!(vwap.is_ready());
    assert_eq!(vwap.count(), 1);

    vwap.reset();

    assert!(!vwap.is_ready());
    assert_eq!(vwap.count(), 0);
    assert_eq!(vwap.pv_sum, 0.0);
    assert_eq!(vwap.volume_sum, 0.0);
}

#[test]
fn test_vwap_is_above() {
    assert!(VWAP::is_above(105.0, 100.0));
    assert!(!VWAP::is_above(95.0, 100.0));
    assert!(!VWAP::is_above(100.0, 100.0));
}

#[test]
fn test_vwap_is_below() {
    assert!(VWAP::is_below(95.0, 100.0));
    assert!(!VWAP::is_below(105.0, 100.0));
    assert!(!VWAP::is_below(100.0, 100.0));
}

#[test]
fn test_vwap_deviation_percentage() {
    // 高于VWAP 5%
    let dev = VWAP::deviation_percentage(105.0, 100.0);
    assert_eq!(dev, 5.0);

    // 低于VWAP 5%
    let dev = VWAP::deviation_percentage(95.0, 100.0);
    assert_eq!(dev, -5.0);

    // 等于VWAP
    let dev = VWAP::deviation_percentage(100.0, 100.0);
    assert_eq!(dev, 0.0);

    // VWAP为0
    let dev = VWAP::deviation_percentage(100.0, 0.0);
    assert_eq!(dev, 0.0);
}

#[test]
fn test_vwap_uptrend() {
    let mut vwap = VWAP::new(0);

    // 模拟上涨趋势
    for i in 0..10 {
        let base = 100.0 + i as f64 * 5.0;
        vwap.update(base + 5.0, base - 5.0, base, 1000.0);
    }

    let result = vwap.update(155.0, 145.0, 150.0, 1000.0);
    assert!(result.is_some());

    // VWAP应该在价格范围内
    let vwap_value = result.unwrap();
    assert!(vwap_value > 100.0 && vwap_value < 150.0);
}

#[test]
fn test_vwap_downtrend() {
    let mut vwap = VWAP::new(0);

    // 模拟下跌趋势
    for i in 0..10 {
        let base = 150.0 - i as f64 * 5.0;
        vwap.update(base + 5.0, base - 5.0, base, 1000.0);
    }

    let result = vwap.update(105.0, 95.0, 100.0, 1000.0);
    assert!(result.is_some());

    // VWAP应该在价格范围内
    let vwap_value = result.unwrap();
    assert!(vwap_value > 100.0 && vwap_value < 150.0);
}

#[test]
fn test_vwap_typical_price() {
    let mut vwap = VWAP::new(0);

    // 典型价格计算: (120 + 80 + 100) / 3 = 100
    let result = vwap.update(120.0, 80.0, 100.0, 1000.0);
    assert!(result.is_some());

    let vwap_value = result.unwrap();
    assert_eq!(vwap_value, 100.0);
}

#[test]
fn test_vwap_large_volume_impact() {
    let mut vwap = VWAP::new(0);

    // 小成交量价格
    vwap.update(110.0, 90.0, 100.0, 100.0);

    // 大成交量高价
    let result = vwap.update(220.0, 180.0, 200.0, 10000.0);
    assert!(result.is_some());

    let vwap_value = result.unwrap();
    
    // VWAP应该被大成交量拉高,更接近200
    assert!(vwap_value > 190.0);
}

#[test]
fn test_vwap_cumulative_vs_rolling() {
    let mut vwap_cumulative = VWAP::new(0);
    let mut vwap_rolling = VWAP::new(3);

    // 添加相同的数据
    for i in 0..5 {
        let base = 100.0 + i as f64 * 10.0;
        vwap_cumulative.update(base + 5.0, base - 5.0, base, 1000.0);
        vwap_rolling.update(base + 5.0, base - 5.0, base, 1000.0);
    }

    // 累积模式应该有5个数据点
    assert_eq!(vwap_cumulative.count(), 5);
    
    // 滚动模式应该只有3个数据点
    assert_eq!(vwap_rolling.count(), 3);
}
