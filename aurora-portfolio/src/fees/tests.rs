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
fn test_fixed_fee() {
    let calculator = TradeCostCalculator::new(
        FeeModel::Fixed(10.0),
        SlippageModel::None,
    );

    let cost = calculator.calculate_buy_cost(100.0, 5.0, None, None, false);
    assert_eq!(cost.fee, 10.0);
    assert_eq!(cost.slippage, 0.0);
    assert_eq!(cost.executed_price, 100.0);
}

#[test]
fn test_percentage_fee() {
    let calculator = TradeCostCalculator::new(
        FeeModel::Percentage(0.1), // 0.1%
        SlippageModel::None,
    );

    let cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, false);
    // 交易金额 = 100 * 10 = 1000
    // 手续费 = 1000 * 0.1% = 1.0
    assert_eq!(cost.fee, 1.0);
}

#[test]
fn test_tiered_fee() {
    let calculator = TradeCostCalculator::new(
        FeeModel::Tiered(vec![
            (1000.0, 0.1),   // <= 1000: 0.1%
            (10000.0, 0.08), // <= 10000: 0.08%
            (f64::MAX, 0.05), // > 10000: 0.05%
        ]),
        SlippageModel::None,
    );

    // 小额交易
    let cost1 = calculator.calculate_buy_cost(100.0, 5.0, None, None, false);
    // 500 * 0.1% = 0.5
    assert_eq!(cost1.fee, 0.5);

    // 中等交易
    let cost2 = calculator.calculate_buy_cost(100.0, 50.0, None, None, false);
    // 5000 * 0.08% = 4.0
    assert_eq!(cost2.fee, 4.0);

    // 大额交易
    let cost3 = calculator.calculate_buy_cost(100.0, 200.0, None, None, false);
    // 20000 * 0.05% = 10.0
    assert_eq!(cost3.fee, 10.0);
}

#[test]
fn test_maker_taker_fee() {
    let calculator = TradeCostCalculator::new(
        FeeModel::MakerTaker {
            maker_fee: 0.05,
            taker_fee: 0.1,
        },
        SlippageModel::None,
    );

    // Maker 订单
    let maker_cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, true);
    // 1000 * 0.05% = 0.5
    assert_eq!(maker_cost.fee, 0.5);

    // Taker 订单
    let taker_cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, false);
    // 1000 * 0.1% = 1.0
    assert_eq!(taker_cost.fee, 1.0);
}

#[test]
fn test_fixed_slippage() {
    let calculator = TradeCostCalculator::new(
        FeeModel::None,
        SlippageModel::Fixed(2.0),
    );

    // 买入滑点为正
    let buy_cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, false);
    assert_eq!(buy_cost.slippage, 2.0);
    assert_eq!(buy_cost.executed_price, 102.0);

    // 卖出滑点导致价格下降
    let sell_cost = calculator.calculate_sell_cost(100.0, 10.0, None, None, false);
    assert_eq!(sell_cost.slippage, 2.0);
    assert_eq!(sell_cost.executed_price, 98.0);
}

#[test]
fn test_percentage_slippage() {
    let calculator = TradeCostCalculator::new(
        FeeModel::None,
        SlippageModel::Percentage(0.5), // 0.5%
    );

    let buy_cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, false);
    // 滑点 = 100 * 0.5% = 0.5
    assert_eq!(buy_cost.slippage, 0.5);
    assert_eq!(buy_cost.executed_price, 100.5);
}

#[test]
fn test_volume_based_slippage() {
    let calculator = TradeCostCalculator::new(
        FeeModel::None,
        SlippageModel::VolumeBased {
            base_slippage: 0.1,       // 基础 0.1%
            volume_coefficient: 0.5,  // 成交量系数
            reference_volume: 1000.0, // 参考成交量
        },
    );

    // 小成交量
    let cost1 = calculator.calculate_buy_cost(100.0, 10.0, Some(1000.0), None, false);
    // base = 100 * 0.1% = 0.1
    // volume_factor = (10 / 1000) * 0.5 = 0.01 * 0.5 = 0.005
    // total = 0.1 + 100 * 0.005% = 0.1 + 0.005 = 0.105
    assert!((cost1.slippage - 0.105).abs() < 1e-10);

    // 大成交量
    let cost2 = calculator.calculate_buy_cost(100.0, 100.0, Some(1000.0), None, false);
    // base = 0.1
    // volume_factor = (100 / 1000) * 0.5 = 0.1 * 0.5 = 0.05
    // total = 0.1 + 100 * 0.05% = 0.1 + 0.05 = 0.15
    assert!((cost2.slippage - 0.15).abs() < 1e-10);
}

