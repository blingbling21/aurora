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

//! 交易手续费和滑点模型
//!
//! 提供多种手续费和滑点计算方式,用于更真实地模拟交易成本。
//! 支持固定值、百分比、以及基于成交量和波动率的动态模型。

use serde::{Deserialize, Serialize};

/// 手续费模型
///
/// 定义不同的手续费计算方式,适应不同交易场景和交易所规则。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeeModel {
    /// 固定金额手续费
    ///
    /// 无论交易金额多少,收取固定费用。适用于某些区块链交易。
    Fixed(f64),

    /// 百分比手续费
    ///
    /// 按交易金额的百分比收取。参数为百分比值(0.1 表示 0.1%)。
    Percentage(f64),

    /// 分层手续费
    ///
    /// 根据交易量大小采用不同费率。格式: (交易量阈值, 费率百分比)
    /// 例: [(1000.0, 0.1), (10000.0, 0.08), (f64::MAX, 0.05)]
    /// 表示交易量 <1000 收费 0.1%, 1000-10000 收费 0.08%, >10000 收费 0.05%
    Tiered(Vec<(f64, f64)>),

    /// Maker-Taker 模型
    ///
    /// 区分流动性提供者(Maker)和消耗者(Taker)的费率。
    /// 参数: (maker费率, taker费率)
    MakerTaker { maker_fee: f64, taker_fee: f64 },

    /// 无手续费
    None,
}

/// 滑点模型
///
/// 定义不同的滑点计算方式,模拟市场深度和流动性对成交价格的影响。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlippageModel {
    /// 固定滑点
    ///
    /// 每次交易固定的价格偏移量。参数为绝对价格偏移。
    Fixed(f64),

    /// 百分比滑点
    ///
    /// 按交易价格的百分比计算滑点。参数为百分比值(0.1 表示 0.1%)。
    Percentage(f64),

    /// 基于成交量的动态滑点
    ///
    /// 交易量越大,滑点越大,模拟市场深度有限的情况。
    /// 参数: (基础滑点百分比, 成交量系数, 参考成交量)
    /// 计算: base_slippage + (volume / reference_volume) * volume_coefficient
    VolumeBased {
        base_slippage: f64,
        volume_coefficient: f64,
        reference_volume: f64,
    },

    /// 基于波动率的动态滑点
    ///
    /// 市场波动越大,滑点越大,模拟高波动期的成交难度。
    /// 参数: (基础滑点百分比, 波动率系数)
    /// 计算: base_slippage + volatility * volatility_coefficient
    VolatilityBased {
        base_slippage: f64,
        volatility_coefficient: f64,
    },

    /// 综合动态滑点
    ///
    /// 同时考虑成交量和波动率的影响。
    /// 参数: (基础滑点, 成交量系数, 参考成交量, 波动率系数)
    Dynamic {
        base_slippage: f64,
        volume_coefficient: f64,
        reference_volume: f64,
        volatility_coefficient: f64,
    },

    /// 无滑点
    None,
}

/// 交易成本计算器
///
/// 综合计算交易的手续费和滑点,提供最终的实际成交价格。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCostCalculator {
    /// 手续费模型
    fee_model: FeeModel,
    /// 滑点模型
    slippage_model: SlippageModel,
}

/// 交易成本明细
///
/// 包含手续费、滑点和最终成交价格的详细信息。
#[derive(Debug, Clone, PartialEq)]
pub struct TradeCost {
    /// 原始价格(未考虑滑点)
    pub original_price: f64,
    /// 滑点金额
    pub slippage: f64,
    /// 实际成交价格(含滑点)
    pub executed_price: f64,
    /// 手续费金额
    pub fee: f64,
    /// 交易总成本(买入时为正,卖出时为负)
    pub total_cost: f64,
}

impl TradeCostCalculator {
    /// 创建新的交易成本计算器
    ///
    /// # 参数
    ///
    /// * `fee_model` - 手续费模型
    /// * `slippage_model` - 滑点模型
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::{TradeCostCalculator, FeeModel, SlippageModel};
    ///
    /// let calculator = TradeCostCalculator::new(
    ///     FeeModel::Percentage(0.1),
    ///     SlippageModel::Percentage(0.05),
    /// );
    /// ```
    pub fn new(fee_model: FeeModel, slippage_model: SlippageModel) -> Self {
        Self {
            fee_model,
            slippage_model,
        }
    }

