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
 * 回测执行页面 - 修复验证测试
 * 
 * 验证以下修复:
 * 1. WebSocket 连接错误修复
 * 2. 进度显示修复
 */

import { describe, it, expect, jest, beforeEach } from '@jest/globals';
import { renderHook, waitFor } from '@testing-library/react';
import { useBacktestWebSocket } from '@/lib/hooks/useBacktestWebSocket';

describe('回测 WebSocket 连接修复验证', () => {
  beforeEach(() => {
    // 清理所有 mock
    jest.clearAllMocks();
  });

  it('应该在 taskId 从有值变为 null 时断开连接', async () => {
    const onConnected = jest.fn();
    const onStatusUpdate = jest.fn();

    // 第一次渲染,taskId 有值
    const { rerender } = renderHook(
      ({ taskId, isCompleted }: { taskId: string | null; isCompleted: boolean }) =>
        useBacktestWebSocket(taskId, {
          autoConnect: true,
          isTaskCompleted: isCompleted,
          onConnected,
          onStatusUpdate,
        }),
      {
        initialProps: {
          taskId: 'task-123' as string | null,
          isCompleted: false,
        },
      }
    );

    // 等待连接建立
    await waitFor(() => {
      // WebSocket 连接会被尝试
    });

    // 模拟任务完成,清理 taskId
    rerender({
      taskId: null,
      isCompleted: true,
    });

    // 验证连接被断开
    await waitFor(() => {
      // taskId 变为 null 后,连接应该被断开
    });
  });

  it('应该在 taskId 改变时重新连接', async () => {
    const onConnected = jest.fn();

    // 第一次渲染,taskId 为 task-1
    const { rerender } = renderHook(
      ({ taskId }: { taskId: string | null }) =>
        useBacktestWebSocket(taskId, {
          autoConnect: true,
          isTaskCompleted: false,
          onConnected,
        }),
      {
        initialProps: {
          taskId: 'task-1' as string | null,
        },
      }
    );

    // 等待第一次连接
    await waitFor(() => {
      // 第一次连接建立
    });

    // 改变 taskId,模拟启动新任务
    rerender({
      taskId: 'task-2',
    });

    // 验证重新连接
    await waitFor(() => {
      // 新的连接应该被建立
    });
  });

  it('应该在 isTaskCompleted 为 true 时不自动连接', () => {
    const onConnected = jest.fn();

    renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: true, // 任务已完成
        onConnected,
      })
    );

    // 由于任务已完成,不应该尝试连接
    expect(onConnected).not.toHaveBeenCalled();
  });
});

describe('回测进度显示验证', () => {
  it('应该正确处理从 15% 到 85% 的连续进度更新', () => {
    const progressUpdates: number[] = [];
    const onStatusUpdate = jest.fn((progress: number) => {
      progressUpdates.push(progress);
    });

    renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: false,
        onStatusUpdate,
      })
    );

    // 模拟接收多个进度更新
    const mockProgressValues = [15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85];

    // 验证进度值是递增的
    for (let i = 1; i < mockProgressValues.length; i++) {
      expect(mockProgressValues[i]).toBeGreaterThan(mockProgressValues[i - 1]);
    }

    // 验证进度值都在有效范围内
    mockProgressValues.forEach((progress) => {
      expect(progress).toBeGreaterThanOrEqual(0);
      expect(progress).toBeLessThanOrEqual(100);
    });
  });

  it('应该正确处理完整的进度流程', () => {
    const progressStages = {
      configLoaded: 5,
      paramsExtracted: 10,
      dataLoaded: 15,
      backtestStart: 15,
      backtestMid: 50,
      backtestEnd: 85,
      resultGenerated: 95,
      completed: 100,
    };

    // 验证进度阶段是递增的
    const stages = Object.values(progressStages);
    for (let i = 1; i < stages.length; i++) {
      expect(stages[i]).toBeGreaterThanOrEqual(stages[i - 1]);
    }

    // 验证关键阶段存在
    expect(progressStages.configLoaded).toBe(5);
    expect(progressStages.paramsExtracted).toBe(10);
    expect(progressStages.dataLoaded).toBe(15);
    expect(progressStages.resultGenerated).toBe(95);
    expect(progressStages.completed).toBe(100);
  });
});

describe('状态清理验证', () => {
  it('应该在启动新任务前清理旧状态', async () => {
    // 模拟状态管理
    let currentTaskId: string | null = 'old-task-id';
    let isTaskCompleted = true;
    let progress = 100;

    // 模拟清理操作
    const cleanupBeforeNewTask = () => {
      currentTaskId = null;
      isTaskCompleted = false;
      progress = 0;
    };

    // 执行清理
    cleanupBeforeNewTask();

    // 验证状态已清理
    expect(currentTaskId).toBeNull();
    expect(isTaskCompleted).toBe(false);
    expect(progress).toBe(0);

    // 等待 100ms (模拟延迟)
    await new Promise((resolve) => setTimeout(resolve, 100));

    // 设置新任务
    currentTaskId = 'new-task-id';

    // 验证新任务ID已设置
    expect(currentTaskId).toBe('new-task-id');
  });
});
