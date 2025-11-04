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
 * 配置管理 Store 测试
 */

import { renderHook, act } from '@testing-library/react';
import { useConfigStore } from './configStore';
import { ConfigFile } from '@/types/schemas';

describe('useConfigStore', () => {
  // 在每个测试前重置 store 状态
  beforeEach(() => {
    const { result } = renderHook(() => useConfigStore());
    act(() => {
      result.current.clearConfigs();
    });
  });

  describe('初始状态', () => {
    it('应该有正确的初始状态', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 验证初始状态
      expect(result.current.configs).toEqual([]);
      expect(result.current.currentConfig).toBeNull();
      expect(result.current.isEditing).toBe(false);
      expect(result.current.editMode).toBe('form');
      expect(result.current.isLoading).toBe(false);
      expect(result.current.error).toBeNull();
    });
  });

  describe('setConfigs', () => {
    it('应该设置配置列表', () => {
      // 准备测试数据
      const mockConfigs: ConfigFile[] = [
        {
          name: 'config1.toml',
          path: '/configs/config1.toml',
          content: '[strategy]\nname = "test1"',
          lastModified: '2025-01-01T00:00:00Z',
        },
        {
          name: 'config2.toml',
          path: '/configs/config2.toml',
          content: '[strategy]\nname = "test2"',
          lastModified: '2025-01-02T00:00:00Z',
        },
      ];

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 执行操作
      act(() => {
        result.current.setConfigs(mockConfigs);
      });

      // 验证结果
      expect(result.current.configs).toEqual(mockConfigs);
      expect(result.current.configs).toHaveLength(2);
      expect(result.current.error).toBeNull();
    });

    it('应该清除错误信息', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置错误
      act(() => {
        result.current.setError('Test error');
      });

      expect(result.current.error).toBe('Test error');

      // 设置配置列表
      act(() => {
        result.current.setConfigs([]);
      });

      // 验证错误被清除
      expect(result.current.error).toBeNull();
    });
  });

  describe('addConfig', () => {
    it('应该添加新配置', () => {
      // 准备测试数据
      const newConfig: ConfigFile = {
        name: 'new-config.toml',
        path: '/configs/new-config.toml',
        content: '[strategy]\nname = "new"',
        lastModified: '2025-01-03T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 执行操作
      act(() => {
        result.current.addConfig(newConfig);
      });

      // 验证结果
      expect(result.current.configs).toHaveLength(1);
      expect(result.current.configs[0]).toEqual(newConfig);
      expect(result.current.error).toBeNull();
    });

    it('应该将新配置添加到现有列表', () => {
      // 准备测试数据
      const existingConfig: ConfigFile = {
        name: 'existing.toml',
        path: '/configs/existing.toml',
        content: 'content1',
        lastModified: '2025-01-01T00:00:00Z',
      };
      const newConfig: ConfigFile = {
        name: 'new.toml',
        path: '/configs/new.toml',
        content: 'content2',
        lastModified: '2025-01-02T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加现有配置
      act(() => {
        result.current.addConfig(existingConfig);
      });

      // 添加新配置
      act(() => {
        result.current.addConfig(newConfig);
      });

      // 验证结果
      expect(result.current.configs).toHaveLength(2);
      expect(result.current.configs[1]).toEqual(newConfig);
    });
  });

  describe('updateConfig', () => {
    it('应该更新指定的配置', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'test.toml',
        path: '/configs/test.toml',
        content: 'old content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加配置
      act(() => {
        result.current.addConfig(config);
      });

      // 更新配置
      act(() => {
        result.current.updateConfig('test.toml', {
          content: 'new content',
          lastModified: '2025-01-02T00:00:00Z',
        });
      });

      // 验证结果
      expect(result.current.configs[0].content).toBe('new content');
      expect(result.current.configs[0].lastModified).toBe(
        '2025-01-02T00:00:00Z'
      );
      expect(result.current.error).toBeNull();
    });

    it('应该同时更新 currentConfig', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'test.toml',
        path: '/configs/test.toml',
        content: 'old content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加并设置为当前配置
      act(() => {
        result.current.addConfig(config);
        result.current.setCurrentConfig(config);
      });

      // 更新配置
      act(() => {
        result.current.updateConfig('test.toml', {
          content: 'new content',
        });
      });

      // 验证 currentConfig 也被更新
      expect(result.current.currentConfig?.content).toBe('new content');
    });

    it('应该不影响其他配置', () => {
      // 准备测试数据
      const config1: ConfigFile = {
        name: 'config1.toml',
        path: '/configs/config1.toml',
        content: 'content1',
        lastModified: '2025-01-01T00:00:00Z',
      };
      const config2: ConfigFile = {
        name: 'config2.toml',
        path: '/configs/config2.toml',
        content: 'content2',
        lastModified: '2025-01-02T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加两个配置
      act(() => {
        result.current.addConfig(config1);
        result.current.addConfig(config2);
      });

      // 更新第一个配置
      act(() => {
        result.current.updateConfig('config1.toml', {
          content: 'updated content',
        });
      });

      // 验证第二个配置未被修改
      expect(result.current.configs[1].content).toBe('content2');
    });
  });

  describe('deleteConfig', () => {
    it('应该删除指定的配置', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'to-delete.toml',
        path: '/configs/to-delete.toml',
        content: 'content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加配置
      act(() => {
        result.current.addConfig(config);
      });

      expect(result.current.configs).toHaveLength(1);

      // 删除配置
      act(() => {
        result.current.deleteConfig('to-delete.toml');
      });

      // 验证结果
      expect(result.current.configs).toHaveLength(0);
      expect(result.current.error).toBeNull();
    });

    it('应该清除被删除的 currentConfig', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'current.toml',
        path: '/configs/current.toml',
        content: 'content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加并设置为当前配置
      act(() => {
        result.current.addConfig(config);
        result.current.setCurrentConfig(config);
      });

      expect(result.current.currentConfig).toBe(config);

      // 删除配置
      act(() => {
        result.current.deleteConfig('current.toml');
      });

      // 验证 currentConfig 被清除
      expect(result.current.currentConfig).toBeNull();
    });

    it('应该不影响其他 currentConfig', () => {
      // 准备测试数据
      const config1: ConfigFile = {
        name: 'config1.toml',
        path: '/configs/config1.toml',
        content: 'content1',
        lastModified: '2025-01-01T00:00:00Z',
      };
      const config2: ConfigFile = {
        name: 'config2.toml',
        path: '/configs/config2.toml',
        content: 'content2',
        lastModified: '2025-01-02T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加两个配置，设置第一个为当前配置
      act(() => {
        result.current.addConfig(config1);
        result.current.addConfig(config2);
        result.current.setCurrentConfig(config1);
      });

      // 删除第二个配置
      act(() => {
        result.current.deleteConfig('config2.toml');
      });

      // 验证 currentConfig 未被清除
      expect(result.current.currentConfig).toBe(config1);
    });
  });

  describe('setCurrentConfig', () => {
    it('应该设置当前配置', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'test.toml',
        path: '/configs/test.toml',
        content: 'content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置当前配置
      act(() => {
        result.current.setCurrentConfig(config);
      });

      // 验证结果
      expect(result.current.currentConfig).toBe(config);
    });

    it('应该能够清除当前配置', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'test.toml',
        path: '/configs/test.toml',
        content: 'content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置当前配置
      act(() => {
        result.current.setCurrentConfig(config);
      });

      expect(result.current.currentConfig).toBe(config);

      // 清除当前配置
      act(() => {
        result.current.setCurrentConfig(null);
      });

      // 验证结果
      expect(result.current.currentConfig).toBeNull();
    });
  });

  describe('getConfig', () => {
    it('应该根据名称获取配置', () => {
      // 准备测试数据
      const config1: ConfigFile = {
        name: 'config1.toml',
        path: '/configs/config1.toml',
        content: 'content1',
        lastModified: '2025-01-01T00:00:00Z',
      };
      const config2: ConfigFile = {
        name: 'config2.toml',
        path: '/configs/config2.toml',
        content: 'content2',
        lastModified: '2025-01-02T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 添加配置
      act(() => {
        result.current.addConfig(config1);
        result.current.addConfig(config2);
      });

      // 获取配置
      const foundConfig = result.current.getConfig('config2.toml');

      // 验证结果
      expect(foundConfig).toEqual(config2);
    });

    it('应该在配置不存在时返回 undefined', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 获取不存在的配置
      const foundConfig = result.current.getConfig('non-existent.toml');

      // 验证结果
      expect(foundConfig).toBeUndefined();
    });
  });

  describe('状态管理', () => {
    it('应该设置编辑状态', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置编辑状态
      act(() => {
        result.current.setIsEditing(true);
      });

      // 验证结果
      expect(result.current.isEditing).toBe(true);
    });

    it('应该设置编辑模式', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置编辑模式
      act(() => {
        result.current.setEditMode('text');
      });

      // 验证结果
      expect(result.current.editMode).toBe('text');
    });

    it('应该设置加载状态', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置加载状态
      act(() => {
        result.current.setLoading(true);
      });

      // 验证结果
      expect(result.current.isLoading).toBe(true);
    });

    it('应该设置错误信息', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置错误信息
      act(() => {
        result.current.setError('Test error message');
      });

      // 验证结果
      expect(result.current.error).toBe('Test error message');
    });

    it('应该清除错误信息', () => {
      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置错误信息
      act(() => {
        result.current.setError('Test error');
      });

      expect(result.current.error).toBe('Test error');

      // 清除错误信息
      act(() => {
        result.current.setError(null);
      });

      // 验证结果
      expect(result.current.error).toBeNull();
    });
  });

  describe('clearConfigs', () => {
    it('应该清空所有配置和状态', () => {
      // 准备测试数据
      const config: ConfigFile = {
        name: 'test.toml',
        path: '/configs/test.toml',
        content: 'content',
        lastModified: '2025-01-01T00:00:00Z',
      };

      // 渲染 hook
      const { result } = renderHook(() => useConfigStore());

      // 设置各种状态
      act(() => {
        result.current.addConfig(config);
        result.current.setCurrentConfig(config);
        result.current.setIsEditing(true);
        result.current.setError('Test error');
      });

      // 验证状态已设置
      expect(result.current.configs).toHaveLength(1);
      expect(result.current.currentConfig).toBe(config);
      expect(result.current.isEditing).toBe(true);
      expect(result.current.error).toBe('Test error');

      // 清空配置
      act(() => {
        result.current.clearConfigs();
      });

      // 验证所有状态被重置
      expect(result.current.configs).toEqual([]);
      expect(result.current.currentConfig).toBeNull();
      expect(result.current.isEditing).toBe(false);
      expect(result.current.error).toBeNull();
    });
  });
});