    /// 计算买入交易的成本
    ///
    /// # 参数
    ///
    /// * `price` - 订单价格
    /// * `quantity` - 交易数量
    /// * `volume` - 市场成交量(用于动态滑点计算)
    /// * `volatility` - 市场波动率(用于动态滑点计算)
    /// * `is_maker` - 是否为 Maker 订单(仅用于 MakerTaker 模型)
    ///
    /// # 返回值
    ///
    /// 返回包含详细成本信息的 `TradeCost` 结构
    pub fn calculate_buy_cost(
        &self,
        price: f64,
        quantity: f64,
        volume: Option<f64>,
        volatility: Option<f64>,
        is_maker: bool,
    ) -> TradeCost {
        // 计算滑点(买入时滑点为正,价格上涨)
        let slippage = self.calculate_slippage(price, quantity, volume, volatility);
        let executed_price = price + slippage;

        // 计算手续费
        let trade_value = executed_price * quantity;
        let fee = self.calculate_fee(trade_value, is_maker);

        // 买入总成本 = 成交金额 + 手续费
        let total_cost = trade_value + fee;

        TradeCost {
            original_price: price,
            slippage,
            executed_price,
            fee,
            total_cost,
        }
    }

    /// 计算卖出交易的成本
    ///
    /// # 参数
    ///
    /// * `price` - 订单价格
    /// * `quantity` - 交易数量
    /// * `volume` - 市场成交量
    /// * `volatility` - 市场波动率
    /// * `is_maker` - 是否为 Maker 订单
    ///
    /// # 返回值
    ///
    /// 返回包含详细成本信息的 `TradeCost` 结构
    pub fn calculate_sell_cost(
        &self,
        price: f64,
        quantity: f64,
        volume: Option<f64>,
        volatility: Option<f64>,
        is_maker: bool,
    ) -> TradeCost {
        // 计算滑点(卖出时滑点为负,价格下跌)
        let slippage = self.calculate_slippage(price, quantity, volume, volatility);
        let executed_price = price - slippage;

        // 计算手续费
        let trade_value = executed_price * quantity;
        let fee = self.calculate_fee(trade_value, is_maker);

        // 卖出总收益 = 成交金额 - 手续费
        let total_cost = -(trade_value - fee); // 负数表示收入

        TradeCost {
            original_price: price,
            slippage,
            executed_price,
            fee,
            total_cost,
        }
    }

    /// 计算滑点金额
    fn calculate_slippage(
        &self,
        price: f64,
        quantity: f64,
        volume: Option<f64>,
        volatility: Option<f64>,
    ) -> f64 {
        match &self.slippage_model {
            SlippageModel::Fixed(amount) => *amount,
            SlippageModel::Percentage(pct) => price * pct / 100.0,
            SlippageModel::VolumeBased {
                base_slippage,
                volume_coefficient,
                reference_volume,
            } => {
                let base = price * base_slippage / 100.0;
                let volume_factor = if let Some(_vol) = volume {
                    (quantity / reference_volume) * volume_coefficient
                } else {
                    0.0
                };
                base + price * volume_factor / 100.0
            }
            SlippageModel::VolatilityBased {
                base_slippage,
                volatility_coefficient,
            } => {
                let base = price * base_slippage / 100.0;
                let vol_factor = if let Some(vol) = volatility {
                    vol * volatility_coefficient
                } else {
                    0.0
                };
                base + price * vol_factor / 100.0
            }
            SlippageModel::Dynamic {
                base_slippage,
                volume_coefficient,
                reference_volume,
                volatility_coefficient,
            } => {
                let base = price * base_slippage / 100.0;
                let volume_factor = if let Some(_vol) = volume {
                    (quantity / reference_volume) * volume_coefficient
                } else {
                    0.0
                };
                let vol_factor = if let Some(vol) = volatility {
                    vol * volatility_coefficient
                } else {
                    0.0
                };
                base + price * (volume_factor + vol_factor) / 100.0
            }
            SlippageModel::None => 0.0,
        }
    }

    /// 计算手续费金额
    fn calculate_fee(&self, trade_value: f64, is_maker: bool) -> f64 {
        match &self.fee_model {
            FeeModel::Fixed(amount) => *amount,
            FeeModel::Percentage(pct) => trade_value * pct / 100.0,
            FeeModel::Tiered(tiers) => {
                // 找到适用的费率档位
                for (threshold, rate) in tiers {
                    if trade_value <= *threshold {
                        return trade_value * rate / 100.0;
                    }
                }
                // 如果没有找到匹配的档位,使用最后一个
                if let Some((_, rate)) = tiers.last() {
                    return trade_value * rate / 100.0;
                }
                0.0
            }
            FeeModel::MakerTaker {
                maker_fee,
                taker_fee,
            } => {
                let rate = if is_maker { maker_fee } else { taker_fee };
                trade_value * rate / 100.0
            }
            FeeModel::None => 0.0,
        }
    }

    /// 获取手续费模型
    pub fn fee_model(&self) -> &FeeModel {
        &self.fee_model
    }

    /// 获取滑点模型
    pub fn slippage_model(&self) -> &SlippageModel {
        &self.slippage_model
    }
}

impl Default for TradeCostCalculator {
    fn default() -> Self {
        Self {
            fee_model: FeeModel::Percentage(0.1), // 默认 0.1% 手续费
            slippage_model: SlippageModel::Percentage(0.05), // 默认 0.05% 滑点
        }
    }
}

#[cfg(test)]
mod tests;
