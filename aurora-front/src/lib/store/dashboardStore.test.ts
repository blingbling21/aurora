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

import { renderHook, act, waitFor } from '@testing-library/react';
import { useDashboardStore } from './dashboardStore';
import { DashboardService } from '@/lib/api/dashboard';

// Mock DashboardService
jest.mock('@/lib/api/dashboard');

describe('useDashboardStore', () => {
  beforeEach(() => {
    // 重置 store 状态
    const { result } = renderHook(() => useDashboardStore());
    act(() => {
      result.current.setLoading(false);
      result.current.setError(null);
    });
    jest.clearAllMocks();
  });

  it('应该有正确的初始状态', () => {
    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 验证初始状态
    expect(result.current.stats).toEqual({
      total_tasks: 0,
      running_tasks: 0,
      completed_tasks: 0,
      failed_tasks: 0,
    });
    expect(result.current.recentTasks).toEqual([]);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it('应该成功加载仪表盘数据', async () => {
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
            name: '测试任务1',
            status: 'completed' as const,
            progress: 100,
            created_at: '2025-01-01T10:00:00Z',
            completed_at: '2025-01-01T10:30:00Z',
            config_path: '/configs/test1.toml',
            data_path: '/data/test1.csv',
          },
        ],
      },
    };

    // Mock API 调用
    (DashboardService.getData as jest.Mock).mockResolvedValue(mockResponse);

    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 调用加载方法
    await act(async () => {
      await result.current.loadData();
    });

    // 等待状态更新
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // 验证状态
    expect(result.current.stats).toEqual({
      total_tasks: 10,
      running_tasks: 2,
      completed_tasks: 7,
      failed_tasks: 1,
    });
    expect(result.current.recentTasks).toHaveLength(1);
    expect(result.current.recentTasks[0].id).toBe('task-1');
    expect(result.current.recentTasks[0].config).toBe('/configs/test1.toml');
    expect(result.current.error).toBeNull();
  });

  it('应该处理 API 错误', async () => {
    // 准备错误响应
    const mockError = {
      success: false,
      error: '服务器错误',
    };

    // Mock API 调用
    (DashboardService.getData as jest.Mock).mockResolvedValue(mockError);

    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 调用加载方法
    await act(async () => {
      await result.current.loadData();
    });

    // 等待状态更新
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // 验证错误状态（数据保持初始值）
    expect(result.current.error).toBe('服务器错误');
    // 注意：错误时不清空数据，保持初始状态
  });

  it('应该处理网络异常', async () => {
    // Mock API 调用抛出异常
    (DashboardService.getData as jest.Mock).mockRejectedValue(
      new Error('网络错误')
    );

    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 调用加载方法
    await act(async () => {
      await result.current.loadData();
    });

    // 等待状态更新
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // 验证错误状态
    expect(result.current.error).toBe('网络错误');
  });

  it('应该避免重复加载', async () => {
    // Mock API 调用
    (DashboardService.getData as jest.Mock).mockResolvedValue({
      success: true,
      data: {
        stats: {
          total_tasks: 5,
          running_tasks: 1,
          completed_tasks: 4,
          failed_tasks: 0,
        },
        recent_tasks: [],
      },
    });

    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 第一次调用
    act(() => {
      result.current.loadData();
    });

    // 立即第二次调用（应该被忽略）
    act(() => {
      result.current.loadData();
    });

    // 等待状态更新
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // 验证 API 只被调用一次
    expect(DashboardService.getData).toHaveBeenCalledTimes(1);
  });

  it('应该支持刷新数据', async () => {
    // Mock API 调用
    (DashboardService.getData as jest.Mock).mockResolvedValue({
      success: true,
      data: {
        stats: {
          total_tasks: 3,
          running_tasks: 0,
          completed_tasks: 3,
          failed_tasks: 0,
        },
        recent_tasks: [],
      },
    });

    // 渲染 hook
    const { result } = renderHook(() => useDashboardStore());

    // 调用刷新方法
    await act(async () => {
      await result.current.refresh();
    });

    // 等待状态更新
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    // 验证数据已加载
    expect(result.current.stats.total_tasks).toBe(3);
    expect(DashboardService.getData).toHaveBeenCalled();
  });
});
