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

//! MFI (Money Flow Index) - 资金流量指数
//!
//! MFI 被称为"成交量加权的RSI",是一个成交量型指标。它将成交量信息融入RSI的
//! 计算中,使其能更好地反映由真实资金驱动的超买/超卖状态。
//!
//! # 计算公式
//!
//! 1. 典型价格(TP) = (最高价 + 最低价 + 收盘价) / 3
//! 2. 资金流量(MF) = 典型价格 × 成交量
//! 3. 如果当前TP > 前一TP,则为正资金流量,否则为负资金流量
//! 4. 资金比率(MR) = N周期正资金流量之和 / N周期负资金流量之和
//! 5. MFI = 100 - (100 / (1 + MR))
//!
//! # 使用场景
//!
//! - **超买/超卖**: MFI > 80 为超买,MFI < 20 为超卖
//! - **背离**: 价格创新高但MFI未创新高,可能预示反转
//! - **趋势确认**: 高MFI确认上涨趋势有资金支持
//! - **与RSI比较**: MFI更敏感,因为考虑了成交量
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::MFI;
//!
//! let mut mfi = MFI::new(14); // 14周期MFI
//!
//! if let Some(mfi_value) = mfi.update(110.0, 90.0, 100.0, 1000.0) {
//!     if mfi_value > 80.0 {
//!         println!("超买! MFI = {:.2}", mfi_value);
//!     } else if mfi_value < 20.0 {
//!         println!("超卖! MFI = {:.2}", mfi_value);
//!     }
//! }
//! ```

use std::collections::VecDeque;

/// MFI (Money Flow Index) 资金流量指数
///
/// 成交量加权的RSI,衡量资金流入流出的强度
pub struct MFI {
    /// 计算周期
    period: usize,
    /// 正资金流量历史
    positive_flows: VecDeque<f64>,
    /// 负资金流量历史
    negative_flows: VecDeque<f64>,
    /// 前一周期的典型价格
    prev_typical_price: Option<f64>,
}

impl MFI {
    /// 创建新的MFI指标实例
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
    /// use aurora_indicators::MFI;
    ///
    /// let mfi = MFI::new(14);
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "MFI周期必须大于0");
        
        Self {
            period,
            positive_flows: VecDeque::with_capacity(period),
            negative_flows: VecDeque::with_capacity(period),
            prev_typical_price: None,
        }
    }

    /// 更新指标并计算新的MFI值
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
    /// - `Some(f64)` - 当累积了足够数据后返回MFI值(0-100)
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MFI;
    ///
    /// let mut mfi = MFI::new(14);
    ///
    /// // 需要至少 period + 1 个数据点
    /// for _ in 0..14 {
    ///     assert_eq!(mfi.update(110.0, 90.0, 100.0, 1000.0), None);
    /// }
    ///
    /// let result = mfi.update(110.0, 90.0, 100.0, 1000.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        // 计算典型价格
        let typical_price = (high + low + close) / 3.0;
        
        // 计算资金流量
        let money_flow = typical_price * volume;

        // 第一个数据点,只记录典型价格
        if self.prev_typical_price.is_none() {
            self.prev_typical_price = Some(typical_price);
            return None;
        }

        // 判断是正资金流量还是负资金流量
        let prev_tp = self.prev_typical_price.unwrap();
        if typical_price > prev_tp {
            // 价格上涨,正资金流量
            self.positive_flows.push_back(money_flow);
            self.negative_flows.push_back(0.0);
        } else if typical_price < prev_tp {
            // 价格下跌,负资金流量
            self.positive_flows.push_back(0.0);
            self.negative_flows.push_back(money_flow);
        } else {
            // 价格不变,两者都为0
            self.positive_flows.push_back(0.0);
            self.negative_flows.push_back(0.0);
        }

        // 更新前一典型价格
        self.prev_typical_price = Some(typical_price);

        // 保持窗口大小
        if self.positive_flows.len() > self.period {
            self.positive_flows.pop_front();
            self.negative_flows.pop_front();
        }

        // 需要足够的数据才能计算
        if self.positive_flows.len() < self.period {
            return None;
        }

        // 计算正负资金流量之和
        let positive_mf_sum: f64 = self.positive_flows.iter().sum();
        let negative_mf_sum: f64 = self.negative_flows.iter().sum();

        // 避免除以零和特殊情况
        if positive_mf_sum == 0.0 && negative_mf_sum == 0.0 {
            // 价格不变,所有资金流量为0,返回中性值50
            return Some(50.0);
        }
        
        if negative_mf_sum == 0.0 {
            // 全部是正资金流量,MFI = 100
            return Some(100.0);
        }
        
        if positive_mf_sum == 0.0 {
            // 全部是负资金流量,MFI = 0
            return Some(0.0);
        }

        // 计算资金比率
        let money_ratio = positive_mf_sum / negative_mf_sum;

        // 计算MFI
        let mfi = 100.0 - (100.0 / (1.0 + money_ratio));

        Some(mfi)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MFI;
    ///
    /// let mut mfi = MFI::new(14);
    /// mfi.update(110.0, 90.0, 100.0, 1000.0);
    /// mfi.reset();
    /// assert_eq!(mfi.update(110.0, 90.0, 100.0, 1000.0), None);
    /// ```
    pub fn reset(&mut self) {
        self.positive_flows.clear();
        self.negative_flows.clear();
        self.prev_typical_price = None;
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MFI;
    ///
    /// let mut mfi = MFI::new(14);
    /// assert_eq!(mfi.count(), 0);
    /// mfi.update(110.0, 90.0, 100.0, 1000.0);
    /// assert_eq!(mfi.count(), 0); // 第一个点不计入
    /// mfi.update(110.0, 90.0, 100.0, 1000.0);
    /// assert_eq!(mfi.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.positive_flows.len()
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MFI;
    ///
    /// let mut mfi = MFI::new(14);
    /// assert!(!mfi.is_ready());
    ///
    /// for _ in 0..15 {
    ///     mfi.update(110.0, 90.0, 100.0, 1000.0);
    /// }
    /// assert!(mfi.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.positive_flows.len() >= self.period
    }

    /// 判断是否处于超买区域
    ///
    /// # 参数
    ///
    /// * `value` - MFI值
    ///
    /// # 返回值
    ///
    /// 如果MFI > 80,返回true
    pub fn is_overbought(value: f64) -> bool {
        value > 80.0
    }

    /// 判断是否处于超卖区域
    ///
    /// # 参数
    ///
    /// * `value` - MFI值
    ///
    /// # 返回值
    ///
    /// 如果MFI < 20,返回true
    pub fn is_oversold(value: f64) -> bool {
        value < 20.0
    }
}

// 默认周期: 14
impl Default for MFI {
    fn default() -> Self {
        Self::new(14)
    }
}

#[cfg(test)]
mod tests;
