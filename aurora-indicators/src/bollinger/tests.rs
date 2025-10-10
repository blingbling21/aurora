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

/// 测试布林带创建
#[test]
fn test_bollinger_creation() {
    let bb = BollingerBands::new(20, 2.0);
    assert_eq!(bb.period(), 20);
    assert_relative_eq!(bb.std_dev_multiplier(), 2.0, epsilon = 1e-10);
    assert!(bb.is_empty());
    assert!(!bb.is_ready());
}

/// 测试默认参数
#[test]
fn test_bollinger_default() {
    let bb = BollingerBands::default();
    assert_eq!(bb.period(), 20);
    assert_relative_eq!(bb.std_dev_multiplier(), 2.0, epsilon = 1e-10);
}

/// 测试周期为0时的panic
#[test]
#[should_panic(expected = "布林带周期必须大于0")]
fn test_bollinger_zero_period_panic() {
    BollingerBands::new(0, 2.0);
}

/// 测试标准差倍数为0时的panic
#[test]
#[should_panic(expected = "标准差倍数必须大于0")]
fn test_bollinger_zero_multiplier_panic() {
    BollingerBands::new(20, 0.0);
}

/// 测试数据不足时的行为
#[test]
fn test_bollinger_insufficient_data() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    assert_eq!(bb.update(100.0), None);
    assert_eq!(bb.update(102.0), None);
    assert!(!bb.is_ready());
}

/// 测试基本计算
#[test]
fn test_bollinger_basic_calculation() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(20.0);
    let result = bb.update(30.0);
    
    assert!(result.is_some());
    let bands = result.unwrap();
    
    // 中轨应该是平均值: (10+20+30)/3 = 20
    assert_relative_eq!(bands.middle, 20.0, epsilon = 1e-10);
    
    // 上轨应该大于中轨
    assert!(bands.upper > bands.middle);
    
    // 下轨应该小于中轨
    assert!(bands.lower < bands.middle);
}

/// 测试价格不变时的布林带
#[test]
fn test_bollinger_no_volatility() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(100.0);
    bb.update(100.0);
    let result = bb.update(100.0);
    
    assert!(result.is_some());
    let bands = result.unwrap();
    
    // 价格不变时，标准差为0
    assert_relative_eq!(bands.middle, 100.0, epsilon = 1e-10);
    assert_relative_eq!(bands.upper, 100.0, epsilon = 1e-10);
    assert_relative_eq!(bands.lower, 100.0, epsilon = 1e-10);
}

/// 测试高波动性的情况
#[test]
fn test_bollinger_high_volatility() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(30.0);
    let result = bb.update(50.0);
    
    assert!(result.is_some());
    let bands = result.unwrap();
    
    // 高波动性时，上下轨距离应该较大
    let bandwidth = bands.upper - bands.lower;
    assert!(bandwidth > 20.0);
}

/// 测试%B指标
#[test]
fn test_bollinger_percent_b() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(20.0);
    let result = bb.update(30.0);
    
    assert!(result.is_some());
    
    // 价格在中轨，%B应该接近0.5
    if let Some(percent_b) = bb.percent_b(20.0) {
        assert!(percent_b >= 0.0 && percent_b <= 1.0);
    }
}

/// 测试带宽计算
#[test]
fn test_bollinger_bandwidth() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(20.0);
    let result = bb.update(30.0);
    
    assert!(result.is_some());
    
    if let Some(bandwidth) = bb.bandwidth() {
        assert!(bandwidth > 0.0);
    }
}

/// 测试重置功能
#[test]
fn test_bollinger_reset() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(20.0);
    bb.update(30.0);
    assert!(bb.is_ready());
    
    bb.reset();
    
    assert!(bb.is_empty());
    assert!(!bb.is_ready());
    assert_eq!(bb.value(), None);
}

/// 测试滑动窗口
#[test]
fn test_bollinger_sliding_window() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    // 初始数据
    bb.update(10.0);
    bb.update(20.0);
    let result1 = bb.update(30.0);
    assert!(result1.is_some());
    let bands1 = result1.unwrap();
    
    // 添加新数据，窗口滑动
    let result2 = bb.update(40.0);
    assert!(result2.is_some());
    let bands2 = result2.unwrap();
    
    // 中轨应该从 (10+20+30)/3=20 变为 (20+30+40)/3=30
    assert_relative_eq!(bands1.middle, 20.0, epsilon = 1e-10);
    assert_relative_eq!(bands2.middle, 30.0, epsilon = 1e-10);
}

/// 测试克隆
#[test]
fn test_bollinger_clone() {
    let mut bb1 = BollingerBands::new(5, 2.0);
    bb1.update(100.0);
    bb1.update(102.0);
    bb1.update(98.0);
    
    let bb2 = bb1.clone();
    
    assert_eq!(bb1.period(), bb2.period());
    assert_relative_eq!(bb1.std_dev_multiplier(), bb2.std_dev_multiplier(), epsilon = 1e-10);
}

/// 测试不同的标准差倍数
#[test]
fn test_bollinger_different_multipliers() {
    let mut bb1 = BollingerBands::new(3, 1.0);
    let mut bb2 = BollingerBands::new(3, 2.0);
    let mut bb3 = BollingerBands::new(3, 3.0);
    
    let prices = vec![10.0, 20.0, 30.0];
    
    for &price in &prices {
        bb1.update(price);
        bb2.update(price);
        bb3.update(price);
    }
    
    if let (Some(bands1), Some(bands2), Some(bands3)) = 
        (bb1.value(), bb2.value(), bb3.value()) {
        // 中轨应该相同
        assert_relative_eq!(bands1.middle, bands2.middle, epsilon = 1e-10);
        assert_relative_eq!(bands2.middle, bands3.middle, epsilon = 1e-10);
        
        // 带宽应该依次增大
        let width1 = bands1.upper - bands1.lower;
        let width2 = bands2.upper - bands2.lower;
        let width3 = bands3.upper - bands3.lower;
        
        assert!(width1 < width2);
        assert!(width2 < width3);
    }
}

/// 测试实际场景数据
#[test]
fn test_bollinger_realistic_data() {
    let mut bb = BollingerBands::new(5, 2.0);
    
    let prices = vec![100.0, 102.0, 98.0, 103.0, 97.0, 105.0];
    let mut last_bands = None;
    
    for price in prices {
        last_bands = bb.update(price);
    }
    
    assert!(last_bands.is_some());
    let bands = last_bands.unwrap();
    
    // 验证布林带的合理性
    assert!(bands.upper > bands.middle);
    assert!(bands.middle > bands.lower);
    assert!(bands.upper < 150.0); // 合理的上界
    assert!(bands.lower > 50.0);  // 合理的下界
}

/// 测试%B的边界情况
#[test]
fn test_bollinger_percent_b_boundaries() {
    let mut bb = BollingerBands::new(3, 2.0);
    
    bb.update(10.0);
    bb.update(20.0);
    let result = bb.update(30.0);
    
    assert!(result.is_some());
    
    if let Some(bands) = bb.value() {
        // 价格在上轨，%B应该接近1
        if let Some(pb_upper) = bb.percent_b(bands.upper) {
            assert_relative_eq!(pb_upper, 1.0, epsilon = 1e-5);
        }
        
        // 价格在下轨，%B应该接近0
        if let Some(pb_lower) = bb.percent_b(bands.lower) {
            assert_relative_eq!(pb_lower, 0.0, epsilon = 1e-5);
        }
    }
}
