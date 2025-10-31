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
import { ConfigFile } from '@/types/schemas';

/**
 * 配置管理状态接口
 */
interface ConfigState {
  // 配置文件列表
  configs: ConfigFile[];
  // 当前编辑的配置
  currentConfig: ConfigFile | null;
  // 是否正在编辑
  isEditing: boolean;
  // 编辑模式：表单或文本
  editMode: 'form' | 'text';
  // 是否正在加载
  isLoading: boolean;
  // 错误信息
  error: string | null;

  // Actions
  // 设置配置列表
  setConfigs: (configs: ConfigFile[]) => void;
  // 添加配置
  addConfig: (config: ConfigFile) => void;
  // 更新配置
  updateConfig: (name: string, updates: Partial<ConfigFile>) => void;
  // 删除配置
  deleteConfig: (name: string) => void;
  // 设置当前编辑的配置
  setCurrentConfig: (config: ConfigFile | null) => void;
  // 获取配置
  getConfig: (name: string) => ConfigFile | undefined;
  // 设置编辑状态
  setIsEditing: (isEditing: boolean) => void;
  // 设置编辑模式
  setEditMode: (mode: 'form' | 'text') => void;
  // 设置加载状态
  setLoading: (isLoading: boolean) => void;
  // 设置错误信息
  setError: (error: string | null) => void;
  // 清空所有配置
  clearConfigs: () => void;
}

/**
 * 配置管理状态Store
 * 用于管理配置文件的全局状态
 */
export const useConfigStore = create<ConfigState>((set, get) => ({
  // 初始状态
  configs: [],
  currentConfig: null,
  isEditing: false,
  editMode: 'form',
  isLoading: false,
  error: null,

  // 设置配置列表
  setConfigs: (configs) => set({ configs, error: null }),

  // 添加配置
  addConfig: (config) =>
    set((state) => ({
      configs: [...state.configs, config],
      error: null,
    })),

  // 更新配置
  updateConfig: (name, updates) =>
    set((state) => ({
      configs: state.configs.map((config) =>
        config.name === name ? { ...config, ...updates } : config
      ),
      currentConfig:
        state.currentConfig?.name === name
          ? { ...state.currentConfig, ...updates }
          : state.currentConfig,
      error: null,
    })),

  // 删除配置
  deleteConfig: (name) =>
    set((state) => ({
      configs: state.configs.filter((config) => config.name !== name),
      currentConfig:
        state.currentConfig?.name === name ? null : state.currentConfig,
      error: null,
    })),

  // 设置当前编辑的配置
  setCurrentConfig: (config) => set({ currentConfig: config }),

  // 获取配置
  getConfig: (name) => {
    const { configs } = get();
    return configs.find((config) => config.name === name);
  },

  // 设置编辑状态
  setIsEditing: (isEditing) => set({ isEditing }),

  // 设置编辑模式
  setEditMode: (mode) => set({ editMode: mode }),

  // 设置加载状态
  setLoading: (isLoading) => set({ isLoading }),

  // 设置错误信息
  setError: (error) => set({ error }),

  // 清空所有配置
  clearConfigs: () =>
    set({
      configs: [],
      currentConfig: null,
      isEditing: false,
      error: null,
    }),
}));
