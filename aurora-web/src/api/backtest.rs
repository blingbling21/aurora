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

//! 回测API

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use tracing::{debug, info};
use uuid::Uuid;

use crate::error::{WebError, WebResult};
use crate::models::SuccessResponse;
use crate::state::{AppState, BacktestTask};

/// 回测路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_backtests).post(create_backtest))
        .route("/history", get(list_backtests)) // 添加 /history 路由，指向列出所有回测任务
        .route("/start", post(create_and_start_backtest)) // 添加直接创建并启动的端点
        .route("/{id}", get(get_backtest).delete(delete_backtest))
        .route("/{id}/start", post(start_backtest))
}

/// 列出所有回测任务
async fn list_backtests(
    State(state): State<AppState>,
) -> WebResult<Json<SuccessResponse<Vec<BacktestTask>>>> {
    debug!("列出所有回测任务");

    let tasks = state.backtest_tasks.read().await;
    let task_list: Vec<BacktestTask> = tasks.values().cloned().collect();

    info!("找到 {} 个回测任务", task_list.len());
    Ok(Json(SuccessResponse::new(task_list)))
}

/// 获取指定回测任务
async fn get_backtest(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> WebResult<Json<SuccessResponse<BacktestTask>>> {
    debug!("获取回测任务: {}", id);

    let tasks = state.backtest_tasks.read().await;
    let task = tasks
        .get(&id)
        .ok_or_else(|| WebError::TaskNotFound(format!("任务不存在: {}", id)))?
        .clone();

    Ok(Json(SuccessResponse::new(task)))
}

/// 创建回测任务
#[derive(serde::Deserialize)]
struct CreateBacktestRequest {
    name: String,
    config_path: String,
    data_path: String,
}

async fn create_backtest(
    State(state): State<AppState>,
    Json(req): Json<CreateBacktestRequest>,
) -> WebResult<Json<SuccessResponse<BacktestTask>>> {
    debug!("创建回测任务: {}", req.name);

    let task = BacktestTask::new(req.name, req.config_path, req.data_path);
    let task_id = task.id;

    let mut tasks = state.backtest_tasks.write().await;
    tasks.insert(task_id, task.clone());

    info!("成功创建回测任务: {}", task_id);
    Ok(Json(SuccessResponse::new(task)))
}

/// 删除回测任务
async fn delete_backtest(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("删除回测任务: {}", id);

    let mut tasks = state.backtest_tasks.write().await;
    tasks
        .remove(&id)
        .ok_or_else(|| WebError::TaskNotFound(format!("任务不存在: {}", id)))?;

    info!("成功删除回测任务: {}", id);
    Ok(Json(SuccessResponse::new(format!("任务已删除: {}", id))))
}

/// 启动回测任务
async fn start_backtest(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("启动回测任务: {}", id);

    let mut tasks = state.backtest_tasks.write().await;
    let task = tasks
        .get_mut(&id)
        .ok_or_else(|| WebError::TaskNotFound(format!("任务不存在: {}", id)))?;

    task.start();

    info!("成功启动回测任务: {}", id);
    Ok(Json(SuccessResponse::new(format!("任务已启动: {}", id))))
}

/// 创建并立即启动回测任务
/// 这是一个便捷方法，结合了创建和启动两个步骤
async fn create_and_start_backtest(
    State(state): State<AppState>,
    Json(req): Json<CreateBacktestRequest>,
) -> WebResult<Json<SuccessResponse<serde_json::Value>>> {
    debug!("创建并启动回测任务: {}", req.name);

    // 创建任务
    let mut task = BacktestTask::new(req.name.clone(), req.config_path, req.data_path);
    let task_id = task.id;

    // 立即启动任务
    task.start();

    // 保存任务
    let mut tasks = state.backtest_tasks.write().await;
    tasks.insert(task_id, task.clone());

    info!("成功创建并启动回测任务: {} (ID: {})", req.name, task_id);
    
    // 返回任务ID和任务信息
    let response_data = serde_json::json!({
        "task_id": task_id,
        "task": task,
        "message": format!("回测任务 '{}' 已创建并启动", req.name)
    });

    Ok(Json(SuccessResponse::new(response_data)))
}
