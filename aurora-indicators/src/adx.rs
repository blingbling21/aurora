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

//! ADX (Average Directional Index) 平均动向指数
//!
//! ADX衡量趋势的强度，但不判断方向，是趋势策略的重要过滤器。

use crate::EMA;

/// ADX指标输出
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ADXOutput {
    /// ADX值 - 趋势强度
    pub adx: f64,
    /// +DI (正向动向指标)
    pub plus_di: f64,
    /// -DI (负向动向指标)
    pub minus_di: f64,
}

/// ADX (Average Directional Index) 平均动向指数
///
/// ADX由三条线组成：
/// - ADX线：衡量趋势的强度（0-100）
/// - +DI线：正向动向指标
/// - -DI线：负向动向指标
///
/// # 算法原理
///
/// 1. 计算真实波幅 TR
/// 2. 计算正向动向 +DM 和负向动向 -DM
/// 3. 计算平滑的 +DI 和 -DI
/// 4. 计算 DX = |+DI - -DI| / (+DI + -DI) * 100
/// 5. ADX = DX 的移动平均
///
/// # 判断标准
///
/// - ADX > 25: 强趋势
/// - ADX < 20: 弱趋势或盘整
/// - +DI > -DI: 上升趋势
/// - -DI > +DI: 下降趋势
#[derive(Debug, Clone)]
pub struct ADX {
    /// ADX周期
    period: usize,
    /// 前一根K线的高点
    prev_high: Option<f64>,
    /// 前一根K线的低点
    prev_low: Option<f64>,
    /// 前一根K线的收盘价
    prev_close: Option<f64>,
    /// 平滑后的真实波幅
    smoothed_tr: Option<f64>,
    /// 平滑后的+DM
    smoothed_plus_dm: Option<f64>,
    /// 平滑后的-DM
    smoothed_minus_dm: Option<f64>,
    /// ADX的EMA
    adx_ema: EMA,
    /// 数据点计数
    count: usize,
}

impl ADX {
    /// 创建新的ADX指标
    ///
    /// # 参数
    ///
    /// * `period` - ADX周期，通常使用14，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "ADX周期必须大于0");

