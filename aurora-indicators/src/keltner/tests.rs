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
fn test_keltner_channels_new() {
    let kc = KeltnerChannels::new(20, 2.0);
    assert_eq!(kc.period, 20);
    assert_eq!(kc.multiplier, 2.0);
    assert!(!kc.is_ready());
}

#[test]
fn test_keltner_channels_default() {
    let kc = KeltnerChannels::default();
    assert_eq!(kc.period, 20);
    assert_eq!(kc.multiplier, 2.0);
}

#[test]
#[should_panic(expected = "Keltner Channels周期必须大于0")]
fn test_keltner_channels_zero_period() {
    KeltnerChannels::new(0, 2.0);
}

#[test]
#[should_panic(expected = "ATR倍数不能为负数")]
fn test_keltner_channels_negative_multiplier() {
    KeltnerChannels::new(20, -1.0);
}

#[test]
fn test_keltner_channels_insufficient_data() {
    let mut kc = KeltnerChannels::new(20, 2.0);

    // Keltner Channels基于EMA和ATR
    // EMA和ATR都从第一个数据点就有输出
    // 所以Keltner Channels也从第一个数据点就有输出
    
    // 第一个数据点就应该有结果
    let result = kc.update(110.0, 90.0, 100.0);
    assert!(result.is_some(), "第一个数据点应该有结果");
    
    // 继续添加更多数据
    for _ in 1..20 {
        let result = kc.update(110.0, 90.0, 100.0);
        assert!(result.is_some());
    }
}

#[test]
fn test_keltner_channels_calculation() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 填充初始数据
    for _ in 0..10 {
        kc.update(105.0, 95.0, 100.0);
    }

    let result = kc.update(108.0, 92.0, 102.0);
    assert!(result.is_some());

    let channels = result.unwrap();
    
    // 验证基本结构
    assert!(channels.upper > channels.middle);
    assert!(channels.middle > channels.lower);
    
    // 验证对称性(由于ATR,可能不完全对称)
    let upper_distance = channels.upper - channels.middle;
    let lower_distance = channels.middle - channels.lower;
    assert!((upper_distance - lower_distance).abs() < 1e-10);
}

#[test]
fn test_keltner_channels_uptrend() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 模拟上涨趋势
    for i in 0..15 {
        let base = 100.0 + i as f64 * 2.0;
        kc.update(base + 5.0, base - 5.0, base);
    }

    let result = kc.update(135.0, 125.0, 130.0);
    assert!(result.is_some());

    let channels = result.unwrap();
    
    // 在上涨趋势中,中轨应该上升
    assert!(channels.middle > 100.0);
}

#[test]
fn test_keltner_channels_downtrend() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 模拟下跌趋势
    for i in 0..15 {
        let base = 130.0 - i as f64 * 2.0;
        kc.update(base + 5.0, base - 5.0, base);
    }

    let result = kc.update(105.0, 95.0, 100.0);
    assert!(result.is_some());

    let channels = result.unwrap();
    
    // 在下跌趋势中,中轨应该下降
    assert!(channels.middle < 130.0);
}

#[test]
fn test_keltner_channels_high_volatility() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 低波动初期
    for _ in 0..10 {
        kc.update(102.0, 98.0, 100.0);
    }

    let low_vol = kc.update(102.0, 98.0, 100.0).unwrap();
    let low_width = low_vol.upper - low_vol.lower;

    // 高波动期
    for _ in 0..5 {
        kc.update(120.0, 80.0, 100.0);
    }

    let high_vol = kc.update(125.0, 75.0, 100.0).unwrap();
    let high_width = high_vol.upper - high_vol.lower;

    // 高波动应该导致通道变宽
    assert!(high_width > low_width);
}

#[test]
fn test_keltner_channels_reset() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 添加足够数据
    for _ in 0..10 {
        kc.update(110.0, 90.0, 100.0);
    }

    assert!(kc.is_ready());

    kc.reset();

    assert!(!kc.is_ready());
    
    // reset后,第一个数据点就有结果
    let result = kc.update(110.0, 90.0, 100.0);
    assert!(result.is_some(), "reset后第一个数据点应该有结果");
    assert!(kc.is_ready());
}

#[test]
fn test_keltner_channels_width_percentage() {
    let output = KeltnerChannelsOutput {
        upper: 110.0,
        middle: 100.0,
        lower: 90.0,
    };

    let width_pct = KeltnerChannels::width_percentage(&output);
    
    // (110 - 90) / 100 * 100 = 20%
    assert!((width_pct - 20.0).abs() < 1e-10);
}

#[test]
fn test_keltner_channels_width_percentage_zero_middle() {
    let output = KeltnerChannelsOutput {
        upper: 10.0,
        middle: 0.0,
        lower: -10.0,
    };

    let width_pct = KeltnerChannels::width_percentage(&output);
    assert_eq!(width_pct, 0.0);
}

#[test]
fn test_keltner_channels_breakout_detection() {
    let output = KeltnerChannelsOutput {
        upper: 110.0,
        middle: 100.0,
        lower: 90.0,
    };

    // 测试突破上轨
    assert!(KeltnerChannels::is_above_upper(115.0, &output));
    assert!(!KeltnerChannels::is_above_upper(105.0, &output));

    // 测试突破下轨
    assert!(KeltnerChannels::is_below_lower(85.0, &output));
    assert!(!KeltnerChannels::is_below_lower(95.0, &output));
}

#[test]
fn test_keltner_channels_different_multipliers() {
    let mut kc1 = KeltnerChannels::new(10, 1.0);
    let mut kc2 = KeltnerChannels::new(10, 3.0);

    // 用相同数据填充两个指标
    for _ in 0..10 {
        kc1.update(110.0, 90.0, 100.0);
        kc2.update(110.0, 90.0, 100.0);
    }

    let result1 = kc1.update(112.0, 88.0, 102.0).unwrap();
    let result2 = kc2.update(112.0, 88.0, 102.0).unwrap();

    // 中轨应该相同
    assert!((result1.middle - result2.middle).abs() < 1e-10);

    // 倍数越大,通道越宽
    let width1 = result1.upper - result1.lower;
    let width2 = result2.upper - result2.lower;
    assert!(width2 > width1);
}

#[test]
fn test_keltner_channels_stable_market() {
    let mut kc = KeltnerChannels::new(10, 2.0);

    // 稳定市场
    for _ in 0..15 {
        kc.update(101.0, 99.0, 100.0);
    }

    let result = kc.update(101.0, 99.0, 100.0).unwrap();

    // 在稳定市场中,通道应该较窄
    let width = result.upper - result.lower;
    assert!(width < 10.0); // 相对于价格水平的窄通道
}

#[test]
fn test_keltner_channels_output_copy() {
    let output1 = KeltnerChannelsOutput {
        upper: 110.0,
        middle: 100.0,
        lower: 90.0,
    };

    let output2 = output1;

    assert_eq!(output1.upper, output2.upper);
    assert_eq!(output1.middle, output2.middle);
    assert_eq!(output1.lower, output2.lower);
}
