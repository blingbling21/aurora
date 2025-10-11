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

//! Accumulation/Distribution Line (累积/派发线)
//!
//! A/D Line 是由 Marc Chaikin 开发的成交量指标，用于衡量资金的累积（买入）
//! 和派发（卖出）情况。它通过分析价格和成交量的关系来识别市场的供需变化。
//!
//! # 工作原理
//!
//! A/D Line 是一个累计指标，它考虑了收盘价在当日价格区间中的位置，
//! 并根据这个位置对成交量进行加权。如果收盘价接近最高价，说明买方力量强，
//! 该周期的成交量大部分被加到 A/D Line 上；反之则被减去。
//!
//! # 计算公式
//!
//! 1. Money Flow Multiplier = [(Close - Low) - (High - Close)] / (High - Low)
//! 2. Money Flow Volume = Money Flow Multiplier × Volume
//! 3. A/D Line = Previous A/D Line + Money Flow Volume
//!
//! # 解读
//!
//! - **A/D Line 上升**: 表示累积（买入）占优，价格可能继续上涨
//! - **A/D Line 下降**: 表示派发（卖出）占优，价格可能继续下跌
//! - **背离**: A/D Line 与价格走势出现背离时，可能预示趋势反转
//!   - 价格创新高但 A/D Line 未创新高：看跌背离
//!   - 价格创新低但 A/D Line 未创新低：看涨背离
//!
//! # 示例
//!
//! ```
//! use aurora_indicators::ADLine;
//!
//! let mut ad = ADLine::new();
//!
//! // 输入高价、低价、收盘价、成交量
//! let result1 = ad.update(105.0, 95.0, 102.0, 10000.0);
//! let result2 = ad.update(110.0, 100.0, 108.0, 12000.0);
//! let result3 = ad.update(115.0, 105.0, 106.0, 15000.0);
//!
//! println!("当前 A/D Line: {:.2}", result3);
//! ```

/// Accumulation/Distribution Line 指标结构
pub struct ADLine {
    // 状态
    ad_line: f64,  // 当前 A/D Line 值
}

impl ADLine {
    /// 创建新的 A/D Line 指标
    pub fn new() -> Self {
        Self {
            ad_line: 0.0,
        }
    }

    /// 更新指标数据
    ///
    /// # 参数
    ///
    /// - `high`: 当前周期最高价
    /// - `low`: 当前周期最低价
    /// - `close`: 当前周期收盘价
    /// - `volume`: 当前周期成交量
    ///
    /// # 返回值
    ///
    /// 返回更新后的 A/D Line 值
    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        // 计算 Money Flow Multiplier
        let mf_multiplier = if high == low {
            // 避免除以零：如果高低价相同，设为0
            0.0
        } else {
            ((close - low) - (high - close)) / (high - low)
        };
        
        // 计算 Money Flow Volume
        let mf_volume = mf_multiplier * volume;
        
        // 累加到 A/D Line
        self.ad_line += mf_volume;
        
        self.ad_line
    }

    /// 获取当前 A/D Line 值
    pub fn value(&self) -> f64 {
        self.ad_line
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.ad_line = 0.0;
    }
}

impl Default for ADLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