#[test]
fn test_volatility_based_slippage() {
    let calculator = TradeCostCalculator::new(
        FeeModel::None,
        SlippageModel::VolatilityBased {
            base_slippage: 0.1,         // 基础 0.1%
            volatility_coefficient: 2.0, // 波动率系数
        },
    );

    // 低波动率
    let cost1 = calculator.calculate_buy_cost(100.0, 10.0, None, Some(0.5), false);
    // base = 100 * 0.1% = 0.1
    // vol_factor = 0.5 * 2.0 = 1.0
    // total = 0.1 + 100 * 1.0% = 1.1
    assert!((cost1.slippage - 1.1).abs() < 1e-10);

    // 高波动率
    let cost2 = calculator.calculate_buy_cost(100.0, 10.0, None, Some(2.0), false);
    // base = 0.1
    // vol_factor = 2.0 * 2.0 = 4.0
    // total = 0.1 + 100 * 4.0% = 4.1
    assert!((cost2.slippage - 4.1).abs() < 1e-10);
}

#[test]
fn test_dynamic_slippage() {
    let calculator = TradeCostCalculator::new(
        FeeModel::None,
        SlippageModel::Dynamic {
            base_slippage: 0.1,
            volume_coefficient: 0.5,
            reference_volume: 1000.0,
            volatility_coefficient: 1.0,
        },
    );

    let cost = calculator.calculate_buy_cost(
        100.0,
        50.0,
        Some(1000.0),
        Some(1.0),
        false,
    );
    
    // base = 100 * 0.1% = 0.1
    // volume_factor = (50 / 1000) * 0.5 = 0.025
    // vol_factor = 1.0 * 1.0 = 1.0
    // total = 0.1 + 100 * (0.025 + 1.0)% = 0.1 + 1.025 = 1.125
    assert!((cost.slippage - 1.125).abs() < 1e-10);
}

#[test]
fn test_buy_total_cost() {
    let calculator = TradeCostCalculator::new(
        FeeModel::Percentage(0.1),     // 0.1% 手续费
        SlippageModel::Percentage(0.05), // 0.05% 滑点
    );

    let cost = calculator.calculate_buy_cost(100.0, 10.0, None, None, false);
    
    // 滑点: 100 * 0.05% = 0.05
    // 成交价: 100.05
    // 成交金额: 100.05 * 10 = 1000.5
    // 手续费: 1000.5 * 0.1% = 1.0005
    // 总成本: 1000.5 + 1.0005 = 1001.5005
    
    assert!((cost.slippage - 0.05).abs() < 1e-10);
    assert!((cost.executed_price - 100.05).abs() < 1e-10);
    assert!((cost.fee - 1.0005).abs() < 1e-10);
    assert!((cost.total_cost - 1001.5005).abs() < 1e-10);
}

#[test]
fn test_sell_total_cost() {
    let calculator = TradeCostCalculator::new(
        FeeModel::Percentage(0.1),
        SlippageModel::Percentage(0.05),
    );

    let cost = calculator.calculate_sell_cost(100.0, 10.0, None, None, false);
    
    // 滑点: 100 * 0.05% = 0.05 (卖出时价格下降)
    // 成交价: 99.95
    // 成交金额: 99.95 * 10 = 999.5
    // 手续费: 999.5 * 0.1% = 0.9995
    // 总收益: 999.5 - 0.9995 = 998.5005
    // total_cost(负数表示收入): -998.5005
    
    assert!((cost.slippage - 0.05).abs() < 1e-10);
    assert!((cost.executed_price - 99.95).abs() < 1e-10);
    assert!((cost.fee - 0.9995).abs() < 1e-10);
    assert!((cost.total_cost + 998.5005).abs() < 1e-10);
}

#[test]
fn test_default_calculator() {
    let calculator = TradeCostCalculator::default();
    
    assert_eq!(calculator.fee_model(), &FeeModel::Percentage(0.1));
    assert_eq!(calculator.slippage_model(), &SlippageModel::Percentage(0.05));
}
