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
 * API 客户端基础设施
 * 提供统一的 HTTP 请求封装和错误处理
 */

import { z } from 'zod';
import type { ApiResponse } from '@/types/api';

/**
 * API 基础 URL
 * 可通过环境变量配置
 */
const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || '/api';

/**
 * API 错误类
 */
export class ApiError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public response?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

/**
 * HTTP 请求选项
 */
export interface RequestOptions extends RequestInit {
  // 是否跳过响应解析
  skipParsing?: boolean;
  // 超时时间（毫秒）
  timeout?: number;
}

/**
 * 创建带超时的 fetch 请求
 */
async function fetchWithTimeout(
  url: string,
  options: RequestOptions = {}
): Promise<Response> {
  const { timeout = 30000, ...fetchOptions } = options;

  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, {
      ...fetchOptions,
      signal: controller.signal,
    });
    clearTimeout(timeoutId);
    return response;
  } catch (error) {
    clearTimeout(timeoutId);
    if (error instanceof Error && error.name === 'AbortError') {
      throw new ApiError('请求超时', 408);
    }
    throw error;
  }
}

/**
 * 统一的 API 请求函数
 * 
 * @param endpoint API 端点路径
 * @param options 请求选项
 * @returns Promise<ApiResponse<T>>
 */
export async function apiRequest<T = unknown>(
  endpoint: string,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const url = `${API_BASE_URL}${endpoint}`;
  
  // 设置默认请求头
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  try {
    // 发送请求
    const response = await fetchWithTimeout(url, {
      ...options,
      headers,
    });

    // 如果跳过解析，直接返回响应
    if (options.skipParsing) {
      return {
        success: response.ok,
        data: response as unknown as T,
      };
    }

    // 尝试解析 JSON 响应
    let data: unknown;
    try {
      data = await response.json();
    } catch {
      // 如果不是 JSON，尝试文本
      const text = await response.text();
      data = { message: text };
    }

    // 检查 HTTP 状态
    if (!response.ok) {
      const errorMessage = 
        (data as { message?: string })?.message || 
        (data as { error?: string })?.error ||
        `请求失败: ${response.status} ${response.statusText}`;
      
      throw new ApiError(errorMessage, response.status, data);
    }

    // 返回标准化响应
    return data as ApiResponse<T>;
  } catch (error) {
    // 处理 ApiError
    if (error instanceof ApiError) {
      throw error;
    }

    // 处理网络错误或其他错误
    if (error instanceof Error) {
      throw new ApiError(
        error.message || '网络请求失败',
        undefined,
        error
      );
    }

    // 未知错误
    throw new ApiError('未知错误');
  }
}

/**
 * 带 Zod 验证的 API 请求函数
 * 
 * @param endpoint API 端点路径
 * @param schema Zod 验证 schema
 * @param options 请求选项
 * @returns Promise<T>
 */
export async function apiRequestWithValidation<T>(
  endpoint: string,
  schema: z.ZodType<T>,
  options: RequestOptions = {}
): Promise<T> {
  const response = await apiRequest<T>(endpoint, options);

  // 验证响应数据
  if (response.data !== undefined) {
    try {
      return schema.parse(response.data);
    } catch (error) {
      if (error instanceof z.ZodError) {
        throw new ApiError(
          '响应数据验证失败: ' + error.issues.map((e) => e.message).join(', '),
          undefined,
          error.issues
        );
      }
      throw error;
    }
  }

  throw new ApiError('响应数据为空');
}

/**
 * GET 请求
 */
export async function get<T = unknown>(
  endpoint: string,
  options?: RequestOptions
): Promise<ApiResponse<T>> {
  return apiRequest<T>(endpoint, {
    ...options,
    method: 'GET',
  });
}

/**
 * POST 请求
 */
export async function post<T = unknown>(
  endpoint: string,
  data?: unknown,
  options?: RequestOptions
): Promise<ApiResponse<T>> {
  return apiRequest<T>(endpoint, {
    ...options,
    method: 'POST',
    body: data ? JSON.stringify(data) : undefined,
  });
}

/**
 * PUT 请求
 */
export async function put<T = unknown>(
  endpoint: string,
  data?: unknown,
  options?: RequestOptions
): Promise<ApiResponse<T>> {
  return apiRequest<T>(endpoint, {
    ...options,
    method: 'PUT',
    body: data ? JSON.stringify(data) : undefined,
  });
}

/**
 * DELETE 请求
 */
export async function del<T = unknown>(
  endpoint: string,
  options?: RequestOptions
): Promise<ApiResponse<T>> {
  return apiRequest<T>(endpoint, {
    ...options,
    method: 'DELETE',
  });
}

/**
 * 构建查询字符串
 */
export function buildQueryString(params: Record<string, unknown>): string {
  const searchParams = new URLSearchParams();
  
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null) {
      searchParams.append(key, String(value));
    }
  });

  const query = searchParams.toString();
  return query ? `?${query}` : '';
}
