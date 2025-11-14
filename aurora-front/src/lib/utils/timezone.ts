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
 * 时区处理工具函数
 */

/**
 * 将指定时区的日期时间字符串转换为 UTC 时间字符串
 * 
 * @param dateTimeStr 日期时间字符串（格式：YYYY-MM-DD 或 YYYY-MM-DD HH:mm:ss）
 * @param timezone IANA 时区标识符（如 'Asia/Shanghai', 'UTC'）
 * @returns UTC 时间字符串（格式：YYYY-MM-DD HH:mm:ss）
 * 
 * @example
 * // 将北京时间转换为 UTC
 * convertToUTC('2025-01-01 00:00:00', 'Asia/Shanghai')
 * // 返回: '2024-12-31 16:00:00'
 * 
 * @example
 * // 仅日期，会默认为 00:00:00
 * convertToUTC('2025-01-01', 'Asia/Shanghai')
 * // 返回: '2024-12-31 16:00:00'
 */
export function convertToUTC(dateTimeStr: string, timezone: string): string {
  // 如果没有时间部分，添加默认时间 00:00:00
  const fullDateTimeStr = dateTimeStr.includes(':') 
    ? dateTimeStr 
    : `${dateTimeStr} 00:00:00`;

  // 使用 Intl.DateTimeFormat 处理时区转换
  const formatter = new Intl.DateTimeFormat('en-US', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  // 解析原始字符串的各部分
  const parts = fullDateTimeStr.match(/(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2}):(\d{2})/);
  if (!parts) {
    throw new Error(`Invalid date time format: ${dateTimeStr}`);
  }

  const [, year, month, day, hour, minute, second] = parts;

  // 创建一个 Date 对象，假设它是 UTC 时间
  const utcDate = new Date(Date.UTC(
    parseInt(year, 10),
    parseInt(month, 10) - 1,
    parseInt(day, 10),
    parseInt(hour, 10),
    parseInt(minute, 10),
    parseInt(second, 10)
  ));

  // 获取该时间在指定时区下的本地时间
  const tzDateStr = formatter.format(utcDate);
  const tzParts = tzDateStr.match(/(\d{2})\/(\d{2})\/(\d{4}),\s+(\d{2}):(\d{2}):(\d{2})/);
  
  if (!tzParts) {
    throw new Error(`Failed to parse timezone date: ${tzDateStr}`);
  }

  const [, tzMonth, tzDay, tzYear, tzHour, tzMinute, tzSecond] = tzParts;
  const tzDate = new Date(Date.UTC(
    parseInt(tzYear, 10),
    parseInt(tzMonth, 10) - 1,
    parseInt(tzDay, 10),
    parseInt(tzHour, 10),
    parseInt(tzMinute, 10),
    parseInt(tzSecond, 10)
  ));

  // 计算时区偏移（毫秒）
  const offset = tzDate.getTime() - utcDate.getTime();

  // 应用偏移到原始日期
  const targetDate = new Date(utcDate.getTime() - offset);

  // 格式化为 UTC 时间字符串
  const utcYear = targetDate.getUTCFullYear();
  const utcMonth = String(targetDate.getUTCMonth() + 1).padStart(2, '0');
  const utcDay = String(targetDate.getUTCDate()).padStart(2, '0');
  const utcHour = String(targetDate.getUTCHours()).padStart(2, '0');
  const utcMinute = String(targetDate.getUTCMinutes()).padStart(2, '0');
  const utcSecond = String(targetDate.getUTCSeconds()).padStart(2, '0');

  return `${utcYear}-${utcMonth}-${utcDay} ${utcHour}:${utcMinute}:${utcSecond}`;
}

/**
 * 将 UTC 时间字符串转换为指定时区的日期时间字符串
 * 
 * @param utcDateTimeStr UTC 时间字符串（格式：YYYY-MM-DD 或 YYYY-MM-DD HH:mm:ss）
 * @param timezone IANA 时区标识符
 * @returns 指定时区的时间字符串（格式：YYYY-MM-DD HH:mm:ss）
 * 
 * @example
 * convertFromUTC('2024-12-31 16:00:00', 'Asia/Shanghai')
 * // 返回: '2025-01-01 00:00:00'
 */
