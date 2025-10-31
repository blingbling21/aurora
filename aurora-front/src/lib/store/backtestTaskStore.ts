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

import { create } from 'zustand';
import { BacktestTask } from '@/types/schemas';

/**
 * 回测任务状态接口
 */
interface BacktestTaskState {
  // 任务列表
  tasks: BacktestTask[];
  // 当前选中的任务ID
  selectedTaskId: string | null;
  // 是否正在加载
  isLoading: boolean;
  // 错误信息
  error: string | null;

  // Actions
  // 设置任务列表
  setTasks: (tasks: BacktestTask[]) => void;
  // 添加任务
  addTask: (task: BacktestTask) => void;
  // 更新任务
  updateTask: (id: string, updates: Partial<BacktestTask>) => void;
  // 删除任务
  deleteTask: (id: string) => void;
  // 选择任务
  selectTask: (id: string | null) => void;
  // 设置加载状态
  setLoading: (isLoading: boolean) => void;
  // 设置错误信息
  setError: (error: string | null) => void;
  // 清空所有任务
  clearTasks: () => void;
  // 获取选中的任务
  getSelectedTask: () => BacktestTask | null;
}

/**
 * 回测任务状态管理Store
 * 用于管理回测任务的全局状态
 */
export const useBacktestTaskStore = create<BacktestTaskState>((set, get) => ({
  // 初始状态
  tasks: [],
  selectedTaskId: null,
  isLoading: false,
  error: null,

  // 设置任务列表
  setTasks: (tasks) => set({ tasks, error: null }),

  // 添加任务
  addTask: (task) =>
    set((state) => ({
      tasks: [...state.tasks, task],
      error: null,
    })),

  // 更新任务
  updateTask: (id, updates) =>
    set((state) => ({
      tasks: state.tasks.map((task) =>
        task.id === id ? { ...task, ...updates } : task
      ),
      error: null,
    })),

  // 删除任务
  deleteTask: (id) =>
    set((state) => ({
      tasks: state.tasks.filter((task) => task.id !== id),
      selectedTaskId: state.selectedTaskId === id ? null : state.selectedTaskId,
      error: null,
    })),

  // 选择任务
  selectTask: (id) => set({ selectedTaskId: id }),

  // 设置加载状态
  setLoading: (isLoading) => set({ isLoading }),

  // 设置错误信息
  setError: (error) => set({ error }),

  // 清空所有任务
  clearTasks: () => set({ tasks: [], selectedTaskId: null, error: null }),

  // 获取选中的任务
  getSelectedTask: () => {
    const { tasks, selectedTaskId } = get();
    return tasks.find((task) => task.id === selectedTaskId) || null;
  },
}));
