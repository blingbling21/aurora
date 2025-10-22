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

//! 响应数据模型

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 通用成功响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

/// 配置列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigListItem {
    pub filename: String,
    pub path: String,
    pub modified: String,
}

/// 数据文件列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFileItem {
    pub filename: String,
    pub path: String,
    pub size: u64,
    pub modified: String,
    pub record_count: Option<usize>,
}

/// 回测任务创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BacktestCreateResponse {
    pub task_id: Uuid,
    pub message: String,
}

/// 配置验证响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigValidateResponse {
    pub valid: bool,
    pub errors: Vec<String>,
}