export function convertFromUTC(utcDateTimeStr: string, timezone: string): string {
  // 如果没有时间部分，添加默认时间
  const fullDateTimeStr = utcDateTimeStr.includes(':')
    ? utcDateTimeStr
    : `${utcDateTimeStr} 00:00:00`;

  // 解析 UTC 时间字符串
  const parts = fullDateTimeStr.match(/(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2}):(\d{2})/);
  if (!parts) {
    throw new Error(`Invalid UTC date time format: ${utcDateTimeStr}`);
  }

  const [, year, month, day, hour, minute, second] = parts;

  // 创建 UTC Date 对象
  const utcDate = new Date(Date.UTC(
    parseInt(year, 10),
    parseInt(month, 10) - 1,
    parseInt(day, 10),
    parseInt(hour, 10),
    parseInt(minute, 10),
    parseInt(second, 10)
  ));

  // 使用 Intl.DateTimeFormat 转换到目标时区
  const formatter = new Intl.DateTimeFormat('en-US', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  const formatted = formatter.format(utcDate);
  const tzParts = formatted.match(/(\d{2})\/(\d{2})\/(\d{4}),\s+(\d{2}):(\d{2}):(\d{2})/);

  if (!tzParts) {
    throw new Error(`Failed to format date in timezone ${timezone}`);
  }

  const [, tzMonth, tzDay, tzYear, tzHour, tzMinute, tzSecond] = tzParts;

  return `${tzYear}-${tzMonth}-${tzDay} ${tzHour}:${tzMinute}:${tzSecond}`;
}

/**
 * 获取指定时区相对于 UTC 的偏移量（分钟）
 * 
 * @param timezone IANA 时区标识符
 * @param date 指定日期（用于处理夏令时），默认为当前时间
 * @returns 偏移量（分钟），正数表示东时区，负数表示西时区
 * 
 * @example
 * getTimezoneOffset('Asia/Shanghai') // 返回: 480 (UTC+8)
 * getTimezoneOffset('America/New_York') // 返回: -300 或 -240 (取决于是否夏令时)
 */
export function getTimezoneOffset(timezone: string, date: Date = new Date()): number {
  // 创建两个格式化器：一个 UTC，一个指定时区
  const utcFormatter = new Intl.DateTimeFormat('en-US', {
    timeZone: 'UTC',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  const tzFormatter = new Intl.DateTimeFormat('en-US', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });

  // 格式化同一时间
  const utcStr = utcFormatter.format(date);
  const tzStr = tzFormatter.format(date);

  // 解析为 Date 对象
  const parseDateTime = (str: string) => {
    const parts = str.match(/(\d{2})\/(\d{2})\/(\d{4}),\s+(\d{2}):(\d{2}):(\d{2})/);
    if (!parts) return null;
    const [, month, day, year, hour, minute, second] = parts;
    return new Date(Date.UTC(
      parseInt(year, 10),
      parseInt(month, 10) - 1,
      parseInt(day, 10),
      parseInt(hour, 10),
      parseInt(minute, 10),
      parseInt(second, 10)
    ));
  };

  const utcDate = parseDateTime(utcStr);
  const tzDate = parseDateTime(tzStr);

  if (!utcDate || !tzDate) {
    throw new Error(`Failed to parse dates for timezone offset calculation`);
  }

  // 计算偏移量（毫秒转分钟）
  return (tzDate.getTime() - utcDate.getTime()) / (1000 * 60);
}

/**
 * 格式化时区偏移量为字符串
 * 
 * @param offsetMinutes 偏移量（分钟）
 * @returns 格式化的偏移量字符串（如 "+08:00", "-05:00"）
 * 
 * @example
 * formatTimezoneOffset(480) // 返回: "+08:00"
 * formatTimezoneOffset(-300) // 返回: "-05:00"
 */
export function formatTimezoneOffset(offsetMinutes: number): string {
  const sign = offsetMinutes >= 0 ? '+' : '-';
  const absOffset = Math.abs(offsetMinutes);
  const hours = Math.floor(absOffset / 60);
  const minutes = absOffset % 60;
  return `${sign}${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}`;
}
