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
import { useBacktestTaskStore } from './backtestTaskStore';
import { BacktestTask } from '@/types/schemas';

describe('useBacktestTaskStore', () => {
  // 在每个测试前重置store状态
  beforeEach(() => {
    const { result } = renderHook(() => useBacktestTaskStore());
    act(() => {
      result.current.clearTasks();
    });
  });

  // 测试初始状态
  it('should have initial state', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    expect(result.current.tasks).toEqual([]);
    expect(result.current.selectedTaskId).toBeNull();
    expect(result.current.isLoading).toBe(false);
    expect(result.current.error).toBeNull();
  });

  // 测试添加任务
  it('should add a task', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    const newTask: BacktestTask = {
      id: '1',
      name: 'Test Task',
      status: 'pending',
      config: 'config.toml',
      dataFile: 'data.csv',
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    act(() => {
      result.current.addTask(newTask);
    });

    expect(result.current.tasks).toHaveLength(1);
    expect(result.current.tasks[0]).toEqual(newTask);
  });

  // 测试更新任务
  it('should update a task', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    const task: BacktestTask = {
      id: '1',
      name: 'Test Task',
      status: 'pending',
      config: 'config.toml',
      dataFile: 'data.csv',
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    act(() => {
      result.current.addTask(task);
      result.current.updateTask('1', { status: 'running', progress: 50 });
    });

    expect(result.current.tasks[0].status).toBe('running');
    expect(result.current.tasks[0].progress).toBe(50);
  });

  // 测试删除任务
  it('should delete a task', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    const task: BacktestTask = {
      id: '1',
      name: 'Test Task',
      status: 'pending',
      config: 'config.toml',
      dataFile: 'data.csv',
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    act(() => {
      result.current.addTask(task);
      result.current.deleteTask('1');
    });

    expect(result.current.tasks).toHaveLength(0);
  });

  // 测试选择任务
  it('should select a task', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    act(() => {
      result.current.selectTask('1');
    });

    expect(result.current.selectedTaskId).toBe('1');
  });

  // 测试获取选中的任务
  it('should get selected task', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    const task: BacktestTask = {
      id: '1',
      name: 'Test Task',
      status: 'pending',
      config: 'config.toml',
      dataFile: 'data.csv',
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    act(() => {
      result.current.addTask(task);
      result.current.selectTask('1');
    });

    expect(result.current.getSelectedTask()).toEqual(task);
  });

  // 测试设置加载状态
  it('should set loading state', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    act(() => {
      result.current.setLoading(true);
    });

    expect(result.current.isLoading).toBe(true);
  });

  // 测试设置错误信息
  it('should set error', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    act(() => {
      result.current.setError('Test error');
    });

    expect(result.current.error).toBe('Test error');
  });

  // 测试清空任务
  it('should clear all tasks', () => {
    const { result } = renderHook(() => useBacktestTaskStore());

    const task: BacktestTask = {
      id: '1',
      name: 'Test Task',
      status: 'pending',
      config: 'config.toml',
      dataFile: 'data.csv',
      progress: 0,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    act(() => {
      result.current.addTask(task);
      result.current.selectTask('1');
      result.current.clearTasks();
    });

    expect(result.current.tasks).toHaveLength(0);
    expect(result.current.selectedTaskId).toBeNull();
  });
});
