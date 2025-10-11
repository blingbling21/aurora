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

//! Parabolic SAR (抛物线转向指标)
//!
//! Parabolic SAR 是一个趋势跟踪指标，由 J. Welles Wilder Jr. 开发。
//! 它在图表上以点的形式显示在价格上方或下方，用于确定潜在的止损位和趋势反转点。
//!
//! # 工作原理
//!
//! - 当 SAR 在价格下方时，表示上升趋势
//! - 当 SAR 在价格上方时，表示下降趋势
//! - SAR 点位翻转时，表示趋势可能反转
//!
//! # 计算公式
//!
//! - SAR(当前) = SAR(前一个) + AF × (EP - SAR(前一个))
//! - AF: 加速因子，起始值为 0.02，每次创新高/低增加 0.02，最大值为 0.20
//! - EP: 极值点，上升趋势中的最高价或下降趋势中的最低价
//!
//! # 参数
//!
//! - `acceleration`: 加速因子起始值 (默认: 0.02)
//! - `max_acceleration`: 加速因子最大值 (默认: 0.20)
//!
//! # 示例
//!
//! ```
//! use aurora_indicators::PSAR;
//!
//! let mut psar = PSAR::default();
//!
//! // 输入高价、低价、收盘价
//! let result1 = psar.update(100.0, 95.0, 98.0);
//! let result2 = psar.update(102.0, 97.0, 101.0);
//! let result3 = psar.update(105.0, 100.0, 104.0);
//!
//! if let Some(sar_value) = result3 {
//!     println!("当前 SAR 值: {:.2}", sar_value);
//! }
//! ```

/// Parabolic SAR 输出结构
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PSAROutput {
    /// SAR 值
    pub sar: f64,
    /// 当前趋势: true=上升, false=下降
    pub is_uptrend: bool,
}

/// Parabolic SAR 指标结构
pub struct PSAR {
    // 参数
    acceleration: f64,        // 加速因子起始值
    max_acceleration: f64,    // 加速因子最大值
    
    // 状态
    current_sar: Option<f64>,      // 当前 SAR 值
    is_uptrend: bool,              // 当前趋势方向
    extreme_point: f64,            // 极值点 (EP)
    acceleration_factor: f64,      // 当前加速因子 (AF)
    
    // 历史数据（用于初始化）
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
    initialized: bool,
}

impl PSAR {
    /// 创建新的 PSAR 指标
    ///
    /// # 参数
    ///
    /// - `acceleration`: 加速因子起始值
    /// - `max_acceleration`: 加速因子最大值
    pub fn new(acceleration: f64, max_acceleration: f64) -> Self {
        Self {
            acceleration,
            max_acceleration,
            current_sar: None,
            is_uptrend: true,
            extreme_point: 0.0,
            acceleration_factor: acceleration,
            highs: Vec::new(),
            lows: Vec::new(),
            closes: Vec::new(),
            initialized: false,
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
    /// 返回 PSAROutput 结构，包含 SAR 值和趋势方向
    pub fn update(&mut self, high: f64, low: f64, close: f64) -> Option<PSAROutput> {
        // 初始化阶段：收集至少2个数据点
        if !self.initialized {
            self.highs.push(high);
            self.lows.push(low);
            self.closes.push(close);
            
            if self.highs.len() < 2 {
                return None;
            }
            
            // 初始化趋势和 SAR
            // 如果第二根K线收盘价高于第一根，则认为是上升趋势
            self.is_uptrend = self.closes[1] >= self.closes[0];
            
            if self.is_uptrend {
                // 上升趋势：SAR 从最低点开始
                self.current_sar = Some(self.lows.iter().copied().fold(f64::INFINITY, f64::min));
                self.extreme_point = self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            } else {
                // 下降趋势：SAR 从最高点开始
                self.current_sar = Some(self.highs.iter().copied().fold(f64::NEG_INFINITY, f64::max));
                self.extreme_point = self.lows.iter().copied().fold(f64::INFINITY, f64::min);
            }
            
            self.acceleration_factor = self.acceleration;
            self.initialized = true;
            
            return Some(PSAROutput {
                sar: self.current_sar.unwrap(),
                is_uptrend: self.is_uptrend,
            });
        }
        
        let mut sar = self.current_sar.unwrap();
        let mut is_uptrend = self.is_uptrend;
        let mut extreme_point = self.extreme_point;
        let mut acceleration_factor = self.acceleration_factor;
        
        // 检查是否发生趋势反转
        let reversal = if is_uptrend {
            low < sar  // 上升趋势中，价格跌破 SAR
        } else {
            high > sar  // 下降趋势中，价格突破 SAR
        };
        
        if reversal {
            // 发生反转
            is_uptrend = !is_uptrend;
            sar = extreme_point;  // SAR 设置为上一个极值点
            extreme_point = if is_uptrend { high } else { low };
            acceleration_factor = self.acceleration;  // 重置加速因子
        } else {
            // 未发生反转，更新 SAR
            sar = sar + acceleration_factor * (extreme_point - sar);
            
            // 更新极值点和加速因子
            if is_uptrend {
                // 上升趋势：检查新高
                if high > extreme_point {
                    extreme_point = high;
                    acceleration_factor = (acceleration_factor + self.acceleration)
                        .min(self.max_acceleration);
                }
                // SAR 不能高于前两个周期的最低价
                sar = sar.min(low);
            } else {
                // 下降趋势：检查新低
                if low < extreme_point {
                    extreme_point = low;
                    acceleration_factor = (acceleration_factor + self.acceleration)
                        .min(self.max_acceleration);
                }
                // SAR 不能低于前两个周期的最高价
                sar = sar.max(high);
            }
        }
        
        // 更新状态
        self.current_sar = Some(sar);
        self.is_uptrend = is_uptrend;
        self.extreme_point = extreme_point;
        self.acceleration_factor = acceleration_factor;
        
        Some(PSAROutput {
            sar,
            is_uptrend,
        })
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.current_sar = None;
        self.is_uptrend = true;
        self.extreme_point = 0.0;
        self.acceleration_factor = self.acceleration;
        self.highs.clear();
        self.lows.clear();
        self.closes.clear();
        self.initialized = false;
    }
}

impl Default for PSAR {
    /// 使用默认参数创建 PSAR 指标
    ///
    /// - acceleration: 0.02
    /// - max_acceleration: 0.20
    fn default() -> Self {
        Self::new(0.02, 0.20)
    }
}

#[cfg(test)]
mod tests;
