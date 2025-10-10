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
fn test_roc_new() {
    let roc = ROC::new(10);
    assert_eq!(roc.period, 10);
    assert_eq!(roc.count(), 0);
    assert!(!roc.is_ready());
}

#[test]
#[should_panic(expected = "ROC周期必须大于0")]
fn test_roc_new_zero_period() {
    ROC::new(0);
}

#[test]
fn test_roc_insufficient_data() {
    let mut roc = ROC::new(10);

    // 前10个数据点应该返回None
    for i in 0..10 {
        assert_eq!(roc.update(100.0), None);
        assert_eq!(roc.count(), i + 1);
    }
}

#[test]
fn test_roc_calculation() {
    let mut roc = ROC::new(10);

    // 添加10个价格为100.0的数据点
    for _ in 0..10 {
        roc.update(100.0);
    }

    // 第11个数据点价格为110.0
    // ROC = ((110.0 - 100.0) / 100.0) × 100 = 10.0%
    let result = roc.update(110.0);
    assert!(result.is_some());
    assert!((result.unwrap() - 10.0).abs() < 1e-10);

    // 第12个数据点价格为95.0
    // ROC = ((95.0 - 100.0) / 100.0) × 100 = -5.0%
    let result = roc.update(95.0);
    assert!(result.is_some());
    assert!((result.unwrap() - (-5.0)).abs() < 1e-10);
}

#[test]
fn test_roc_uptrend() {
    let mut roc = ROC::new(5);

    // 模拟上升趋势: 100, 102, 104, 106, 108, 110
    let prices = vec![100.0, 102.0, 104.0, 106.0, 108.0, 110.0];
    let mut results = Vec::new();

    for price in prices {
        if let Some(value) = roc.update(price) {
            results.push(value);
        }
    }

    // 第6个数据点: ROC = ((110 - 100) / 100) × 100 = 10.0%
    assert_eq!(results.len(), 1);
    assert!((results[0] - 10.0).abs() < 1e-10);
}

#[test]
fn test_roc_downtrend() {
    let mut roc = ROC::new(5);

    // 模拟下降趋势: 110, 108, 106, 104, 102, 100
    let prices = vec![110.0, 108.0, 106.0, 104.0, 102.0, 100.0];
    let mut results = Vec::new();

    for price in prices {
        if let Some(value) = roc.update(price) {
            results.push(value);
        }
    }

    // 第6个数据点: ROC = ((100 - 110) / 110) × 100 ≈ -9.09%
    assert_eq!(results.len(), 1);
    assert!((results[0] - (-9.090909090909)).abs() < 1e-10);
}

#[test]
fn test_roc_zero_old_price() {
    let mut roc = ROC::new(2);

    roc.update(0.0);
    roc.update(50.0);
    let result = roc.update(100.0);

    // 当旧价格为0时,应该返回0.0
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_roc_reset() {
    let mut roc = ROC::new(5);

    for _ in 0..6 {
        roc.update(100.0);
    }

    assert!(roc.is_ready());
    assert_eq!(roc.count(), 6);

    roc.reset();

    assert!(!roc.is_ready());
    assert_eq!(roc.count(), 0);
    assert_eq!(roc.update(100.0), None);
}

#[test]
fn test_roc_sliding_window() {
    let mut roc = ROC::new(3);

    // 添加足够的数据使窗口满
    roc.update(100.0);
    roc.update(110.0);
    roc.update(105.0);
    roc.update(115.0);

    // 窗口应该保持大小为 period + 1 = 4
    assert_eq!(roc.count(), 4);

    roc.update(120.0);

    // 继续保持窗口大小
    assert_eq!(roc.count(), 4);
}

#[test]
fn test_roc_volatile_market() {
    let mut roc = ROC::new(3);

    // 模拟剧烈波动的市场
    let prices = vec![100.0, 120.0, 95.0, 130.0, 85.0];
    let mut results = Vec::new();

    for price in prices {
        if let Some(value) = roc.update(price) {
            results.push(value);
        }
    }

    // 第4个数据点: ROC = ((130 - 100) / 100) × 100 = 30.0%
    assert!((results[0] - 30.0).abs() < 1e-10);

    // 第5个数据点: ROC = ((85 - 120) / 120) × 100 ≈ -29.17%
    assert!((results[1] - (-29.166666666667)).abs() < 1e-10);
}
