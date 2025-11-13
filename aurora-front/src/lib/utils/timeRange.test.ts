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

import {
  parseDateToTimestamp,
  formatTimestamp,
  validateTimeRange,
  extractTimeRangeFromFilename,
} from './timeRange';

describe('timeRange utilities', () => {
  describe('parseDateToTimestamp', () => {
    it('应该正确解析日期格式 YYYY-MM-DD', () => {
      const result = parseDateToTimestamp('2024-01-01');
      expect(result).toBeGreaterThan(0);
    });

    it('应该正确解析日期时间格式 YYYY-MM-DD HH:MM:SS', () => {
      const result = parseDateToTimestamp('2024-01-01 12:30:45');
      expect(result).toBeGreaterThan(0);
    });

    it('应该对无效格式抛出错误', () => {
      expect(() => parseDateToTimestamp('invalid-date')).toThrow();
    });
  });

  describe('formatTimestamp', () => {
    it('应该正确格式化时间戳', () => {
      const timestamp = new Date('2024-01-01T00:00:00Z').getTime();
      const formatted = formatTimestamp(timestamp);
      expect(formatted).toContain('2024-01-01');
    });
  });

  describe('validateTimeRange', () => {
    const dataStart = new Date('2025-01-01').getTime();
    const dataEnd = new Date('2025-11-13').getTime();

    it('没有配置时间范围时应返回有效', () => {
      const result = validateTimeRange(undefined, undefined, dataStart, dataEnd);
      expect(result.isValid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it('配置范围在数据范围内应返回有效', () => {
      const result = validateTimeRange(
        '2025-02-01',
        '2025-10-01',
        dataStart,
        dataEnd
      );
      expect(result.isValid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it('完全不重叠应返回错误', () => {
      const result = validateTimeRange(
        '2024-01-01',
        '2024-12-31',
        dataStart,
        dataEnd
      );
      expect(result.isValid).toBe(false);
      expect(result.error).toContain('完全不重叠');
    });

    it('开始时间晚于结束时间应返回错误', () => {
      const result = validateTimeRange(
        '2025-12-31',
        '2025-01-01',
        dataStart,
        dataEnd
      );
      expect(result.isValid).toBe(false);
      expect(result.error).toContain('无效的时间范围');
    });

    it('开始时间早于数据应生成警告', () => {
      const result = validateTimeRange(
        '2024-12-01',
        '2025-10-01',
        dataStart,
        dataEnd
      );
      expect(result.isValid).toBe(true);
      expect(result.warning).toContain('早于数据开始时间');
    });

    it('结束时间晚于数据应生成警告', () => {
      const result = validateTimeRange(
        '2025-02-01',
        '2025-12-31',
        dataStart,
        dataEnd
      );
      expect(result.isValid).toBe(true);
      expect(result.warning).toContain('晚于数据结束时间');
    });
  });

  describe('extractTimeRangeFromFilename', () => {
    it('应该从标准文件名提取时间范围', () => {
      const result = extractTimeRangeFromFilename(
        'binance_btcusdt_1m_20250101_to_20251113.csv'
      );
      expect(result.start).toBe('2025-01-01');
      expect(result.end).toBe('2025-11-13');
    });

    it('应该处理不同的文件名格式', () => {
      const result = extractTimeRangeFromFilename(
        'data_20240601_to_20240630.csv'
      );
      expect(result.start).toBe('2024-06-01');
      expect(result.end).toBe('2024-06-30');
    });

    it('对于不匹配的文件名应返回空对象', () => {
      const result = extractTimeRangeFromFilename('simple_data.csv');
      expect(result.start).toBeUndefined();
      expect(result.end).toBeUndefined();
    });
  });
});
