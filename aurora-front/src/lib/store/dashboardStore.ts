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
import { DashboardStats } from '@/types/api';
import { BacktestTask } from '@/types/schemas';
import { DashboardService } from '@/lib/api/dashboard';
import { convertApiTaskSummariesToLocal } from '@/lib/utils/apiConverters';

/**
 * 仪表盘状态接口
 */
interface DashboardState {
  // 统计数据
  stats: DashboardStats;
  // 最近任务列表（使用前端格式）
  recentTasks: BacktestTask[];
  // 是否正在加载
  isLoading: boolean;
  // 错误信息
  error: string | null;

  // Actions
  // 加载仪表盘数据
  loadData: () => Promise<void>;
  // 刷新数据
  refresh: () => Promise<void>;
  // 设置加载状态
  setLoading: (isLoading: boolean) => void;
  // 设置错误信息
  setError: (error: string | null) => void;
}

/**
 * 仪表盘状态管理Store
 * 用于管理仪表盘页面的统计数据和最近任务列表
 */
export const useDashboardStore = create<DashboardState>((set, get) => ({
  // 初始状态
  stats: {
    total_tasks: 0,
    running_tasks: 0,
    completed_tasks: 0,
    failed_tasks: 0,
  },
  recentTasks: [],
  isLoading: false,
  error: null,

  // 加载仪表盘数据
  loadData: async () => {
    // 如果已经在加载中，直接返回
    if (get().isLoading) {
      return;
    }

    set({ isLoading: true, error: null });

    try {
      // 调用 API 获取仪表盘数据
      const response = await DashboardService.getData();

      if (response.success && response.data) {
        // 更新状态，转换 API 数据为前端格式
        set({
          stats: response.data.stats,
          recentTasks: convertApiTaskSummariesToLocal(response.data.recent_tasks),
          isLoading: false,
          error: null,
        });
      } else {
        // API 返回失败
        set({
          isLoading: false,
          error: response.error || '获取仪表盘数据失败',
        });
      }
    } catch (error) {
      // 捕获异常
      console.error('加载仪表盘数据失败:', error);
      set({
        isLoading: false,
        error: error instanceof Error ? error.message : '未知错误',
      });
    }
  },

  // 刷新数据（重新加载）
  refresh: async () => {
    await get().loadData();
  },

  // 设置加载状态
  setLoading: (isLoading) => set({ isLoading }),

  // 设置错误信息
  setError: (error) => set({ error }),
}));
