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
 * 格式化工具函数
 */

/**
 * 格式化文件大小
 * 
 * @param bytes 字节数
 * @returns 格式化后的文件大小字符串
 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

/**
 * 格式化日期时间
 * 
 * @param dateString ISO 日期字符串
 * @returns 格式化后的日期时间字符串
 */
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

/**
 * 将 Date 对象转换为本地时间的日期字符串（YYYY-MM-DD）
 * 用于前端显示和存储本地日期
 * 
 * @param date Date 对象
 * @returns 本地时间的日期字符串，格式：YYYY-MM-DD
 * 
 * @example
 * const date = new Date(2025, 0, 1); // 2025-01-01 本地时间
 * formatDateToLocal(date); // "2025-01-01"
 */
export function formatDateToLocal(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

/**
 * 将本地时间的 Date 对象转换为 UTC 时间的日期字符串（YYYY-MM-DD）
 * 用于将前端的本地时间转换为后端需要的 UTC 时间格式
 * 
 * @param date Date 对象（本地时间）
 * @returns UTC 时间的日期字符串，格式：YYYY-MM-DD
 * 
 * @example
 * // 假设本地时区是 UTC+8
 * const date = new Date(2025, 0, 1, 0, 0, 0); // 2025-01-01 00:00:00 本地时间
 * formatDateToUTC(date); // "2024-12-31" (UTC时间，减去8小时)
 * 
 * @example
 * // 正午不会跨天
 * const date = new Date(2025, 0, 1, 12, 0, 0); // 2025-01-01 12:00:00 本地时间
 * formatDateToUTC(date); // "2025-01-01" (UTC时间，减去8小时后仍是同一天)
 */
export function formatDateToUTC(date: Date): string {
  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, '0');
  const day = String(date.getUTCDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

/**
 * 格式化百分比
 * 
 * @param value 数值
 * @param decimals 小数位数，默认 2
 * @returns 格式化后的百分比字符串
 */
export function formatPercent(value: number, decimals = 2): string {
  return `${value.toFixed(decimals)}%`;
}

/**
 * 格式化货币
 * 
 * @param value 数值
 * @param currency 货币符号，默认 $
 * @param decimals 小数位数，默认 2
 * @returns 格式化后的货币字符串
 */
export function formatCurrency(value: number, currency = '$', decimals = 2): string {
  return `${currency}${value.toFixed(decimals)}`;
}

/**
 * 获取任务状态的中文文本
 * 
 * @param status 任务状态
 * @returns 中文状态文本
 */
export function getTaskStatusText(status: string): string {
  const statusMap: Record<string, string> = {
    pending: '等待中',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
  };
  return statusMap[status] || status;
}

/**
 * 获取任务状态的颜色类名
 * 
 * @param status 任务状态
 * @returns CSS 类名
 */
export function getTaskStatusColor(status: string): string {
  const colorMap: Record<string, string> = {
    pending: 'text-yellow-500',
    running: 'text-blue-500',
    completed: 'text-green-500',
    failed: 'text-red-500',
  };
  return colorMap[status] || 'text-gray-500';
}

/**
 * 生成数据文件名
 * 根据交易所、交易对、时间间隔和日期范围生成标准文件名
 * 
 * @param params 文件名参数
 * @returns 生成的文件名
 */
export function generateDataFilename(params: {
  exchange: string;
  symbol: string;
  interval: string;
  startDate: string;
  endDate: string;
}): string {
  const { exchange, symbol, interval, startDate, endDate } = params;
  
  // 格式化日期（移除连字符）
  const formattedStart = startDate.replace(/-/g, '');
  const formattedEnd = endDate.replace(/-/g, '');
  
  return `${exchange.toLowerCase()}_${symbol.toLowerCase()}_${interval}_${formattedStart}_to_${formattedEnd}.csv`;
}

/**
 * 防抖函数
 * 
 * @param fn 要防抖的函数
 * @param delay 延迟时间（毫秒）
 * @returns 防抖后的函数
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: NodeJS.Timeout;
  
  return function (this: unknown, ...args: Parameters<T>) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => fn.apply(this, args), delay);
  };
}

/**
 * 节流函数
 * 
 * @param fn 要节流的函数
 * @param limit 时间限制（毫秒）
 * @returns 节流后的函数
 */
export function throttle<T extends (...args: unknown[]) => unknown>(
  fn: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean;
  
  return function (this: unknown, ...args: Parameters<T>) {
    if (!inThrottle) {
      fn.apply(this, args);
      inThrottle = true;
      setTimeout(() => (inThrottle = false), limit);
    }
  };
}
