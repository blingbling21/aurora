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
fn test_cci_new() {
    let cci = CCI::new(20);
    assert_eq!(cci.period, 20);
    assert_eq!(cci.count(), 0);
    assert!(!cci.is_ready());
}

#[test]
#[should_panic(expected = "CCI周期必须大于0")]
fn test_cci_new_zero_period() {
    CCI::new(0);
}

#[test]
fn test_cci_insufficient_data() {
    let mut cci = CCI::new(20);

    // 前19个数据点应该返回None
    for i in 0..19 {
        assert_eq!(cci.update(105.0, 95.0, 100.0), None);
        assert_eq!(cci.count(), i + 1);
    }

    // 第20个数据点应该有结果
    assert!(cci.update(105.0, 95.0, 100.0).is_some());
}

#[test]
fn test_cci_stable_price() {
    let mut cci = CCI::new(10);

    // 如果价格稳定,CCI应该接近0
    for _ in 0..10 {
        cci.update(100.0, 100.0, 100.0);
    }

    let result = cci.update(100.0, 100.0, 100.0);
    assert!(result.is_some());
    // 当所有价格相同时,平均偏差为0,应返回0
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_cci_overbought() {
    let mut cci = CCI::new(10);

    // 先建立一个基准
    for _ in 0..10 {
        cci.update(102.0, 98.0, 100.0);
    }

    // 然后突然价格上涨
    let result = cci.update(115.0, 105.0, 110.0);
    assert!(result.is_some());
    
    // CCI应该显著为正(超买)
    let cci_value = result.unwrap();
    assert!(cci_value > 100.0, "CCI应该 > 100,实际值: {}", cci_value);
}

#[test]
fn test_cci_oversold() {
    let mut cci = CCI::new(10);

    // 先建立一个基准
    for _ in 0..10 {
        cci.update(102.0, 98.0, 100.0);
    }

    // 然后突然价格下跌
    let result = cci.update(95.0, 85.0, 90.0);
    assert!(result.is_some());
    
    // CCI应该显著为负(超卖)
    let cci_value = result.unwrap();
    assert!(cci_value < -100.0, "CCI应该 < -100,实际值: {}", cci_value);
}

#[test]
fn test_cci_typical_price_calculation() {
    let mut cci = CCI::new(5);

    // 典型价格 = (高 + 低 + 收) / 3
    // (110 + 90 + 100) / 3 = 100
    for _ in 0..5 {
        cci.update(110.0, 90.0, 100.0);
    }

    // 验证典型价格被正确计算
    assert_eq!(cci.typical_prices.len(), 5);
    for &tp in &cci.typical_prices {
        assert!((tp - 100.0).abs() < 1e-10);
    }
}

#[test]
fn test_cci_reset() {
    let mut cci = CCI::new(10);

    for _ in 0..10 {
        cci.update(105.0, 95.0, 100.0);
    }

    assert!(cci.is_ready());
    assert_eq!(cci.count(), 10);

    cci.reset();

    assert!(!cci.is_ready());
    assert_eq!(cci.count(), 0);
    assert_eq!(cci.tp_sum, 0.0);
}

#[test]
fn test_cci_sliding_window() {
    let mut cci = CCI::new(5);

    // 填满窗口
    for i in 0..5 {
        cci.update(100.0 + i as f64, 98.0, 99.0);
    }

    assert_eq!(cci.count(), 5);

    // 继续添加数据
    cci.update(110.0, 108.0, 109.0);

    // 窗口应该保持大小
    assert_eq!(cci.count(), 5);
}

#[test]
fn test_cci_uptrend() {
    let mut cci = CCI::new(10);

    // 模拟逐步上涨的趋势
    for i in 0..10 {
        let base = 100.0 + i as f64 * 2.0;
        cci.update(base + 2.0, base - 2.0, base);
    }

    // 继续上涨
    let result = cci.update(125.0, 121.0, 123.0);
    assert!(result.is_some());

    // 在上涨趋势中,CCI应该为正
    let cci_value = result.unwrap();
    assert!(cci_value > 0.0, "上涨趋势CCI应该 > 0,实际值: {}", cci_value);
}

#[test]
fn test_cci_downtrend() {
    let mut cci = CCI::new(10);

    // 模拟逐步下跌的趋势
    for i in 0..10 {
        let base = 120.0 - i as f64 * 2.0;
        cci.update(base + 2.0, base - 2.0, base);
    }

    // 继续下跌
    let result = cci.update(98.0, 94.0, 96.0);
    assert!(result.is_some());

    // 在下跌趋势中,CCI应该为负
    let cci_value = result.unwrap();
    assert!(cci_value < 0.0, "下跌趋势CCI应该 < 0,实际值: {}", cci_value);
}

#[test]
fn test_cci_range_bound() {
    let mut cci = CCI::new(10);

    // 模拟在范围内波动的市场
    let prices = vec![100.0, 105.0, 95.0, 102.0, 98.0, 103.0, 97.0, 101.0, 99.0, 100.0];
    
    for price in prices {
        cci.update(price + 2.0, price - 2.0, price);
    }

    let result = cci.update(102.0, 98.0, 100.0);
    assert!(result.is_some());

    // 在范围波动的市场中,CCI应该接近0
    let cci_value = result.unwrap();
    assert!(
        cci_value.abs() < 50.0,
        "范围波动市场CCI应该接近0,实际值: {}",
        cci_value
    );
}
