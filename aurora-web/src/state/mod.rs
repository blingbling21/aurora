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

//! 应用状态管理

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod backtest;
pub mod download;

pub use backtest::{BacktestTask, BacktestStatus};
pub use download::{DownloadTask, DownloadStatus};

/// 应用全局状态
#[derive(Clone)]
pub struct AppState {
    /// 回测任务映射表
    pub backtest_tasks: Arc<RwLock<HashMap<Uuid, BacktestTask>>>,
    /// 数据下载任务映射表
    pub download_tasks: Arc<RwLock<HashMap<Uuid, DownloadTask>>>,
    /// 数据文件目录
    pub data_dir: PathBuf,
    /// 配置文件目录
    pub config_dir: PathBuf,
}
