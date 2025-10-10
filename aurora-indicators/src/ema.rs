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

//! 指数移动平均线 (Exponential Moving Average) 指标
//!
//! EMA对近期数据赋予更高权重，相比SMA更快地响应价格变化。

/// 指数移动平均线 (Exponential Moving Average) 指标
///
/// EMA通过给予近期数据更高的权重来平滑价格数据，使其对最新价格变化更加敏感。
///
/// # 算法原理
///
/// EMA的计算公式为：
/// EMA(t) = α * P(t) + (1 - α) * EMA(t-1)
///
/// 其中：
/// - P(t) 是当前价格
/// - α = 2 / (period + 1) 是平滑系数
/// - period 是周期长度
///
/// # 时间复杂度
///
/// 每次更新的时间复杂度为O(1)
///
/// # 内存复杂度
///
/// 空间复杂度为O(1)，只需存储上一个EMA值和平滑系数
#[derive(Debug, Clone)]
pub struct EMA {
    /// EMA周期长度
    period: usize,
    /// 平滑系数 α = 2 / (period + 1)
    alpha: f64,
    /// 当前EMA值
    current_ema: Option<f64>,
    /// 已接收的数据点数量
    count: usize,
}

impl EMA {
    /// 创建新的EMA指标
    ///
    /// # 参数
    ///
    /// * `period` - EMA周期，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "EMA周期必须大于0");

        let alpha = 2.0 / (period as f64 + 1.0);

        Self {
            period,
            alpha,
            current_ema: None,
            count: 0,
        }
    }

    /// 更新指标并返回最新的EMA值
    ///
    /// 第一个值会直接作为初始EMA值，后续值会按照EMA公式计算。
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// 返回更新后的EMA值，第一次调用时返回输入的价格值
    pub fn update(&mut self, price: f64) -> f64 {
        self.count += 1;

        match self.current_ema {
            None => {
                // 第一个值直接作为EMA初始值
                self.current_ema = Some(price);
                price
            }
            Some(prev_ema) => {
                // 应用EMA公式
                let new_ema = self.alpha * price + (1.0 - self.alpha) * prev_ema;
                self.current_ema = Some(new_ema);
                new_ema
            }
        }
    }

    /// 获取当前的EMA值
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有数据，返回当前EMA值
    /// * `None` - 如果还没有数据，返回None
    pub fn value(&self) -> Option<f64> {
        self.current_ema
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.current_ema = None;
        self.count = 0;
    }

    /// 获取周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 获取平滑系数α
    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    /// 获取已接收的数据点数量
    pub fn count(&self) -> usize {
        self.count
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.current_ema.is_none()
    }

    /// 检查指标是否已有数据
    pub fn is_ready(&self) -> bool {
        self.current_ema.is_some()
    }
}

#[cfg(test)]
mod tests;
