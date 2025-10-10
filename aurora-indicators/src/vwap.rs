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

//! VWAP (Volume Weighted Average Price) - 成交量加权平均价
//!
//! VWAP 是一个成交量型指标,计算当日(或指定周期)的成交量加权平均价格。
//! 它是日内交易者和机构投资者判断"公平"交易价格的重要基准。
//!
//! # 计算公式
//!
//! 1. 典型价格(TP) = (最高价 + 最低价 + 收盘价) / 3
//! 2. VWAP = Σ(TP × 成交量) / Σ(成交量)
//!
//! # 使用场景
//!
//! - **价格基准**: 价格在VWAP上方被视为强势,下方为弱势
//! - **日内交易**: 作为关键支撑/阻力位
//! - **机构交易**: 评估交易执行质量
//! - **趋势确认**: 价格持续在VWAP上方确认上涨趋势
//!
//! # 注意事项
//!
//! - 传统VWAP是日内指标,每日重置
//! - 本实现支持滚动窗口VWAP,可用于任意周期
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::VWAP;
//!
//! let mut vwap = VWAP::new(20); // 20周期滚动VWAP
//!
//! let vwap_value = vwap.update(110.0, 90.0, 100.0, 1000.0);
//! if let Some(value) = vwap_value {
//!     if 100.0 > value {
//!         println!("价格强势,高于VWAP");
//!     }
//! }
//! ```

use std::collections::VecDeque;

/// VWAP (Volume Weighted Average Price) 成交量加权平均价
///
/// 计算成交量加权的平均价格
pub struct VWAP {
    /// 计算周期(0表示累积模式,不限周期)
    period: usize,
    /// 典型价格×成交量的历史
    pv_values: VecDeque<f64>,
    /// 成交量历史
    volumes: VecDeque<f64>,
    /// 典型价格×成交量的总和
    pv_sum: f64,
    /// 成交量总和
    volume_sum: f64,
}

impl VWAP {
    /// 创建新的VWAP指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,0表示累积模式(不限周期)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let vwap = VWAP::new(20); // 20周期滚动VWAP
    /// let vwap_cumulative = VWAP::new(0); // 累积VWAP(日内模式)
    /// ```
    pub fn new(period: usize) -> Self {
        let capacity = if period > 0 { period } else { 100 };
        
        Self {
            period,
            pv_values: VecDeque::with_capacity(capacity),
            volumes: VecDeque::with_capacity(capacity),
            pv_sum: 0.0,
            volume_sum: 0.0,
        }
    }

    /// 更新指标并计算新的VWAP值
    ///
    /// # 参数
    ///
    /// * `high` - 最高价
    /// * `low` - 最低价
    /// * `close` - 收盘价
    /// * `volume` - 成交量
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 返回VWAP值
    /// - `None` - 成交量总和为0时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let mut vwap = VWAP::new(20);
    ///
    /// let result = vwap.update(110.0, 90.0, 100.0, 1000.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        // 计算典型价格
        let typical_price = (high + low + close) / 3.0;
        
        // 计算价格×成交量
        let pv = typical_price * volume;

        // 添加到历史
        self.pv_values.push_back(pv);
        self.volumes.push_back(volume);
        self.pv_sum += pv;
        self.volume_sum += volume;

        // 如果设置了周期限制,维护滑动窗口
        if self.period > 0 && self.pv_values.len() > self.period {
            if let Some(old_pv) = self.pv_values.pop_front() {
                self.pv_sum -= old_pv;
            }
            if let Some(old_volume) = self.volumes.pop_front() {
                self.volume_sum -= old_volume;
            }
        }

        // 避免除以零
        if self.volume_sum == 0.0 {
            return None;
        }

        // 计算VWAP
        let vwap = self.pv_sum / self.volume_sum;

        Some(vwap)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let mut vwap = VWAP::new(20);
    /// vwap.update(110.0, 90.0, 100.0, 1000.0);
    /// vwap.reset();
    /// assert_eq!(vwap.count(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.pv_values.clear();
        self.volumes.clear();
        self.pv_sum = 0.0;
        self.volume_sum = 0.0;
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let mut vwap = VWAP::new(20);
    /// assert_eq!(vwap.count(), 0);
    /// vwap.update(110.0, 90.0, 100.0, 1000.0);
    /// assert_eq!(vwap.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.pv_values.len()
    }

    /// 检查是否有足够数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let mut vwap = VWAP::new(20);
    /// assert!(!vwap.is_ready());
    ///
    /// vwap.update(110.0, 90.0, 100.0, 1000.0);
    /// assert!(vwap.is_ready()); // VWAP第一个数据点就可用
    /// ```
    pub fn is_ready(&self) -> bool {
        !self.pv_values.is_empty()
    }

    /// 判断价格是否高于VWAP
    ///
    /// # 参数
    ///
    /// * `price` - 当前价格
    /// * `vwap` - VWAP值
    ///
    /// # 返回值
    ///
    /// 如果价格高于VWAP,返回true(强势)
    pub fn is_above(price: f64, vwap: f64) -> bool {
        price > vwap
    }

    /// 判断价格是否低于VWAP
    ///
    /// # 参数
    ///
    /// * `price` - 当前价格
    /// * `vwap` - VWAP值
    ///
    /// # 返回值
    ///
    /// 如果价格低于VWAP,返回true(弱势)
    pub fn is_below(price: f64, vwap: f64) -> bool {
        price < vwap
    }

    /// 计算价格与VWAP的偏离百分比
    ///
    /// # 参数
    ///
    /// * `price` - 当前价格
    /// * `vwap` - VWAP值
    ///
    /// # 返回值
    ///
    /// 返回偏离百分比,正值表示高于VWAP,负值表示低于VWAP
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::VWAP;
    ///
    /// let deviation = VWAP::deviation_percentage(105.0, 100.0);
    /// assert_eq!(deviation, 5.0); // 高于VWAP 5%
    ///
    /// let deviation = VWAP::deviation_percentage(95.0, 100.0);
    /// assert_eq!(deviation, -5.0); // 低于VWAP 5%
    /// ```
    pub fn deviation_percentage(price: f64, vwap: f64) -> f64 {
        if vwap == 0.0 {
            return 0.0;
        }
        ((price - vwap) / vwap) * 100.0
    }
}

#[cfg(test)]
mod tests;
