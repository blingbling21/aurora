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

//! 数据下载WebSocket处理

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
};
use futures_util::SinkExt;
use tracing::{debug, info};
use uuid::Uuid;

use crate::state::{AppState, DownloadStatus};

/// WebSocket处理器
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Response {
    info!("数据下载WebSocket连接请求: {}", task_id);

    // 验证任务是否存在
    let task_exists = {
        let tasks = state.download_tasks.read().await;
        tasks.contains_key(&task_id)
    };

    if !task_exists {
        // 任务不存在，返回错误响应
        return axum::response::Json(serde_json::json!({
            "error": "下载任务不存在",
            "task_id": task_id.to_string()
        }))
        .into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state, task_id))
}

/// 处理WebSocket连接
async fn handle_socket(mut socket: WebSocket, state: AppState, task_id: Uuid) {
    // 发送欢迎消息
    let welcome_msg = serde_json::json!({
        "type": "connected",
        "task_id": task_id.to_string(),
        "message": "已连接到数据下载进度推送"
    });
    if socket
        .send(Message::Text(welcome_msg.to_string().into()))
        .await
        .is_err()
    {
        return;
    }

    // 定期推送任务状态
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // 获取任务状态并推送
                let tasks = state.download_tasks.read().await;
                if let Some(task) = tasks.get(&task_id) {
                    let status_json = serde_json::json!({
                        "type": "progress",
                        "task_id": task_id.to_string(),
                        "status": task.status,
                        "progress": task.progress,
                        "progress_message": task.progress_message,
                        "downloaded_count": task.downloaded_count,
                        "estimated_total": task.estimated_total,
                        "error": task.error
                    });

                    if socket.send(Message::Text(status_json.to_string().into())).await.is_err() {
                        debug!("客户端断开连接: {}", task_id);
                        break;
                    }

                    // 如果任务已完成或失败,发送最终消息并关闭连接
                    if matches!(task.status, DownloadStatus::Completed | DownloadStatus::Failed) {
                        let final_msg = serde_json::json!({
                            "type": "complete",
                            "task_id": task_id.to_string(),
                            "status": task.status,
                            "message": task.progress_message,
                            "downloaded_count": task.downloaded_count
                        });
                        let _ = socket.send(Message::Text(final_msg.to_string().into())).await;
                        
                        // 给客户端一点时间处理消息
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        let _ = socket.close().await;
                        break;
                    }
                } else {
                    let error_msg = serde_json::json!({
                        "type": "error",
                        "message": "下载任务不存在"
                    });
                    let _ = socket.send(Message::Text(error_msg.to_string().into())).await;
                    let _ = socket.close().await;
                    break;
                }
            }
            Some(msg) = socket.recv() => {
                match msg {
                    Ok(Message::Ping(data)) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    Ok(Message::Close(_)) | Err(_) => {
                        info!("客户端主动关闭连接: {}", task_id);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    info!("数据下载WebSocket连接关闭: {}", task_id);
}
