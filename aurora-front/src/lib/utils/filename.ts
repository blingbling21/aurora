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
 * 格式化日期为 YYYYMMDD 格式
 * @param date - 要格式化的日期
 * @returns 格式化后的日期字符串
 */
export function formatDateToYYYYMMDD(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}${month}${day}`;
}

/**
 * 生成数据文件名
 * 
 * 根据交易所、交易对、时间周期和日期范围生成标准化的数据文件名
 * 文件名格式: exchange_symbol_interval_startdate_to_enddate.csv
 * 
 * @param exchange - 交易所名称（如：binance, okx）
 * @param symbol - 交易对（如：BTCUSDT, ETHUSDT）
 * @param interval - 时间周期（如：1m, 5m, 1h, 1d）
 * @param startDate - 开始日期
 * @param endDate - 结束日期
 * @returns 生成的文件名，如果必填参数缺失则返回空字符串
 * 
 * @example
 * ```ts
 * const filename = generateDataFilename(
 *   'binance',
 *   'BTCUSDT',
 *   '1h',
 *   new Date('2025-01-01'),
 *   new Date('2025-01-31')
 * );
 * // 返回: 'binance_btcusdt_1h_20250101_to_20250131.csv'
 * ```
 */
export function generateDataFilename(
  exchange: string,
  symbol: string,
  interval: string,
  startDate: Date | undefined,
  endDate: Date | undefined
): string {
  // 验证所有必填字段
  if (!exchange || !symbol || !interval || !startDate || !endDate) {
    return '';
  }

  // 格式化日期
  const formattedStart = formatDateToYYYYMMDD(startDate);
  const formattedEnd = formatDateToYYYYMMDD(endDate);

  // 生成文件名，所有组件都转为小写
  return `${exchange.toLowerCase()}_${symbol.toLowerCase()}_${interval}_${formattedStart}_to_${formattedEnd}.csv`;
}
