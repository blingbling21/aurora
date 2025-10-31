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
import { BacktestResult } from '@/types/schemas';

/**
 * 回测结果状态接口
 */
interface BacktestResultState {
  // 结果映射表，key为taskId
  results: Map<string, BacktestResult>;
  // 当前查看的结果ID
  currentResultId: string | null;
  // 是否正在加载
  isLoading: boolean;
  // 错误信息
  error: string | null;

  // Actions
  // 设置结果
  setResult: (taskId: string, result: BacktestResult) => void;
  // 获取结果
  getResult: (taskId: string) => BacktestResult | undefined;
  // 删除结果
  deleteResult: (taskId: string) => void;
  // 设置当前查看的结果
  setCurrentResult: (taskId: string | null) => void;
  // 获取当前查看的结果
  getCurrentResult: () => BacktestResult | null;
  // 设置加载状态
  setLoading: (isLoading: boolean) => void;
  // 设置错误信息
  setError: (error: string | null) => void;
  // 清空所有结果
  clearResults: () => void;
}

/**
 * 回测结果状态管理Store
 * 用于管理回测结果的全局状态
 */
export const useBacktestResultStore = create<BacktestResultState>((set, get) => ({
  // 初始状态
  results: new Map(),
  currentResultId: null,
  isLoading: false,
  error: null,

  // 设置结果
  setResult: (taskId, result) =>
    set((state) => {
      const newResults = new Map(state.results);
      newResults.set(taskId, result);
      return { results: newResults, error: null };
    }),

  // 获取结果
  getResult: (taskId) => {
    const { results } = get();
    return results.get(taskId);
  },

  // 删除结果
  deleteResult: (taskId) =>
    set((state) => {
      const newResults = new Map(state.results);
      newResults.delete(taskId);
      return {
        results: newResults,
        currentResultId:
          state.currentResultId === taskId ? null : state.currentResultId,
        error: null,
      };
    }),

  // 设置当前查看的结果
  setCurrentResult: (taskId) => set({ currentResultId: taskId }),

  // 获取当前查看的结果
  getCurrentResult: () => {
    const { results, currentResultId } = get();
    if (!currentResultId) return null;
    return results.get(currentResultId) || null;
  },

  // 设置加载状态
  setLoading: (isLoading) => set({ isLoading }),

  // 设置错误信息
  setError: (error) => set({ error }),

  // 清空所有结果
  clearResults: () =>
    set({ results: new Map(), currentResultId: null, error: null }),
}));
