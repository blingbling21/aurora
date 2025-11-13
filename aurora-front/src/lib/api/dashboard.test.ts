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

import { DashboardService } from './dashboard';
import * as client from './client';

// Mock client 模块
jest.mock('./client');

describe('DashboardService', () => {
  describe('getData', () => {
    it('应该成功获取仪表盘数据', async () => {
      // 准备 mock 响应
      const mockResponse = {
        success: true,
        data: {
          stats: {
            total_tasks: 10,
            running_tasks: 2,
            completed_tasks: 7,
            failed_tasks: 1,
          },
          recent_tasks: [
            {
              id: 'task-1',
              name: '最近任务1',
              status: 'completed' as const,
              progress: 100,
              created_at: '2025-01-01T10:00:00Z',
              completed_at: '2025-01-01T10:30:00Z',
              config_path: '/configs/task1.toml',
              data_path: '/data/task1.csv',
            },
            {
              id: 'task-2',
              name: '最近任务2',
              status: 'running' as const,
              progress: 50,
              created_at: '2025-01-01T11:00:00Z',
              started_at: '2025-01-01T11:05:00Z',
              config_path: '/configs/task2.toml',
              data_path: '/data/task2.csv',
            },
          ],
        },
      };

      // Mock get 方法
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 调用 API
      const result = await DashboardService.getData();

      // 验证结果
      expect(result).toEqual(mockResponse);
      expect(client.get).toHaveBeenCalledWith('/dashboard');
    });

    it('应该处理 API 错误', async () => {
      // 准备错误响应
      const mockError = {
        success: false,
        error: '服务器错误',
      };

      // Mock get 方法返回错误
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 调用 API
      const result = await DashboardService.getData();

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('服务器错误');
    });

    it('应该处理网络异常', async () => {
      // Mock get 方法抛出异常
      (client.get as jest.Mock).mockRejectedValue(new Error('网络错误'));

      // 调用 API 并期望抛出异常
      await expect(DashboardService.getData()).rejects.toThrow('网络错误');
    });
  });
});
