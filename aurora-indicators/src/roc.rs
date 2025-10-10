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

//! ROC (Rate of Change) - 变动率指标
//!
//! ROC 是一个动量型指标,用于衡量当前价格相对于N个周期前价格的变化百分比。
//! 它能够快速反应价格的爆发力,帮助判断动量的加速或减速。
//!
//! # 计算公式
//!
//! ROC = ((当前价格 - N周期前价格) / N周期前价格) × 100
//!
//! # 使用场景
//!
//! - **零轴穿越**: ROC从负转正表示上涨动量,从正转负表示下跌动量
//! - **背离**: 价格创新高但ROC未创新高,可能预示趋势减弱
//! - **极值**: ROC达到极端值可能表示超买或超卖状态
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::ROC;
//!
//! let mut roc = ROC::new(10); // 10周期变动率
//!
//! // 前10个数据点不会产生结果
//! for i in 0..10 {
//!     assert_eq!(roc.update(100.0 + i as f64), None);
//! }
//!
//! // 第11个数据点开始计算ROC
//! // 如果第11个价格是110.0,第1个价格是100.0
//! // ROC = ((110.0 - 100.0) / 100.0) × 100 = 10.0%
//! let result = roc.update(110.0);
//! assert!(result.is_some());
//! ```

use std::collections::VecDeque;

/// ROC (Rate of Change) 变动率指标
///
/// 计算当前价格相对于N个周期前价格的变化百分比
pub struct ROC {
    /// 计算周期
    period: usize,
    /// 价格历史数据窗口
    prices: VecDeque<f64>,
}

impl ROC {
    /// 创建新的ROC指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,通常使用10、12或20
    ///
    /// # Panics
    ///
    /// 当 `period` 为 0 时会 panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::ROC;
    ///
    /// let roc = ROC::new(12);
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "ROC周期必须大于0");
        
        Self {
            period,
            prices: VecDeque::with_capacity(period + 1),
        }
    }

    /// 更新指标并计算新的ROC值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 当累积了足够数据后返回ROC值(百分比)
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::ROC;
    ///
    /// let mut roc = ROC::new(10);
    ///
    /// // 需要至少 period + 1 个数据点
    /// for _ in 0..10 {
    ///     assert_eq!(roc.update(100.0), None);
    /// }
    ///
    /// let result = roc.update(110.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, price: f64) -> Option<f64> {
        // 添加新价格
        self.prices.push_back(price);

        // 保持窗口大小为 period + 1
        if self.prices.len() > self.period + 1 {
            self.prices.pop_front();
        }

        // 需要至少 period + 1 个数据点才能计算
        if self.prices.len() <= self.period {
            return None;
        }

        // 获取当前价格和N周期前的价格
        let current_price = *self.prices.back().unwrap();
        let old_price = *self.prices.front().unwrap();

        // 避免除以零
        if old_price == 0.0 {
            return Some(0.0);
        }

        // 计算ROC: ((当前价格 - 旧价格) / 旧价格) × 100
        let roc = ((current_price - old_price) / old_price) * 100.0;

        Some(roc)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::ROC;
    ///
    /// let mut roc = ROC::new(10);
    /// roc.update(100.0);
    /// roc.reset();
    /// assert_eq!(roc.update(100.0), None);
    /// ```
    pub fn reset(&mut self) {
        self.prices.clear();
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::ROC;
    ///
    /// let mut roc = ROC::new(10);
    /// assert_eq!(roc.count(), 0);
    /// roc.update(100.0);
    /// assert_eq!(roc.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.prices.len()
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::ROC;
    ///
    /// let mut roc = ROC::new(10);
    /// assert!(!roc.is_ready());
    ///
    /// for _ in 0..=10 {
    ///     roc.update(100.0);
    /// }
    /// assert!(roc.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.prices.len() > self.period
    }
}

#[cfg(test)]
mod tests;
