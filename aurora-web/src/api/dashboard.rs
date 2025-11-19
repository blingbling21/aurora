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

//! 仪表盘API

use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::WebResult;
use crate::models::SuccessResponse;
use crate::state::{AppState, BacktestTask, BacktestTaskSummary, BacktestStatus};

/// 仪表盘路由
pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(get_dashboard_data))
}

/// 仪表盘统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
}

/// 仪表盘数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub stats: DashboardStats,
    pub recent_tasks: Vec<BacktestTaskSummary>,
}

/// 获取仪表盘数据
async fn get_dashboard_data(
    State(state): State<AppState>,
) -> WebResult<Json<SuccessResponse<DashboardData>>> {
    debug!("获取仪表盘数据");

    // 读取所有回测任务
    let tasks = state.backtest_tasks.read().await;
    let all_tasks: Vec<BacktestTask> = tasks.values().cloned().collect();

    // 统计各状态的任务数量
    let stats = DashboardStats {
        total_tasks: all_tasks.len(),
        running_tasks: all_tasks.iter().filter(|t| t.status == BacktestStatus::Running).count(),
        completed_tasks: all_tasks.iter().filter(|t| t.status == BacktestStatus::Completed).count(),
        failed_tasks: all_tasks.iter().filter(|t| t.status == BacktestStatus::Failed).count(),
    };

    // 获取最近的任务（按创建时间排序，最多返回10个）
    let mut recent_tasks = all_tasks;
    recent_tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    recent_tasks.truncate(10);
    
    // 转换为摘要格式（不包含完整的回测结果）
    let recent_tasks_summary: Vec<BacktestTaskSummary> = recent_tasks.iter().map(|t| t.into()).collect();

    let dashboard_data = DashboardData {
        stats,
        recent_tasks: recent_tasks_summary,
    };

    info!(
        "仪表盘数据: 总任务数={}, 运行中={}, 已完成={}, 失败={}",
        dashboard_data.stats.total_tasks,
        dashboard_data.stats.running_tasks,
        dashboard_data.stats.completed_tasks,
        dashboard_data.stats.failed_tasks
    );

    Ok(Json(SuccessResponse::new(dashboard_data)))
}
