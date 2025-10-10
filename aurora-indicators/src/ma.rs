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

//! 移动平均线 (Moving Average) 指标
//!
//! 计算指定周期内价格的算术平均值，是最基础的趋势跟踪指标。

use std::collections::VecDeque;

/// 移动平均线 (Moving Average) 指标
///
/// 计算指定周期内价格的算术平均值，是最基础的趋势跟踪指标。
/// 移动平均线能够平滑价格波动，帮助识别趋势方向。
///
/// # 算法原理
///
/// 对于周期为N的移动平均线，在时刻t的值为：
/// MA(t) = (P(t) + P(t-1) + ... + P(t-N+1)) / N
///
/// 其中P(t)表示时刻t的价格。
///
/// # 内存复杂度
///
/// 空间复杂度为O(N)，其中N是周期长度。使用双端队列(VecDeque)
/// 实现滑动窗口，确保内存使用效率。
///
/// # 时间复杂度
///
/// 每次更新的时间复杂度为O(1)，通过维护累计和避免重复计算。
#[derive(Debug, Clone)]
pub struct MA {
    /// 移动平均的周期长度
    period: usize,
    /// 存储最近N个价格的滑动窗口
    values: VecDeque<f64>,
    /// 当前窗口内所有价格的累计和
    sum: f64,
}

impl MA {
    /// 创建新的移动平均线指标
    ///
    /// # 参数
    ///
    /// * `period` - 移动平均的周期，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "移动平均周期必须大于0");

        Self {
            period,
            values: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    /// 更新指标并返回最新的移动平均值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有足够的数据点（>= period），返回移动平均值
    /// * `None` - 如果数据点不足，返回None
    pub fn update(&mut self, price: f64) -> Option<f64> {
        // 添加新的价格到队列尾部
        self.values.push_back(price);
        self.sum += price;

        // 如果超出了周期长度，移除最旧的值
        if self.values.len() > self.period {
            if let Some(old_value) = self.values.pop_front() {
                self.sum -= old_value;
            }
        }

        // 只有当有足够的数据点时才返回平均值
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// 获取当前的移动平均值（如果可用）
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有足够的数据点，返回当前移动平均值
    /// * `None` - 如果数据点不足，返回None
    pub fn value(&self) -> Option<f64> {
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// 重置指标状态
    ///
    /// 清空所有历史数据，将指标恢复到初始状态。
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }

    /// 获取当前的周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 获取当前已接收的数据点数量
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// 检查指标是否为空（没有数据）
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// 检查指标是否已准备好（有足够的数据）
    pub fn is_ready(&self) -> bool {
        self.values.len() == self.period
    }
}

#[cfg(test)]
mod tests;
