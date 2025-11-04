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
 * 配置管理 API 服务测试
 */

import { ConfigService, configApi } from './config';
import * as client from './client';
import type {
  ConfigListItem,
  CreateConfigRequest,
  UpdateConfigRequest,
  ConfigValidateResponse,
} from '@/types/api';

// Mock client 模块
jest.mock('./client');

describe('ConfigService', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('list', () => {
    it('应该成功获取配置文件列表', async () => {
      // 准备测试数据
      const mockConfigs: ConfigListItem[] = [
        {
          filename: 'test-config.toml',
          modified: '2025-01-01T00:00:00Z',
          size: 1024,
        },
        {
          filename: 'another-config.toml',
          modified: '2025-01-02T00:00:00Z',
          size: 2048,
        },
      ];
      const mockResponse = {
        success: true,
        data: mockConfigs,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.list();

      // 验证结果
      expect(client.get).toHaveBeenCalledWith('/config');
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockConfigs);
      expect(result.data).toHaveLength(2);
    });

    it('应该处理空列表', async () => {
      // 模拟空列表响应
      const mockResponse = {
        success: true,
        data: [],
      };
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.list();

      // 验证结果
      expect(result.data).toEqual([]);
    });

    it('应该处理 API 错误', async () => {
      // 模拟 API 错误
      const mockError = {
        success: false,
        error: 'Failed to fetch configs',
      };
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.list();

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Failed to fetch configs');
    });
  });

  describe('get', () => {
    it('应该成功获取配置文件内容', async () => {
      // 准备测试数据
      const filename = 'test-config.toml';
      const mockContent = '[strategy]\nname = "test"';
      const mockResponse = {
        success: true,
        data: mockContent,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.get(filename);

      // 验证结果
      expect(client.get).toHaveBeenCalledWith(`/config/${filename}`);
      expect(result).toEqual(mockResponse);
      expect(result.data).toBe(mockContent);
    });

    it('应该正确编码特殊字符', async () => {
      // 准备测试数据
      const filename = 'config with spaces.toml';
      const mockResponse = {
        success: true,
        data: '',
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await ConfigService.get(filename);

      // 验证 URL 编码
      expect(client.get).toHaveBeenCalledWith(
        `/config/${encodeURIComponent(filename)}`
      );
    });

    it('应该处理文件不存在的情况', async () => {
      // 模拟文件不存在错误
      const mockError = {
        success: false,
        error: 'Config file not found',
      };
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.get('non-existent.toml');

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Config file not found');
    });
  });

  describe('create', () => {
    it('应该成功创建配置文件', async () => {
      // 准备测试数据
      const request: CreateConfigRequest = {
        filename: 'new-config.toml',
        content: '[strategy]\nname = "new"',
      };
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.create(request);

      // 验证结果
      expect(client.post).toHaveBeenCalledWith('/config', request);
      expect(result).toEqual(mockResponse);
      expect(result.success).toBe(true);
    });

    it('应该处理文件名冲突', async () => {
      // 准备测试数据
      const request: CreateConfigRequest = {
        filename: 'existing-config.toml',
        content: 'content',
      };
      const mockError = {
        success: false,
        error: 'Config file already exists',
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.create(request);

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Config file already exists');
    });

    it('应该处理无效的配置内容', async () => {
      // 准备测试数据
      const request: CreateConfigRequest = {
        filename: 'invalid-config.toml',
        content: 'invalid toml content [[[',
      };
      const mockError = {
        success: false,
        error: 'Invalid TOML format',
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.create(request);

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('update', () => {
    it('应该成功更新配置文件', async () => {
      // 准备测试数据
      const filename = 'test-config.toml';
      const request: UpdateConfigRequest = {
        content: '[strategy]\nname = "updated"',
      };
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.put as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.update(filename, request);

      // 验证结果
      expect(client.put).toHaveBeenCalledWith(`/config/${filename}`, request);
      expect(result).toEqual(mockResponse);
      expect(result.success).toBe(true);
    });

    it('应该正确编码文件名', async () => {
      // 准备测试数据
      const filename = 'config/with/slash.toml';
      const request: UpdateConfigRequest = {
        content: 'content',
      };
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.put as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await ConfigService.update(filename, request);

      // 验证 URL 编码
      expect(client.put).toHaveBeenCalledWith(
        `/config/${encodeURIComponent(filename)}`,
        request
      );
    });

    it('应该处理文件不存在的情况', async () => {
      // 模拟文件不存在错误
      const mockError = {
        success: false,
        error: 'Config file not found',
      };
      (client.put as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.update('non-existent.toml', {
        content: 'content',
      });

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('delete', () => {
    it('应该成功删除配置文件', async () => {
      // 准备测试数据
      const filename = 'to-delete.toml';
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.del as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.delete(filename);

      // 验证结果
      expect(client.del).toHaveBeenCalledWith(`/config/${filename}`);
      expect(result).toEqual(mockResponse);
      expect(result.success).toBe(true);
    });

    it('应该正确编码文件名', async () => {
      // 准备测试数据
      const filename = 'file with spaces.toml';
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.del as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await ConfigService.delete(filename);

      // 验证 URL 编码
      expect(client.del).toHaveBeenCalledWith(
        `/config/${encodeURIComponent(filename)}`
      );
    });

    it('应该处理文件不存在的情况', async () => {
      // 模拟文件不存在错误
      const mockError = {
        success: false,
        error: 'Config file not found',
      };
      (client.del as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await ConfigService.delete('non-existent.toml');

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('validate', () => {
    it('应该成功验证有效的配置', async () => {
      // 准备测试数据
      const content = '[strategy]\nname = "test"';
      const mockValidateResponse: ConfigValidateResponse = {
        valid: true,
        errors: [],
        warnings: [],
      };
      const mockResponse = {
        success: true,
        data: mockValidateResponse,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.validate(content);

      // 验证结果
      expect(client.post).toHaveBeenCalledWith('/config/validate', {
        content,
      });
      expect(result).toEqual(mockResponse);
      expect(result.data?.valid).toBe(true);
    });

    it('应该返回验证错误', async () => {
      // 准备测试数据
      const content = 'invalid toml [[[';
      const mockValidateResponse: ConfigValidateResponse = {
        valid: false,
        errors: ['Invalid TOML syntax at line 1'],
        warnings: [],
      };
      const mockResponse = {
        success: true,
        data: mockValidateResponse,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.validate(content);

      // 验证结果
      expect(result.data?.valid).toBe(false);
      expect(result.data?.errors).toHaveLength(1);
    });

    it('应该返回验证警告', async () => {
      // 准备测试数据
      const content = '[strategy]\n# Missing required fields';
      const mockValidateResponse: ConfigValidateResponse = {
        valid: true,
        errors: [],
        warnings: ['Missing recommended field: initial_capital'],
      };
      const mockResponse = {
        success: true,
        data: mockValidateResponse,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await ConfigService.validate(content);

      // 验证结果
      expect(result.data?.valid).toBe(true);
      expect(result.data?.warnings).toHaveLength(1);
    });
  });
});

describe('configApi', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('应该暴露所有 ConfigService 方法', () => {
    // 验证所有方法都已导出
    expect(configApi.list).toBeDefined();
    expect(configApi.get).toBeDefined();
    expect(configApi.create).toBeDefined();
    expect(configApi.update).toBeDefined();
    expect(configApi.delete).toBeDefined();
  });

  it('list 方法应该调用 ConfigService.list', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: [] };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    await configApi.list();

    // 验证调用
    expect(client.get).toHaveBeenCalledWith('/config');
  });

  it('get 方法应该调用 ConfigService.get', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: '' };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    const filename = 'test.toml';
    await configApi.get(filename);

    // 验证调用
    expect(client.get).toHaveBeenCalledWith(`/config/${filename}`);
  });

  it('create 方法应该调用 ConfigService.create', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: undefined };
    (client.post as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    const request: CreateConfigRequest = {
      filename: 'new.toml',
      content: 'content',
    };
    await configApi.create(request);

    // 验证调用
    expect(client.post).toHaveBeenCalledWith('/config', request);
  });
});
