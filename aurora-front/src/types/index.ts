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

// 导出所有Zod schemas和类型
export * from './schemas';

// 导出验证函数
export * from './validators';

// 导出配置相关的schema和类型
export * from './config-schema';

// 导出 API 类型（避免命名冲突，仅导出类型）
export type {
  ApiResponse,
  ConfigListItem,
  CreateConfigRequest,
  UpdateConfigRequest,
  ConfigValidateResponse,
  DataFileItem,
  FetchDataRequest,
  Kline,
  TaskStatus as ApiTaskStatus,
  BacktestTask as ApiBacktestTask,
  BacktestTaskSummary as ApiBacktestTaskSummary,
  StartBacktestRequest,
  BacktestMetrics as ApiBacktestMetrics,
  EquityPoint,
  Trade as ApiTrade,
  BacktestResult as ApiBacktestResult,
  FullBacktestResult,
  WsMessageType,
  WsMessage,
  DashboardStats,
} from './api';

// 为了向后兼容，保留原有的导出方式
export type {
  TaskStatus,
  BacktestTask,
  ConfigFile,
  DataFile,
  BacktestMetrics,
  Trade,
  EquityCurvePoint,
  BacktestResult,
  NavMenuItem,
  NotificationType,
  Notification,
  DataDownloadRequest,
  BacktestConfig,
} from './schemas';
