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
 * 数据管理 API 服务
 */

import { get, post, del, buildQueryString } from './client';
import type {
  ApiResponse,
  DataFileItem,
  FetchDataRequest,
  Kline,
} from '@/types/api';

/**
 * 数据管理服务类
 */
export class DataService {
  /**
   * 获取所有数据文件列表
   */
  static async list(): Promise<ApiResponse<DataFileItem[]>> {
    return get<DataFileItem[]>('/data/list');
  }

  /**
   * 获取指定数据文件内容
   * 
   * @param filename 数据文件名
   */
  static async get(filename: string): Promise<ApiResponse<string>> {
    return get<string>(`/data/${encodeURIComponent(filename)}`);
  }

  /**
   * 删除数据文件
   * 
   * @param filename 数据文件名
   */
  static async delete(filename: string): Promise<ApiResponse<void>> {
    return del<void>(`/data/${encodeURIComponent(filename)}`);
  }

  /**
   * 获取历史数据（从交易所下载）
   * 
   * @param request 获取数据请求
   */
  static async fetch(request: FetchDataRequest): Promise<ApiResponse<void>> {
    return post<void>('/data/fetch', request);
  }

  /**
   * 获取 K线数据
   * 
   * @param params 查询参数
   */
  static async getKlines(params: {
    filename: string;
    start?: number;
    end?: number;
    limit?: number;
  }): Promise<ApiResponse<Kline[]>> {
    const query = buildQueryString(params);
    return get<Kline[]>(`/data/klines${query}`);
  }

  /**
   * 生成文件名
   * 根据交易所、交易对、时间间隔和日期范围生成标准文件名
   * 
   * @param params 文件名参数
   */
  static generateFilename(params: {
    exchange: string;
    symbol: string;
    interval: string;
    startDate: string;
    endDate: string;
  }): string {
    const { exchange, symbol, interval, startDate, endDate } = params;
    
    // 格式化日期（移除连字符）
    const formattedStart = startDate.replace(/-/g, '');
    const formattedEnd = endDate.replace(/-/g, '');
    
    return `${exchange.toLowerCase()}_${symbol.toLowerCase()}_${interval}_${formattedStart}_to_${formattedEnd}.csv`;
  }
}

// 导出单例方法供直接使用
export const dataApi = {
  list: () => DataService.list(),
  get: (filename: string) => DataService.get(filename),
  delete: (filename: string) => DataService.delete(filename),
  fetch: (request: FetchDataRequest) => DataService.fetch(request),
  getKlines: (params: Parameters<typeof DataService.getKlines>[0]) =>
    DataService.getKlines(params),
  generateFilename: (
    params: Parameters<typeof DataService.generateFilename>[0]
  ) => DataService.generateFilename(params),
};
