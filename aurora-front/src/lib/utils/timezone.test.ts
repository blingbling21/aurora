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

import {
  convertToUTC,
  convertFromUTC,
  getTimezoneOffset,
  formatTimezoneOffset,
} from './timezone';

describe('timezone utilities', () => {
  describe('convertToUTC', () => {
    it('应该将北京时间转换为 UTC（日期+时间）', () => {
      const result = convertToUTC('2025-01-01 00:00:00', 'Asia/Shanghai');
      expect(result).toBe('2024-12-31 16:00:00');
    });

    it('应该将北京时间转换为 UTC（仅日期）', () => {
      const result = convertToUTC('2025-01-01', 'Asia/Shanghai');
      expect(result).toBe('2024-12-31 16:00:00');
    });

    it('应该将东京时间转换为 UTC', () => {
      const result = convertToUTC('2025-01-01 00:00:00', 'Asia/Tokyo');
      expect(result).toBe('2024-12-31 15:00:00');
    });

    it('应该将纽约时间转换为 UTC（标准时间）', () => {
      // 1月是标准时间 EST (UTC-5)
      const result = convertToUTC('2025-01-01 00:00:00', 'America/New_York');
      expect(result).toBe('2025-01-01 05:00:00');
    });

    it('应该将纽约时间转换为 UTC（夏令时）', () => {
      // 7月是夏令时 EDT (UTC-4)
      const result = convertToUTC('2025-07-01 00:00:00', 'America/New_York');
      expect(result).toBe('2025-07-01 04:00:00');
    });

    it('应该将伦敦时间转换为 UTC（标准时间）', () => {
      // 1月是标准时间 GMT (UTC+0)
      const result = convertToUTC('2025-01-01 00:00:00', 'Europe/London');
      expect(result).toBe('2025-01-01 00:00:00');
    });

    it('应该将伦敦时间转换为 UTC（夏令时）', () => {
      // 7月是夏令时 BST (UTC+1)
      const result = convertToUTC('2025-07-01 00:00:00', 'Europe/London');
      expect(result).toBe('2025-06-30 23:00:00');
    });

    it('应该处理 UTC 时区', () => {
      const result = convertToUTC('2025-01-01 12:00:00', 'UTC');
      expect(result).toBe('2025-01-01 12:00:00');
    });

    it('应该处理跨月份的转换', () => {
      const result = convertToUTC('2025-01-01 02:00:00', 'Asia/Shanghai');
      expect(result).toBe('2024-12-31 18:00:00');
    });

    it('应该处理跨年份的转换', () => {
      const result = convertToUTC('2025-01-01 00:00:00', 'Asia/Shanghai');
      expect(result).toBe('2024-12-31 16:00:00');
    });

    it('应该在无效格式时抛出错误', () => {
      expect(() => convertToUTC('invalid-date', 'Asia/Shanghai')).toThrow();
      expect(() => convertToUTC('2025/01/01', 'Asia/Shanghai')).toThrow();
    });
  });

  describe('convertFromUTC', () => {
    it('应该将 UTC 转换为北京时间（日期+时间）', () => {
      const result = convertFromUTC('2024-12-31 16:00:00', 'Asia/Shanghai');
      expect(result).toBe('2025-01-01 00:00:00');
    });

    it('应该将 UTC 转换为北京时间（仅日期）', () => {
      const result = convertFromUTC('2024-12-31', 'Asia/Shanghai');
      expect(result).toBe('2024-12-31 08:00:00');
    });

    it('应该将 UTC 转换为东京时间', () => {
      const result = convertFromUTC('2024-12-31 15:00:00', 'Asia/Tokyo');
      expect(result).toBe('2025-01-01 00:00:00');
    });

    it('应该将 UTC 转换为纽约时间（标准时间）', () => {
      const result = convertFromUTC('2025-01-01 05:00:00', 'America/New_York');
      expect(result).toBe('2025-01-01 00:00:00');
    });

    it('应该将 UTC 转换为纽约时间（夏令时）', () => {
      const result = convertFromUTC('2025-07-01 04:00:00', 'America/New_York');
      expect(result).toBe('2025-07-01 00:00:00');
    });

    it('应该处理 UTC 时区', () => {
      const result = convertFromUTC('2025-01-01 12:00:00', 'UTC');
      expect(result).toBe('2025-01-01 12:00:00');
    });

    it('应该在无效格式时抛出错误', () => {
      expect(() => convertFromUTC('invalid-date', 'Asia/Shanghai')).toThrow();
    });
  });

  describe('convertToUTC 和 convertFromUTC 往返转换', () => {
    it('应该正确往返转换（北京时间）', () => {
      const original = '2025-01-01 12:30:45';
      const utc = convertToUTC(original, 'Asia/Shanghai');
      const back = convertFromUTC(utc, 'Asia/Shanghai');
      expect(back).toBe(original);
    });

    it('应该正确往返转换（纽约时间）', () => {
      const original = '2025-07-15 09:00:00';
      const utc = convertToUTC(original, 'America/New_York');
      const back = convertFromUTC(utc, 'America/New_York');
      expect(back).toBe(original);
    });

    it('应该正确往返转换（伦敦时间）', () => {
      const original = '2025-06-20 18:45:30';
      const utc = convertToUTC(original, 'Europe/London');
      const back = convertFromUTC(utc, 'Europe/London');
      expect(back).toBe(original);
    });
  });

  describe('getTimezoneOffset', () => {
    it('应该返回北京时间的偏移量', () => {
      const offset = getTimezoneOffset('Asia/Shanghai');
      expect(offset).toBe(480); // UTC+8 = 480分钟
    });

    it('应该返回东京时间的偏移量', () => {
      const offset = getTimezoneOffset('Asia/Tokyo');
      expect(offset).toBe(540); // UTC+9 = 540分钟
    });

    it('应该返回 UTC 的偏移量', () => {
      const offset = getTimezoneOffset('UTC');
      expect(offset).toBe(0);
    });

    it('应该返回纽约时间的偏移量（标准时间）', () => {
      // 1月是标准时间
      const janDate = new Date('2025-01-01T12:00:00Z');
      const offset = getTimezoneOffset('America/New_York', janDate);
      expect(offset).toBe(-300); // UTC-5 = -300分钟
    });

    it('应该返回纽约时间的偏移量（夏令时）', () => {
      // 7月是夏令时
      const julyDate = new Date('2025-07-01T12:00:00Z');
      const offset = getTimezoneOffset('America/New_York', julyDate);
      expect(offset).toBe(-240); // UTC-4 = -240分钟
    });

    it('应该返回伦敦时间的偏移量（标准时间）', () => {
      const janDate = new Date('2025-01-01T12:00:00Z');
      const offset = getTimezoneOffset('Europe/London', janDate);
      expect(offset).toBe(0); // GMT = UTC+0
    });

    it('应该返回伦敦时间的偏移量（夏令时）', () => {
      const julyDate = new Date('2025-07-01T12:00:00Z');
      const offset = getTimezoneOffset('Europe/London', julyDate);
      expect(offset).toBe(60); // BST = UTC+1 = 60分钟
    });
  });

  describe('formatTimezoneOffset', () => {
    it('应该格式化正偏移量', () => {
      expect(formatTimezoneOffset(480)).toBe('+08:00'); // UTC+8
      expect(formatTimezoneOffset(540)).toBe('+09:00'); // UTC+9
      expect(formatTimezoneOffset(330)).toBe('+05:30'); // UTC+5:30
    });

    it('应该格式化负偏移量', () => {
      expect(formatTimezoneOffset(-300)).toBe('-05:00'); // UTC-5
      expect(formatTimezoneOffset(-240)).toBe('-04:00'); // UTC-4
      expect(formatTimezoneOffset(-480)).toBe('-08:00'); // UTC-8
    });

    it('应该格式化零偏移量', () => {
      expect(formatTimezoneOffset(0)).toBe('+00:00');
    });

    it('应该正确填充数字', () => {
      expect(formatTimezoneOffset(60)).toBe('+01:00');
      expect(formatTimezoneOffset(-60)).toBe('-01:00');
    });
  });
});
