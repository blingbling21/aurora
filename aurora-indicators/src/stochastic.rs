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

//! Stochastic (随机震荡指标)
//!
//! 用于比较收盘价与价格区间的相对位置，判断超买超卖。

use crate::MA;
use std::collections::VecDeque;

/// Stochastic指标输出
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StochasticOutput {
    /// %K线 - 快速随机指标
    pub k: f64,
    /// %D线 - %K的移动平均（慢速随机指标）
    pub d: f64,
}

/// Stochastic (随机震荡指标)
///
/// 比较收盘价与一段时间内最高价和最低价的相对位置。
///
/// # 算法原理
///
/// %K = 100 * (Close - LowestLow) / (HighestHigh - LowestLow)
/// %D = MA(%K, d_period)
///
/// 其中：
/// - LowestLow 是K期内的最低价
/// - HighestHigh 是K期内的最高价
///
/// # 默认参数
///
/// - K周期：14
/// - D周期：3
///
/// # 超买超卖判断
///
/// - %K > 80: 超买
/// - %K < 20: 超卖
#[derive(Debug, Clone)]
pub struct Stochastic {
    /// %K周期
    k_period: usize,
    /// %D周期
    d_period: usize,
    /// 存储最近K期的最高价
    highs: VecDeque<f64>,
    /// 存储最近K期的最低价
    lows: VecDeque<f64>,
    /// %D线的移动平均
    d_ma: MA,
    /// 当前%K值
    current_k: Option<f64>,
    /// 当前%D值
    current_d: Option<f64>,
}

impl Stochastic {
    /// 创建新的Stochastic指标
    ///
    /// # 参数
    ///
    /// * `k_period` - %K周期，必须大于0
    /// * `d_period` - %D周期，必须大于0
    ///
    /// # Panics
    ///
    /// 如果任何周期为0，函数会panic
    pub fn new(k_period: usize, d_period: usize) -> Self {
        assert!(k_period > 0, "K周期必须大于0");
        assert!(d_period > 0, "D周期必须大于0");

        Self {
            k_period,
            d_period,
            highs: VecDeque::with_capacity(k_period),
            lows: VecDeque::with_capacity(k_period),
            d_ma: MA::new(d_period),
            current_k: None,
            current_d: None,
        }
    }

    /// 使用默认参数创建Stochastic（14, 3）
    pub fn default() -> Self {
        Self::new(14, 3)
    }

    /// 更新指标并返回%K和%D值
    ///
    /// # 参数
    ///
    /// * `high` - 当前K线最高价
    /// * `low` - 当前K线最低价
    /// * `close` - 当前K线收盘价
    ///
    /// # 返回值
    ///
    /// * `Some(StochasticOutput)` - 如果有足够数据，返回%K和%D值
    /// * `None` - 如果数据不足，返回None
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<StochasticOutput> {
        // 更新最高价和最低价队列
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() > self.k_period {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        // 需要足够的数据才能计算
        if self.highs.len() < self.k_period {
            return None;
        }

        // 找出K期内的最高价和最低价
        let highest_high = self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let lowest_low = self.lows.iter().copied().fold(f64::INFINITY, f64::min);

        // 计算%K
        let k = if highest_high == lowest_low {
            // 如果最高价等于最低价，%K设为50
            50.0
        } else {
            100.0 * (close - lowest_low) / (highest_high - lowest_low)
        };

        self.current_k = Some(k);

        // 计算%D（%K的移动平均）
        if let Some(d) = self.d_ma.update(k) {
            self.current_d = Some(d);
            Some(StochasticOutput { k, d })
        } else {
            None
        }
    }

    /// 获取当前的Stochastic值
    pub fn value(&self) -> Option<StochasticOutput> {
        let k = self.current_k?;
        let d = self.current_d?;
        Some(StochasticOutput { k, d })
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.d_ma.reset();
        self.current_k = None;
        self.current_d = None;
    }

    /// 获取K周期
    pub fn k_period(&self) -> usize {
        self.k_period
    }

    /// 获取D周期
    pub fn d_period(&self) -> usize {
        self.d_period
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.highs.is_empty()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.current_k.is_some() && self.current_d.is_some()
    }

    /// 判断是否超买（%K > 80）
    pub fn is_overbought(&self) -> bool {
        self.current_k.map_or(false, |k| k > 80.0)
    }

    /// 判断是否超卖（%K < 20）
    pub fn is_oversold(&self) -> bool {
        self.current_k.map_or(false, |k| k < 20.0)
    }

    /// 检查是否出现金叉（%K上穿%D）
    pub fn is_bullish_crossover(&self, prev: &StochasticOutput, current: &StochasticOutput) -> bool {
        prev.k < prev.d && current.k > current.d
    }

    /// 检查是否出现死叉（%K下穿%D）
    pub fn is_bearish_crossover(&self, prev: &StochasticOutput, current: &StochasticOutput) -> bool {
        prev.k > prev.d && current.k < current.d
    }
}

#[cfg(test)]
mod tests;
