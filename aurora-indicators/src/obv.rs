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

//! OBV (On-Balance Volume) 能量潮指标
//!
//! 通过成交量变化预测价格趋势。

/// OBV (On-Balance Volume) 能量潮指标
///
/// OBV通过累积成交量来衡量买卖压力。
///
/// # 算法原理
///
/// - 如果当前收盘价 > 前收盘价：OBV += 当前成交量
/// - 如果当前收盘价 < 前收盘价：OBV -= 当前成交量
/// - 如果当前收盘价 = 前收盘价：OBV 不变
///
/// # 用途
///
/// - 确认价格趋势
/// - 发现背离信号（价格创新高但OBV未创新高，或相反）
/// - 衡量买卖力量
#[derive(Debug, Clone)]
pub struct OBV {
    /// 当前OBV值
    obv: f64,
    /// 前一根K线的收盘价
    prev_close: Option<f64>,
}

impl OBV {
    /// 创建新的OBV指标
    pub fn new() -> Self {
        Self {
            obv: 0.0,
            prev_close: None,
        }
    }

    /// 更新指标并返回最新的OBV值
    ///
    /// # 参数
    ///
    /// * `close` - 当前K线收盘价
    /// * `volume` - 当前K线成交量
    ///
    /// # 返回值
    ///
    /// 返回更新后的OBV值
    pub fn update(&mut self, close: f64, volume: f64) -> f64 {
        match self.prev_close {
            None => {
                // 第一根K线，OBV初始化为0
                self.prev_close = Some(close);
                self.obv
            }
            Some(prev) => {
                // 根据价格变化更新OBV
                if close > prev {
                    self.obv += volume;
                } else if close < prev {
                    self.obv -= volume;
                }
                // 价格不变时OBV不变
                
                self.prev_close = Some(close);
                self.obv
            }
        }
    }

    /// 获取当前的OBV值
    pub fn value(&self) -> f64 {
        self.obv
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.obv = 0.0;
        self.prev_close = None;
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.prev_close.is_none()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.prev_close.is_some()
    }

    /// 检查OBV是否上升（相对于给定值）
    pub fn is_rising(&self, prev_obv: f64) -> bool {
        self.obv > prev_obv
    }

    /// 检查OBV是否下降（相对于给定值）
    pub fn is_falling(&self, prev_obv: f64) -> bool {
        self.obv < prev_obv
    }
}

impl Default for OBV {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
