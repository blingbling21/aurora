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

import { formatTimeForLightweightCharts, convertToUnixTimestamp } from './timeFormatting';

describe('timeFormatting', () => {
  describe('formatTimeForLightweightCharts', () => {
    it('应该正确格式化 ISO 8601 日期字符串', () => {
      const result = formatTimeForLightweightCharts('2024-01-15T10:30:00.000Z');
      expect(result).toBe('2024-01-15');
    });

    it('应该正确格式化本地日期时间字符串', () => {
      const result = formatTimeForLightweightCharts('2024-01-15T10:30:00');
      expect(result).toBe('2024-01-15');
    });

    it('应该正确格式化 Date 对象', () => {
      const date = new Date('2024-01-15T10:30:00.000Z');
      const result = formatTimeForLightweightCharts(date);
      expect(result).toBe('2024-01-15');
    });

    it('应该正确处理单位数的月份和日期', () => {
      const result = formatTimeForLightweightCharts('2024-01-05T10:30:00.000Z');
      expect(result).toBe('2024-01-05');
    });

    it('应该正确处理双位数的月份和日期', () => {
      const result = formatTimeForLightweightCharts('2024-12-25T10:30:00.000Z');
      expect(result).toBe('2024-12-25');
    });

    it('应该正确处理年初日期', () => {
      const result = formatTimeForLightweightCharts('2024-01-01T00:00:00.000Z');
      expect(result).toBe('2024-01-01');
    });

    it('应该正确处理年末日期', () => {
      const result = formatTimeForLightweightCharts('2024-12-31T23:59:59.000Z');
      expect(result).toBe('2024-12-31');
    });

    it('应该忽略时间部分', () => {
      const result1 = formatTimeForLightweightCharts('2024-01-15T00:00:00.000Z');
      const result2 = formatTimeForLightweightCharts('2024-01-15T23:59:59.999Z');
      expect(result1).toBe('2024-01-15');
      expect(result2).toBe('2024-01-15');
    });
  });

  describe('convertToUnixTimestamp', () => {
    it('应该正确转换 ISO 8601 日期字符串为 Unix 时间戳', () => {
      // 2024-01-15T10:30:00.000Z = 1705314600 秒
      const result = convertToUnixTimestamp('2024-01-15T10:30:00.000Z');
      expect(result).toBe(1705314600);
    });

    it('应该正确转换 Date 对象为 Unix 时间戳', () => {
      const date = new Date('2024-01-15T10:30:00.000Z');
      const result = convertToUnixTimestamp(date);
      expect(result).toBe(1705314600);
    });

    it('应该返回整数时间戳（去除毫秒）', () => {
      const result = convertToUnixTimestamp('2024-01-15T10:30:00.999Z');
      expect(Number.isInteger(result)).toBe(true);
      expect(result).toBe(1705314600);
    });

    it('应该正确处理 Unix 纪元时间', () => {
      const result = convertToUnixTimestamp('1970-01-01T00:00:00.000Z');
      expect(result).toBe(0);
    });

    it('应该正确处理负数时间戳（1970年之前）', () => {
      const result = convertToUnixTimestamp('1969-12-31T23:59:59.000Z');
      expect(result).toBe(-1);
    });
  });
});
