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
 * API 数据转换工具
 * 将 API 返回的 snake_case 数据转换为前端使用的 camelCase 格式
 */

import { BacktestTask as ApiBacktestTask, BacktestTaskSummary as ApiBacktestTaskSummary } from '@/types/api';
import { BacktestTask } from '@/types/schemas';

/**
 * 将 API 返回的回测任务摘要数据转换为前端格式
 * 
 * @param apiTask API 返回的任务摘要数据
 * @returns 前端使用的任务数据
 */
export function convertApiTaskSummaryToLocal(apiTask: ApiBacktestTaskSummary): BacktestTask {
  return {
    id: apiTask.id,
    name: apiTask.name,
    status: apiTask.status,
    config: apiTask.config_path || '',
    dataFile: apiTask.data_path || '',
    progress: apiTask.progress,
    createdAt: apiTask.created_at,
    updatedAt: apiTask.completed_at || apiTask.started_at || apiTask.created_at,
  };
}

/**
 * 将 API 返回的回测任务数据转换为前端格式
 * 
 * @param apiTask API 返回的任务数据
 * @returns 前端使用的任务数据
 */
export function convertApiTaskToLocal(apiTask: ApiBacktestTask): BacktestTask {
  return {
    id: apiTask.id,
    name: apiTask.name,
    status: apiTask.status,
    config: apiTask.config_path || '',
    dataFile: apiTask.data_path || '',
    progress: apiTask.progress,
    createdAt: apiTask.created_at,
    updatedAt: apiTask.completed_at || apiTask.started_at || apiTask.created_at,
  };
}

/**
 * 批量转换 API 任务摘要数据
 * 
 * @param apiTasks API 返回的任务摘要列表
 * @returns 前端使用的任务列表
 */
export function convertApiTaskSummariesToLocal(apiTasks: ApiBacktestTaskSummary[]): BacktestTask[] {
  return apiTasks.map(convertApiTaskSummaryToLocal);
}

/**
 * 批量转换 API 任务数据
 * 
 * @param apiTasks API 返回的任务列表
 * @returns 前端使用的任务列表
 */
export function convertApiTasksToLocal(apiTasks: ApiBacktestTask[]): BacktestTask[] {
  return apiTasks.map(convertApiTaskToLocal);
}
