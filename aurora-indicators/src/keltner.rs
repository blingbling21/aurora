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

//! Keltner Channels - 肯特纳通道
//!
//! 肯特纳通道是一个波动率型指标,与布林带类似,但其通道宽度是基于ATR(平均真实波幅)
//! 计算的,而非标准差。常与布林带结合使用(如TTM Squeeze策略)来识别波动率极度
//! 压缩的状态。
//!
//! # 计算公式
//!
//! 1. 中轨 = EMA(收盘价, period)
//! 2. 上轨 = 中轨 + multiplier × ATR(period)
//! 3. 下轨 = 中轨 - multiplier × ATR(period)
//!
//! # 使用场景
//!
//! - **趋势识别**: 价格突破上轨表示强势上涨,突破下轨表示强势下跌
//! - **波动率压缩**: 与布林带结合,识别"挤压"(Squeeze)状态
//! - **支撑/阻力**: 通道可作为动态支撑和阻力位
//! - **止损设置**: 基于ATR的通道适合动态止损
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::KeltnerChannels;
//!
//! let mut kc = KeltnerChannels::new(20, 2.0); // 20周期,2倍ATR
//!
//! if let Some(channels) = kc.update(110.0, 90.0, 100.0) {
//!     println!("上轨: {:.2}, 中轨: {:.2}, 下轨: {:.2}",
//!         channels.upper, channels.middle, channels.lower);
//! }
//! ```

use crate::{EMA, ATR};

/// Keltner Channels 输出结构
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeltnerChannelsOutput {
    /// 上轨
    pub upper: f64,
    /// 中轨(EMA)
    pub middle: f64,
    /// 下轨
    pub lower: f64,
}

/// Keltner Channels 肯特纳通道
///
/// 基于EMA和ATR构建的价格通道
pub struct KeltnerChannels {
    /// EMA周期(存储以备将来扩展使用)
    #[allow(dead_code)]
    period: usize,
    /// ATR倍数
    multiplier: f64,
    /// 指数移动平均线
    ema: EMA,
    /// 平均真实波幅
    atr: ATR,
}

impl KeltnerChannels {
    /// 创建新的Keltner Channels指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,通常使用20
    /// * `multiplier` - ATR倍数,通常使用2.0
    ///
    /// # Panics
    ///
    /// 当 `period` 为 0 或 `multiplier` 为负数时会 panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::KeltnerChannels;
    ///
    /// let kc = KeltnerChannels::new(20, 2.0);
    /// ```
    pub fn new(period: usize, multiplier: f64) -> Self {
        assert!(period > 0, "Keltner Channels周期必须大于0");
        assert!(multiplier >= 0.0, "ATR倍数不能为负数");
        
        Self {
            period,
            multiplier,
            ema: EMA::new(period),
            atr: ATR::new(period),
        }
    }

    /// 更新指标并计算新的通道值
    ///
    /// # 参数
    ///
    /// * `high` - 最高价
    /// * `low` - 最低价
    /// * `close` - 收盘价
    ///
    /// # 返回值
    ///
    /// - `Some(KeltnerChannelsOutput)` - 返回通道值
    /// - `None` - 仅在第一个数据点且ATR无法计算时返回(实际上ATR总是有输出)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::KeltnerChannels;
    ///
    /// let mut kc = KeltnerChannels::new(20, 2.0);
    ///
    /// // 第一个数据点就有结果
    /// let result = kc.update(110.0, 90.0, 100.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<KeltnerChannelsOutput> {
        // 更新EMA(中轨)
        let middle = self.ema.update(close);

        // 更新ATR
        let atr_value = self.atr.update(high, low, close)?;

        // 计算通道宽度
        let width = self.multiplier * atr_value;

        // 计算上下轨
        let upper = middle + width;
        let lower = middle - width;

        Some(KeltnerChannelsOutput {
            upper,
            middle,
            lower,
        })
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::KeltnerChannels;
    ///
    /// let mut kc = KeltnerChannels::new(20, 2.0);
    /// kc.update(110.0, 90.0, 100.0);
    /// kc.reset();
    /// assert!(!kc.is_ready());
    /// // reset后第一个数据点就有结果
    /// assert!(kc.update(110.0, 90.0, 100.0).is_some());
    /// ```
    pub fn reset(&mut self) {
        self.ema.reset();
        self.atr.reset();
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::KeltnerChannels;
    ///
    /// let mut kc = KeltnerChannels::new(20, 2.0);
    /// assert!(!kc.is_ready());
    ///
    /// for _ in 0..20 {
    ///     kc.update(110.0, 90.0, 100.0);
    /// }
    /// assert!(kc.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.atr.is_ready()
    }

    /// 计算通道宽度百分比
    ///
    /// # 参数
    ///
    /// * `output` - Keltner Channels输出
    ///
    /// # 返回值
    ///
    /// 返回通道宽度占中轨的百分比
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::KeltnerChannels;
    ///
    /// let mut kc = KeltnerChannels::new(10, 2.0);
    /// for _ in 0..10 {
    ///     kc.update(110.0, 90.0, 100.0);
    /// }
    ///
    /// if let Some(output) = kc.update(110.0, 90.0, 105.0) {
    ///     let width_pct = KeltnerChannels::width_percentage(&output);
    ///     println!("通道宽度: {:.2}%", width_pct);
    /// }
    /// ```
    pub fn width_percentage(output: &KeltnerChannelsOutput) -> f64 {
        if output.middle == 0.0 {
            return 0.0;
        }
        ((output.upper - output.lower) / output.middle) * 100.0
    }

    /// 判断价格是否突破上轨
    ///
    /// # 参数
    ///
    /// * `price` - 当前价格
    /// * `output` - Keltner Channels输出
    ///
    /// # 返回值
    ///
    /// 如果价格高于上轨,返回true
    pub fn is_above_upper(price: f64, output: &KeltnerChannelsOutput) -> bool {
        price > output.upper
    }

    /// 判断价格是否突破下轨
    ///
    /// # 参数
    ///
    /// * `price` - 当前价格
    /// * `output` - Keltner Channels输出
    ///
    /// # 返回值
    ///
    /// 如果价格低于下轨,返回true
    pub fn is_below_lower(price: f64, output: &KeltnerChannelsOutput) -> bool {
        price < output.lower
    }
}

// 默认参数: 20周期, 2倍ATR
impl Default for KeltnerChannels {
    fn default() -> Self {
        Self::new(20, 2.0)
    }
}

#[cfg(test)]
mod tests;
