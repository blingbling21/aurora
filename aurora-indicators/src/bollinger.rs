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

//! 布林带 (Bollinger Bands) 指标
//!
//! 布林带由中轨、上轨和下轨组成，用于衡量价格的波动范围。

use crate::MA;
use std::collections::VecDeque;

/// 布林带指标的输出
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BollingerBandsOutput {
    /// 上轨 (Upper Band)
    pub upper: f64,
    /// 中轨 (Middle Band) - 移动平均线
    pub middle: f64,
    /// 下轨 (Lower Band)
    pub lower: f64,
}

/// 布林带 (Bollinger Bands) 指标
///
/// 布林带包含三条线：
/// - 中轨：N期移动平均线
/// - 上轨：中轨 + K倍标准差
/// - 下轨：中轨 - K倍标准差
///
/// # 算法原理
///
/// Middle Band = MA(N)
/// Upper Band = MA(N) + K * σ
/// Lower Band = MA(N) - K * σ
///
/// 其中σ是N期价格的标准差，K通常取2
///
/// # 时间复杂度
///
/// 每次更新的时间复杂度为O(1)
///
/// # 内存复杂度
///
/// 空间复杂度为O(N)，其中N是周期长度
#[derive(Debug, Clone)]
pub struct BollingerBands {
    /// 周期长度
    period: usize,
    /// 标准差倍数（通常为2）
    std_dev_multiplier: f64,
    /// 移动平均线（作为中轨）
    ma: MA,
    /// 存储最近N个价格用于计算标准差
    values: VecDeque<f64>,
}

impl BollingerBands {
    /// 创建新的布林带指标
    ///
    /// # 参数
    ///
    /// * `period` - 周期长度，通常使用20，必须大于0
    /// * `std_dev_multiplier` - 标准差倍数，通常使用2.0，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0或标准差倍数小于等于0，函数会panic
    pub fn new(period: usize, std_dev_multiplier: f64) -> Self {
        assert!(period > 0, "布林带周期必须大于0");
        assert!(std_dev_multiplier > 0.0, "标准差倍数必须大于0");

        Self {
            period,
            std_dev_multiplier,
            ma: MA::new(period),
            values: VecDeque::with_capacity(period),
        }
    }

    /// 使用默认参数创建布林带（周期20，标准差倍数2）
    pub fn default() -> Self {
        Self::new(20, 2.0)
    }

    /// 更新指标并返回布林带的三条线
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// * `Some(BollingerBandsOutput)` - 如果有足够数据，返回上中下三条线的值
    /// * `None` - 如果数据不足，返回None
    pub fn update(&mut self, price: f64) -> Option<BollingerBandsOutput> {
        // 更新移动平均线
        let middle = self.ma.update(price)?;

        // 更新价格队列
        self.values.push_back(price);
        if self.values.len() > self.period {
            self.values.pop_front();
        }

        // 计算标准差
        let std_dev = self.calculate_std_dev(middle);

        // 计算上下轨
        let upper = middle + self.std_dev_multiplier * std_dev;
        let lower = middle - self.std_dev_multiplier * std_dev;

        Some(BollingerBandsOutput {
            upper,
            middle,
            lower,
        })
    }

    /// 获取当前的布林带值
    pub fn value(&self) -> Option<BollingerBandsOutput> {
        let middle = self.ma.value()?;
        
        if self.values.len() != self.period {
            return None;
        }

        let std_dev = self.calculate_std_dev(middle);
        let upper = middle + self.std_dev_multiplier * std_dev;
        let lower = middle - self.std_dev_multiplier * std_dev;

        Some(BollingerBandsOutput {
            upper,
            middle,
            lower,
        })
    }

    /// 计算标准差
    fn calculate_std_dev(&self, mean: f64) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }

        // 计算方差
        let variance: f64 = self
            .values
            .iter()
            .map(|&value| {
                let diff = value - mean;
                diff * diff
            })
            .sum::<f64>()
            / self.values.len() as f64;

        // 返回标准差
        variance.sqrt()
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.ma.reset();
        self.values.clear();
    }

    /// 获取周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 获取标准差倍数
    pub fn std_dev_multiplier(&self) -> f64 {
        self.std_dev_multiplier
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.ma.is_empty()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.ma.is_ready()
    }

    /// 计算价格相对于布林带的位置（%B指标）
    ///
    /// %B = (价格 - 下轨) / (上轨 - 下轨)
    ///
    /// %B > 1: 价格在上轨之上
    /// %B = 1: 价格触及上轨
    /// %B = 0: 价格触及下轨
    /// %B < 0: 价格在下轨之下
    pub fn percent_b(&self, price: f64) -> Option<f64> {
        let bands = self.value()?;
        let bandwidth = bands.upper - bands.lower;
        
        if bandwidth == 0.0 {
            return Some(0.5); // 如果带宽为0，返回中间值
        }
        
        Some((price - bands.lower) / bandwidth)
    }

    /// 计算布林带宽度
    ///
    /// 带宽 = (上轨 - 下轨) / 中轨
    ///
    /// 带宽越大表示波动越大
    pub fn bandwidth(&self) -> Option<f64> {
        let bands = self.value()?;
        
        if bands.middle == 0.0 {
            return None;
        }
        
        Some((bands.upper - bands.lower) / bands.middle)
    }
}

#[cfg(test)]
mod tests;
