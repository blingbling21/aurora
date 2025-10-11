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

//! Aurora 技术指标库
//!
//! 提供各种技术分析指标的计算功能，这些指标是构建量化交易策略的基础组件。
//!
//! # 支持的指标
//!
//! ## 趋势指标
//! - **MA (移动平均线)**: 简单移动平均线,用于识别趋势方向
//! - **EMA (指数移动平均线)**: 对近期数据赋予更高权重的移动平均线
//! - **MACD (移动平均收敛散度)**: 用于判断趋势变化和买卖时机
//! - **ADX (平均动向指数)**: 衡量趋势强度但不判断方向
//! - **PSAR (抛物线转向指标)**: 用于确定止损位和趋势反转点
//! - **Ichimoku (一目均衡表)**: 综合性趋势系统，提供多维度市场信息
//!
//! ## 动量指标
//! - **RSI (相对强弱指数)**: 衡量价格变动速度和幅度
//! - **Stochastic (随机震荡指标)**: 比较收盘价与价格区间的位置
//! - **ROC (变动率指标)**: 衡量当前价格相对于N周期前的变化百分比
//! - **CCI (商品通道指数)**: 衡量价格相对于统计平均值的偏离程度
//! - **Williams %R (威廉指标)**: 衡量收盘价在N周期高低区间中的相对位置
//!
//! ## 波动率指标
//! - **Bollinger Bands (布林带)**: 基于标准差的价格通道
//! - **ATR (平均真实波幅)**: 衡量市场波动程度
//! - **StdDev (标准差)**: 衡量价格相对于平均值的离散程度
//! - **Keltner Channels (肯特纳通道)**: 基于ATR的价格通道
//!
//! ## 成交量指标
//! - **OBV (能量潮)**: 通过成交量变化预测价格趋势
//! - **MFI (资金流量指数)**: 成交量加权的RSI
//! - **VWAP (成交量加权平均价)**: 计算成交量加权的平均价格
//! - **CMF (佳庆资金流)**: 衡量特定时期内资金流入和流出情况
//! - **ADLine (累积/派发线)**: 通过价格和成交量关系识别供需变化
//!
//! # 设计原则
//!
//! - **状态管理**: 每个指标维护自己的内部状态，支持流式数据处理
//! - **内存效率**: 使用滑动窗口避免存储过多历史数据
//! - **类型安全**: 利用Rust类型系统确保计算正确性
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::MA;
//!
//! let mut ma = MA::new(5); // 5周期移动平均线
//!
//! // 前4个数据点不会产生结果
//! assert_eq!(ma.update(100.0), None);
//! assert_eq!(ma.update(102.0), None);
//! assert_eq!(ma.update(98.0), None);
//! assert_eq!(ma.update(105.0), None);
//!
//! // 第5个数据点开始产生移动平均值
//! let avg = ma.update(95.0).unwrap();
//! assert_eq!(avg, 100.0); // (100+102+98+105+95)/5 = 100
//! ```

// 模块导出
mod ma;
mod ema;
mod rsi;
mod bollinger;
mod macd;
mod atr;
mod stochastic;
mod obv;
mod adx;
mod roc;
mod cci;
mod williams_r;
mod stddev;
mod keltner;
mod mfi;
mod vwap;
mod psar;
mod ichimoku;
mod cmf;
mod adline;

// 公开导出所有指标
pub use ma::MA;
pub use ema::EMA;
pub use rsi::RSI;
pub use bollinger::{BollingerBands, BollingerBandsOutput};
pub use macd::{MACD, MACDOutput};
pub use atr::ATR;
pub use stochastic::{Stochastic, StochasticOutput};
pub use obv::OBV;
pub use adx::{ADX, ADXOutput};
pub use roc::ROC;
pub use cci::CCI;
pub use williams_r::WilliamsR;
pub use stddev::StdDev;
pub use keltner::{KeltnerChannels, KeltnerChannelsOutput};
pub use mfi::MFI;
pub use vwap::VWAP;
pub use psar::{PSAR, PSAROutput};
pub use ichimoku::{Ichimoku, IchimokuOutput};
pub use cmf::CMF;
pub use adline::ADLine;
