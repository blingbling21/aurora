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
 * API 客户端测试
 */

import { apiRequest, ApiError, get, post, put, del, buildQueryString } from './client';

// Mock fetch
global.fetch = jest.fn();

describe('API Client', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('apiRequest', () => {
    it('应该成功发送 GET 请求', async () => {
      // 模拟成功响应
      const mockResponse = {
        success: true,
        data: { test: 'data' },
      };

      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      });

      // 发送请求
      const result = await apiRequest('/test');

      // 验证
      expect(global.fetch).toHaveBeenCalledWith(
        '/api/test',
        expect.objectContaining({
          headers: expect.objectContaining({
            'Content-Type': 'application/json',
          }),
        })
      );
      expect(result).toEqual(mockResponse);
    });

    it('应该处理 HTTP 错误', async () => {
      // 模拟错误响应
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: 'Not Found',
        json: async () => ({ error: '未找到资源' }),
        text: async () => '未找到资源',
      });

      // 发送请求并期望抛出错误
      await expect(apiRequest('/test')).rejects.toThrow(ApiError);
      
      // 重新 mock 第二次调用
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: 'Not Found',
        json: async () => ({ error: '未找到资源' }),
        text: async () => '未找到资源',
      });
      
      await expect(apiRequest('/test')).rejects.toThrow('未找到资源');
    });

    it('应该处理网络错误', async () => {
      // 模拟网络错误
      (global.fetch as jest.Mock).mockRejectedValueOnce(new Error('网络错误'));

      // 发送请求并期望抛出错误
      await expect(apiRequest('/test')).rejects.toThrow(ApiError);
    });

    it('应该处理超时', async () => {
      // 模拟超时 - 使用 AbortController 来触发超时
      (global.fetch as jest.Mock).mockImplementationOnce(
        (_url: string, options: RequestInit) => {
          return new Promise((_, reject) => {
            // 监听 abort 信号
            if (options.signal) {
              options.signal.addEventListener('abort', () => {
                const error = new Error('The operation was aborted');
                error.name = 'AbortError';
                reject(error);
              });
            }
          });
        }
      );

      // 发送请求并期望超时
      await expect(
        apiRequest('/test', { timeout: 50 })
      ).rejects.toThrow('请求超时');
    });

    it('应该正确设置请求头', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ success: true }),
      });

      await apiRequest('/test', {
        headers: {
          'X-Custom-Header': 'custom-value',
        },
      });

      expect(global.fetch).toHaveBeenCalledWith(
        expect.anything(),
        expect.objectContaining({
          headers: expect.objectContaining({
            'Content-Type': 'application/json',
            'X-Custom-Header': 'custom-value',
          }),
        })
      );
    });
  });

  describe('HTTP 方法快捷函数', () => {
    beforeEach(() => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ success: true }),
      });
    });

    it('get() 应该发送 GET 请求', async () => {
      await get('/test');

      expect(global.fetch).toHaveBeenCalledWith(
        expect.anything(),
        expect.objectContaining({
          method: 'GET',
        })
      );
    });

    it('post() 应该发送 POST 请求并包含 body', async () => {
      const data = { key: 'value' };
      await post('/test', data);

      expect(global.fetch).toHaveBeenCalledWith(
        expect.anything(),
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(data),
        })
      );
    });

    it('put() 应该发送 PUT 请求并包含 body', async () => {
      const data = { key: 'value' };
      await put('/test', data);

      expect(global.fetch).toHaveBeenCalledWith(
        expect.anything(),
        expect.objectContaining({
          method: 'PUT',
          body: JSON.stringify(data),
        })
      );
    });

    it('del() 应该发送 DELETE 请求', async () => {
      await del('/test');

      expect(global.fetch).toHaveBeenCalledWith(
        expect.anything(),
        expect.objectContaining({
          method: 'DELETE',
        })
      );
    });
  });

  describe('buildQueryString', () => {
    it('应该构建正确的查询字符串', () => {
      const params = {
        key1: 'value1',
        key2: 'value2',
        key3: 123,
      };

      const query = buildQueryString(params);

      expect(query).toBe('?key1=value1&key2=value2&key3=123');
    });

    it('应该跳过 undefined 和 null 值', () => {
      const params = {
        key1: 'value1',
        key2: undefined,
        key3: null,
        key4: 'value4',
      };

      const query = buildQueryString(params);

      expect(query).toBe('?key1=value1&key4=value4');
    });

    it('空参数应该返回空字符串', () => {
      const query = buildQueryString({});

      expect(query).toBe('');
    });
  });

  describe('ApiError', () => {
    it('应该正确创建错误对象', () => {
      const error = new ApiError('测试错误', 404, { test: 'data' });

      expect(error.message).toBe('测试错误');
      expect(error.statusCode).toBe(404);
      expect(error.response).toEqual({ test: 'data' });
      expect(error.name).toBe('ApiError');
    });
  });
});
