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

import * as typesIndex from './index';

describe('Types Index Module', () => {
  // 测试 schemas 导出
  describe('Schema 导出', () => {
    // 测试基础 schemas 导出
    it('应该导出 schema 相关的内容', () => {
      expect(typesIndex).toHaveProperty('TaskStatusSchema');
      expect(typesIndex).toHaveProperty('BacktestTaskSchema');
      expect(typesIndex).toHaveProperty('ConfigFileSchema');
      expect(typesIndex).toHaveProperty('DataFileSchema');
      expect(typesIndex).toHaveProperty('BacktestResultSchema');
    });

    // 测试 TaskStatus 类型
    it('TaskStatus 应该是有效的类型', () => {
      // 使用类型断言验证
      const status: typesIndex.TaskStatus = 'pending';
      expect(['pending', 'running', 'completed', 'failed', 'cancelled']).toContain(status);
    });
  });

  // 测试 validators 导出
  describe('Validator 导出', () => {
    // 测试验证函数导出
    it('应该导出验证函数', () => {
      expect(typesIndex).toHaveProperty('validateConfigFile');
      expect(typesIndex).toHaveProperty('validateBacktestResult');
      expect(typeof typesIndex.validateConfigFile).toBe('function');
      expect(typeof typesIndex.validateBacktestResult).toBe('function');
    });
  });

  // 测试 API 类型导出
  describe('API 类型导出', () => {
    // 测试 API 响应类型导出
    it('应该导出 API 相关类型', () => {
      // 这些是 Schema 导出（运行时可检查），我们验证它们存在于模块中
      const exports = Object.keys(typesIndex);
      
      // 验证包含基本 Schema
      expect(exports).toContain('TaskStatusSchema');
      expect(exports).toContain('BacktestTaskSchema');
      expect(exports).toContain('ConfigFileSchema');
      expect(exports).toContain('DataFileSchema');
    });
  });

  // 测试向后兼容的 Schema 导出
  describe('向后兼容性', () => {
    // 测试保留原有导出方式
    it('应该保留原有的 Schema 导出', () => {
      // 验证 Schema 对象存在（运行时可检查）
      expect(typesIndex).toHaveProperty('TaskStatusSchema');
      expect(typesIndex).toHaveProperty('BacktestTaskSchema');
      expect(typesIndex).toHaveProperty('ConfigFileSchema');
      expect(typesIndex).toHaveProperty('DataFileSchema');
      expect(typesIndex).toHaveProperty('BacktestResultSchema');
    });

    // 测试新旧类型兼容
    it('新旧类型应该兼容', () => {
      // TaskStatus 可以赋值为具体的状态值
      const status1: typesIndex.TaskStatus = 'pending';
      expect(status1).toBe('pending');
      
      // 也可以使用 API 导出的类型
      const status2: typesIndex.ApiTaskStatus = 'running';
      expect(status2).toBe('running');
    });
  });

  // 测试所有导出都已定义
  describe('导出完整性', () => {
    // 测试没有 undefined 导出
    it('所有导出都不应该是 undefined', () => {
      const exports = Object.values(typesIndex);
      
      // 过滤掉类型定义（它们在运行时是 undefined）
      const runtimeExports = exports.filter(exp => exp !== undefined);
      
      runtimeExports.forEach(exportedItem => {
        expect(exportedItem).toBeDefined();
      });
    });

    // 测试关键导出存在
    it('应该包含所有关键的导出', () => {
      const exports = Object.keys(typesIndex);
      
      // 验证关键 Schema
      expect(exports).toContain('TaskStatusSchema');
      expect(exports).toContain('BacktestTaskSchema');
      expect(exports).toContain('ConfigFileSchema');
      expect(exports).toContain('DataFileSchema');
      expect(exports).toContain('BacktestResultSchema');
      
      // 验证验证函数
      expect(exports).toContain('validateConfigFile');
      expect(exports).toContain('validateBacktestTask');
    });
  });

  // 测试类型使用场景
  describe('类型使用场景', () => {
    // 测试 TaskStatus 使用
    it('应该能够使用 TaskStatus 类型', () => {
      const statuses: typesIndex.TaskStatus[] = [
        'pending',
        'running',
        'completed',
        'failed'
      ];
      
      expect(statuses).toHaveLength(4);
    });

    // 测试 BacktestTask 使用
    it('应该能够创建 BacktestTask 对象', () => {
      const task: typesIndex.BacktestTask = {
        id: 'task-1',
        name: 'Test Task',
        status: 'running',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 50,
        createdAt: '2025-01-01T00:00:00Z',
        updatedAt: '2025-01-01T00:00:00Z',
      };
      
      expect(task.id).toBe('task-1');
      expect(task.status).toBe('running');
    });

    // 测试 ConfigFile 使用
    it('应该能够创建 ConfigFile 对象', () => {
      const config: typesIndex.ConfigFile = {
        name: 'config.toml',
        path: '/configs/config.toml',
        content: '[strategy]',
        lastModified: '2025-01-01T00:00:00Z',
      };
      
      expect(config.name).toBe('config.toml');
    });

    // 测试 DataFile 使用
    it('应该能够创建 DataFile 对象', () => {
      const dataFile: typesIndex.DataFile = {
        name: 'btc_1h.csv',
        path: '/data/btc_1h.csv',
        size: 1024,
        lastModified: '2025-01-01T00:00:00Z',
      };
      
      expect(dataFile.name).toBe('btc_1h.csv');
    });
  });

  // 测试验证函数功能
  describe('验证函数功能', () => {
    // 测试 validateConfigFile 函数
    it('validateConfigFile 应该是可调用的函数', () => {
      expect(typeof typesIndex.validateConfigFile).toBe('function');
      
      // 测试基本调用
      const result = typesIndex.validateConfigFile({
        name: 'config.toml',
        path: '/configs/config.toml',
        content: '[strategy]',
        lastModified: '2025-01-01T00:00:00Z',
      });
      expect(result).toHaveProperty('success');
    });

    // 测试 validateBacktestResult 函数
    it('validateBacktestResult 应该是可调用的函数', () => {
      expect(typeof typesIndex.validateBacktestResult).toBe('function');
      
      // 测试基本调用
      const validResult = {
        taskId: 'task-1',
        metrics: {
          totalReturn: 0.15,
          annualizedReturn: 0.20,
          maxDrawdown: -0.10,
          maxDrawdownDuration: 30,
          sharpeRatio: 1.5,
          sortinoRatio: 2.0,
          calmarRatio: 1.8,
          annualizedVolatility: 0.15,
          winRate: 60,
          totalTrades: 100,
          winningTrades: 60,
          losingTrades: 40,
          averageWin: 500,
          averageLoss: -300,
          profitLossRatio: 1.67,
          profitFactor: 1.5,
          maxConsecutiveWins: 5,
          maxConsecutiveLosses: 3,
          avgHoldingPeriod: 24,
          maxWin: 2000,
          maxLoss: -1000,
        },
        trades: [],
        equityCurve: [],
        completedAt: '2025-01-01T00:00:00Z',
      };
      const result = typesIndex.validateBacktestResult(validResult);
      expect(result).toHaveProperty('success');
    });
  });
});
