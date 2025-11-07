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

/**
 * 数据下载状态管理
 * 用于管理数据下载任务的全局状态
 */

import { create } from 'zustand';
import type { DownloadStatus } from '@/types/api';

/**
 * 下载任务信息
 */
export interface DownloadTask {
  // 任务ID
  taskId: string;
  // 文件名
  filename: string;
  // 下载状态
  status: DownloadStatus;
  // 进度百分比 (0-100)
  progress: number;
  // 进度消息
  progressMessage: string;
  // 已下载数量
  downloadedCount: number;
  // 预估总数
  estimatedTotal: number | null;
  // 错误信息
  error: string | null;
  // 创建时间
  createdAt: Date;
  // 完成时间
  completedAt: Date | null;
}

/**
 * 数据下载状态接口
 */
interface DataDownloadState {
  // 当前活动的下载任务
  activeTask: DownloadTask | null;
  // 历史下载任务列表
  taskHistory: DownloadTask[];
  // 是否显示下载进度面板
  showProgressPanel: boolean;

  // Actions
  // 开始新的下载任务
  startDownload: (taskId: string, filename: string) => void;
  // 更新下载进度
  updateProgress: (
    progress: number,
    status: DownloadStatus,
    progressMessage: string,
    downloadedCount: number,
    estimatedTotal: number | null
  ) => void;
  // 完成下载
  completeDownload: (downloadedCount: number) => void;
  // 下载失败
  failDownload: (error: string) => void;
  // 取消下载
  cancelDownload: () => void;
  // 清空当前任务
  clearActiveTask: () => void;
  // 显示/隐藏进度面板
  setShowProgressPanel: (show: boolean) => void;
  // 清空任务历史
  clearHistory: () => void;
  // 获取任务信息
  getTask: (taskId: string) => DownloadTask | null;
}

/**
 * 数据下载状态Store
 * 使用 zustand 管理全局下载状态
 */
export const useDataDownloadStore = create<DataDownloadState>((set, get) => ({
  // 初始状态
  activeTask: null,
  taskHistory: [],
  showProgressPanel: false,

  // 开始新的下载任务
  startDownload: (taskId, filename) => {
    const task: DownloadTask = {
      taskId,
      filename,
      status: 'Pending',
      progress: 0,
      progressMessage: '正在准备下载...',
      downloadedCount: 0,
      estimatedTotal: null,
      error: null,
      createdAt: new Date(),
      completedAt: null,
    };

    set({
      activeTask: task,
      showProgressPanel: true,
    });
  },

  // 更新下载进度
  updateProgress: (progress, status, progressMessage, downloadedCount, estimatedTotal) => {
    const { activeTask } = get();
    if (!activeTask) return;

    const updatedTask: DownloadTask = {
      ...activeTask,
      status,
      progress,
      progressMessage,
      downloadedCount,
      estimatedTotal,
    };

    set({ activeTask: updatedTask });
  },

  // 完成下载
  completeDownload: (downloadedCount) => {
    const { activeTask, taskHistory } = get();
    if (!activeTask) return;

    const completedTask: DownloadTask = {
      ...activeTask,
      status: 'Completed',
      progress: 100,
      progressMessage: `下载完成，共获取 ${downloadedCount} 条数据`,
      downloadedCount,
      completedAt: new Date(),
    };

    set({
      activeTask: completedTask,
      taskHistory: [completedTask, ...taskHistory].slice(0, 10), // 只保留最近10条记录
    });

    // 3秒后自动隐藏进度面板
    setTimeout(() => {
      const currentTask = get().activeTask;
      if (currentTask?.taskId === completedTask.taskId) {
        set({ showProgressPanel: false });
      }
    }, 3000);

    // 10秒后自动清理活动任务，防止页面刷新时重连
    setTimeout(() => {
      const currentTask = get().activeTask;
      if (currentTask?.taskId === completedTask.taskId && currentTask.status === 'Completed') {
        set({ activeTask: null });
      }
    }, 10000);
  },

  // 下载失败
  failDownload: (error) => {
    const { activeTask, taskHistory } = get();
    if (!activeTask) return;

    const failedTask: DownloadTask = {
      ...activeTask,
      status: 'Failed',
      error,
      progressMessage: `下载失败: ${error}`,
      completedAt: new Date(),
    };

    set({
      activeTask: failedTask,
      taskHistory: [failedTask, ...taskHistory].slice(0, 10),
    });
  },

  // 取消下载
  cancelDownload: () => {
    const { activeTask, taskHistory } = get();
    if (!activeTask) return;

    const cancelledTask: DownloadTask = {
      ...activeTask,
      status: 'Failed',
      error: '用户取消下载',
      progressMessage: '下载已取消',
      completedAt: new Date(),
    };

    set({
      activeTask: null,
      taskHistory: [cancelledTask, ...taskHistory].slice(0, 10),
      showProgressPanel: false,
    });
  },

  // 清空当前任务
  clearActiveTask: () => {
    set({
      activeTask: null,
      showProgressPanel: false,
    });
  },

  // 显示/隐藏进度面板
  setShowProgressPanel: (show) => {
    set({ showProgressPanel: show });
  },

  // 清空任务历史
  clearHistory: () => {
    set({ taskHistory: [] });
  },

  // 获取任务信息
  getTask: (taskId) => {
    const { activeTask, taskHistory } = get();
    
    // 先检查活动任务
    if (activeTask?.taskId === taskId) {
      return activeTask;
    }
    
    // 再检查历史记录
    return taskHistory.find((task) => task.taskId === taskId) || null;
  },
}));
