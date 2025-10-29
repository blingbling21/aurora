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

//! 数据下载任务状态管理

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 数据下载任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    /// 等待执行
    Pending,
    /// 正在下载
    Downloading,
    /// 已完成
    Completed,
    /// 执行失败
    Failed,
}

/// 数据下载任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    /// 任务ID
    pub id: Uuid,
    /// 交易所
    pub exchange: String,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: String,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 输出文件名
    pub filename: String,
    /// 任务状态
    pub status: DownloadStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 进度(0-100)
    pub progress: u8,
    /// 当前进度描述
    pub progress_message: String,
    /// 已获取数据条数
    pub downloaded_count: usize,
    /// 预计总数据条数
    pub estimated_total: Option<usize>,
    /// 错误信息
    pub error: Option<String>,
}

impl DownloadTask {
    /// 创建新任务
    pub fn new(
        exchange: String,
        symbol: String,
        interval: String,
        start_date: String,
        end_date: String,
        filename: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            exchange,
            symbol,
            interval,
            start_date,
            end_date,
            filename,
            status: DownloadStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            progress: 0,
            progress_message: "等待开始...".to_string(),
            downloaded_count: 0,
            estimated_total: None,
            error: None,
        }
    }

    /// 标记任务开始
    pub fn start(&mut self) {
        self.status = DownloadStatus::Downloading;
        self.started_at = Some(Utc::now());
        self.progress_message = "正在连接交易所...".to_string();
    }

    /// 更新进度
    pub fn update_progress(
        &mut self,
        downloaded_count: usize,
        estimated_total: Option<usize>,
        message: String,
    ) {
        self.downloaded_count = downloaded_count;
        self.estimated_total = estimated_total;
        self.progress_message = message;

        // 计算进度百分比
        if let Some(total) = estimated_total {
            if total > 0 {
                self.progress = ((downloaded_count as f64 / total as f64) * 100.0).min(100.0) as u8;
            }
        }
    }

    /// 标记任务完成
    pub fn complete(&mut self, total_count: usize) {
        self.status = DownloadStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress = 100;
        self.downloaded_count = total_count;
        self.progress_message = format!("下载完成! 共获取 {} 条数据", total_count);
    }

    /// 标记任务失败
    pub fn fail(&mut self, error: String) {
        self.status = DownloadStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(error.clone());
        self.progress_message = format!("下载失败: {}", error);
    }
}
