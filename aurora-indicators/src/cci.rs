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

//! CCI (Commodity Channel Index) - 商品通道指数
//!
//! CCI 是一个动量型指标,用于衡量当前价格与其移动平均值的偏离程度。
//! 与 RSI 和 Stochastic 不同,CCI 没有固定的上下限。
//!
//! # 计算公式
//!
//! 1. 典型价格(TP) = (最高价 + 最低价 + 收盘价) / 3
//! 2. TP的移动平均(SMA)
//! 3. 平均偏差(MD) = Σ|TP - SMA| / N
//! 4. CCI = (TP - SMA) / (0.015 × MD)
//!
//! 其中常数 0.015 确保大约 70%-80% 的 CCI 值落在 -100 到 +100 之间
//!
//! # 使用场景
//!
//! - **超买/超卖**: 通常 +100 以上为超买,-100 以下为超卖
//! - **趋势识别**: CCI > 0 表示价格在平均水平之上,可能是上涨趋势
//! - **背离**: 价格创新高但 CCI 未创新高,可能预示反转
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::CCI;
//!
//! let mut cci = CCI::new(20); // 20周期CCI
//!
//! // 需要输入高、低、收盘价
//! let result = cci.update(105.0, 95.0, 100.0);
//! ```

use std::collections::VecDeque;

/// CCI (Commodity Channel Index) 商品通道指数
///
/// 衡量当前价格相对于统计平均值的偏离程度
pub struct CCI {
    /// 计算周期
    period: usize,
    /// 典型价格历史数据
    typical_prices: VecDeque<f64>,
    /// 典型价格总和(用于快速计算SMA)
    tp_sum: f64,
}

impl CCI {
    /// 创建新的CCI指标实例
    ///
    /// # 参数
    ///
    /// * `period` - 计算周期,通常使用14或20
    ///
    /// # Panics
    ///
    /// 当 `period` 为 0 时会 panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::CCI;
    ///
    /// let cci = CCI::new(20);
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "CCI周期必须大于0");
        
        Self {
            period,
            typical_prices: VecDeque::with_capacity(period),
            tp_sum: 0.0,
        }
    }

    /// 更新指标并计算新的CCI值
    ///
    /// # 参数
    ///
    /// * `high` - 最高价
    /// * `low` - 最低价
    /// * `close` - 收盘价
    ///
    /// # 返回值
    ///
    /// - `Some(f64)` - 当累积了足够数据后返回CCI值
    /// - `None` - 数据不足时返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::CCI;
    ///
    /// let mut cci = CCI::new(20);
    ///
    /// // 需要至少 period 个数据点
    /// for _ in 0..19 {
    ///     assert_eq!(cci.update(105.0, 95.0, 100.0), None);
    /// }
    ///
    /// let result = cci.update(105.0, 95.0, 100.0);
    /// assert!(result.is_some());
    /// ```
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        // 计算典型价格
        let typical_price = (high + low + close) / 3.0;

        // 添加到窗口
        self.typical_prices.push_back(typical_price);
        self.tp_sum += typical_price;

        // 保持窗口大小
        if self.typical_prices.len() > self.period {
            if let Some(old_tp) = self.typical_prices.pop_front() {
                self.tp_sum -= old_tp;
            }
        }

        // 需要足够的数据才能计算
        if self.typical_prices.len() < self.period {
            return None;
        }

        // 计算典型价格的简单移动平均
        let sma = self.tp_sum / self.period as f64;

        // 计算平均偏差
        let mean_deviation: f64 = self.typical_prices
            .iter()
            .map(|&tp| (tp - sma).abs())
            .sum::<f64>() / self.period as f64;

        // 避免除以零
        if mean_deviation == 0.0 {
            return Some(0.0);
        }

        // 计算CCI
        // 常数0.015确保约70%-80%的值落在-100到+100之间
        let cci = (typical_price - sma) / (0.015 * mean_deviation);

        Some(cci)
    }

    /// 重置指标状态
    ///
    /// 清除所有历史数据,重新开始计算
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::CCI;
    ///
    /// let mut cci = CCI::new(20);
    /// cci.update(105.0, 95.0, 100.0);
    /// cci.reset();
    /// assert_eq!(cci.update(105.0, 95.0, 100.0), None);
    /// ```
    pub fn reset(&mut self) {
        self.typical_prices.clear();
        self.tp_sum = 0.0;
    }

    /// 获取当前已积累的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::CCI;
    ///
    /// let mut cci = CCI::new(20);
    /// assert_eq!(cci.count(), 0);
    /// cci.update(105.0, 95.0, 100.0);
    /// assert_eq!(cci.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.typical_prices.len()
    }

    /// 检查是否已准备好输出结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::CCI;
    ///
    /// let mut cci = CCI::new(20);
    /// assert!(!cci.is_ready());
    ///
    /// for _ in 0..20 {
    ///     cci.update(105.0, 95.0, 100.0);
    /// }
    /// assert!(cci.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.typical_prices.len() >= self.period
    }
}

#[cfg(test)]
mod tests;
