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
 * 仪表盘 API 服务
 */

import { get } from './client';
import type { ApiResponse, DashboardData } from '@/types/api';

/**
 * 仪表盘服务类
 */
export class DashboardService {
  /**
   * 获取仪表盘数据（统计信息和最近任务）
   * 
   * @returns 仪表盘数据，包含任务统计和最近任务列表
   */
  static async getData(): Promise<ApiResponse<DashboardData>> {
    return get<DashboardData>('/dashboard');
  }
}
