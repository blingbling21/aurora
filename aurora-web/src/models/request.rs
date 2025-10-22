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

//! 请求数据模型

use serde::{Deserialize, Serialize};

/// 启动回测请求
#[derive(Debug, Deserialize, Serialize)]
pub struct StartBacktestRequest {
    /// 任务名称
    pub name: String,
    /// 配置文件路径(相对于config_dir)
    pub config_path: String,
    /// 数据文件路径(相对于data_dir)
    pub data_path: String,
}

/// 创建配置请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateConfigRequest {
    /// 配置文件名
    pub filename: String,
    /// 配置内容(TOML格式)
    pub content: String,
}

/// 更新配置请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateConfigRequest {
    /// 配置内容(TOML格式)
    pub content: String,
}

/// 获取历史数据请求
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchDataRequest {
    /// 交易所名称 (binance, okx, bybit, coinbase等)
    pub exchange: String,
    /// 交易对 (例如: BTCUSDT)
    pub symbol: String,
    /// 时间周期 (1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w等)
    pub interval: String,
    /// 开始日期 (格式: YYYY-MM-DD)
    pub start_date: String,
    /// 结束日期 (格式: YYYY-MM-DD)
    pub end_date: String,
    /// 可选的文件名 (如果为空则自动生成)
    pub filename: Option<String>,
}
