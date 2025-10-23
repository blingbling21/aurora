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
use anyhow;

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
        .route("/result/{id}", get(get_backtest_result)) // 添加获取回测结果的路由
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

/// 获取回测任务结果
async fn get_backtest_result(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> WebResult<Json<SuccessResponse<serde_json::Value>>> {
    debug!("获取回测任务结果: {}", id);

    let tasks = state.backtest_tasks.read().await;
    let task = tasks
        .get(&id)
        .ok_or_else(|| WebError::TaskNotFound(format!("任务不存在: {}", id)))?;

    // 检查任务是否完成
    if task.result.is_none() {
        return Err(WebError::TaskNotFound(format!(
            "任务 {} 尚未完成或没有结果",
            id
        )));
    }

    // 构建结果响应
    let result = serde_json::json!({
        "task_id": id,
        "name": task.name,
        "status": task.status,
        "created_at": task.created_at,
        "started_at": task.started_at,
        "completed_at": task.completed_at,
        "progress": task.progress,
        "config_path": task.config_path,
        "data_path": task.data_path,
        "result": task.result,
        "error": task.error,
    });

    info!("成功获取回测任务结果: {}", id);
    Ok(Json(SuccessResponse::new(result)))
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

    // 先获取任务信息（需要释放锁）
    let (config_path, data_path) = {
        let mut tasks = state.backtest_tasks.write().await;
        let task = tasks
            .get_mut(&id)
            .ok_or_else(|| WebError::TaskNotFound(format!("任务不存在: {}", id)))?;

        task.start();
        (task.config_path.clone(), task.data_path.clone())
    };

    // 在后台异步执行回测
    let state_clone = state.clone();
    tokio::spawn(async move {
        let state_ref = state_clone.clone();
        if let Err(e) = execute_backtest(id, config_path, data_path, state_clone).await {
            tracing::error!("回测执行失败: {}", e);
            // 标记任务为失败
            let mut tasks = state_ref.backtest_tasks.write().await;
            if let Some(task) = tasks.get_mut(&id) {
                task.fail(e.to_string());
            }
        }
    });

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
    let mut task = BacktestTask::new(req.name.clone(), req.config_path.clone(), req.data_path.clone());
    let task_id = task.id;

    // 立即启动任务
    task.start();

    // 保存任务
    {
        let mut tasks = state.backtest_tasks.write().await;
        tasks.insert(task_id, task.clone());
    }

    info!("成功创建并启动回测任务: {} (ID: {})", req.name, task_id);
    
    // 在后台异步执行回测
    let state_clone = state.clone();
    let config_path = req.config_path;
    let data_path = req.data_path;
    tokio::spawn(async move {
        if let Err(e) = execute_backtest(task_id, config_path, data_path, state_clone.clone()).await {
            tracing::error!("回测执行失败: {}", e);
            // 标记任务为失败
            let mut tasks = state_clone.backtest_tasks.write().await;
            if let Some(task) = tasks.get_mut(&task_id) {
                task.fail(e.to_string());
            }
        }
    });
    
    // 返回任务ID和任务信息
    let response_data = serde_json::json!({
        "task_id": task_id,
        "task": task,
        "message": format!("回测任务 '{}' 已创建并启动", req.name)
    });

    Ok(Json(SuccessResponse::new(response_data)))
}

/// 执行回测的实际逻辑
async fn execute_backtest(
    task_id: Uuid,
    config_path: String,
    data_path: String,
    state: AppState,
) -> Result<(), anyhow::Error> {
    use aurora_backtester::run_backtest;
    use aurora_config::Config;

    info!("开始执行回测任务: {}", task_id);

    // 构建完整路径
    let config_full_path = state.config_dir.join(&config_path);
    let data_full_path = state.data_dir.join(&data_path);

    // 验证文件是否存在
    if !config_full_path.exists() {
        return Err(anyhow::anyhow!("配置文件不存在: {:?}", config_full_path));
    }
    if !data_full_path.exists() {
        return Err(anyhow::anyhow!("数据文件不存在: {:?}", data_full_path));
    }

    // 加载配置
    let full_config = Config::from_file(config_full_path.to_str().unwrap())?;
    
    // 提取回测配置
    let config = full_config.backtest.as_ref()
        .ok_or_else(|| anyhow::anyhow!("配置文件中缺少回测配置"))?;
    
    // 更新进度: 10%
    {
        let mut tasks = state.backtest_tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.update_progress(10);
        }
    }

    // 执行回测
    info!("配置已加载，开始运行回测引擎");
    
    // 更新进度: 30%
    {
        let mut tasks = state.backtest_tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.update_progress(30);
        }
    }

    // 从策略列表中提取第一个策略参数（如果存在）
    let (short_period, long_period) = if !full_config.strategies.is_empty() {
        let strategy = &full_config.strategies[0];
        let short = strategy.parameters.get("short_period")
            .and_then(|v| match v {
                aurora_config::StrategyParameter::Integer(i) => Some(*i as usize),
                aurora_config::StrategyParameter::Float(f) => Some(*f as usize),
                _ => None,
            })
            .unwrap_or(5);
        let long = strategy.parameters.get("long_period")
            .and_then(|v| match v {
                aurora_config::StrategyParameter::Integer(i) => Some(*i as usize),
                aurora_config::StrategyParameter::Float(f) => Some(*f as usize),
                _ => None,
            })
            .unwrap_or(20);
        (short, long)
    } else {
        (5, 20) // 默认值
    };

    // 更新进度: 50%
    {
        let mut tasks = state.backtest_tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.update_progress(50);
        }
    }

    // 运行回测并获取结果
    info!("开始运行回测引擎...");
    let backtest_result = run_backtest(
        data_full_path.to_str().unwrap(),
        "ma-crossover",
        short_period,
        long_period,
        &full_config.portfolio,
        config.pricing_mode.as_ref(),
    )
    .await?;

    // 更新进度: 90%
    {
        let mut tasks = state.backtest_tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.update_progress(90);
        }
    }

    // 创建详细结果
    let result = serde_json::json!({
        "status": "completed",
        "message": "回测执行成功",
        "config_path": config_path,
        "data_path": data_path,
        "metrics": {
            "total_return": backtest_result.metrics.total_return,
            "annualized_return": backtest_result.metrics.annualized_return,
            "max_drawdown": backtest_result.metrics.max_drawdown,
            "max_drawdown_duration": backtest_result.metrics.max_drawdown_duration,
            "annualized_volatility": backtest_result.metrics.annualized_volatility,
            "sharpe_ratio": backtest_result.metrics.sharpe_ratio,
            "sortino_ratio": backtest_result.metrics.sortino_ratio,
            "calmar_ratio": backtest_result.metrics.calmar_ratio,
            "win_rate": backtest_result.metrics.win_rate,
            "total_trades": backtest_result.metrics.total_trades,
            "winning_trades": backtest_result.metrics.winning_trades,
            "losing_trades": backtest_result.metrics.losing_trades,
            "average_win": backtest_result.metrics.average_win,
            "average_loss": backtest_result.metrics.average_loss,
            "profit_loss_ratio": backtest_result.metrics.profit_loss_ratio,
            "profit_factor": backtest_result.metrics.profit_factor,
            "max_consecutive_wins": backtest_result.metrics.max_consecutive_wins,
            "max_consecutive_losses": backtest_result.metrics.max_consecutive_losses,
            "avg_holding_period": backtest_result.metrics.avg_holding_period,
            "max_win": backtest_result.metrics.max_win,
            "max_loss": backtest_result.metrics.max_loss,
        },
        "equity_curve": backtest_result.equity_curve,
        "trades": backtest_result.trades,
        "time_period_days": backtest_result.time_period_days,
        "initial_equity": backtest_result.initial_equity,
        "final_equity": backtest_result.final_equity,
    });

    // 标记任务完成
    {
        let mut tasks = state.backtest_tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.complete(result);
        }
    }

    info!("回测任务执行完成: {}", task_id);
    Ok(())
}
