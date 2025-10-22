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

//! 错误类型定义

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Web API错误类型
#[derive(Debug, Error)]
pub enum WebError {
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("文件IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON序列化错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML解析错误: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("回测任务未找到: {0}")]
    TaskNotFound(String),

    #[error("回测执行失败: {0}")]
    BacktestError(String),

    #[error("数据文件未找到: {0}")]
    DataNotFound(String),

    #[error("数据错误: {0}")]
    DataError(String),

    #[error("请求参数无效: {0}")]
    InvalidRequest(String),

    #[error("内部服务器错误: {0}")]
    InternalError(String),
}

/// 错误响应JSON格式
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl WebError {
    /// 获取对应的HTTP状态码
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::ConfigError(_) => StatusCode::BAD_REQUEST,
            WebError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::JsonError(_) => StatusCode::BAD_REQUEST,
            WebError::TomlError(_) => StatusCode::BAD_REQUEST,
            WebError::TaskNotFound(_) => StatusCode::NOT_FOUND,
            WebError::BacktestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::DataNotFound(_) => StatusCode::NOT_FOUND,
            WebError::DataError(_) => StatusCode::BAD_REQUEST,
            WebError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            WebError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: status_code
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
            message: self.to_string(),
        };
        (status_code, Json(error_response)).into_response()
    }
}

/// Result类型别名
pub type WebResult<T> = Result<T, WebError>;
