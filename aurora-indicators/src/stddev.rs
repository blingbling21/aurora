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

//! Standard Deviation - 标准差
//!
//! 标准差是一个波动率型指标,用于衡量价格在一定时期内相对于其平均值的离散程度。
//! 它是布林带(Bollinger Bands)指标的数学基础,也可以作为独立的波动率指标使用。
//!
//! # 计算公式
//!
//! 1. 计算平均值: μ = Σx / N
//! 2. 计算方差: σ² = Σ(x - μ)² / N
//! 3. 标准差: σ = √σ²
//!
//! # 使用场景
//!
//! - **波动率测量**: 标准差越大,市场波动越剧烈
//! - **风险评估**: 用于衡量投资组合的风险水平
//! - **技术指标基础**: 布林带、肯特纳通道等指标的组成部分
//! - **异常检测**: 识别价格的异常偏离
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::StdDev;
//!
//! let mut stddev = StdDev::new(20); // 20周期标准差
//!
//! let result = stddev.update(100.0);
//! ```

use std::collections::VecDeque;

/// Standard Deviation 标准差指标
///
/// 衡量价格数据的离散程度
pub struct StdDev {
    /// 计算周期
    period: usize,
    /// 价格历史数据
    prices: VecDeque<f64>,
    /// 价格总和(用于快速计算平均值)
    sum: f64,
}

impl StdDev {
    /// 创建新的标准差指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,通常使用20(与布林带相同)
    ///
    /// # Panics
    ///
    /// 当 `period` 为 0 时会 panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let stddev = StdDev::new(20);
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "标准差周期必须大于0");
        
        Self {
            period,
            prices: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    /// 更新指标并计算新的标准差值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 当累积了足够数据后返回标准差值
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let mut stddev = StdDev::new(20);
    ///
    /// // 需要至少 period 个数据点
    /// for _ in 0..19 {
    ///     assert_eq!(stddev.update(100.0), None);
    /// }
    ///
    /// let result = stddev.update(100.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, price: f64) -> Option<f64> {
        // 添加新价格
        self.prices.push_back(price);
        self.sum += price;

        // 保持窗口大小
        if self.prices.len() > self.period {
            if let Some(old_price) = self.prices.pop_front() {
                self.sum -= old_price;
            }
        }

        // 需要足够的数据才能计算
        if self.prices.len() < self.period {
            return None;
        }

        // 计算平均值
        let mean = self.sum / self.period as f64;

        // 计算方差
        let variance: f64 = self.prices
            .iter()
            .map(|&price| {
                let diff = price - mean;
                diff * diff
            })
            .sum::<f64>() / self.period as f64;

        // 计算标准差
        let std_dev = variance.sqrt();

        Some(std_dev)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let mut stddev = StdDev::new(20);
    /// stddev.update(100.0);
    /// stddev.reset();
    /// assert_eq!(stddev.update(100.0), None);
    /// ```
    pub fn reset(&mut self) {
        self.prices.clear();
        self.sum = 0.0;
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let mut stddev = StdDev::new(20);
    /// assert_eq!(stddev.count(), 0);
    /// stddev.update(100.0);
    /// assert_eq!(stddev.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.prices.len()
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let mut stddev = StdDev::new(20);
    /// assert!(!stddev.is_ready());
    ///
    /// for _ in 0..20 {
    ///     stddev.update(100.0);
    /// }
    /// assert!(stddev.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.prices.len() >= self.period
    }

    /// 获取当前的平均值
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 当有足够数据时返回平均值
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::StdDev;
    ///
    /// let mut stddev = StdDev::new(5);
    /// for _ in 0..5 {
    ///     stddev.update(100.0);
    /// }
    /// assert_eq!(stddev.mean(), Some(100.0));
    /// ```
    pub fn mean(&self) -> Option<f64> {
        if self.prices.len() < self.period {
            return None;
        }
        Some(self.sum / self.period as f64)
    }
}

#[cfg(test)]
mod tests;
