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
import { DataFile } from '@/types/schemas';

/**
 * 数据管理状态接口
 */
interface DataState {
  // 数据文件列表
  dataFiles: DataFile[];
  // 是否正在下载
  isDownloading: boolean;
  // 下载进度 (0-100)
  downloadProgress: number;
  // 当前下载的文件名
  currentDownloadFile: string | null;
  // 是否正在加载
  isLoading: boolean;
  // 错误信息
  error: string | null;

  // Actions
  // 设置数据文件列表
  setDataFiles: (files: DataFile[]) => void;
  // 添加数据文件
  addDataFile: (file: DataFile) => void;
  // 更新数据文件
  updateDataFile: (name: string, updates: Partial<DataFile>) => void;
  // 删除数据文件
  deleteDataFile: (name: string) => void;
  // 获取数据文件
  getDataFile: (name: string) => DataFile | undefined;
  // 设置下载状态
  setIsDownloading: (isDownloading: boolean) => void;
  // 设置下载进度
  setDownloadProgress: (progress: number) => void;
  // 设置当前下载文件
  setCurrentDownloadFile: (fileName: string | null) => void;
  // 开始下载
  startDownload: (fileName: string) => void;
  // 完成下载
  completeDownload: () => void;
  // 设置加载状态
  setLoading: (isLoading: boolean) => void;
  // 设置错误信息
  setError: (error: string | null) => void;
  // 清空所有数据文件
  clearDataFiles: () => void;
}

/**
 * 数据管理状态Store
 * 用于管理数据文件的全局状态
 */
export const useDataStore = create<DataState>((set, get) => ({
  // 初始状态
  dataFiles: [],
  isDownloading: false,
  downloadProgress: 0,
  currentDownloadFile: null,
  isLoading: false,
  error: null,

  // 设置数据文件列表
  setDataFiles: (files) => set({ dataFiles: files, error: null }),

  // 添加数据文件
  addDataFile: (file) =>
    set((state) => ({
      dataFiles: [...state.dataFiles, file],
      error: null,
    })),

  // 更新数据文件
  updateDataFile: (name, updates) =>
    set((state) => ({
      dataFiles: state.dataFiles.map((file) =>
        file.name === name ? { ...file, ...updates } : file
      ),
      error: null,
    })),

  // 删除数据文件
  deleteDataFile: (name) =>
    set((state) => ({
      dataFiles: state.dataFiles.filter((file) => file.name !== name),
      error: null,
    })),

  // 获取数据文件
  getDataFile: (name) => {
    const { dataFiles } = get();
    return dataFiles.find((file) => file.name === name);
  },

  // 设置下载状态
  setIsDownloading: (isDownloading) => set({ isDownloading }),

  // 设置下载进度
  setDownloadProgress: (progress) => set({ downloadProgress: progress }),

  // 设置当前下载文件
  setCurrentDownloadFile: (fileName) => set({ currentDownloadFile: fileName }),

  // 开始下载
  startDownload: (fileName) =>
    set({
      isDownloading: true,
      downloadProgress: 0,
      currentDownloadFile: fileName,
      error: null,
    }),

  // 完成下载
  completeDownload: () =>
    set({
      isDownloading: false,
      downloadProgress: 100,
      currentDownloadFile: null,
    }),

  // 设置加载状态
  setLoading: (isLoading) => set({ isLoading }),

  // 设置错误信息
  setError: (error) => set({ error }),

  // 清空所有数据文件
  clearDataFiles: () =>
    set({
      dataFiles: [],
      isDownloading: false,
      downloadProgress: 0,
      currentDownloadFile: null,
      error: null,
    }),
}));
