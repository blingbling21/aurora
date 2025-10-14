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

//! 配置错误类型定义

use thiserror::Error;

/// 配置错误类型
#[derive(Debug, Error)]
pub enum ConfigError {
    /// 文件读取错误
    #[error("配置文件读取失败: {0}")]
    FileRead(#[from] std::io::Error),

    /// TOML解析错误
    #[error("配置文件解析失败: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// 配置验证错误
    #[error("配置验证失败: {0}")]
    Validation(String),

    /// 缺失必需字段
    #[error("缺失必需配置字段: {0}")]
    MissingField(String),

    /// 无效的配置值
    #[error("无效的配置值: {field} = {value}, {reason}")]
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },
}

/// 配置结果类型
pub type ConfigResult<T> = Result<T, ConfigError>;
