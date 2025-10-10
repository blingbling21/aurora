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

//! Williams %R - 威廉指标
//!
//! Williams %R 是一个动量型震荡指标,与随机指标(Stochastic)非常相似,
//! 但刻度是反向的(0到-100)。它衡量收盘价在过去N个周期的最高-最低区间中的相对位置。
//!
//! # 计算公式
//!
//! %R = -100 × (最高价 - 收盘价) / (最高价 - 最低价)
//!
//! 其中:
//! - 最高价 = N周期内的最高价
//! - 最低价 = N周期内的最低价
//! - 收盘价 = 当前收盘价
//!
//! # 使用场景
//!
//! - **超买/超卖**: -20以上为超买区,-80以下为超卖区
//! - **趋势确认**: 在上涨趋势中,常在-20到-80之间波动
//! - **背离**: 价格创新高但%R未创新高,可能预示反转
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::WilliamsR;
//!
//! let mut wr = WilliamsR::new(14); // 14周期威廉指标
//!
//! // 输入高、低、收盘价
//! let result = wr.update(110.0, 90.0, 100.0);
//! ```

use std::collections::VecDeque;

/// Williams %R 威廉指标
///
/// 衡量收盘价在N周期内高低区间中的相对位置,值域为0到-100
pub struct WilliamsR {
    /// 计算周期
    period: usize,
    /// 最高价历史数据
    highs: VecDeque<f64>,
    /// 最低价历史数据
    lows: VecDeque<f64>,
}

impl WilliamsR {
    /// 创建新的Williams %R指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,通常使用14
    ///
    /// # Panics
    ///
    /// 当 `period` 为 0 时会 panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::WilliamsR;
    ///
    /// let wr = WilliamsR::new(14);
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "Williams %R周期必须大于0");
        
        Self {
            period,
            highs: VecDeque::with_capacity(period),
            lows: VecDeque::with_capacity(period),
        }
    }

    /// 更新指标并计算新的Williams %R值
    ///
    /// # 参数
    ///
    /// * `high` - 最高价
    /// * `low` - 最低价
    /// * `close` - 收盘价
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 当累积了足够数据后返回%R值(范围: 0到-100)
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::WilliamsR;
    ///
    /// let mut wr = WilliamsR::new(14);
    ///
    /// // 需要至少 period 个数据点
    /// for _ in 0..13 {
    ///     assert_eq!(wr.update(110.0, 90.0, 100.0), None);
    /// }
    ///
    /// let result = wr.update(110.0, 90.0, 100.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        // 添加新数据
        self.highs.push_back(high);
        self.lows.push_back(low);

        // 保持窗口大小
        if self.highs.len() > self.period {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        // 需要足够的数据才能计算
        if self.highs.len() < self.period {
            return None;
        }

        // 找出N周期内的最高价和最低价
        let highest_high = self.highs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest_low = self.lows.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        // 避免除以零
        let range = highest_high - lowest_low;
        if range == 0.0 {
            return Some(-50.0); // 当区间为0时,返回中间值
        }

        // 计算Williams %R
        // %R = -100 × (最高价 - 收盘价) / (最高价 - 最低价)
        let williams_r = -100.0 * (highest_high - close) / range;

        Some(williams_r)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::WilliamsR;
    ///
    /// let mut wr = WilliamsR::new(14);
    /// wr.update(110.0, 90.0, 100.0);
    /// wr.reset();
    /// assert_eq!(wr.update(110.0, 90.0, 100.0), None);
    /// ```
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::WilliamsR;
    ///
    /// let mut wr = WilliamsR::new(14);
    /// assert_eq!(wr.count(), 0);
    /// wr.update(110.0, 90.0, 100.0);
    /// assert_eq!(wr.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.highs.len()
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::WilliamsR;
    ///
    /// let mut wr = WilliamsR::new(14);
    /// assert!(!wr.is_ready());
    ///
    /// for _ in 0..14 {
    ///     wr.update(110.0, 90.0, 100.0);
    /// }
    /// assert!(wr.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.highs.len() >= self.period
    }

    /// 判断是否处于超买区域
    ///
    /// # 参数
    ///
    /// * `value` - Williams %R值
    /// * `threshold` - 超买阈值,默认为-20
    ///
    /// # 返回值
    ///
    /// 如果值高于阈值(更接近0),返回true
    pub fn is_overbought(value: f64, threshold: f64) -> bool {
        value > threshold
    }

    /// 判断是否处于超卖区域
    ///
    /// # 参数
    ///
    /// * `value` - Williams %R值
    /// * `threshold` - 超卖阈值,默认为-80
    ///
    /// # 返回值
    ///
    /// 如果值低于阈值(更接近-100),返回true
    pub fn is_oversold(value: f64, threshold: f64) -> bool {
        value < threshold
    }
}

#[cfg(test)]
mod tests;
