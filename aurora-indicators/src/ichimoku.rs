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

//! Ichimoku Cloud (一目均衡表)
//!
//! Ichimoku Kinko Hyo（一目均衡表）是由日本记者细田悟一在1930年代开发的综合性技术指标系统。
//! 它提供了多维度的市场信息，包括趋势方向、支撑/阻力位、动量和交易信号。
//!
//! # 组成部分
//!
//! Ichimoku 由五条线组成：
//!
//! 1. **转换线 (Tenkan-sen)**: (9周期最高价 + 9周期最低价) / 2
//! 2. **基准线 (Kijun-sen)**: (26周期最高价 + 26周期最低价) / 2
//! 3. **先行带A (Senkou Span A)**: (转换线 + 基准线) / 2，向前移动26周期
//! 4. **先行带B (Senkou Span B)**: (52周期最高价 + 52周期最低价) / 2，向前移动26周期
//! 5. **滞后线 (Chikou Span)**: 收盘价，向后移动26周期
//!
//! # 云层 (Kumo)
//!
//! 先行带A和先行带B之间的区域称为"云层"，是Ichimoku最重要的特征：
//! - 当价格在云层上方时，表示上升趋势
//! - 当价格在云层下方时，表示下降趋势
//! - 当价格在云层内部时，表示盘整
//!
//! # 参数
//!
//! - `tenkan_period`: 转换线周期 (默认: 9)
//! - `kijun_period`: 基准线周期 (默认: 26)
//! - `senkou_b_period`: 先行带B周期 (默认: 52)
//!
//! # 示例
//!
//! ```
//! use aurora_indicators::Ichimoku;
//!
//! let mut ichimoku = Ichimoku::default();
//!
//! // 输入高价、低价、收盘价
//! let result = ichimoku.update(105.0, 95.0, 100.0);
//!
//! if let Some(output) = result {
//!     println!("转换线: {:.2}", output.tenkan_sen);
//!     println!("基准线: {:.2}", output.kijun_sen);
//!     println!("先行带A: {:.2}", output.senkou_span_a);
//!     println!("先行带B: {:.2}", output.senkou_span_b);
//!     println!("滞后线: {:.2}", output.chikou_span);
//! }
//! ```

use std::collections::VecDeque;

/// Ichimoku 输出结构
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IchimokuOutput {
    /// 转换线 (Tenkan-sen)
    pub tenkan_sen: f64,
    /// 基准线 (Kijun-sen)
    pub kijun_sen: f64,
    /// 先行带A (Senkou Span A) - 未来26周期的值
    pub senkou_span_a: f64,
    /// 先行带B (Senkou Span B) - 未来26周期的值
    pub senkou_span_b: f64,
    /// 滞后线 (Chikou Span) - 当前收盘价（显示在26周期前）
    pub chikou_span: f64,
}

/// Ichimoku 指标结构
pub struct Ichimoku {
    // 参数
    tenkan_period: usize,      // 转换线周期
    kijun_period: usize,       // 基准线周期
    senkou_b_period: usize,    // 先行带B周期
    displacement: usize,        // 位移周期（固定为26）
    
    // 历史数据缓冲区
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    closes: VecDeque<f64>,
}

impl Ichimoku {
    /// 创建新的 Ichimoku 指标
    ///
    /// # 参数
    ///
    /// - `tenkan_period`: 转换线周期
    /// - `kijun_period`: 基准线周期
    /// - `senkou_b_period`: 先行带B周期
    pub fn new(tenkan_period: usize, kijun_period: usize, senkou_b_period: usize) -> Self {
        Self {
            tenkan_period,
            kijun_period,
            senkou_b_period,
            displacement: 26,  // 标准位移固定为26
            highs: VecDeque::new(),
            lows: VecDeque::new(),
            closes: VecDeque::new(),
        }
    }

    /// 更新指标数据
    ///
    /// # 参数
    ///
    /// - `high`: 当前周期最高价
    /// - `low`: 当前周期最低价
    /// - `close`: 当前周期收盘价
    ///
    /// # 返回值
    ///
    /// 返回 IchimokuOutput 结构，包含所有五条线的值
    /// 当数据不足时返回 None
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<IchimokuOutput> {
        // 添加新数据
        self.highs.push_back(high);
        self.lows.push_back(low);
        self.closes.push_back(close);
        
        // 保持缓冲区大小（需要足够的历史数据）
        let max_len = self.senkou_b_period.max(self.displacement);
        if self.highs.len() > max_len {
            self.highs.pop_front();
            self.lows.pop_front();
            self.closes.pop_front();
        }
        
        // 需要至少 senkou_b_period 个数据点才能计算所有线
        if self.highs.len() < self.senkou_b_period {
            return None;
        }
        
        // 计算转换线 (Tenkan-sen)
        let tenkan_sen = self.calculate_midpoint(self.tenkan_period);
        
        // 计算基准线 (Kijun-sen)
        let kijun_sen = self.calculate_midpoint(self.kijun_period);
        
        // 计算先行带A (Senkou Span A)
        // 这是转换线和基准线的平均值，向前位移26周期
        let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0;
        
        // 计算先行带B (Senkou Span B)
        // 52周期的中点，向前位移26周期
        let senkou_span_b = self.calculate_midpoint(self.senkou_b_period);
        
        // 滞后线 (Chikou Span) 就是当前收盘价
        let chikou_span = close;
        
        Some(IchimokuOutput {
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        })
    }

    /// 计算指定周期内的中点 (最高价 + 最低价) / 2
    fn calculate_midpoint(&self, period: usize) -> f64 {
        let start_idx = self.highs.len().saturating_sub(period);
        
        let high_max = self.highs
            .iter()
            .skip(start_idx)
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        
        let low_min = self.lows
            .iter()
            .skip(start_idx)
            .copied()
            .fold(f64::INFINITY, f64::min);
        
        (high_max + low_min) / 2.0
    }

    /// 获取当前趋势状态
    ///
    /// 返回元组 (价格位置, 云层厚度)
    /// 价格位置: 1=云上, 0=云中, -1=云下
    pub fn get_trend_position(&self, current_price: f64) -> Option<(i8, f64)> {
        if self.highs.len() < self.senkou_b_period {
            return None;
        }
        
        let tenkan = self.calculate_midpoint(self.tenkan_period);
        let kijun = self.calculate_midpoint(self.kijun_period);
        let span_a = (tenkan + kijun) / 2.0;
        let span_b = self.calculate_midpoint(self.senkou_b_period);
        
        let cloud_top = span_a.max(span_b);
        let cloud_bottom = span_a.min(span_b);
        let cloud_thickness = cloud_top - cloud_bottom;
        
        let position = if current_price > cloud_top {
            1  // 云上 - 上升趋势
        } else if current_price < cloud_bottom {
            -1  // 云下 - 下降趋势
        } else {
            0  // 云中 - 盘整
        };
        
        Some((position, cloud_thickness))
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.closes.clear();
    }
}

impl Default for Ichimoku {
    /// 使用标准参数创建 Ichimoku 指标
    ///
    /// - tenkan_period: 9
    /// - kijun_period: 26
    /// - senkou_b_period: 52
    fn default() -> Self {
        Self::new(9, 26, 52)
    }
}

#[cfg(test)]
mod tests;
