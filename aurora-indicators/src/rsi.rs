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

//! 相对强弱指数 (Relative Strength Index) 指标
//!
//! RSI衡量价格变动的速度和幅度，用于判断超买超卖状态。

use crate::EMA;

/// 相对强弱指数 (RSI) 指标
///
/// RSI通过比较一段时期内的平均涨幅和平均跌幅来衡量价格动量。
/// RSI值在0-100之间，通常70以上表示超买，30以下表示超卖。
///
/// # 算法原理
///
/// RSI = 100 - (100 / (1 + RS))
/// 其中 RS = 平均涨幅 / 平均跌幅
///
/// 平均涨幅和平均跌幅使用EMA计算
///
/// # 时间复杂度
///
/// 每次更新的时间复杂度为O(1)
///
/// # 内存复杂度
///
/// 空间复杂度为O(1)
#[derive(Debug, Clone)]
pub struct RSI {
    /// RSI周期长度
    period: usize,
    /// 上涨幅度的EMA
    avg_gain: EMA,
    /// 下跌幅度的EMA
    avg_loss: EMA,
    /// 上一个价格，用于计算涨跌幅
    prev_price: Option<f64>,
    /// 当前RSI值
    current_rsi: Option<f64>,
}

impl RSI {
    /// 创建新的RSI指标
    ///
    /// # 参数
    ///
    /// * `period` - RSI周期，通常使用14，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "RSI周期必须大于0");

        Self {
            period,
            avg_gain: EMA::new(period),
            avg_loss: EMA::new(period),
            prev_price: None,
            current_rsi: None,
        }
    }

    /// 更新指标并返回最新的RSI值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果有上一个价格可以计算涨跌，返回RSI值（0-100）
    /// * `None` - 如果是第一个价格，返回None
    pub fn update(&mut self, price: f64) -> Option<f64> {
        match self.prev_price {
            None => {
                // 第一个价格，无法计算涨跌
                self.prev_price = Some(price);
                None
            }
            Some(prev) => {
                // 计算涨跌幅
                let change = price - prev;
                let gain = if change > 0.0 { change } else { 0.0 };
                let loss = if change < 0.0 { -change } else { 0.0 };

                // 更新平均涨跌幅
                let avg_gain = self.avg_gain.update(gain);
                let avg_loss = self.avg_loss.update(loss);

                // 计算RSI
                let rsi = if avg_loss == 0.0 {
                    // 如果平均跌幅为0，RSI为100
                    100.0
                } else {
                    let rs = avg_gain / avg_loss;
                    100.0 - (100.0 / (1.0 + rs))
                };

                self.prev_price = Some(price);
                self.current_rsi = Some(rsi);
                Some(rsi)
            }
        }
    }

    /// 获取当前的RSI值
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有RSI值，返回0-100之间的值
    /// * `None` - 如果还没有足够数据，返回None
    pub fn value(&self) -> Option<f64> {
        self.current_rsi
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.avg_gain.reset();
        self.avg_loss.reset();
        self.prev_price = None;
        self.current_rsi = None;
    }

    /// 获取周期设置
    pub fn period(&self) -> usize {
        self.period
    }

    /// 检查指标是否为空
    pub fn is_empty(&self) -> bool {
        self.prev_price.is_none()
    }

    /// 检查指标是否已准备好
    pub fn is_ready(&self) -> bool {
        self.current_rsi.is_some()
    }

    /// 判断是否超买（RSI > 70）
    pub fn is_overbought(&self) -> bool {
        self.current_rsi.map_or(false, |rsi| rsi > 70.0)
    }

    /// 判断是否超卖（RSI < 30）
    pub fn is_oversold(&self) -> bool {
        self.current_rsi.map_or(false, |rsi| rsi < 30.0)
    }
}

#[cfg(test)]
mod tests;
