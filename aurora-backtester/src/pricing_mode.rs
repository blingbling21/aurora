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

use aurora_core::Kline;
use serde::{Deserialize, Serialize};

/// 回测定价模式
///
/// 用于控制回测中交易价格的计算方式，以模拟不同的真实度水平。
///
/// # 变体
///
/// * `Close` - 使用收盘价执行买卖（简单模式，真实度较低）
/// * `BidAsk` - 使用买一卖一价（买入用卖一价，卖出用买一价，真实度更高）
///
/// # 示例
///
/// ```rust
/// use aurora_backtester::PricingMode;
///
/// let mode = PricingMode::Close;
/// assert!(matches!(mode, PricingMode::Close));
///
/// let realistic_mode = PricingMode::BidAsk { spread_pct: 0.001 };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PricingMode {
    /// 使用收盘价执行交易
    ///
    /// 这是最简单的模式，买入和卖出都使用K线的收盘价。
    /// 这种模式计算简单但不够真实，因为实际交易中
    /// 买卖价格存在价差。
    Close,

    /// 使用买一卖一价执行交易
    ///
    /// 买入时使用卖一价（ask），卖出时使用买一价（bid）。
    /// 这种模式更接近真实交易情况。
    ///
    /// `spread_pct` 参数表示价差百分比（例如 0.001 表示 0.1% 的价差）
    BidAsk {
        /// 买卖价差百分比（相对于中间价）
        spread_pct: f64,
    },
}

impl Default for PricingMode {
    fn default() -> Self {
        Self::Close
    }
}

impl PricingMode {
    /// 计算买入价格
    ///
    /// 根据定价模式和K线数据计算买入时应该使用的价格。
    ///
    /// # 参数
    ///
    /// * `kline` - K线数据
    ///
    /// # 返回
    ///
    /// 买入价格
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_backtester::PricingMode;
    /// use aurora_core::Kline;
    ///
    /// let kline = Kline {
    ///     timestamp: 1640995200000,
    ///     open: 100.0,
    ///     high: 105.0,
    ///     low: 95.0,
    ///     close: 102.0,
    ///     volume: 1000.0,
    /// };
    ///
    /// let close_mode = PricingMode::Close;
    /// assert_eq!(close_mode.get_buy_price(&kline), 102.0);
    ///
    /// let bid_ask_mode = PricingMode::BidAsk { spread_pct: 0.001 };
    /// let buy_price = bid_ask_mode.get_buy_price(&kline);
    /// assert!(buy_price > 102.0); // 买入价格会略高于收盘价
    /// ```
    pub fn get_buy_price(&self, kline: &Kline) -> f64 {
        match self {
            Self::Close => kline.close,
            Self::BidAsk { spread_pct } => {
                // 买入使用卖一价（ask price）
                // ask = mid_price * (1 + spread_pct/2)
                kline.close * (1.0 + spread_pct / 2.0)
            }
        }
    }

    /// 计算卖出价格
    ///
    /// 根据定价模式和K线数据计算卖出时应该使用的价格。
    ///
    /// # 参数
    ///
    /// * `kline` - K线数据
    ///
    /// # 返回
    ///
    /// 卖出价格
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_backtester::PricingMode;
    /// use aurora_core::Kline;
    ///
    /// let kline = Kline {
    ///     timestamp: 1640995200000,
    ///     open: 100.0,
    ///     high: 105.0,
    ///     low: 95.0,
    ///     close: 102.0,
    ///     volume: 1000.0,
    /// };
    ///
    /// let close_mode = PricingMode::Close;
    /// assert_eq!(close_mode.get_sell_price(&kline), 102.0);
    ///
    /// let bid_ask_mode = PricingMode::BidAsk { spread_pct: 0.001 };
    /// let sell_price = bid_ask_mode.get_sell_price(&kline);
    /// assert!(sell_price < 102.0); // 卖出价格会略低于收盘价
    /// ```
    pub fn get_sell_price(&self, kline: &Kline) -> f64 {
        match self {
            Self::Close => kline.close,
            Self::BidAsk { spread_pct } => {
                // 卖出使用买一价（bid price）
                // bid = mid_price * (1 - spread_pct/2)
                kline.close * (1.0 - spread_pct / 2.0)
            }
        }
    }

    /// 获取用于权益计算的估值价格
    ///
    /// 在计算当前权益时使用的价格（通常使用中间价）。
    ///
    /// # 参数
    ///
    /// * `kline` - K线数据
    ///
    /// # 返回
    ///
    /// 估值价格
    pub fn get_mark_price(&self, kline: &Kline) -> f64 {
        // 无论哪种模式，权益估值都使用中间价（收盘价）
        kline.close
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_kline() -> Kline {
        Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 100.0,
            volume: 1000.0,
        }
    }

    #[test]
    fn test_close_mode() {
        let mode = PricingMode::Close;
        let kline = create_test_kline();

        assert_eq!(mode.get_buy_price(&kline), 100.0);
        assert_eq!(mode.get_sell_price(&kline), 100.0);
        assert_eq!(mode.get_mark_price(&kline), 100.0);
    }

    #[test]
    fn test_bid_ask_mode() {
        let spread_pct = 0.002; // 0.2% 价差
        let mode = PricingMode::BidAsk { spread_pct };
        let kline = create_test_kline();

        let buy_price = mode.get_buy_price(&kline);
        let sell_price = mode.get_sell_price(&kline);
        let mark_price = mode.get_mark_price(&kline);

        // 买入价格应该高于收盘价
        assert!(buy_price > 100.0);
        assert_eq!(buy_price, 100.0 * 1.001); // 100 * (1 + 0.002/2)

        // 卖出价格应该低于收盘价
        assert!(sell_price < 100.0);
        assert_eq!(sell_price, 100.0 * 0.999); // 100 * (1 - 0.002/2)

        // 标记价格就是收盘价
        assert_eq!(mark_price, 100.0);

        // 价差应该等于设定的百分比
        let actual_spread_pct = (buy_price - sell_price) / mark_price;
        assert!((actual_spread_pct - spread_pct).abs() < 1e-10);
    }

    #[test]
    fn test_default_mode() {
        let mode = PricingMode::default();
        assert_eq!(mode, PricingMode::Close);
    }

    #[test]
    fn test_pricing_mode_serialization() {
        let mode1 = PricingMode::Close;
        let json1 = serde_json::to_string(&mode1).unwrap();
        let deserialized1: PricingMode = serde_json::from_str(&json1).unwrap();
        assert_eq!(mode1, deserialized1);

        let mode2 = PricingMode::BidAsk { spread_pct: 0.001 };
        let json2 = serde_json::to_string(&mode2).unwrap();
        let deserialized2: PricingMode = serde_json::from_str(&json2).unwrap();
        assert_eq!(mode2, deserialized2);
    }

    #[test]
    fn test_spread_with_different_prices() {
        let mode = PricingMode::BidAsk { spread_pct: 0.001 };

        let kline1 = Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50000.0,
            volume: 100.0,
        };

        let buy_price1 = mode.get_buy_price(&kline1);
        let sell_price1 = mode.get_sell_price(&kline1);

        // 价差应该随价格等比例变化
        assert_eq!(buy_price1, 50000.0 * 1.0005);
        assert_eq!(sell_price1, 50000.0 * 0.9995);
    }
}
