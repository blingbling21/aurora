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

//! 回测任务状态管理

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 回测任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BacktestStatus {
    /// 等待执行
    Pending,
    /// 正在运行
    Running,
    /// 已完成
    Completed,
    /// 执行失败
    Failed,
}

/// 回测任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestTask {
    /// 任务ID
    pub id: Uuid,
    /// 任务名称
    pub name: String,
    /// 配置文件路径
    pub config_path: String,
    /// 数据文件路径
    pub data_path: String,
    /// 任务状态
    pub status: BacktestStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 进度(0-100)
    pub progress: u8,
    /// 错误信息
    pub error: Option<String>,
    /// 回测结果(JSON格式)
    pub result: Option<serde_json::Value>,
}

impl BacktestTask {
    /// 创建新任务
    pub fn new(name: String, config_path: String, data_path: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            config_path,
            data_path,
            status: BacktestStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            progress: 0,
            error: None,
            result: None,
        }
    }

    /// 标记任务开始
    pub fn start(&mut self) {
        self.status = BacktestStatus::Running;
        self.started_at = Some(Utc::now());
    }

    /// 更新进度
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }

    /// 标记任务完成
    pub fn complete(&mut self, result: serde_json::Value) {
        self.status = BacktestStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress = 100;
        self.result = Some(result);
    }

    /// 标记任务失败
    pub fn fail(&mut self, error: String) {
        self.status = BacktestStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(error);
    }
}
