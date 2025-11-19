/**
 * Copyright 2025 blingbling21
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/**
 * 将日期时间转换为 lightweight-charts 所需的格式（yyyy-mm-dd）
 * 
 * lightweight-charts 要求时间格式为 yyyy-mm-dd，不支持 ISO 8601 格式
 * 使用 UTC 时间来保证在不同时区的一致性
 * 
 * 注意：此函数仅适用于日级或更低频率的数据
 * 对于小时级数据，请使用 convertToUnixTimestamp 函数
 * 
 * @param dateString - ISO 8601 格式的日期时间字符串或 Date 对象
 * @returns 格式化后的日期字符串（yyyy-mm-dd）
 * 
 * @example
 * ```ts
 * formatTimeForLightweightCharts('2024-01-15T10:30:00.000Z')
 * // 返回: '2024-01-15'
 * 
 * formatTimeForLightweightCharts(new Date('2024-01-15'))
 * // 返回: '2024-01-15'
 * ```
 */
export function formatTimeForLightweightCharts(dateString: string | Date): string {
  // 将输入转换为 Date 对象
  const date = typeof dateString === 'string' ? new Date(dateString) : dateString;
  
  // 使用 UTC 时间提取年、月、日，避免时区问题
  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, '0');
  const day = String(date.getUTCDate()).padStart(2, '0');
  
  // 返回格式化的日期字符串
  return `${year}-${month}-${day}`;
}

/**
 * 将日期时间转换为 lightweight-charts 的 Unix 时间戳（秒）
 * 
 * lightweight-charts 支持 Unix 时间戳作为时间值（秒级精度）
 * 适用于小时级或更高频率的数据
 * 
 * @param dateString - ISO 8601 格式的日期时间字符串或 Date 对象
 * @returns Unix 时间戳（秒）
 * 
 * @example
 * ```ts
 * convertToUnixTimestamp('2024-01-15T10:30:00.000Z')
 * // 返回: 1705318200
 * 
 * convertToUnixTimestamp(new Date('2024-01-15T10:30:00.000Z'))
 * // 返回: 1705318200
 * ```
 */
export function convertToUnixTimestamp(dateString: string | Date): number {
  // 将输入转换为 Date 对象
  const date = typeof dateString === 'string' ? new Date(dateString) : dateString;
  
  // 返回 Unix 时间戳（秒）
  return Math.floor(date.getTime() / 1000);
}
