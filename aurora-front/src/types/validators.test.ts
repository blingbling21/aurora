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
  validateBacktestTask,
  validateBacktestResult,
  validateConfigFile,
  validateDataFile,
  validateNotification,
  validateDataDownloadRequest,
  validateBacktestConfig,
  safeParseData,
  formatValidationErrors,
} from './validators';
import {
  NotificationSchema,
} from './schemas';

describe('validators', () => {
  // 测试 validateBacktestTask
  describe('validateBacktestTask', () => {
    it('应该验证有效的回测任务数据', () => {
      const validData = {
        id: 'task-1',
        name: '测试任务',
        status: 'pending',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 0,
        createdAt: '2024-01-01T00:00:00Z',
        updatedAt: '2024-01-01T00:00:00Z',
      };

      const result = validateBacktestTask(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data).toEqual(validData);
      }
    });

    it('应该拒绝无效的回测任务数据', () => {
      const invalidData = {
        id: '',
        name: '',
        status: 'invalid',
        progress: 150,
      };

      const result = validateBacktestTask(invalidData);
      
      expect(result.success).toBe(false);
      if (!result.success) {
        expect(result.errors).toBeDefined();
        expect(result.errors.length).toBeGreaterThan(0);
      }
    });

    it('应该拒绝进度超出范围的数据', () => {
      const invalidData = {
        id: 'task-1',
        name: '测试任务',
        status: 'running',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 150, // 超过100
        createdAt: '2024-01-01T00:00:00Z',
        updatedAt: '2024-01-01T00:00:00Z',
      };

      const result = validateBacktestTask(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 validateBacktestResult
  describe('validateBacktestResult', () => {
    it('应该验证有效的回测结果数据', () => {
      const validData = {
        taskId: 'task-1',
        metrics: {
          totalReturn: 15.5,
          annualizedReturn: 18.2,
          maxDrawdown: -8.2,
          maxDrawdownDuration: 5,
          sharpeRatio: 1.8,
          sortinoRatio: 2.1,
          calmarRatio: 2.2,
          annualizedVolatility: 12.5,
          winRate: 65,
          totalTrades: 100,
          winningTrades: 65,
          losingTrades: 35,
          averageWin: 150,
          averageLoss: -80,
          profitLossRatio: 1.875,
          profitFactor: 2.1,
          maxConsecutiveWins: 8,
          maxConsecutiveLosses: 4,
          avgHoldingPeriod: 24,
          maxWin: 500,
          maxLoss: -250,
        },
        equityCurve: [
          { time: '2024-01-01T00:00:00Z', value: 10000 },
          { time: '2024-01-02T00:00:00Z', value: 10100 },
        ],
        trades: [],
      };

      const result = validateBacktestResult(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.taskId).toBe('task-1');
      }
    });

    it('应该拒绝缺少必需字段的结果数据', () => {
      const invalidData = {
        taskId: 'task-1',
        metrics: {
          totalReturn: 15.5,
        },
        equityCurve: [],
        trades: [],
      };

      const result = validateBacktestResult(invalidData);
      
      expect(result.success).toBe(false);
    });

    it('应该拒绝权益曲线为空的结果数据', () => {
      const invalidData = {
        taskId: 'task-1',
        metrics: {
          totalReturn: 15.5,
          annualizedReturn: 18.2,
          maxDrawdown: -8.2,
          maxDrawdownDuration: 5,
          sharpeRatio: 1.8,
          sortinoRatio: 2.1,
          calmarRatio: 2.2,
          annualizedVolatility: 12.5,
          winRate: 65,
          totalTrades: 100,
          winningTrades: 65,
          losingTrades: 35,
          averageWin: 150,
          averageLoss: -80,
          profitLossRatio: 1.875,
          profitFactor: 2.1,
          maxConsecutiveWins: 8,
          maxConsecutiveLosses: 4,
          avgHoldingPeriod: 24,
          maxWin: 500,
          maxLoss: -250,
        },
        equityCurve: [], // 空数组
        trades: [],
      };

      const result = validateBacktestResult(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 validateConfigFile
  describe('validateConfigFile', () => {
    it('应该验证有效的配置文件数据', () => {
      const validData = {
        name: 'config.toml',
        path: '/path/to/config.toml',
        content: '[strategy]\nname = "MA Cross"',
        lastModified: '2024-01-01T00:00:00Z',
      };

      const result = validateConfigFile(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.name).toBe('config.toml');
      }
    });

    it('应该拒绝无效的配置文件数据', () => {
      const invalidData = {
        name: '',
        path: '',
        lastModified: 'invalid-date',
      };

      const result = validateConfigFile(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 validateDataFile
  describe('validateDataFile', () => {
    it('应该验证有效的数据文件信息', () => {
      const validData = {
        name: 'btc_1h.csv',
        path: '/path/to/btc_1h.csv',
        size: 1024000,
        lastModified: '2024-01-01T00:00:00Z',
      };

      const result = validateDataFile(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.size).toBe(1024000);
      }
    });

    it('应该拒绝文件大小为负数的数据', () => {
      const invalidData = {
        name: 'btc_1h.csv',
        path: '/path/to/btc_1h.csv',
        size: -1000,
        lastModified: '2024-01-01T00:00:00Z',
      };

      const result = validateDataFile(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 validateNotification
  describe('validateNotification', () => {
    it('应该验证有效的通知数据', () => {
      const validData = {
        id: 'notif-1',
        type: 'success',
        message: '操作成功',
        duration: 3000,
      };

      const result = validateNotification(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.type).toBe('success');
      }
    });

    it('应该拒绝无效的通知类型', () => {
      const invalidData = {
        id: 'notif-1',
        type: 'invalid-type',
        message: '测试消息',
      };

      const result = validateNotification(invalidData);
      
      expect(result.success).toBe(false);
    });

    it('应该接受没有duration的通知', () => {
      const validData = {
        id: 'notif-1',
        type: 'info',
        message: '信息提示',
      };

      const result = validateNotification(validData);
      
      expect(result.success).toBe(true);
    });
  });

  // 测试 validateDataDownloadRequest
  describe('validateDataDownloadRequest', () => {
    it('应该验证有效的数据下载请求', () => {
      const validData = {
        exchange: 'binance',
        symbol: 'BTCUSDT',
        interval: '1h',
        startDate: '2024-01-01T00:00:00Z',
        endDate: '2024-01-31T23:59:59Z',
      };

      const result = validateDataDownloadRequest(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.symbol).toBe('BTCUSDT');
      }
    });

    it('应该拒绝结束日期早于开始日期的请求', () => {
      const invalidData = {
        exchange: 'binance',
        symbol: 'BTCUSDT',
        interval: '1h',
        startDate: '2024-01-31T00:00:00Z',
        endDate: '2024-01-01T00:00:00Z',
      };

      const result = validateDataDownloadRequest(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 validateBacktestConfig
  describe('validateBacktestConfig', () => {
    it('应该验证有效的回测配置', () => {
      const validData = {
        taskName: '测试任务',
        configFile: 'config.toml',
        dataFile: 'data.csv',
        description: '这是一个测试任务',
      };

      const result = validateBacktestConfig(validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.taskName).toBe('测试任务');
      }
    });

    it('应该接受没有描述的配置', () => {
      const validData = {
        taskName: '测试任务',
        configFile: 'config.toml',
        dataFile: 'data.csv',
      };

      const result = validateBacktestConfig(validData);
      
      expect(result.success).toBe(true);
    });

    it('应该拒绝任务名称过长的配置', () => {
      const invalidData = {
        taskName: 'a'.repeat(101), // 超过100字符
        configFile: 'config.toml',
        dataFile: 'data.csv',
      };

      const result = validateBacktestConfig(invalidData);
      
      expect(result.success).toBe(false);
    });

    it('应该拒绝描述过长的配置', () => {
      const invalidData = {
        taskName: '测试任务',
        configFile: 'config.toml',
        dataFile: 'data.csv',
        description: 'a'.repeat(501), // 超过500字符
      };

      const result = validateBacktestConfig(invalidData);
      
      expect(result.success).toBe(false);
    });
  });

  // 测试 safeParseData
  describe('safeParseData', () => {
    it('应该安全解析有效数据', () => {
      const validData = {
        id: 'notif-1',
        type: 'success',
        message: '测试消息',
      };

      const result = safeParseData(NotificationSchema, validData);
      
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.id).toBe('notif-1');
      }
    });

    it('应该安全解析无效数据并返回错误', () => {
      const invalidData = {
        id: '',
        type: 'invalid',
      };

      const result = safeParseData(NotificationSchema, invalidData);
      
      expect(result.success).toBe(false);
      if (!result.success) {
        expect(result.error).toBeDefined();
      }
    });
  });

  // 测试 formatValidationErrors
  describe('formatValidationErrors', () => {
    it('应该格式化验证错误信息', () => {
      const invalidData = {
        id: '',
        name: '',
        progress: 150,
      };

      const result = validateBacktestTask(invalidData);
      
      if (!result.success && 'code' in result.errors[0]) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const formatted = formatValidationErrors(result.errors as any);
        expect(formatted).toBeTruthy();
        expect(typeof formatted).toBe('string');
        expect(formatted.length).toBeGreaterThan(0);
      }
    });

    it('应该包含字段路径和错误消息', () => {
      const errors = [
        { path: ['name'], message: '名称不能为空', code: 'custom' as const },
        { path: ['progress'], message: '进度不能大于100', code: 'custom' as const },
      ];

      const formatted = formatValidationErrors(errors);
      
      expect(formatted).toContain('name:');
      expect(formatted).toContain('progress:');
      expect(formatted).toContain('名称不能为空');
      expect(formatted).toContain('进度不能大于100');
    });

    it('应该处理没有路径的错误', () => {
      const errors = [
        { path: [], message: '整体验证失败', code: 'custom' as const },
      ];

      const formatted = formatValidationErrors(errors);
      
      expect(formatted).toBe('整体验证失败');
    });

    it('应该用分号分隔多个错误', () => {
      const errors = [
        { path: ['field1'], message: '错误1', code: 'custom' as const },
        { path: ['field2'], message: '错误2', code: 'custom' as const },
      ];

      const formatted = formatValidationErrors(errors);
      
      expect(formatted).toContain(';');
      expect(formatted.split(';').length).toBe(2);
    });
  });
});
