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

/**
 * 时间范围验证工具
 * 用于验证回测配置的时间范围与数据文件的时间范围
 */

/**
 * 时间范围验证结果
 */
export interface TimeRangeValidation {
  isValid: boolean;
  error?: string;
  warning?: string;
}

/**
 * 解析日期字符串为时间戳（毫秒）
 * 支持格式: YYYY-MM-DD 或 YYYY-MM-DD HH:MM:SS
 */
export function parseDateToTimestamp(dateStr: string): number {
  const date = new Date(dateStr);
  
  if (isNaN(date.getTime())) {
    throw new Error(
      `无法解析日期字符串: ${dateStr}，支持格式: YYYY-MM-DD 或 YYYY-MM-DD HH:MM:SS`
    );
  }
  
  return date.getTime();
}

/**
 * 格式化时间戳为可读字符串
 */
export function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toISOString().replace('T', ' ').substring(0, 19);
}

/**
 * 验证配置的时间范围与数据的时间范围
 * 
 * @param configStart 配置的开始时间（可选）
 * @param configEnd 配置的结束时间（可选）
 * @param dataStart 数据的开始时间
 * @param dataEnd 数据的结束时间
 */
export function validateTimeRange(
  configStart: string | undefined,
  configEnd: string | undefined,
  dataStart: number,
  dataEnd: number
): TimeRangeValidation {
  // 如果没有配置时间范围，直接返回有效
  if (!configStart && !configEnd) {
    return { isValid: true };
  }

  try {
    // 解析配置的时间范围
    const start = configStart ? parseDateToTimestamp(configStart) : dataStart;
    const end = configEnd ? parseDateToTimestamp(configEnd) : dataEnd;

    // 检查配置的时间范围是否有效
    if (start > end) {
      return {
        isValid: false,
        error: `无效的时间范围: 开始时间 ${formatTimestamp(start)} 晚于结束时间 ${formatTimestamp(end)}`,
      };
    }

    // 检查是否完全不重叠
    if (end < dataStart || start > dataEnd) {
      return {
        isValid: false,
        error: `配置的时间范围与数据完全不重叠！\n配置范围: ${formatTimestamp(start)} 到 ${formatTimestamp(end)}\n数据范围: ${formatTimestamp(dataStart)} 到 ${formatTimestamp(dataEnd)}`,
      };
    }

    // 检查是否部分重叠，生成警告
    let warning: string | undefined;

    if (configStart && start < dataStart) {
      warning = `配置的开始时间 ${formatTimestamp(start)} 早于数据开始时间 ${formatTimestamp(dataStart)}，将使用数据开始时间`;
    }

    if (configEnd && end > dataEnd) {
      const msg = `配置的结束时间 ${formatTimestamp(end)} 晚于数据结束时间 ${formatTimestamp(dataEnd)}，将使用数据结束时间`;
      warning = warning ? `${warning}\n${msg}` : msg;
    }

    return {
      isValid: true,
      warning,
    };
  } catch (error) {
    return {
      isValid: false,
      error: error instanceof Error ? error.message : '时间解析失败',
    };
  }
}

/**
 * 从数据文件名提取时间范围
 * 例如: binance_btcusdt_1m_20250101_to_20251113.csv
 * 返回: { start: '2025-01-01', end: '2025-11-13' }
 */
export function extractTimeRangeFromFilename(filename: string): {
  start?: string;
  end?: string;
} {
  // 匹配格式: YYYYMMDD_to_YYYYMMDD
  const match = filename.match(/(\d{8})_to_(\d{8})/);
  
  if (!match) {
    return {};
  }

  const [, startStr, endStr] = match;
  
  // 转换为 YYYY-MM-DD 格式
  const formatDate = (str: string) => {
    const year = str.substring(0, 4);
    const month = str.substring(4, 6);
    const day = str.substring(6, 8);
    return `${year}-${month}-${day}`;
  };

  return {
    start: formatDate(startStr),
    end: formatDate(endStr),
  };
}
