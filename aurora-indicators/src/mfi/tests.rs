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
fn test_mfi_new() {
    let mfi = MFI::new(14);
    assert_eq!(mfi.period, 14);
    assert_eq!(mfi.count(), 0);
    assert!(!mfi.is_ready());
}

#[test]
fn test_mfi_default() {
    let mfi = MFI::default();
    assert_eq!(mfi.period, 14);
}

#[test]
#[should_panic(expected = "MFI周期必须大于0")]
fn test_mfi_zero_period() {
    MFI::new(0);
}

#[test]
fn test_mfi_insufficient_data() {
    let mut mfi = MFI::new(14);

    // 第一个数据点返回None
    assert_eq!(mfi.update(110.0, 90.0, 100.0, 1000.0), None);

    // 接下来13个数据点也返回None
    for i in 0..13 {
        assert_eq!(mfi.update(110.0, 90.0, 100.0, 1000.0), None);
        assert_eq!(mfi.count(), i + 1);
    }

    // 第15个数据点应该有结果
    assert!(mfi.update(110.0, 90.0, 100.0, 1000.0).is_some());
}

#[test]
fn test_mfi_all_positive_flow() {
    let mut mfi = MFI::new(5);

    // 第一个数据点
    mfi.update(100.0, 90.0, 95.0, 1000.0);

    // 后续价格持续上涨
    for i in 1..=5 {
        mfi.update(100.0 + i as f64, 90.0 + i as f64, 95.0 + i as f64, 1000.0);
    }

    let result = mfi.update(110.0, 100.0, 105.0, 1000.0);
    assert!(result.is_some());

    // 全部正资金流量,MFI应该接近100
    let mfi_value = result.unwrap();
    assert_eq!(mfi_value, 100.0);
}

#[test]
fn test_mfi_all_negative_flow() {
    let mut mfi = MFI::new(5);

    // 第一个数据点
    mfi.update(110.0, 100.0, 105.0, 1000.0);

    // 后续价格持续下跌
    for i in 1..=5 {
        mfi.update(110.0 - i as f64, 100.0 - i as f64, 105.0 - i as f64, 1000.0);
    }

    let result = mfi.update(100.0, 90.0, 95.0, 1000.0);
    assert!(result.is_some());

    // 全部负资金流量,MFI应该接近0
    let mfi_value = result.unwrap();
    assert!(mfi_value < 1.0, "MFI应该接近0,实际值: {}", mfi_value);
}

#[test]
fn test_mfi_overbought() {
    let mut mfi = MFI::new(5);

    // 建立基准
    mfi.update(100.0, 90.0, 95.0, 1000.0);

    // 快速上涨
    for i in 1..=6 {
        mfi.update(100.0 + i as f64 * 3.0, 90.0 + i as f64 * 3.0, 95.0 + i as f64 * 3.0, 1500.0);
    }

    let result = mfi.update(130.0, 120.0, 125.0, 2000.0);
    assert!(result.is_some());

    let mfi_value = result.unwrap();
    assert!(
        MFI::is_overbought(mfi_value),
        "MFI应该超买,实际值: {}",
        mfi_value
    );
}

#[test]
fn test_mfi_oversold() {
    let mut mfi = MFI::new(5);

    // 建立基准
    mfi.update(130.0, 120.0, 125.0, 2000.0);

    // 快速下跌
    for i in 1..=6 {
        mfi.update(130.0 - i as f64 * 3.0, 120.0 - i as f64 * 3.0, 125.0 - i as f64 * 3.0, 1500.0);
    }

    let result = mfi.update(100.0, 90.0, 95.0, 1000.0);
    assert!(result.is_some());

    let mfi_value = result.unwrap();
    assert!(
        MFI::is_oversold(mfi_value),
        "MFI应该超卖,实际值: {}",
        mfi_value
    );
}

