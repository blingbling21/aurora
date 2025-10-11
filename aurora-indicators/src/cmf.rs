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

//! Chaikin Money Flow (佳庆资金流)
//!
//! Chaikin Money Flow (CMF) 是由 Marc Chaikin 开发的成交量加权指标，
//! 用于衡量特定时期内资金流入和流出的情况。
//!
//! # 工作原理
//!
//! CMF 结合了价格和成交量信息，通过分析收盘价在当日高低价区间中的位置，
//! 以及对应的成交量，来判断市场的买卖压力。
//!
//! # 计算公式
//!
//! 1. Money Flow Multiplier = [(Close - Low) - (High - Close)] / (High - Low)
//! 2. Money Flow Volume = Money Flow Multiplier × Volume
//! 3. CMF = Sum(Money Flow Volume, n) / Sum(Volume, n)
//!
//! # 解读
//!
//! - CMF > 0: 买方压力占优，资金流入
//! - CMF < 0: 卖方压力占优，资金流出
//! - CMF 接近 +1: 强烈的买方压力
//! - CMF 接近 -1: 强烈的卖方压力
//! - CMF 接近 0: 市场平衡
//!
//! # 参数
//!
//! - `period`: 计算周期 (默认: 20)
//!
//! # 示例
//!
//! ```
//! use aurora_indicators::CMF;
//!
//! let mut cmf = CMF::new(20);
//!
//! // 输入高价、低价、收盘价、成交量
//! let result = cmf.update(105.0, 95.0, 102.0, 10000.0);
//!
//! if let Some(value) = result {
//!     if value > 0.0 {
//!         println!("资金流入: {:.4}", value);
//!     } else {
//!         println!("资金流出: {:.4}", value);
//!     }
//! }
//! ```

use std::collections::VecDeque;

/// Chaikin Money Flow 指标结构
pub struct CMF {
    // 参数
    period: usize,
    
    // 历史数据缓冲区
    money_flow_volumes: VecDeque<f64>,  // Money Flow Volume 历史
    volumes: VecDeque<f64>,              // 成交量历史
}

impl CMF {
    /// 创建新的 CMF 指标
    ///
    /// # 参数
    ///
    /// - `period`: 计算周期
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "CMF period must be greater than 0");
        
        Self {
            period,
            money_flow_volumes: VecDeque::new(),
            volumes: VecDeque::new(),
        }
    }

    /// 更新指标数据
    ///
    /// # 参数
    ///
    /// - `high`: 当前周期最高价
    /// - `low`: 当前周期最低价
    /// - `close`: 当前周期收盘价
    /// - `volume`: 当前周期成交量
    ///
    /// # 返回值
    ///
    /// 返回 CMF 值，范围在 -1 到 +1 之间
    /// 当数据不足时返回 None
    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        // 计算 Money Flow Multiplier
        let mf_multiplier = if high == low {
            // 避免除以零：如果高低价相同，设为0（表示无方向性）
            0.0
        } else {
            ((close - low) - (high - close)) / (high - low)
        };
        
        // 计算 Money Flow Volume
        let mf_volume = mf_multiplier * volume;
        
        // 添加到缓冲区
        self.money_flow_volumes.push_back(mf_volume);
        self.volumes.push_back(volume);
        
        // 维持窗口大小
        if self.money_flow_volumes.len() > self.period {
            self.money_flow_volumes.pop_front();
            self.volumes.pop_front();
        }
        
        // 需要足够的数据才能计算
        if self.money_flow_volumes.len() < self.period {
            return None;
        }
        
        // 计算 CMF
        let sum_mf_volume: f64 = self.money_flow_volumes.iter().sum();
        let sum_volume: f64 = self.volumes.iter().sum();
        
        // 避免除以零
        if sum_volume == 0.0 {
            return Some(0.0);
        }
        
        Some(sum_mf_volume / sum_volume)
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.money_flow_volumes.clear();
        self.volumes.clear();
    }

    /// 获取当前周期
    pub fn period(&self) -> usize {
        self.period
    }
}

impl Default for CMF {
    /// 使用默认参数创建 CMF 指标
    ///
    /// - period: 20
    fn default() -> Self {
        Self::new(20)
    }
}

#[cfg(test)]
mod tests;
