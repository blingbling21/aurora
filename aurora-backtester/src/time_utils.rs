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

//! 时间处理工具模块
//!
//! 提供时间解析、验证和过滤功能

use anyhow::{anyhow, Result};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// 解析日期字符串为毫秒时间戳
///
/// 支持格式:
/// - YYYY-MM-DD
/// - YYYY-MM-DD HH:MM:SS
///
/// # 参数
///
/// * `date_str` - 日期字符串
///
/// # 返回值
///
/// 返回毫秒时间戳
pub fn parse_date_to_timestamp(date_str: &str) -> Result<i64> {
    // 尝试解析完整的日期时间格式 (YYYY-MM-DD HH:MM:SS)
    if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(datetime.and_utc().timestamp_millis());
    }

    // 尝试解析日期格式 (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return Ok(datetime.and_utc().timestamp_millis());
    }

    Err(anyhow!(
        "无法解析日期字符串: {}，支持格式: YYYY-MM-DD 或 YYYY-MM-DD HH:MM:SS",
        date_str
    ))
}

/// 时间范围验证结果
#[derive(Debug, Clone, PartialEq)]
pub enum TimeRangeValidation {
    /// 完全匹配或在数据范围内
    Valid,
    /// 配置的时间范围与数据完全不重叠
    NoOverlap {
        config_start: i64,
        config_end: i64,
        data_start: i64,
        data_end: i64,
    },
    /// 部分重叠，配置的开始时间早于数据
    StartBeforeData {
        config_start: i64,
        data_start: i64,
    },
    /// 部分重叠，配置的结束时间晚于数据
    EndAfterData {
        config_end: i64,
        data_end: i64,
    },
    /// 配置的开始时间晚于结束时间
    InvalidRange {
        start: i64,
        end: i64,
    },
}

/// 验证配置的时间范围与数据的时间范围
///
/// # 参数
///
/// * `config_start` - 配置的开始时间（可选）
/// * `config_end` - 配置的结束时间（可选）
/// * `data_start` - 数据的开始时间
/// * `data_end` - 数据的结束时间
///
/// # 返回值
///
/// 返回验证结果
pub fn validate_time_range(
    config_start: Option<i64>,
    config_end: Option<i64>,
    data_start: i64,
    data_end: i64,
) -> TimeRangeValidation {
    // 如果没有配置时间范围，直接返回有效
    let (start, end) = match (config_start, config_end) {
        (None, None) => return TimeRangeValidation::Valid,
        (Some(s), None) => (s, data_end),
        (None, Some(e)) => (data_start, e),
        (Some(s), Some(e)) => (s, e),
    };

    // 检查配置的时间范围是否有效
    if start > end {
        return TimeRangeValidation::InvalidRange { start, end };
    }

    // 检查是否完全不重叠
    if end < data_start || start > data_end {
        return TimeRangeValidation::NoOverlap {
            config_start: start,
            config_end: end,
            data_start,
            data_end,
        };
    }

    // 检查是否开始时间早于数据
    if config_start.is_some() && start < data_start {
        return TimeRangeValidation::StartBeforeData {
            config_start: start,
            data_start,
        };
    }

    // 检查是否结束时间晚于数据
    if config_end.is_some() && end > data_end {
        return TimeRangeValidation::EndAfterData {
            config_end: end,
            data_end,
        };
    }

    TimeRangeValidation::Valid
}

/// 格式化时间戳为可读字符串
///
/// # 参数
///
/// * `timestamp` - 毫秒时间戳
///
/// # 返回值
///
/// 返回格式化的日期字符串 (YYYY-MM-DD HH:MM:SS)
pub fn format_timestamp(timestamp: i64) -> String {
    let datetime = chrono::DateTime::from_timestamp_millis(timestamp)
        .unwrap_or_else(|| chrono::Utc::now());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_to_timestamp() {
        // 测试日期格式 YYYY-MM-DD
        let result = parse_date_to_timestamp("2024-01-01");
        assert!(result.is_ok());

        // 测试日期时间格式 YYYY-MM-DD HH:MM:SS
        let result = parse_date_to_timestamp("2024-01-01 12:30:45");
        assert!(result.is_ok());

        // 测试无效格式
        let result = parse_date_to_timestamp("invalid-date");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_time_range_no_config() {
        // 没有配置时间范围，应该返回 Valid
        let result = validate_time_range(None, None, 1000, 2000);
        assert_eq!(result, TimeRangeValidation::Valid);
    }

    #[test]
    fn test_validate_time_range_valid() {
        // 配置范围在数据范围内
        let result = validate_time_range(Some(1100), Some(1900), 1000, 2000);
        assert_eq!(result, TimeRangeValidation::Valid);
    }

    #[test]
    fn test_validate_time_range_no_overlap() {
        // 完全不重叠
        let result = validate_time_range(Some(3000), Some(4000), 1000, 2000);
        assert!(matches!(result, TimeRangeValidation::NoOverlap { .. }));
    }

    #[test]
    fn test_validate_time_range_start_before_data() {
        // 开始时间早于数据
        let result = validate_time_range(Some(500), Some(1500), 1000, 2000);
        assert!(matches!(
            result,
            TimeRangeValidation::StartBeforeData { .. }
        ));
    }

    #[test]
    fn test_validate_time_range_end_after_data() {
        // 结束时间晚于数据
        let result = validate_time_range(Some(1500), Some(2500), 1000, 2000);
        assert!(matches!(result, TimeRangeValidation::EndAfterData { .. }));
    }

    #[test]
    fn test_validate_time_range_invalid_range() {
        // 开始时间晚于结束时间
        let result = validate_time_range(Some(2000), Some(1000), 1000, 3000);
        assert!(matches!(result, TimeRangeValidation::InvalidRange { .. }));
    }

    #[test]
    fn test_format_timestamp() {
        // 测试时间戳格式化
        let timestamp = 1704067200000i64; // 2024-01-01 00:00:00 UTC
        let formatted = format_timestamp(timestamp);
        assert!(formatted.starts_with("2024-01-01"));
    }
}
