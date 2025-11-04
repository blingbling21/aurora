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
 * 回测管理 API 服务测试
 */

import { BacktestService, backtestApi } from './backtest';
import * as client from './client';
import type {
  BacktestTask,
  StartBacktestRequest,
  FullBacktestResult,
} from '@/types/api';

// Mock client 模块
jest.mock('./client');

describe('BacktestService', () => {
  // 重置所有 mock
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('list', () => {
    it('应该成功获取回测任务列表', async () => {
      // 准备测试数据
      const mockTasks: BacktestTask[] = [
        {
          id: 'task-1',
          name: 'Test Task',
          status: 'completed',
          progress: 100,
          created_at: '2025-01-01T00:00:00Z',
        },
      ];
      const mockResponse = {
        success: true,
        data: mockTasks,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await BacktestService.list();

      // 验证结果
      expect(client.get).toHaveBeenCalledWith('/backtest/history');
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockTasks);
    });

    it('应该处理 API 错误', async () => {
      // 模拟 API 错误
      const mockError = {
        success: false,
        error: 'Network error',
      };
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await BacktestService.list();

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Network error');
    });
  });

  describe('get', () => {
    it('应该成功获取指定任务详情', async () => {
      // 准备测试数据
      const taskId = 'task-123';
      const mockTask: BacktestTask = {
        id: taskId,
        name: 'Test Task',
        status: 'running',
        progress: 50,
        created_at: '2025-01-01T00:00:00Z',
        started_at: '2025-01-01T00:10:00Z',
      };
      const mockResponse = {
        success: true,
        data: mockTask,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await BacktestService.get(taskId);

      // 验证结果
      expect(client.get).toHaveBeenCalledWith(`/backtest/${taskId}`);
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockTask);
    });

    it('应该正确编码特殊字符', async () => {
      // 准备测试数据
      const taskId = 'task/with/slash';
      const mockResponse = { success: true, data: {} as BacktestTask };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await BacktestService.get(taskId);

      // 验证 URL 编码
      expect(client.get).toHaveBeenCalledWith(
        `/backtest/${encodeURIComponent(taskId)}`
      );
    });
  });

  describe('start', () => {
    it('应该成功启动回测任务', async () => {
      // 准备测试数据
      const request: StartBacktestRequest = {
        name: 'My Backtest',
        config_path: 'configs/test-config.toml',
        data_path: 'data/test-data.csv',
      };
      const mockResponse = {
        success: true,
        data: { task_id: 'new-task-123' },
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await BacktestService.start(request);

      // 验证结果
      expect(client.post).toHaveBeenCalledWith('/backtest/start', request);
      expect(result).toEqual(mockResponse);
      expect(result.data?.task_id).toBe('new-task-123');
    });

    it('应该处理启动失败的情况', async () => {
      // 准备测试数据
      const request: StartBacktestRequest = {
        name: 'Invalid Backtest',
        config_path: 'configs/invalid-config.toml',
        data_path: 'data/invalid-data.csv',
      };
      const mockError = {
        success: false,
        error: 'Config not found',
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await BacktestService.start(request);

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Config not found');
    });
  });

  describe('delete', () => {
    it('应该成功删除回测任务', async () => {
      // 准备测试数据
      const taskId = 'task-to-delete';
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.del as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await BacktestService.delete(taskId);

      // 验证结果
      expect(client.del).toHaveBeenCalledWith(`/backtest/${taskId}`);
      expect(result).toEqual(mockResponse);
    });
  });

  describe('getResult', () => {
    it('应该成功获取回测结果', async () => {
      // 准备测试数据
      const taskId = 'completed-task';
      const mockResult: FullBacktestResult = {
        result: {
          metrics: {
            total_return: 0.15,
            sharpe_ratio: 1.5,
            max_drawdown: 0.1,
            win_rate: 0.6,
            total_trades: 100,
          },
          trades: [],
          equity_curve: [],
        },
        benchmark_equity_curve: [],
      };
      const mockResponse = {
        success: true,
        data: mockResult,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await BacktestService.getResult(taskId);

      // 验证结果
      expect(client.get).toHaveBeenCalledWith(`/backtest/result/${taskId}`);
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockResult);
    });
  });

  describe('getWebSocketUrl', () => {
    it('应该根据当前协议生成正确的 WebSocket URL', () => {
      // 准备测试数据
      const taskId = 'test-task';
      
      // 执行测试（使用当前环境的 window.location）
      const url = BacktestService.getWebSocketUrl(taskId);

      // 验证URL格式正确
      expect(url).toMatch(/^wss?:\/\//); // 以 ws: 或 wss: 开头
      expect(url).toContain('/ws/backtest/');
      expect(url).toContain(taskId);
    });

    it('应该正确编码 taskId', () => {
      // 准备测试数据
      const taskId = 'task/with/slash';

      // 执行测试
      const url = BacktestService.getWebSocketUrl(taskId);

      // 验证 taskId 被正确编码
      expect(url).toContain(encodeURIComponent(taskId));
      expect(url).not.toContain('task/with/slash'); // 不应包含未编码的斜杠
    });
  });
});

describe('backtestApi', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('应该暴露所有 BacktestService 方法', () => {
    // 验证所有方法都已导出
    expect(backtestApi.list).toBeDefined();
    expect(backtestApi.get).toBeDefined();
    expect(backtestApi.start).toBeDefined();
    expect(backtestApi.delete).toBeDefined();
    expect(backtestApi.getResult).toBeDefined();
    expect(backtestApi.getWebSocketUrl).toBeDefined();
  });

  it('list 方法应该调用 BacktestService.list', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: [] };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    await backtestApi.list();

    // 验证调用
    expect(client.get).toHaveBeenCalledWith('/backtest/history');
  });

  it('get 方法应该调用 BacktestService.get', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: {} as BacktestTask };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    const taskId = 'test-task';
    await backtestApi.get(taskId);

    // 验证调用
    expect(client.get).toHaveBeenCalledWith(`/backtest/${taskId}`);
  });
});
