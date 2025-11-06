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

import { z } from 'zod';
import {
  ApiResponseSchema,
  ConfigListItemSchema,
  CreateConfigRequestSchema,
  UpdateConfigRequestSchema,
  ConfigValidateResponseSchema,
  DataFileItemSchema,
  FetchDataRequestSchema,
  KlineSchema,
  TaskStatusSchema,
  BacktestTaskSchema,
  StartBacktestRequestSchema,
} from './api';

describe('API Types and Schemas', () => {
  // 测试 ApiResponseSchema
  describe('ApiResponseSchema', () => {
    // 测试有效的成功响应
    it('应该验证成功的响应', () => {
      const schema = ApiResponseSchema(z.string());
      const validData = {
        success: true,
        data: 'test data',
      };
      
      expect(() => schema.parse(validData)).not.toThrow();
    });

    // 测试有效的失败响应
    it('应该验证失败的响应', () => {
      const schema = ApiResponseSchema(z.string());
      const validData = {
        success: false,
        error: '错误信息',
      };
      
      expect(() => schema.parse(validData)).not.toThrow();
    });

    // 测试包含消息的响应
    it('应该支持可选的 message 字段', () => {
      const schema = ApiResponseSchema(z.string());
      const validData = {
        success: true,
        message: '操作成功',
        data: 'test',
      };
      
      expect(() => schema.parse(validData)).not.toThrow();
    });

    // 测试缺少必需字段
    it('缺少 success 字段应该失败', () => {
      const schema = ApiResponseSchema(z.string());
      const invalidData = {
        data: 'test',
      };
      
      expect(() => schema.parse(invalidData)).toThrow();
    });
  });

  // 测试 ConfigListItemSchema
  describe('ConfigListItemSchema', () => {
    // 测试有效的配置项
    it('应该验证有效的配置列表项', () => {
      const validItem = {
        filename: 'config.toml',
        path: '/configs/config.toml',
        modified: '2025-01-01',
      };
      
      expect(() => ConfigListItemSchema.parse(validItem)).not.toThrow();
    });

    // 测试所有字段
    it('应该包含 path 字段', () => {
      const validItem = {
        filename: 'config.toml',
        path: '/configs/config.toml',
        modified: '2025-01-01',
      };
      
      const result = ConfigListItemSchema.parse(validItem);
      expect(result.path).toBe('/configs/config.toml');
    });

    // 测试无效数据
    it('缺少必需字段应该失败', () => {
      const invalidItem = {
        filename: 'config.toml',
      };
      
      expect(() => ConfigListItemSchema.parse(invalidItem)).toThrow();
    });

    it('缺少 path 字段应该失败', () => {
      const invalidItem = {
        filename: 'config.toml',
        modified: '2025-01-01',
      };
      
      expect(() => ConfigListItemSchema.parse(invalidItem)).toThrow();
    });
  });

  // 测试 CreateConfigRequestSchema
  describe('CreateConfigRequestSchema', () => {
    // 测试有效的创建请求
    it('应该验证有效的创建配置请求', () => {
      const validRequest = {
        filename: 'new_config.toml',
        content: '[strategy]\nname = "test"',
      };
      
      expect(() => CreateConfigRequestSchema.parse(validRequest)).not.toThrow();
    });

    // 测试空文件名
    it('空文件名应该失败', () => {
      const invalidRequest = {
        filename: '',
        content: 'content',
      };
      
      expect(() => CreateConfigRequestSchema.parse(invalidRequest)).toThrow();
    });

    // 测试空内容
    it('空内容应该失败', () => {
      const invalidRequest = {
        filename: 'config.toml',
        content: '',
      };
      
      expect(() => CreateConfigRequestSchema.parse(invalidRequest)).toThrow();
    });
  });

  // 测试 UpdateConfigRequestSchema
  describe('UpdateConfigRequestSchema', () => {
    // 测试有效的更新请求
    it('应该验证有效的更新配置请求', () => {
      const validRequest = {
        content: '[strategy]\nname = "updated"',
      };
      
      expect(() => UpdateConfigRequestSchema.parse(validRequest)).not.toThrow();
    });

    // 测试空内容
    it('空内容应该失败', () => {
      const invalidRequest = {
        content: '',
      };
      
      expect(() => UpdateConfigRequestSchema.parse(invalidRequest)).toThrow();
    });
  });

  // 测试 ConfigValidateResponseSchema
  describe('ConfigValidateResponseSchema', () => {
    // 测试有效的验证响应
    it('应该验证有效的配置验证响应', () => {
      const validResponse = {
        valid: true,
      };
      
      expect(() => ConfigValidateResponseSchema.parse(validResponse)).not.toThrow();
    });

    // 测试包含错误的响应
    it('应该支持错误列表', () => {
      const validResponse = {
        valid: false,
        errors: ['错误1', '错误2'],
      };
      
      expect(() => ConfigValidateResponseSchema.parse(validResponse)).not.toThrow();
    });

    // 测试包含警告的响应
    it('应该支持警告列表', () => {
      const validResponse = {
        valid: true,
        warnings: ['警告1', '警告2'],
      };
      
      expect(() => ConfigValidateResponseSchema.parse(validResponse)).not.toThrow();
    });
  });

  // 测试 DataFileItemSchema
  describe('DataFileItemSchema', () => {
    // 测试有效的数据文件项
    it('应该验证有效的数据文件项', () => {
      const validItem = {
        filename: 'btc_1h.csv',
        size: 10240,
        modified: '2025-01-01',
        record_count: 1000,
      };
      
      expect(() => DataFileItemSchema.parse(validItem)).not.toThrow();
    });

    // 测试 record_count 可选
    it('record_count 应该是可选的', () => {
      const validItem = {
        filename: 'btc_1h.csv',
        size: 10240,
        modified: '2025-01-01',
      };
      
      expect(() => DataFileItemSchema.parse(validItem)).not.toThrow();
    });

    // 测试无效的大小类型
    it('size 必须是数字', () => {
      const invalidItem = {
        filename: 'btc_1h.csv',
        size: '10240',
        modified: '2025-01-01',
      };
      
      expect(() => DataFileItemSchema.parse(invalidItem)).toThrow();
    });
  });

  // 测试 FetchDataRequestSchema
  describe('FetchDataRequestSchema', () => {
    // 测试有效的数据获取请求
    it('应该验证有效的数据获取请求', () => {
      const validRequest = {
        exchange: 'binance',
        symbol: 'BTCUSDT',
        interval: '1h',
        start_date: '2025-01-01',
        end_date: '2025-01-31',
      };
      
      expect(() => FetchDataRequestSchema.parse(validRequest)).not.toThrow();
    });

    // 测试缺少必需字段
    it('缺少必需字段应该失败', () => {
      const invalidRequest = {
        exchange: 'binance',
        symbol: 'BTCUSDT',
      };
      
      expect(() => FetchDataRequestSchema.parse(invalidRequest)).toThrow();
    });
  });

  // 测试 KlineSchema
  describe('KlineSchema', () => {
    // 测试有效的 K 线数据
    it('应该验证有效的 K 线数据', () => {
      const validKline = {
        timestamp: 1704067200000,
        open: 45000,
        high: 46000,
        low: 44000,
        close: 45500,
        volume: 1000,
      };
      
      expect(() => KlineSchema.parse(validKline)).not.toThrow();
    });

    // 测试缺少必需字段
    it('缺少必需字段应该失败', () => {
      const invalidKline = {
        timestamp: 1704067200000,
        open: 45000,
        high: 46000,
        // 缺少 low, close, volume
      };
      
      expect(() => KlineSchema.parse(invalidKline)).toThrow();
    });
  });

  // 测试 TaskStatusSchema
  describe('TaskStatusSchema', () => {
    // 测试有效的任务状态
    it('应该验证有效的任务状态', () => {
      const validStatuses = ['pending', 'running', 'completed', 'failed'];
      
      validStatuses.forEach(status => {
        expect(() => TaskStatusSchema.parse(status)).not.toThrow();
      });
    });

    // 测试无效的状态
    it('无效的状态应该失败', () => {
      expect(() => TaskStatusSchema.parse('invalid_status')).toThrow();
    });
  });

  // 测试 BacktestTaskSchema
  describe('BacktestTaskSchema', () => {
    // 测试有效的回测任务
    it('应该验证有效的回测任务', () => {
      const validTask = {
        id: 'task-123',
        name: 'Test Backtest',
        status: 'running',
        progress: 50,
        created_at: '2025-01-01T00:00:00Z',
        config_path: 'config.toml',
        data_path: 'btc_1h.csv',
      };
      
      expect(() => BacktestTaskSchema.parse(validTask)).not.toThrow();
    });

    // 测试可选字段
    it('应该支持可选字段', () => {
      const validTask = {
        id: 'task-123',
        name: 'Test Backtest',
        status: 'pending',
        progress: 0,
        created_at: '2025-01-01T00:00:00Z',
      };
      
      expect(() => BacktestTaskSchema.parse(validTask)).not.toThrow();
    });
  });

  // 测试 StartBacktestRequestSchema
  describe('StartBacktestRequestSchema', () => {
    // 测试有效的启动回测请求
    it('应该验证有效的启动回测请求', () => {
      const validRequest = {
        name: 'My Backtest',
        config_path: 'config.toml',
        data_path: 'btc_1h.csv',
      };
      
      expect(() => StartBacktestRequestSchema.parse(validRequest)).not.toThrow();
    });

    // 测试缺少必需字段
    it('缺少必需字段应该失败', () => {
      const invalidRequest = {
        name: 'My Backtest',
      };
      
      expect(() => StartBacktestRequestSchema.parse(invalidRequest)).toThrow();
    });
  });
});
