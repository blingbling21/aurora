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

//! Aurora Web - Web界面主入口
//!
//! 提供HTTP服务器,用于配置管理、回测执行和结果可视化

use axum::Router;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;

mod api;
mod error;
mod models;
mod state;
mod ws;

use state::AppState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // 创建应用状态
    let app_state = AppState {
        backtest_tasks: Arc::new(RwLock::new(std::collections::HashMap::new())),
        download_tasks: Arc::new(RwLock::new(std::collections::HashMap::new())),
        data_dir: std::path::PathBuf::from("./data"),
        config_dir: std::path::PathBuf::from("./configs"),
    };

    // 确保目录存在
    std::fs::create_dir_all(&app_state.data_dir).ok();
    std::fs::create_dir_all(&app_state.config_dir).ok();

    let bind_address = "127.0.0.1:8080";
    info!("🚀 启动 Aurora Web 服务器: http://{}", bind_address);
    info!("📊 数据目录: {:?}", app_state.data_dir);
    info!("⚙️  配置目录: {:?}", app_state.config_dir);

    // 构建路由
    let app = Router::new()
        // API路由
        .nest("/api/config", api::config::routes())
        .nest("/api/backtest", api::backtest::routes())
        .nest("/api/data", api::data::routes())
        // WebSocket路由
        .nest("/ws", ws::routes())
        // 共享状态
        .with_state(app_state)
        // 静态文件服务（必须在最后，作为 fallback）
        .fallback_service(ServeDir::new("./static").append_index_html_on_directories(true))
        // 中间件
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CorsLayer::permissive());

    // 创建监听器
    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .unwrap();

    info!("✅ Aurora Web 服务器已启动");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
