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

//! MACD (Moving Average Convergence Divergence) 指标
//!
//! MACD用于判断趋势变化和买卖时机。

use crate::EMA;

/// MACD指标输出
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MACDOutput {
    /// MACD线（快线-慢线）
    pub macd: f64,
    /// 信号线（MACD的EMA）
    pub signal: f64,
    /// 柱状图（MACD-信号线）
    pub histogram: f64,
}

/// MACD (Moving Average Convergence Divergence) 指标
///
/// MACD由三部分组成：
/// - MACD线：快速EMA - 慢速EMA
/// - 信号线：MACD线的EMA
/// - 柱状图：MACD线 - 信号线
///
/// # 算法原理
///
/// MACD = EMA(fast) - EMA(slow)
/// Signal = EMA(MACD, signal_period)
/// Histogram = MACD - Signal
///
/// # 默认参数
///
/// - 快线周期：12
/// - 慢线周期：26
/// - 信号线周期：9
#[derive(Debug, Clone)]
pub struct MACD {
    /// 快速EMA
    fast_ema: EMA,
    /// 慢速EMA
    slow_ema: EMA,
    /// 信号线EMA
    signal_ema: EMA,
    /// 快线周期
    fast_period: usize,
    /// 慢线周期
    slow_period: usize,
    /// 信号线周期
    signal_period: usize,
}

impl MACD {
    /// 创建新的MACD指标
    ///
    /// # 参数
    ///
    /// * `fast_period` - 快速EMA周期
    /// * `slow_period` - 慢速EMA周期
    /// * `signal_period` - 信号线周期
    ///
    /// # Panics
    ///
    /// 如果任何周期为0或快线周期大于等于慢线周期，函数会panic
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        assert!(fast_period > 0, "快线周期必须大于0");
        assert!(slow_period > 0, "慢线周期必须大于0");
        assert!(signal_period > 0, "信号线周期必须大于0");
        assert!(fast_period < slow_period, "快线周期必须小于慢线周期");

        Self {
            fast_ema: EMA::new(fast_period),
            slow_ema: EMA::new(slow_period),
            signal_ema: EMA::new(signal_period),
            fast_period,
            slow_period,
            signal_period,
        }
    }

    /// 使用默认参数创建MACD（12, 26, 9）
    pub fn default() -> Self {
        Self::new(12, 26, 9)
    }

    /// 更新指标并返回MACD值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// 返回MACDOutput，包含MACD线、信号线和柱状图
    pub fn update(&mut self, price: f64) -> MACDOutput {
        // 更新快慢EMA
        let fast = self.fast_ema.update(price);
        let slow = self.slow_ema.update(price);

        // 计算MACD线
        let macd = fast - slow;

        // 更新信号线
        let signal = self.signal_ema.update(macd);

        // 计算柱状图
        let histogram = macd - signal;

        MACDOutput {
            macd,
            signal,
            histogram,
        }
    }

    /// 获取当前的MACD值
    pub fn value(&self) -> Option<MACDOutput> {
        let fast = self.fast_ema.value()?;
        let slow = self.slow_ema.value()?;
        let macd = fast - slow;
        let signal = self.signal_ema.value()?;
        let histogram = macd - signal;

        Some(MACDOutput {
            macd,
            signal,
            histogram,
        })
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
    }

    /// 获取快线周期
    pub fn fast_period(&self) -> usize {
        self.fast_period
    }

    /// 获取慢线周期
    pub fn slow_period(&self) -> usize {
        self.slow_period
    }

    /// 获取信号线周期
    pub fn signal_period(&self) -> usize {
        self.signal_period
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.fast_ema.is_empty() || self.slow_ema.is_empty()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.fast_ema.is_ready() && self.slow_ema.is_ready() && self.signal_ema.is_ready()
    }

    /// 检查是否出现金叉（MACD上穿信号线）
    ///
    /// 需要传入上一次的MACD输出进行比较
    pub fn is_bullish_crossover(&self, prev: &MACDOutput, current: &MACDOutput) -> bool {
        prev.histogram < 0.0 && current.histogram > 0.0
    }

    /// 检查是否出现死叉（MACD下穿信号线）
    ///
    /// 需要传入上一次的MACD输出进行比较
    pub fn is_bearish_crossover(&self, prev: &MACDOutput, current: &MACDOutput) -> bool {
        prev.histogram > 0.0 && current.histogram < 0.0
    }
}

#[cfg(test)]
mod tests;
