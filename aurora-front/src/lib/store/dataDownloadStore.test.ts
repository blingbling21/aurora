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

import { renderHook, act } from '@testing-library/react';
import { useDataDownloadStore } from './dataDownloadStore';

describe('useDataDownloadStore', () => {
  beforeEach(() => {
    // 重置 store 状态
    const { result } = renderHook(() => useDataDownloadStore());
    act(() => {
      result.current.clearActiveTask();
      result.current.clearHistory();
    });
  });

  describe('开始下载', () => {
    it('应该正确初始化下载任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      expect(result.current.activeTask).toBeDefined();
      expect(result.current.activeTask?.taskId).toBe('task-123');
      expect(result.current.activeTask?.filename).toBe('test.csv');
      expect(result.current.activeTask?.status).toBe('Pending');
      expect(result.current.activeTask?.progress).toBe(0);
      expect(result.current.showProgressPanel).toBe(true);
    });
  });

  describe('更新进度', () => {
    it('应该正确更新下载进度', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      // 先开始下载
      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      // 更新进度
      act(() => {
        result.current.updateProgress(
          50,
          'Downloading',
          '已下载 500 / 1000 条数据',
          500,
          1000
        );
      });

      expect(result.current.activeTask?.progress).toBe(50);
      expect(result.current.activeTask?.status).toBe('Downloading');
      expect(result.current.activeTask?.progressMessage).toBe(
        '已下载 500 / 1000 条数据'
      );
      expect(result.current.activeTask?.downloadedCount).toBe(500);
      expect(result.current.activeTask?.estimatedTotal).toBe(1000);
    });

    it('在没有活动任务时不应该更新', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      // 尝试在没有任务时更新进度
      act(() => {
        result.current.updateProgress(
          50,
          'Downloading',
          '已下载 500 条数据',
          500,
          null
        );
      });

      expect(result.current.activeTask).toBeNull();
    });
  });

  describe('完成下载', () => {
    it('应该正确标记下载完成', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      // 先开始下载
      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      // 完成下载
      act(() => {
        result.current.completeDownload(1000);
      });

      expect(result.current.activeTask?.status).toBe('Completed');
      expect(result.current.activeTask?.progress).toBe(100);
      expect(result.current.activeTask?.downloadedCount).toBe(1000);
      expect(result.current.activeTask?.completedAt).toBeDefined();
    });

    it('应该将完成的任务添加到历史记录', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.completeDownload(1000);
      });

      expect(result.current.taskHistory).toHaveLength(1);
      expect(result.current.taskHistory[0].taskId).toBe('task-123');
      expect(result.current.taskHistory[0].status).toBe('Completed');
    });
  });

  describe('下载失败', () => {
    it('应该正确标记下载失败', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.failDownload('网络错误');
      });

      expect(result.current.activeTask?.status).toBe('Failed');
      expect(result.current.activeTask?.error).toBe('网络错误');
      expect(result.current.activeTask?.completedAt).toBeDefined();
    });

    it('应该将失败的任务添加到历史记录', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.failDownload('网络错误');
      });

      expect(result.current.taskHistory).toHaveLength(1);
      expect(result.current.taskHistory[0].status).toBe('Failed');
      expect(result.current.taskHistory[0].error).toBe('网络错误');
    });
  });

  describe('取消下载', () => {
    it('应该正确取消下载并清空活动任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.cancelDownload();
      });

      expect(result.current.activeTask).toBeNull();
      expect(result.current.showProgressPanel).toBe(false);
      expect(result.current.taskHistory).toHaveLength(1);
      expect(result.current.taskHistory[0].error).toBe('用户取消下载');
    });
  });

  describe('任务查询', () => {
    it('应该能够根据 taskId 获取任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      const task = result.current.getTask('task-123');
      expect(task).toBeDefined();
      expect(task?.taskId).toBe('task-123');
    });

    it('应该能够从历史记录中获取任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.completeDownload(1000);
      });

      act(() => {
        result.current.clearActiveTask();
      });

      // 从历史记录中查找
      const task = result.current.getTask('task-123');
      expect(task).toBeDefined();
      expect(task?.taskId).toBe('task-123');
    });

    it('未找到任务时应该返回 null', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      const task = result.current.getTask('non-existent');
      expect(task).toBeNull();
    });
  });

  describe('清空操作', () => {
    it('应该能够清空活动任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.clearActiveTask();
      });

      expect(result.current.activeTask).toBeNull();
      expect(result.current.showProgressPanel).toBe(false);
    });

    it('应该能够清空任务历史', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
        result.current.completeDownload(1000);
      });

      expect(result.current.taskHistory).toHaveLength(1);

      act(() => {
        result.current.clearHistory();
      });

      expect(result.current.taskHistory).toHaveLength(0);
    });
  });

  describe('历史记录限制', () => {
    it('应该只保留最近10条历史记录', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      // 创建12个任务
      for (let i = 0; i < 12; i++) {
        act(() => {
          result.current.startDownload(`task-${i}`, `test-${i}.csv`);
          result.current.completeDownload(100 * i);
        });
      }

      // 应该只保留最近10条
      expect(result.current.taskHistory).toHaveLength(10);
      // 最新的任务应该排在前面
      expect(result.current.taskHistory[0].taskId).toBe('task-11');
      expect(result.current.taskHistory[9].taskId).toBe('task-2');
    });
  });

  describe('自动清理', () => {
    beforeEach(() => {
      jest.useFakeTimers();
    });

    afterEach(() => {
      jest.useRealTimers();
    });

    it('应该在3秒后自动隐藏进度面板', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.completeDownload(1000);
      });

      expect(result.current.showProgressPanel).toBe(true);

      // 快进3秒
      act(() => {
        jest.advanceTimersByTime(3000);
      });

      expect(result.current.showProgressPanel).toBe(false);
    });

    it('应该在10秒后自动清理已完成的任务', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.completeDownload(1000);
      });

      expect(result.current.activeTask).toBeDefined();

      // 快进10秒
      act(() => {
        jest.advanceTimersByTime(10000);
      });

      expect(result.current.activeTask).toBeNull();
    });

    it('不应该清理其他任务的状态', () => {
      const { result } = renderHook(() => useDataDownloadStore());

      act(() => {
        result.current.startDownload('task-123', 'test.csv');
      });

      act(() => {
        result.current.completeDownload(1000);
      });

      // 在定时器触发前创建新任务
      act(() => {
        jest.advanceTimersByTime(5000);
        result.current.startDownload('task-456', 'test2.csv');
      });

      // 快进到原始任务的清理时间
      act(() => {
        jest.advanceTimersByTime(5000);
      });

      // 新任务应该仍然存在
      expect(result.current.activeTask).toBeDefined();
      expect(result.current.activeTask?.taskId).toBe('task-456');
    });
  });
});