        Self {
            period,
            prev_high: None,
            prev_low: None,
            prev_close: None,
            smoothed_tr: None,
            smoothed_plus_dm: None,
            smoothed_minus_dm: None,
            adx_ema: EMA::new(period),
            count: 0,
        }
    }

    /// 使用默认周期14创建ADX
    pub fn default() -> Self {
        Self::new(14)
    }

    /// 更新指标并返回ADX值
    ///
    /// # 参数
    ///
    /// * `high` - 当前K线最高价
    /// * `low` - 当前K线最低价
    /// * `close` - 当前K线收盘价
    ///
    /// # 返回值
    ///
    /// * `Some(ADXOutput)` - 如果有足够数据，返回ADX、+DI、-DI值
    /// * `None` - 如果数据不足，返回None
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<ADXOutput> {
        self.count += 1;

        // 第一根K线，只记录数据
        if self.prev_high.is_none() {
            self.prev_high = Some(high);
            self.prev_low = Some(low);
            self.prev_close = Some(close);
            return None;
        }

        let prev_high = self.prev_high.unwrap();
        let prev_low = self.prev_low.unwrap();
        let prev_close = self.prev_close.unwrap();

        // 计算真实波幅 TR
        let tr = self.calculate_tr(high, low, prev_close);

        // 计算动向指标 +DM 和 -DM
        let (plus_dm, minus_dm) = self.calculate_dm(high, low, prev_high, prev_low);

        // 平滑TR和DM
        if self.smoothed_tr.is_none() {
            // 初始化：使用第一个period的简单平均
            self.smoothed_tr = Some(tr);
            self.smoothed_plus_dm = Some(plus_dm);
            self.smoothed_minus_dm = Some(minus_dm);
        } else {
            // Wilder's smoothing: (prev * (period - 1) + current) / period
            let smoothed_tr = (self.smoothed_tr.unwrap() * (self.period - 1) as f64 + tr) 
                / self.period as f64;
            let smoothed_plus_dm = (self.smoothed_plus_dm.unwrap() * (self.period - 1) as f64 + plus_dm) 
                / self.period as f64;
            let smoothed_minus_dm = (self.smoothed_minus_dm.unwrap() * (self.period - 1) as f64 + minus_dm) 
                / self.period as f64;

            self.smoothed_tr = Some(smoothed_tr);
            self.smoothed_plus_dm = Some(smoothed_plus_dm);
            self.smoothed_minus_dm = Some(smoothed_minus_dm);
        }

        // 更新前值
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.prev_close = Some(close);

        // 需要足够的数据才能计算ADX
        if self.count < self.period + 1 {
            return None;
        }

        // 计算+DI和-DI
        let smoothed_tr = self.smoothed_tr.unwrap();
        let plus_di = if smoothed_tr > 0.0 {
            100.0 * self.smoothed_plus_dm.unwrap() / smoothed_tr
        } else {
            0.0
        };
        let minus_di = if smoothed_tr > 0.0 {
            100.0 * self.smoothed_minus_dm.unwrap() / smoothed_tr
        } else {
            0.0
        };

        // 计算DX
        let di_sum = plus_di + minus_di;
        let dx = if di_sum > 0.0 {
            100.0 * (plus_di - minus_di).abs() / di_sum
        } else {
            0.0
        };

        // ADX是DX的EMA
        let adx = self.adx_ema.update(dx);

        Some(ADXOutput {
            adx,
            plus_di,
            minus_di,
        })
    }

    /// 计算真实波幅
    fn calculate_tr(&self, high: f64, low: f64, prev_close: f64) -> f64 {
        let hl = high - low;
        let hc = (high - prev_close).abs();
        let lc = (low - prev_close).abs();
        hl.max(hc).max(lc)
    }

    /// 计算动向指标
    fn calculate_dm(&self, high: f64, low: f64, prev_high: f64, prev_low: f64) -> (f64, f64) {
        let up_move = high - prev_high;
        let down_move = prev_low - low;

        let plus_dm = if up_move > down_move && up_move > 0.0 {
            up_move
        } else {
            0.0
        };

        let minus_dm = if down_move > up_move && down_move > 0.0 {
            down_move
        } else {
            0.0
        };

        (plus_dm, minus_dm)
    }

    /// 获取当前的ADX值
    pub fn value(&self) -> Option<ADXOutput> {
        if self.count < self.period + 1 {
            return None;
        }

        let smoothed_tr = self.smoothed_tr?;
        let plus_di = if smoothed_tr > 0.0 {
            100.0 * self.smoothed_plus_dm? / smoothed_tr
        } else {
            0.0
        };
        let minus_di = if smoothed_tr > 0.0 {
            100.0 * self.smoothed_minus_dm? / smoothed_tr
        } else {
            0.0
        };

        let adx = self.adx_ema.value()?;

        Some(ADXOutput {
            adx,
            plus_di,
            minus_di,
        })
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.prev_high = None;
        self.prev_low = None;
        self.prev_close = None;
        self.smoothed_tr = None;
        self.smoothed_plus_dm = None;
        self.smoothed_minus_dm = None;
        self.adx_ema.reset();
        self.count = 0;
    }

    /// 获取周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.count >= self.period + 1 && self.adx_ema.is_ready()
    }

    /// 判断是否为强趋势 (ADX > 25)
    pub fn is_strong_trend(&self) -> bool {
        self.value().map_or(false, |output| output.adx > 25.0)
    }

    /// 判断是否为弱趋势或盘整 (ADX < 20)
    pub fn is_weak_trend(&self) -> bool {
        self.value().map_or(false, |output| output.adx < 20.0)
    }

    /// 判断是否为上升趋势 (+DI > -DI)
    pub fn is_uptrend(&self) -> bool {
        self.value().map_or(false, |output| output.plus_di > output.minus_di)
    }

    /// 判断是否为下降趋势 (-DI > +DI)
    pub fn is_downtrend(&self) -> bool {
        self.value().map_or(false, |output| output.minus_di > output.plus_di)
    }
}

#[cfg(test)]
mod tests;
