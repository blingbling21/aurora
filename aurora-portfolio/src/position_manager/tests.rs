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
fn test_fixed_amount_strategy() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedAmount(1000.0));

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 1000.0);

    // 账户权益改变,仓位大小不变
    let size2 = manager.calculate_position_size(20000.0, 0.0).unwrap();
    assert_eq!(size2, 1000.0);
}

#[test]
fn test_fixed_percentage_strategy() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedPercentage(0.2));

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 2000.0); // 20%

    let size2 = manager.calculate_position_size(5000.0, 0.0).unwrap();
    assert_eq!(size2, 1000.0); // 20%
}

#[test]
fn test_fixed_percentage_invalid() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedPercentage(1.5));

    let result = manager.calculate_position_size(10000.0, 0.0);
    assert!(result.is_err());

    let manager2 = PositionManager::new(PositionSizingStrategy::FixedPercentage(-0.1));
    let result2 = manager2.calculate_position_size(10000.0, 0.0);
    assert!(result2.is_err());
}

#[test]
fn test_kelly_criterion_strategy() {
    // 胜率60%,盈亏比2:1,使用半凯利
    let manager = PositionManager::new(PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,
        profit_loss_ratio: 2.0,
        kelly_fraction: 0.5,
    });

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    
    // Kelly = (0.6 * 2 - 0.4) / 2 = 0.4
    // 半凯利 = 0.4 * 0.5 = 0.2
    // 仓位 = 10000 * 0.2 = 2000
    assert!((size - 2000.0).abs() < 0.01);
}

#[test]
fn test_kelly_negative_edge() {
    // 胜率30%,盈亏比1.5:1,期望为负
    let manager = PositionManager::new(PositionSizingStrategy::KellyCriterion {
        win_rate: 0.3,
        profit_loss_ratio: 1.5,
        kelly_fraction: 1.0,
    });

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    
    // Kelly = (0.3 * 1.5 - 0.7) / 1.5 = -0.167 -> 取0
    // 仓位应该接近最小值
    assert!(size >= 10.0); // 最小交易金额
    assert!(size < 100.0);
}

#[test]
fn test_kelly_invalid_params() {
    // 无效的盈亏比
    let manager = PositionManager::new(PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,
        profit_loss_ratio: -1.0,
        kelly_fraction: 0.5,
    });

    assert!(manager.calculate_position_size(10000.0, 0.0).is_err());

    // 无效的Kelly系数
    let manager2 = PositionManager::new(PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,
        profit_loss_ratio: 2.0,
        kelly_fraction: 1.5,
    });

    assert!(manager2.calculate_position_size(10000.0, 0.0).is_err());
}

#[test]
fn test_pyramid_initial_position() {
    let manager = PositionManager::new(PositionSizingStrategy::Pyramid {
        initial_percentage: 0.1,
        profit_threshold: 5.0,
        max_percentage: 0.5,
        increment: 0.1,
    });

    // 无盈利时使用初始仓位
    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 1000.0); // 10%
}

#[test]
fn test_pyramid_add_position() {
    let manager = PositionManager::new(PositionSizingStrategy::Pyramid {
        initial_percentage: 0.1,
        profit_threshold: 5.0,
        max_percentage: 0.5,
        increment: 0.1,
    });

    // 盈利6%,触发一次加仓
    let size = manager.calculate_position_size(10000.0, 6.0).unwrap();
    assert_eq!(size, 2000.0); // 10% + 10% = 20%

    // 盈利12%,触发两次加仓
    let size2 = manager.calculate_position_size(10000.0, 12.0).unwrap();
    assert!((size2 - 3000.0).abs() < 0.01); // 10% + 10% + 10% = 30%
}

#[test]
fn test_pyramid_max_position() {
    let manager = PositionManager::new(PositionSizingStrategy::Pyramid {
        initial_percentage: 0.1,
        profit_threshold: 5.0,
        max_percentage: 0.3,
        increment: 0.1,
    });

    // 盈利30%,但不超过最大仓位30%
    let size = manager.calculate_position_size(10000.0, 30.0).unwrap();
    assert_eq!(size, 3000.0); // 最大30%
}

#[test]
fn test_all_in_strategy() {
    let manager = PositionManager::new(PositionSizingStrategy::AllIn);

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    assert_eq!(size, 10000.0);

    let size2 = manager.calculate_position_size(5000.0, 0.0).unwrap();
    assert_eq!(size2, 5000.0);
}

#[test]
fn test_min_position_value() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedPercentage(0.001))
        .with_min_position_value(50.0);

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    // 0.1% * 10000 = 10, 但最小值是50
    assert_eq!(size, 50.0);
}

#[test]
fn test_leverage() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedPercentage(0.5))
        .with_max_leverage(2.0);

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    // 50% * 10000 * 2倍杠杆 = 10000
    assert_eq!(size, 10000.0);
}

#[test]
fn test_leverage_not_exceed_max() {
    let manager = PositionManager::new(PositionSizingStrategy::AllIn)
        .with_max_leverage(2.0);

    let size = manager.calculate_position_size(10000.0, 0.0).unwrap();
    // 全仓 * 2倍杠杆,但最多不超过 10000 * 2 = 20000
    assert_eq!(size, 20000.0);
}

#[test]
fn test_invalid_equity() {
    let manager = PositionManager::new(PositionSizingStrategy::FixedAmount(1000.0));

    let result = manager.calculate_position_size(0.0, 0.0);
    assert!(result.is_err());

    let result2 = manager.calculate_position_size(-1000.0, 0.0);
    assert!(result2.is_err());
}

#[test]
fn test_strategy_equality() {
    let s1 = PositionSizingStrategy::FixedAmount(1000.0);
    let s2 = PositionSizingStrategy::FixedAmount(1000.0);
    let s3 = PositionSizingStrategy::FixedAmount(2000.0);

    assert_eq!(s1, s2);
    assert_ne!(s1, s3);
}

#[test]
fn test_get_and_set_strategy() {
    let mut manager = PositionManager::new(PositionSizingStrategy::FixedAmount(1000.0));

    assert_eq!(
        *manager.get_strategy(),
        PositionSizingStrategy::FixedAmount(1000.0)
    );

    manager.set_strategy(PositionSizingStrategy::FixedPercentage(0.2));
    assert_eq!(
        *manager.get_strategy(),
        PositionSizingStrategy::FixedPercentage(0.2)
    );
}
