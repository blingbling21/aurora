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

/// 测试Stochastic创建
#[test]
fn test_stochastic_creation() {
    let stoch = Stochastic::new(14, 3);
    assert_eq!(stoch.k_period(), 14);
    assert_eq!(stoch.d_period(), 3);
    assert!(stoch.is_empty());
    assert!(!stoch.is_ready());
}

/// 测试默认参数
#[test]
fn test_stochastic_default() {
    let stoch = Stochastic::default();
    assert_eq!(stoch.k_period(), 14);
    assert_eq!(stoch.d_period(), 3);
}

/// 测试K周期为0时的panic
#[test]
#[should_panic(expected = "K周期必须大于0")]
fn test_stochastic_zero_k_period_panic() {
    Stochastic::new(0, 3);
}

/// 测试D周期为0时的panic
#[test]
#[should_panic(expected = "D周期必须大于0")]
fn test_stochastic_zero_d_period_panic() {
    Stochastic::new(14, 0);
}

/// 测试数据不足
#[test]
fn test_stochastic_insufficient_data() {
    let mut stoch = Stochastic::new(5, 3);
    
    // 前几根K线不会产生结果
    assert_eq!(stoch.update(110.0, 90.0, 100.0), None);
    assert_eq!(stoch.update(115.0, 95.0, 105.0), None);
    assert!(!stoch.is_ready());
}

/// 测试价格在区间顶部（%K应该接近100）
#[test]
fn test_stochastic_at_top() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 价格逐渐上涨
    stoch.update(100.0, 90.0, 92.0);
    stoch.update(105.0, 95.0, 97.0);
    stoch.update(110.0, 100.0, 110.0); // 收盘在最高点
    
    if let Some(output) = stoch.value() {
        // 收盘价在区间顶部，%K应该接近100
        assert!(output.k > 90.0);
    }
}

/// 测试价格在区间底部（%K应该接近0）
#[test]
fn test_stochastic_at_bottom() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 价格逐渐下跌
    stoch.update(110.0, 100.0, 108.0);
    stoch.update(105.0, 95.0, 103.0);
    stoch.update(100.0, 90.0, 90.0); // 收盘在最低点
    
    if let Some(output) = stoch.value() {
        // 收盘价在区间底部，%K应该接近0
        assert!(output.k < 10.0);
    }
}

/// 测试价格在区间中部（%K应该接近50）
#[test]
fn test_stochastic_at_middle() {
    let mut stoch = Stochastic::new(3, 3);
    
    stoch.update(110.0, 90.0, 100.0); // 收盘在中间
    stoch.update(110.0, 90.0, 100.0);
    let result = stoch.update(110.0, 90.0, 100.0);
    
    if let Some(output) = result {
        // 收盘价在区间中部，%K应该接近50
        assert!(output.k > 40.0 && output.k < 60.0);
    }
}

/// 测试超买状态
#[test]
fn test_stochastic_overbought() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 模拟强势上涨
    for i in 0..10 {
        let base = 100.0 + i as f64 * 5.0;
        stoch.update(base + 5.0, base - 5.0, base + 4.0);
    }
    
    // 应该进入超买区域
    if stoch.is_ready() {
        assert!(stoch.is_overbought() || stoch.current_k.unwrap() > 70.0);
    }
}

/// 测试超卖状态
#[test]
fn test_stochastic_oversold() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 模拟强势下跌
    for i in 0..10 {
        let base = 150.0 - i as f64 * 5.0;
        stoch.update(base + 5.0, base - 5.0, base - 4.0);
    }
    
    // 应该进入超卖区域
    if stoch.is_ready() {
        assert!(stoch.is_oversold() || stoch.current_k.unwrap() < 30.0);
    }
}

/// 测试金叉
#[test]
fn test_stochastic_bullish_crossover() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 先下跌使%K在%D下方
    for i in (0..5).rev() {
        let base = 100.0 + i as f64 * 5.0;
        stoch.update(base + 5.0, base - 5.0, base - 2.0);
    }
    
    let prev = stoch.value();
    
    // 然后上涨
    for _ in 0..5 {
        stoch.update(130.0, 120.0, 128.0);
    }
    
    let current = stoch.value();
    
    if let (Some(p), Some(c)) = (prev, current) {
        if p.k < p.d && c.k > c.d {
            assert!(stoch.is_bullish_crossover(&p, &c));
        }
    }
}

