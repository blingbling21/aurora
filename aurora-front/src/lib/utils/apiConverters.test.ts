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
  convertApiTaskToLocal, 
  convertApiTasksToLocal,
  convertApiTaskSummaryToLocal,
  convertApiTaskSummariesToLocal
} from './apiConverters';
import { BacktestTask as ApiBacktestTask, BacktestTaskSummary as ApiBacktestTaskSummary } from '@/types/api';

describe('apiConverters', () => {
  describe('convertApiTaskToLocal', () => {
    it('应该正确转换 API 任务数据为前端格式', () => {
      // 准备测试数据
      const apiTask: ApiBacktestTask = {
        id: 'task-123',
        name: '测试任务',
        status: 'running',
        progress: 50,
        created_at: '2025-01-01T10:00:00Z',
        started_at: '2025-01-01T10:05:00Z',
        completed_at: undefined,
        config_path: '/configs/test.toml',
        data_path: '/data/test.csv',
      };

      // 执行转换
      const result = convertApiTaskToLocal(apiTask);

      // 验证结果
      expect(result).toEqual({
        id: 'task-123',
        name: '测试任务',
        status: 'running',
        config: '/configs/test.toml',
        dataFile: '/data/test.csv',
        progress: 50,
        createdAt: '2025-01-01T10:00:00Z',
        updatedAt: '2025-01-01T10:05:00Z',
      });
    });

    it('应该处理缺少可选字段的情况', () => {
      // 准备最小测试数据
      const apiTask: ApiBacktestTask = {
        id: 'task-456',
        name: '最小任务',
        status: 'pending',
        progress: 0,
        created_at: '2025-01-01T12:00:00Z',
      };

      // 执行转换
      const result = convertApiTaskToLocal(apiTask);

      // 验证结果
      expect(result).toEqual({
        id: 'task-456',
        name: '最小任务',
        status: 'pending',
        config: '',
        dataFile: '',
        progress: 0,
        createdAt: '2025-01-01T12:00:00Z',
        updatedAt: '2025-01-01T12:00:00Z',
      });
    });

    it('应该使用 completed_at 作为 updatedAt（如果存在）', () => {
      // 准备测试数据
      const apiTask: ApiBacktestTask = {
        id: 'task-789',
        name: '已完成任务',
        status: 'completed',
        progress: 100,
        created_at: '2025-01-01T10:00:00Z',
        started_at: '2025-01-01T10:05:00Z',
        completed_at: '2025-01-01T10:30:00Z',
        config_path: '/configs/completed.toml',
        data_path: '/data/completed.csv',
      };

      // 执行转换
      const result = convertApiTaskToLocal(apiTask);

      // 验证 updatedAt 使用 completed_at
      expect(result.updatedAt).toBe('2025-01-01T10:30:00Z');
    });
  });

  describe('convertApiTasksToLocal', () => {
    it('应该正确转换任务列表', () => {
      // 准备测试数据
      const apiTasks: ApiBacktestTask[] = [
        {
          id: 'task-1',
          name: '任务1',
          status: 'completed',
          progress: 100,
          created_at: '2025-01-01T10:00:00Z',
          completed_at: '2025-01-01T10:30:00Z',
          config_path: '/configs/task1.toml',
          data_path: '/data/task1.csv',
        },
        {
          id: 'task-2',
          name: '任务2',
          status: 'running',
          progress: 50,
          created_at: '2025-01-01T11:00:00Z',
          started_at: '2025-01-01T11:05:00Z',
          config_path: '/configs/task2.toml',
          data_path: '/data/task2.csv',
        },
      ];

      // 执行转换
      const result = convertApiTasksToLocal(apiTasks);

      // 验证结果
      expect(result).toHaveLength(2);
      expect(result[0].id).toBe('task-1');
      expect(result[0].status).toBe('completed');
      expect(result[1].id).toBe('task-2');
      expect(result[1].status).toBe('running');
    });

    it('应该处理空数组', () => {
      // 执行转换
      const result = convertApiTasksToLocal([]);

      // 验证结果
      expect(result).toEqual([]);
    });
  });

  describe('convertApiTaskSummaryToLocal', () => {
    it('应该正确转换 API 任务摘要数据为前端格式', () => {
      // 准备测试数据
      const apiTaskSummary: ApiBacktestTaskSummary = {
        id: 'task-123',
        name: '测试任务',
        status: 'running',
        progress: 50,
        created_at: '2025-01-01T10:00:00Z',
        started_at: '2025-01-01T10:05:00Z',
        completed_at: undefined,
        config_path: '/configs/test.toml',
        data_path: '/data/test.csv',
      };

      // 执行转换
      const result = convertApiTaskSummaryToLocal(apiTaskSummary);

      // 验证结果
      expect(result).toEqual({
        id: 'task-123',
        name: '测试任务',
        status: 'running',
        config: '/configs/test.toml',
        dataFile: '/data/test.csv',
        progress: 50,
        createdAt: '2025-01-01T10:00:00Z',
        updatedAt: '2025-01-01T10:05:00Z',
      });
    });
  });

  describe('convertApiTaskSummariesToLocal', () => {
    it('应该正确转换任务摘要列表', () => {
      // 准备测试数据
      const apiTaskSummaries: ApiBacktestTaskSummary[] = [
        {
          id: 'task-1',
          name: '任务1',
          status: 'completed',
          progress: 100,
          created_at: '2025-01-01T10:00:00Z',
          completed_at: '2025-01-01T10:30:00Z',
          config_path: '/configs/task1.toml',
          data_path: '/data/task1.csv',
        },
        {
          id: 'task-2',
          name: '任务2',
          status: 'running',
          progress: 50,
          created_at: '2025-01-01T11:00:00Z',
          started_at: '2025-01-01T11:05:00Z',
          config_path: '/configs/task2.toml',
          data_path: '/data/task2.csv',
        },
      ];

      // 执行转换
      const result = convertApiTaskSummariesToLocal(apiTaskSummaries);

      // 验证结果
      expect(result).toHaveLength(2);
      expect(result[0].id).toBe('task-1');
      expect(result[0].status).toBe('completed');
      expect(result[1].id).toBe('task-2');
      expect(result[1].status).toBe('running');
    });

    it('应该处理空数组', () => {
      // 执行转换
      const result = convertApiTaskSummariesToLocal([]);

      // 验证结果
      expect(result).toEqual([]);
    });
  });
});
