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
 * 回测管理 API 服务
 */

import { get, post, del } from './client';
import type {
  ApiResponse,
  BacktestTask,
  BacktestTaskSummary,
  StartBacktestRequest,
  FullBacktestResult,
} from '@/types/api';

/**
 * 回测管理服务类
 */
export class BacktestService {
  /**
   * 获取所有回测任务列表（历史记录）
   * 返回任务摘要列表，不包含完整的回测结果数据
   */
  static async list(): Promise<ApiResponse<BacktestTaskSummary[]>> {
    return get<BacktestTaskSummary[]>('/backtest/history');
  }

  /**
   * 获取指定回测任务详情
   * 
   * @param taskId 任务 ID
   */
  static async get(taskId: string): Promise<ApiResponse<BacktestTask>> {
    return get<BacktestTask>(`/backtest/${encodeURIComponent(taskId)}`);
  }

  /**
   * 创建并启动回测任务
   * 
   * @param request 启动回测请求
   */
  static async start(
    request: StartBacktestRequest
  ): Promise<ApiResponse<{ task_id: string }>> {
    return post<{ task_id: string }>('/backtest/start', request);
  }

  /**
   * 删除回测任务
   * 
   * @param taskId 任务 ID
   */
  static async delete(taskId: string): Promise<ApiResponse<void>> {
    return del<void>(`/backtest/${encodeURIComponent(taskId)}`);
  }

  /**
   * 获取回测结果
   * 
   * @param taskId 任务 ID
   */
  static async getResult(
    taskId: string
  ): Promise<ApiResponse<FullBacktestResult>> {
    return get<FullBacktestResult>(
      `/backtest/result/${encodeURIComponent(taskId)}`
    );
  }

  /**
   * 创建 WebSocket 连接 URL
   * 
   * @param taskId 任务 ID
   */
  static getWebSocketUrl(taskId: string): string {
    // 从环境变量获取API基础URL
    const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:8080/api';
    
    // 提取主机地址（移除/api路径）
    const url = new URL(apiBaseUrl);
    const protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = url.host; // 这里会包含端口号
    
    // 构建WebSocket URL
    const wsUrl = `${protocol}//${host}/ws/backtest/${encodeURIComponent(taskId)}`;
    
    console.log('WebSocket URL:', wsUrl); // 调试日志
    return wsUrl;
  }
}

// 导出单例方法供直接使用
export const backtestApi = {
  list: () => BacktestService.list(),
  get: (taskId: string) => BacktestService.get(taskId),
  start: (request: StartBacktestRequest) => BacktestService.start(request),
  delete: (taskId: string) => BacktestService.delete(taskId),
  getResult: (taskId: string) => BacktestService.getResult(taskId),
  getWebSocketUrl: (taskId: string) => BacktestService.getWebSocketUrl(taskId),
};
