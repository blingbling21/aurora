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
 * 配置管理 API 服务
 */

import { get, post, put, del } from './client';
import type {
  ApiResponse,
  ConfigListItem,
  CreateConfigRequest,
  UpdateConfigRequest,
  ConfigValidateResponse,
} from '@/types/api';

/**
 * 配置管理服务类
 */
export class ConfigService {
  /**
   * 获取所有配置文件列表
   */
  static async list(): Promise<ApiResponse<ConfigListItem[]>> {
    return get<ConfigListItem[]>('/config');
  }

  /**
   * 获取指定配置文件内容
   * 
   * @param filename 配置文件名
   */
  static async get(filename: string): Promise<ApiResponse<string>> {
    return get<string>(`/config/${encodeURIComponent(filename)}`);
  }

  /**
   * 创建新配置文件
   * 
   * @param request 创建配置请求
   */
  static async create(
    request: CreateConfigRequest
  ): Promise<ApiResponse<void>> {
    return post<void>('/config', request);
  }

  /**
   * 更新配置文件
   * 
   * @param filename 配置文件名
   * @param request 更新配置请求
   */
  static async update(
    filename: string,
    request: UpdateConfigRequest
  ): Promise<ApiResponse<void>> {
    return put<void>(`/config/${encodeURIComponent(filename)}`, request);
  }

  /**
   * 删除配置文件
   * 
   * @param filename 配置文件名
   */
  static async delete(filename: string): Promise<ApiResponse<void>> {
    return del<void>(`/config/${encodeURIComponent(filename)}`);
  }

  /**
   * 验证配置内容
   * 
   * @param content 配置内容（TOML 格式）
   */
  static async validate(
    content: string
  ): Promise<ApiResponse<ConfigValidateResponse>> {
    return post<ConfigValidateResponse>('/config/validate', { content });
  }
}

// 导出单例方法供直接使用
export const configApi = {
  list: () => ConfigService.list(),
  get: (filename: string) => ConfigService.get(filename),
  create: (request: CreateConfigRequest) => ConfigService.create(request),
  update: (filename: string, request: UpdateConfigRequest) =>
    ConfigService.update(filename, request),
  delete: (filename: string) => ConfigService.delete(filename),
  validate: (content: string) => ConfigService.validate(content),
};
