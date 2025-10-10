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
fn test_williams_r_new() {
    let wr = WilliamsR::new(14);
    assert_eq!(wr.period, 14);
    assert_eq!(wr.count(), 0);
    assert!(!wr.is_ready());
}

#[test]
#[should_panic(expected = "Williams %R周期必须大于0")]
fn test_williams_r_new_zero_period() {
    WilliamsR::new(0);
}

#[test]
fn test_williams_r_insufficient_data() {
    let mut wr = WilliamsR::new(14);

    // 前13个数据点应该返回None
    for i in 0..13 {
        assert_eq!(wr.update(110.0, 90.0, 100.0), None);
        assert_eq!(wr.count(), i + 1);
    }

    // 第14个数据点应该有结果
    assert!(wr.update(110.0, 90.0, 100.0).is_some());
}

#[test]
fn test_williams_r_at_high() {
    let mut wr = WilliamsR::new(5);

    // 建立一个价格范围
    for _ in 0..5 {
        wr.update(110.0, 90.0, 95.0);
    }

    // 收盘价在最高点
    let result = wr.update(110.0, 90.0, 110.0);
    assert!(result.is_some());

    // 当收盘价=最高价时,%R应该为0
    let wr_value = result.unwrap();
    assert!((wr_value - 0.0).abs() < 1e-10);
}

#[test]
fn test_williams_r_at_low() {
    let mut wr = WilliamsR::new(5);

    // 建立一个价格范围
    for _ in 0..5 {
        wr.update(110.0, 90.0, 105.0);
    }

    // 收盘价在最低点
    let result = wr.update(110.0, 90.0, 90.0);
    assert!(result.is_some());

    // 当收盘价=最低价时,%R应该为-100
    let wr_value = result.unwrap();
    assert!((wr_value - (-100.0)).abs() < 1e-10);
}

#[test]
fn test_williams_r_at_middle() {
    let mut wr = WilliamsR::new(5);

    // 价格范围: 90-110, 中点是100
    for _ in 0..5 {
        wr.update(110.0, 90.0, 100.0);
    }

    let result = wr.update(110.0, 90.0, 100.0);
    assert!(result.is_some());

    // 当收盘价在中点时,%R应该为-50
    let wr_value = result.unwrap();
    assert!((wr_value - (-50.0)).abs() < 1e-10);
}

#[test]
fn test_williams_r_overbought() {
    let mut wr = WilliamsR::new(5);

    // 建立基准
    for _ in 0..5 {
        wr.update(110.0, 90.0, 95.0);
    }

    // 价格接近最高点
    let result = wr.update(110.0, 90.0, 108.0);
    assert!(result.is_some());

    let wr_value = result.unwrap();
    // 应该在超买区(-20以上)
    assert!(
        WilliamsR::is_overbought(wr_value, -20.0),
        "Williams %R应该在超买区,实际值: {}",
        wr_value
    );
}

#[test]
fn test_williams_r_oversold() {
    let mut wr = WilliamsR::new(5);

    // 建立基准
    for _ in 0..5 {
        wr.update(110.0, 90.0, 105.0);
    }

    // 价格接近最低点
    let result = wr.update(110.0, 90.0, 92.0);
    assert!(result.is_some());

    let wr_value = result.unwrap();
    // 应该在超卖区(-80以下)
    assert!(
        WilliamsR::is_oversold(wr_value, -80.0),
        "Williams %R应该在超卖区,实际值: {}",
        wr_value
    );
}

#[test]
fn test_williams_r_zero_range() {
    let mut wr = WilliamsR::new(3);

    // 所有价格相同(无波动)
    for _ in 0..3 {
        wr.update(100.0, 100.0, 100.0);
    }

    let result = wr.update(100.0, 100.0, 100.0);
    assert!(result.is_some());

    // 当价格区间为0时,应该返回-50(中间值)
    assert_eq!(result.unwrap(), -50.0);
}

#[test]
fn test_williams_r_reset() {
    let mut wr = WilliamsR::new(5);

    for _ in 0..5 {
        wr.update(110.0, 90.0, 100.0);
    }

    assert!(wr.is_ready());
    assert_eq!(wr.count(), 5);

    wr.reset();

    assert!(!wr.is_ready());
    assert_eq!(wr.count(), 0);
}

#[test]
fn test_williams_r_sliding_window() {
    let mut wr = WilliamsR::new(5);

    // 填满窗口
    for i in 0..5 {
        wr.update(110.0, 90.0, 100.0 + i as f64);
    }

    assert_eq!(wr.count(), 5);

    // 继续添加数据
    wr.update(115.0, 85.0, 105.0);

    // 窗口应该保持大小
    assert_eq!(wr.count(), 5);
}

#[test]
fn test_williams_r_uptrend() {
    let mut wr = WilliamsR::new(10);

    // 模拟上涨趋势:价格不断创新高
    for i in 0..10 {
        let base = 100.0 + i as f64 * 2.0;
        wr.update(base + 5.0, base, base + 4.0);
    }

    // 继续上涨并接近高点
    let result = wr.update(125.0, 120.0, 124.0);
    assert!(result.is_some());

    // 在上涨趋势中接近高点时,应该在超买区
    let wr_value = result.unwrap();
    assert!(wr_value > -30.0, "上涨趋势应该有较高的%R值,实际值: {}", wr_value);
}

#[test]
fn test_williams_r_downtrend() {
    let mut wr = WilliamsR::new(10);

    // 模拟下跌趋势:价格不断创新低
    for i in 0..10 {
        let base = 120.0 - i as f64 * 2.0;
        wr.update(base, base - 5.0, base - 4.0);
    }

    // 继续下跌并接近低点
    let result = wr.update(102.0, 97.0, 98.0);
    assert!(result.is_some());

    // 在下跌趋势中接近低点时,应该在超卖区
    let wr_value = result.unwrap();
    assert!(wr_value < -70.0, "下跌趋势应该有较低的%R值,实际值: {}", wr_value);
}

#[test]
fn test_williams_r_oscillation() {
    let mut wr = WilliamsR::new(5);

    // 在范围内振荡
    let closes = vec![100.0, 105.0, 95.0, 103.0, 97.0];

    for close in closes {
        wr.update(110.0, 90.0, close);
    }

    // 振荡中的值应该在-20到-80之间
    let result = wr.update(110.0, 90.0, 100.0);
    assert!(result.is_some());

    let wr_value = result.unwrap();
    assert!(
        wr_value > -80.0 && wr_value < -20.0,
        "振荡市场%R应该在中间区域,实际值: {}",
        wr_value
    );
}

#[test]
fn test_williams_r_extreme_volatility() {
    let mut wr = WilliamsR::new(5);

    // 极端波动的市场
    let data = vec![
        (120.0, 80.0, 100.0),
        (130.0, 70.0, 110.0),
        (125.0, 75.0, 90.0),
        (135.0, 65.0, 120.0),
        (140.0, 60.0, 75.0),
    ];

    for (h, l, c) in data {
        wr.update(h, l, c);
    }

    let result = wr.update(145.0, 55.0, 100.0);
    assert!(result.is_some());

    // 验证值在有效范围内
    let wr_value = result.unwrap();
    assert!(
        wr_value >= -100.0 && wr_value <= 0.0,
        "Williams %R应该在-100到0之间,实际值: {}",
        wr_value
    );
}
