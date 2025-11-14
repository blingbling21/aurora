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
 * 格式化工具函数测试
 */

import {
  formatFileSize,
  formatDate,
  formatDateToLocal,
  formatDateToUTC,
  formatPercent,
  formatCurrency,
  getTaskStatusText,
  getTaskStatusColor,
  generateDataFilename,
  debounce,
  throttle,
} from './format';

describe('Format Utils', () => {
  describe('formatFileSize', () => {
    it('应该正确格式化字节', () => {
      expect(formatFileSize(500)).toBe('500 B');
      expect(formatFileSize(1023)).toBe('1023 B');
    });

    it('应该正确格式化 KB', () => {
      expect(formatFileSize(1024)).toBe('1.00 KB');
      expect(formatFileSize(2048)).toBe('2.00 KB');
    });

    it('应该正确格式化 MB', () => {
      expect(formatFileSize(1048576)).toBe('1.00 MB');
      expect(formatFileSize(2097152)).toBe('2.00 MB');
    });

    it('应该正确格式化 GB', () => {
      expect(formatFileSize(1073741824)).toBe('1.00 GB');
      expect(formatFileSize(2147483648)).toBe('2.00 GB');
    });
  });

  describe('formatDate', () => {
    it('应该正确格式化日期', () => {
      const dateString = '2024-01-15T10:30:00Z';
      const result = formatDate(dateString);
      
      // 由于本地化可能有所不同，只检查是否包含年月日时分
      expect(result).toContain('2024');
      expect(result).toContain('01');
      expect(result).toContain('15');
    });
  });

  describe('formatDateToLocal', () => {
    it('应该将 Date 对象转换为本地时间的日期字符串（YYYY-MM-DD）', () => {
      const date = new Date(2025, 0, 1); // 2025-01-01 本地时间
      const result = formatDateToLocal(date);
      expect(result).toBe('2025-01-01');
    });

    it('应该正确处理单数字的月份和日期（补零）', () => {
      const date = new Date(2025, 0, 5); // 2025-01-05
      const result = formatDateToLocal(date);
      expect(result).toBe('2025-01-05');
    });

    it('应该使用本地时间方法进行格式化', () => {
      const date = new Date(2025, 11, 31); // 2025-12-31
      const result = formatDateToLocal(date);
      expect(result).toBe('2025-12-31');
    });

    it('应该处理年初和年末的日期', () => {
      const date1 = new Date(2025, 0, 1); // 年初
      expect(formatDateToLocal(date1)).toBe('2025-01-01');
      
      const date2 = new Date(2025, 11, 31); // 年末
      expect(formatDateToLocal(date2)).toBe('2025-12-31');
    });
  });

  describe('formatDateToUTC', () => {
    it('应该将本地时间的 Date 对象转换为 UTC 时间的日期字符串（YYYY-MM-DD）', () => {
      // 创建一个本地时间的 Date 对象
      // 假设我们在 UTC+8 时区
      const date = new Date(2025, 0, 1, 0, 0, 0); // 2025-01-01 00:00:00 本地时间
      const result = formatDateToUTC(date);
      
      // UTC 时间应该是前一天（减去8小时）
      // 2025-01-01 00:00:00 UTC+8 = 2024-12-31 16:00:00 UTC
      expect(result).toBe('2024-12-31');
    });

    it('应该正确处理不同时区的转换', () => {
      // 本地时间正午，转换为 UTC 不会跨天（对于 UTC+8 以内的时区）
      const date1 = new Date(2025, 0, 1, 12, 0, 0); // 2025-01-01 12:00:00 本地时间
      // 2025-01-01 12:00:00 UTC+8 = 2025-01-01 04:00:00 UTC
      expect(formatDateToUTC(date1)).toBe('2025-01-01');
      
      // 凌晨时间会跨天
      const date2 = new Date(2025, 0, 1, 0, 0, 0); // 2025-01-01 00:00:00 本地时间
      expect(formatDateToUTC(date2)).toBe('2024-12-31');
    });

    it('应该正确处理单数字的月份和日期（补零）', () => {
      const date = new Date(2025, 0, 1, 12, 0, 0); // 2025-01-01 12:00:00 本地时间
      const result = formatDateToUTC(date);
      
      // 验证月份和日期都有补零
      expect(result).toMatch(/^\d{4}-\d{2}-\d{2}$/);
    });

    it('应该使用 UTC 时间方法进行转换', () => {
      // 这个测试确保我们使用的是 UTC 时间方法
      const date = new Date(2025, 0, 1, 0, 0, 0); // 本地时间
      const result = formatDateToUTC(date);
      
      // 使用 getUTC* 方法应该得到 UTC 时间
      const expectedYear = date.getUTCFullYear();
      const expectedMonth = String(date.getUTCMonth() + 1).padStart(2, '0');
      const expectedDay = String(date.getUTCDate()).padStart(2, '0');
      const expected = `${expectedYear}-${expectedMonth}-${expectedDay}`;
      
      expect(result).toBe(expected);
    });
  });

  describe('formatPercent', () => {
    it('应该正确格式化百分比', () => {
      expect(formatPercent(12.345)).toBe('12.35%');
      expect(formatPercent(0.12)).toBe('0.12%');
      expect(formatPercent(100)).toBe('100.00%');
    });

    it('应该支持自定义小数位数', () => {
      expect(formatPercent(12.3456, 3)).toBe('12.346%');
      expect(formatPercent(12.3456, 0)).toBe('12%');
    });
  });

  describe('formatCurrency', () => {
    it('应该正确格式化货币', () => {
      expect(formatCurrency(1234.56)).toBe('$1234.56');
      expect(formatCurrency(0.99)).toBe('$0.99');
    });

    it('应该支持自定义货币符号', () => {
      expect(formatCurrency(1234.56, '¥')).toBe('¥1234.56');
      expect(formatCurrency(1234.56, '€')).toBe('€1234.56');
    });

    it('应该支持自定义小数位数', () => {
      expect(formatCurrency(1234.567, '$', 3)).toBe('$1234.567');
      expect(formatCurrency(1234.567, '$', 0)).toBe('$1235');
    });
  });

  describe('getTaskStatusText', () => {
    it('应该返回正确的中文状态文本', () => {
      expect(getTaskStatusText('pending')).toBe('等待中');
      expect(getTaskStatusText('running')).toBe('运行中');
      expect(getTaskStatusText('completed')).toBe('已完成');
      expect(getTaskStatusText('failed')).toBe('失败');
    });

    it('应该返回原状态文本（如果未定义）', () => {
      expect(getTaskStatusText('unknown')).toBe('unknown');
    });
  });

  describe('getTaskStatusColor', () => {
    it('应该返回正确的颜色类名', () => {
      expect(getTaskStatusColor('pending')).toBe('text-yellow-500');
      expect(getTaskStatusColor('running')).toBe('text-blue-500');
      expect(getTaskStatusColor('completed')).toBe('text-green-500');
      expect(getTaskStatusColor('failed')).toBe('text-red-500');
    });

    it('应该返回默认颜色类名（如果未定义）', () => {
      expect(getTaskStatusColor('unknown')).toBe('text-gray-500');
    });
  });

  describe('generateDataFilename', () => {
    it('应该生成正确的数据文件名', () => {
      const params = {
        exchange: 'Binance',
        symbol: 'BTCUSDT',
        interval: '1h',
        startDate: '2024-01-01',
        endDate: '2024-12-31',
      };

      const filename = generateDataFilename(params);

      expect(filename).toBe('binance_btcusdt_1h_20240101_to_20241231.csv');
    });

    it('应该转换为小写', () => {
      const params = {
        exchange: 'BINANCE',
        symbol: 'BTCUSDT',
        interval: '1h',
        startDate: '2024-01-01',
        endDate: '2024-12-31',
      };

      const filename = generateDataFilename(params);

      expect(filename).toBe('binance_btcusdt_1h_20240101_to_20241231.csv');
    });
  });

  describe('debounce', () => {
    jest.useFakeTimers();

    it('应该延迟执行函数', () => {
      const fn = jest.fn();
      const debouncedFn = debounce(fn, 100);

      // 调用多次
      debouncedFn();
      debouncedFn();
      debouncedFn();

      // 函数不应该立即执行
      expect(fn).not.toHaveBeenCalled();

      // 等待延迟时间
      jest.runAllTimers();

      // 函数应该只执行一次
      expect(fn).toHaveBeenCalledTimes(1);
    });

    it('应该取消之前的调用', () => {
      const fn = jest.fn();
      const debouncedFn = debounce(fn, 100);

      debouncedFn();
      jest.advanceTimersByTime(50);
      
      debouncedFn();
      jest.advanceTimersByTime(50);
      
      // 还没到 100ms
      expect(fn).not.toHaveBeenCalled();
      
      jest.advanceTimersByTime(50);
      
      // 第二次调用后 100ms
      expect(fn).toHaveBeenCalledTimes(1);
    });

    afterAll(() => {
      jest.useRealTimers();
    });
  });

  describe('throttle', () => {
    beforeEach(() => {
      jest.useFakeTimers();
    });

    afterEach(() => {
      jest.useRealTimers();
    });

    it('应该限制函数执行频率', () => {
      const fn = jest.fn();
      const throttledFn = throttle(fn, 100);

      // 第一次调用应该立即执行
      throttledFn();
      expect(fn).toHaveBeenCalledTimes(1);

      // 在限制时间内的调用应该被忽略
      throttledFn();
      throttledFn();
      expect(fn).toHaveBeenCalledTimes(1);

      // 等待限制时间
      jest.advanceTimersByTime(100);

      // 现在可以再次执行
      throttledFn();
      expect(fn).toHaveBeenCalledTimes(2);
    });
  });
});
