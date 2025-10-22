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

//! 配置管理API

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::fs;
use tracing::{debug, info};

use crate::error::{WebError, WebResult};
use crate::models::{
    ConfigListItem, ConfigValidateResponse, CreateConfigRequest, SuccessResponse,
    UpdateConfigRequest,
};
use crate::state::AppState;

/// 配置路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_configs).post(create_config))
        .route("/validate", post(validate_config))
        .route("/{filename}", get(get_config).put(update_config).delete(delete_config))
}

/// 列出所有配置文件
async fn list_configs(State(state): State<AppState>) -> WebResult<Json<SuccessResponse<Vec<ConfigListItem>>>> {
    debug!("列出配置文件");

    let mut configs = Vec::new();
    let entries = fs::read_dir(&state.config_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            let metadata = entry.metadata()?;
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    let dt: chrono::DateTime<chrono::Utc> = t.into();
                    Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                });

            configs.push(ConfigListItem {
                filename: filename.clone(),
                path: path.to_string_lossy().to_string(),
                modified: modified.unwrap_or_else(|| "未知".to_string()),
            });
        }
    }

    info!("找到 {} 个配置文件", configs.len());
    Ok(Json(SuccessResponse::new(configs)))
}

/// 获取指定配置文件
async fn get_config(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("获取配置文件: {}", filename);

    let config_path = state.config_dir.join(&filename);
    if !config_path.exists() {
        return Err(WebError::ConfigError(format!(
            "配置文件不存在: {}",
            filename
        )));
    }

    let content = fs::read_to_string(&config_path)?;

    info!("成功读取配置文件: {}", filename);
    Ok(Json(SuccessResponse::new(content)))
}

/// 创建新配置文件
async fn create_config(
    State(state): State<AppState>,
    Json(req): Json<CreateConfigRequest>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("创建配置文件: {}", req.filename);

    // 验证文件名
    if !req.filename.ends_with(".toml") {
        return Err(WebError::InvalidRequest(
            "配置文件必须以.toml结尾".to_string(),
        ));
    }

    let config_path = state.config_dir.join(&req.filename);
    if config_path.exists() {
        return Err(WebError::ConfigError(format!(
            "配置文件已存在: {}",
            req.filename
        )));
    }

    // 验证TOML格式
    toml::from_str::<aurora_config::Config>(&req.content)
        .map_err(|e| WebError::ConfigError(format!("配置格式无效: {}", e)))?;

    // 写入文件
    fs::write(&config_path, &req.content)?;

    info!("成功创建配置文件: {}", req.filename);
    Ok(Json(SuccessResponse::new(format!(
        "配置文件已创建: {}",
        req.filename
    ))))
}

/// 更新配置文件
async fn update_config(
    State(state): State<AppState>,
    Path(filename): Path<String>,
    Json(req): Json<UpdateConfigRequest>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("更新配置文件: {}", filename);

    let config_path = state.config_dir.join(&filename);
    if !config_path.exists() {
        return Err(WebError::ConfigError(format!(
            "配置文件不存在: {}",
            filename
        )));
    }

    // 验证TOML格式
    toml::from_str::<aurora_config::Config>(&req.content)
        .map_err(|e| WebError::ConfigError(format!("配置格式无效: {}", e)))?;

    // 写入文件
    fs::write(&config_path, &req.content)?;

    info!("成功更新配置文件: {}", filename);
    Ok(Json(SuccessResponse::new(format!(
        "配置文件已更新: {}",
        filename
    ))))
}

/// 删除配置文件
async fn delete_config(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> WebResult<Json<SuccessResponse<String>>> {
    debug!("删除配置文件: {}", filename);

    let config_path = state.config_dir.join(&filename);
    if !config_path.exists() {
        return Err(WebError::ConfigError(format!(
            "配置文件不存在: {}",
            filename
        )));
    }

    fs::remove_file(&config_path)?;

    info!("成功删除配置文件: {}", filename);
    Ok(Json(SuccessResponse::new(format!(
        "配置文件已删除: {}",
        filename
    ))))
}

/// 验证配置文件
async fn validate_config(
    Json(req): Json<UpdateConfigRequest>,
) -> WebResult<Json<SuccessResponse<ConfigValidateResponse>>> {
    debug!("验证配置内容");

    let mut errors = Vec::new();

    match toml::from_str::<aurora_config::Config>(&req.content) {
        Ok(_) => {
            info!("配置验证成功");
            Ok(Json(SuccessResponse::new(ConfigValidateResponse {
                valid: true,
                errors,
            })))
        }
        Err(e) => {
            errors.push(format!("TOML解析错误: {}", e));
            info!("配置验证失败: {:?}", errors);
            Ok(Json(SuccessResponse::new(ConfigValidateResponse {
                valid: false,
                errors,
            })))
        }
    }
}