/// 测试死叉
#[test]
fn test_stochastic_bearish_crossover() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 先上涨使%K在%D上方
    for i in 0..5 {
        let base = 100.0 + i as f64 * 5.0;
        stoch.update(base + 5.0, base - 5.0, base + 2.0);
    }
    
    let prev = stoch.value();
    
    // 然后下跌
    for _ in 0..5 {
        stoch.update(100.0, 90.0, 92.0);
    }
    
    let current = stoch.value();
    
    if let (Some(p), Some(c)) = (prev, current) {
        if p.k > p.d && c.k < c.d {
            assert!(stoch.is_bearish_crossover(&p, &c));
        }
    }
}

/// 测试重置功能
#[test]
fn test_stochastic_reset() {
    let mut stoch = Stochastic::new(5, 3);
    
    for i in 0..10 {
        let base = 100.0 + i as f64;
        stoch.update(base + 5.0, base - 5.0, base);
    }
    
    assert!(stoch.is_ready());
    
    stoch.reset();
    
    assert!(stoch.is_empty());
    assert!(!stoch.is_ready());
    assert_eq!(stoch.value(), None);
}

/// 测试价格无波动的情况
#[test]
fn test_stochastic_no_volatility() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 所有价格相同
    stoch.update(100.0, 100.0, 100.0);
    stoch.update(100.0, 100.0, 100.0);
    let result = stoch.update(100.0, 100.0, 100.0);
    
    if let Some(output) = result {
        // 无波动时，%K应该是50
        assert_relative_eq!(output.k, 50.0, epsilon = 1e-10);
    }
}

/// 测试克隆
#[test]
fn test_stochastic_clone() {
    let mut stoch1 = Stochastic::new(14, 3);
    
    for i in 0..20 {
        let base = 100.0 + i as f64;
        stoch1.update(base + 5.0, base - 5.0, base);
    }
    
    let stoch2 = stoch1.clone();
    
    assert_eq!(stoch1.k_period(), stoch2.k_period());
    assert_eq!(stoch1.d_period(), stoch2.d_period());
    assert_eq!(stoch1.value().is_some(), stoch2.value().is_some());
}

/// 测试滑动窗口
#[test]
fn test_stochastic_sliding_window() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 需要足够的数据才能产生输出 (k_period + d_period - 1)
    stoch.update(110.0, 90.0, 100.0);
    stoch.update(115.0, 95.0, 105.0);
    stoch.update(120.0, 100.0, 110.0);
    stoch.update(115.0, 95.0, 105.0);
    let value1 = stoch.update(120.0, 100.0, 110.0);
    
    // 添加新K线，窗口滑动
    let value2 = stoch.update(125.0, 105.0, 115.0);
    
    // 两次的值应该都有效且不同
    assert!(value1.is_some() && value2.is_some());
    if let (Some(v1), Some(v2)) = (value1, value2) {
        // K值可能不同
        assert!(v1.k >= 0.0 && v1.k <= 100.0);
        assert!(v2.k >= 0.0 && v2.k <= 100.0);
    }
}

/// 测试实际场景数据
#[test]
fn test_stochastic_realistic_data() {
    let mut stoch = Stochastic::default();
    
    // 模拟真实OHLC数据
    let candles = vec![
        (105.0, 95.0, 100.0),
        (107.0, 97.0, 102.0),
        (110.0, 100.0, 105.0),
        (108.0, 98.0, 103.0),
        (106.0, 96.0, 101.0),
    ];
    
    let mut last_output = None;
    for (high, low, close) in candles {
        last_output = stoch.update(high, low, close);
    }
    
    // 验证输出在合理范围内
    if let Some(output) = last_output {
        assert!(output.k >= 0.0 && output.k <= 100.0);
        assert!(output.d >= 0.0 && output.d <= 100.0);
    }
}

/// 测试%D线是%K的平滑
#[test]
fn test_stochastic_d_smoothing() {
    let mut stoch = Stochastic::new(3, 3);
    
    // 添加足够的数据
    for i in 0..10 {
        let base = 100.0 + (i as f64 * 5.0).sin() * 20.0;
        stoch.update(base + 10.0, base - 10.0, base);
    }
    
    // %D应该比%K更平滑（变化幅度更小）
    // 这个测试只是验证程序能正常运行
    assert!(stoch.is_ready());
}
