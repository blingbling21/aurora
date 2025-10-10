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

//! ATR (Average True Range) 指标
//!
//! ATR衡量市场波动程度，不考虑价格方向。

use crate::EMA;

/// ATR (Average True Range) 指标
///
/// ATR通过计算真实波幅的移动平均来衡量市场波动性。
/// 真实波幅是以下三者中的最大值：
/// 1. 当前最高价 - 当前最低价
/// 2. |当前最高价 - 前收盘价|
/// 3. |当前最低价 - 前收盘价|
///
/// # 算法原理
///
/// TR = max(high - low, |high - prev_close|, |low - prev_close|)
/// ATR = EMA(TR, period)
///
/// # 用途
///
/// - 衡量市场波动程度
/// - 设置止损位置
/// - 调整仓位大小
#[derive(Debug, Clone)]
pub struct ATR {
    /// ATR周期
    period: usize,
    /// 真实波幅的EMA
    atr_ema: EMA,
    /// 前一根K线的收盘价
    prev_close: Option<f64>,
}

impl ATR {
    /// 创建新的ATR指标
    ///
    /// # 参数
    ///
    /// * `period` - ATR周期，通常使用14，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "ATR周期必须大于0");

        Self {
            period,
            atr_ema: EMA::new(period),
            prev_close: None,
        }
    }

    /// 使用默认周期14创建ATR
    pub fn default() -> Self {
        Self::new(14)
    }

    /// 更新指标并返回ATR值
    ///
    /// # 参数
    ///
    /// * `high` - 当前K线最高价
    /// * `low` - 当前K线最低价
    /// * `close` - 当前K线收盘价
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果有前一根K线数据，返回ATR值
    /// * `None` - 如果是第一根K线，返回None
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        // 计算真实波幅
        let tr = match self.prev_close {
            None => {
                // 第一根K线，真实波幅就是最高价-最低价
                high - low
            }
            Some(prev_close) => {
                // 计算三个值的最大值
                let hl = high - low;
                let hc = (high - prev_close).abs();
                let lc = (low - prev_close).abs();
                hl.max(hc).max(lc)
            }
        };

        // 更新前收盘价
        self.prev_close = Some(close);

        // 更新ATR（使用EMA）
        let atr = self.atr_ema.update(tr);
        Some(atr)
    }

    /// 获取当前的ATR值
    pub fn value(&self) -> Option<f64> {
        self.atr_ema.value()
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.atr_ema.reset();
        self.prev_close = None;
    }

    /// 获取周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.prev_close.is_none()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.atr_ema.is_ready()
    }

    /// 计算基于ATR的止损价格
    ///
    /// # 参数
    ///
    /// * `entry_price` - 入场价格
    /// * `multiplier` - ATR倍数（通常使用2-3）
    /// * `is_long` - 是否为多头持仓
    ///
    /// # 返回值
    ///
    /// 返回建议的止损价格，如果ATR未准备好则返回None
    pub fn stop_loss(&self, entry_price: f64, multiplier: f64, is_long: bool) -> Option<f64> {
        let atr = self.value()?;
        
        if is_long {
            // 多头止损在下方
            Some(entry_price - atr * multiplier)
        } else {
            // 空头止损在上方
            Some(entry_price + atr * multiplier)
        }
    }
}

#[cfg(test)]
mod tests;