#[test]
fn test_mfi_with_volume_impact() {
    let mut mfi1 = MFI::new(5);
    let mut mfi2 = MFI::new(5);

    // 相同的价格变动
    mfi1.update(100.0, 90.0, 95.0, 1000.0);
    mfi2.update(100.0, 90.0, 95.0, 1000.0);

    for i in 1..=5 {
        let price = 95.0 + i as f64 * 2.0;
        // mfi1使用小成交量
        mfi1.update(price + 5.0, price - 5.0, price, 500.0);
        // mfi2使用大成交量
        mfi2.update(price + 5.0, price - 5.0, price, 2000.0);
    }

    // 由于资金流量 = 典型价格 × 成交量,两者应该不同
    // 但由于是比率计算,可能相同或接近
    let result1 = mfi1.update(110.0, 100.0, 105.0, 500.0);
    let result2 = mfi2.update(110.0, 100.0, 105.0, 2000.0);

    assert!(result1.is_some());
    assert!(result2.is_some());
}

#[test]
fn test_mfi_reset() {
    let mut mfi = MFI::new(5);

    for _ in 0..6 {
        mfi.update(110.0, 90.0, 100.0, 1000.0);
    }

    assert!(mfi.is_ready());
    assert!(mfi.count() > 0);

    mfi.reset();

    assert!(!mfi.is_ready());
    assert_eq!(mfi.count(), 0);
    assert_eq!(mfi.prev_typical_price, None);
}

#[test]
fn test_mfi_price_unchanged() {
    let mut mfi = MFI::new(5);

    // 第一个数据点
    mfi.update(100.0, 90.0, 95.0, 1000.0);

    // 价格不变
    for _ in 0..5 {
        mfi.update(100.0, 90.0, 95.0, 1000.0);
    }

    let result = mfi.update(100.0, 90.0, 95.0, 1000.0);
    assert!(result.is_some());

    // 价格不变时,应该是50(中性)
    let mfi_value = result.unwrap();
    assert!((mfi_value - 50.0).abs() < 1e-10);
}

#[test]
fn test_mfi_mixed_signals() {
    let mut mfi = MFI::new(5);

    // 建立基准
    mfi.update(100.0, 90.0, 95.0, 1000.0);

    // 交替上涨下跌
    let changes = vec![2.0, -1.0, 3.0, -2.0, 1.0];
    let mut base = 95.0;

    for change in changes {
        base += change;
        mfi.update(base + 5.0, base - 5.0, base, 1000.0);
    }

    let result = mfi.update(105.0, 95.0, 100.0, 1000.0);
    assert!(result.is_some());

    let mfi_value = result.unwrap();
    // 混合信号应该产生中间值
    assert!(mfi_value > 30.0 && mfi_value < 70.0);
}

#[test]
fn test_mfi_sliding_window() {
    let mut mfi = MFI::new(5);

    // 填满窗口
    mfi.update(100.0, 90.0, 95.0, 1000.0);
    for i in 0..5 {
        mfi.update(100.0 + i as f64, 90.0 + i as f64, 95.0 + i as f64, 1000.0);
    }

    assert_eq!(mfi.count(), 5);

    // 继续添加数据
    mfi.update(110.0, 100.0, 105.0, 1000.0);

    // 窗口应该保持大小
    assert_eq!(mfi.count(), 5);
}

#[test]
fn test_mfi_typical_price_calculation() {
    let mut mfi = MFI::new(3);

    // 典型价格 = (高 + 低 + 收) / 3
    // (110 + 90 + 100) / 3 = 100
    mfi.update(110.0, 90.0, 100.0, 1000.0);
    
    // 上涨到典型价格105
    mfi.update(115.0, 95.0, 105.0, 1000.0);
    
    // 继续上涨
    mfi.update(120.0, 100.0, 110.0, 1000.0);
    
    let result = mfi.update(125.0, 105.0, 115.0, 1000.0);
    assert!(result.is_some());

    // 持续上涨应该产生高MFI
    let mfi_value = result.unwrap();
    assert!(mfi_value > 50.0);
}

#[test]
fn test_mfi_zero_volume() {
    let mut mfi = MFI::new(3);

    mfi.update(100.0, 90.0, 95.0, 1000.0);
    mfi.update(105.0, 95.0, 100.0, 0.0); // 零成交量
    mfi.update(110.0, 100.0, 105.0, 1000.0);
    
    let result = mfi.update(115.0, 105.0, 110.0, 1000.0);
    assert!(result.is_some());

    // 零成交量不应该导致崩溃
    let mfi_value = result.unwrap();
    assert!(mfi_value >= 0.0 && mfi_value <= 100.0);
}
