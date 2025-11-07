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
 * API 响应和请求类型定义
 * 使用 Zod 进行类型声明和验证
 */

import { z } from 'zod';

// ============ 通用响应类型 ============

/**
 * API 统一响应结构 Schema
 */
export const ApiResponseSchema = <T extends z.ZodTypeAny>(dataSchema: T) =>
  z.object({
    success: z.boolean(),
    message: z.string().optional(),
    data: dataSchema.optional(),
    error: z.string().optional(),
  });

/**
 * API 响应类型
 */
export type ApiResponse<T> = {
  success: boolean;
  message?: string;
  data?: T;
  error?: string;
};

// ============ 配置相关类型 ============

/**
 * 配置列表项 Schema
 */
export const ConfigListItemSchema = z.object({
  filename: z.string(),
  path: z.string(),
  modified: z.string(),
});

export type ConfigListItem = z.infer<typeof ConfigListItemSchema>;

/**
 * 创建配置请求 Schema
 */
export const CreateConfigRequestSchema = z.object({
  filename: z.string().min(1, '文件名不能为空'),
  content: z.string().min(1, '配置内容不能为空'),
});

export type CreateConfigRequest = z.infer<typeof CreateConfigRequestSchema>;

/**
 * 更新配置请求 Schema
 */
export const UpdateConfigRequestSchema = z.object({
  content: z.string().min(1, '配置内容不能为空'),
});

export type UpdateConfigRequest = z.infer<typeof UpdateConfigRequestSchema>;

/**
 * 配置验证响应 Schema
 */
export const ConfigValidateResponseSchema = z.object({
  valid: z.boolean(),
  errors: z.array(z.string()).optional(),
  warnings: z.array(z.string()).optional(),
});

export type ConfigValidateResponse = z.infer<typeof ConfigValidateResponseSchema>;

// ============ 数据文件相关类型 ============

/**
 * 数据文件项 Schema
 */
export const DataFileItemSchema = z.object({
  filename: z.string(),
  size: z.number(),
  modified: z.string(),
  record_count: z.number().optional(),
});

export type DataFileItem = z.infer<typeof DataFileItemSchema>;

/**
 * 获取历史数据请求 Schema
 */
export const FetchDataRequestSchema = z.object({
  exchange: z.string().min(1, '交易所不能为空'),
  symbol: z.string().min(1, '交易对不能为空'),
  interval: z.string().min(1, '时间间隔不能为空'),
  start_date: z.string().min(1, '开始日期不能为空'),
  end_date: z.string().min(1, '结束日期不能为空'),
  filename: z.string().optional(),
});

export type FetchDataRequest = z.infer<typeof FetchDataRequestSchema>;

/**
 * K线数据 Schema
 */
export const KlineSchema = z.object({
  timestamp: z.number(),
  open: z.number(),
  high: z.number(),
  low: z.number(),
  close: z.number(),
  volume: z.number(),
});

export type Kline = z.infer<typeof KlineSchema>;

/**
 * 数据下载任务响应 Schema
 */
export const DownloadTaskResponseSchema = z.object({
  task_id: z.string(),
  message: z.string(),
  filename: z.string(),
});

export type DownloadTaskResponse = z.infer<typeof DownloadTaskResponseSchema>;

/**
 * 下载任务状态
 */
export const DownloadStatusSchema = z.enum(['Pending', 'Downloading', 'Completed', 'Failed']);
export type DownloadStatus = z.infer<typeof DownloadStatusSchema>;

/**
 * 数据下载进度消息 Schema
 */
export const DownloadProgressMessageSchema = z.object({
  type: z.enum(['connected', 'progress', 'complete', 'error']),
  task_id: z.string().optional(),
  status: DownloadStatusSchema.optional(),
  progress: z.number().optional(),
  progress_message: z.string().optional(),
  downloaded_count: z.number().optional(),
  estimated_total: z.number().optional().nullable(),
  error: z.string().optional().nullable(),
  message: z.string().optional(),
});

export type DownloadProgressMessage = z.infer<typeof DownloadProgressMessageSchema>;

// ============ 回测相关类型 ============

/**
 * 回测任务状态
 */
export const TaskStatusSchema = z.enum(['pending', 'running', 'completed', 'failed']);
export type TaskStatus = z.infer<typeof TaskStatusSchema>;

/**
 * 回测任务 Schema
 */
export const BacktestTaskSchema = z.object({
  id: z.string(),
  name: z.string(),
  status: TaskStatusSchema,
  progress: z.number(),
  created_at: z.string(),
  started_at: z.string().optional(),
  completed_at: z.string().optional(),
  error: z.string().optional(),
  config_path: z.string().optional(),
  data_path: z.string().optional(),
});

export type BacktestTask = z.infer<typeof BacktestTaskSchema>;

/**
 * 创建并启动回测请求 Schema
 */
export const StartBacktestRequestSchema = z.object({
  name: z.string().min(1, '任务名称不能为空'),
  config_path: z.string().min(1, '配置文件路径不能为空'),
  data_path: z.string().min(1, '数据文件路径不能为空'),
});

export type StartBacktestRequest = z.infer<typeof StartBacktestRequestSchema>;

/**
 * 回测指标 Schema
 */
export const BacktestMetricsSchema = z.object({
  total_return: z.number().optional(),
  annualized_return: z.number().optional(),
  max_drawdown: z.number().optional(),
  max_drawdown_duration: z.number().optional(),
  sharpe_ratio: z.number().optional(),
  sortino_ratio: z.number().optional(),
  calmar_ratio: z.number().optional(),
  annualized_volatility: z.number().optional(),
  total_trades: z.number().optional(),
  win_rate: z.number().optional(),
  profit_loss_ratio: z.number().optional(),
  profit_factor: z.number().optional(),
  average_win: z.number().optional(),
  average_loss: z.number().optional(),
  max_win: z.number().optional(),
  max_loss: z.number().optional(),
  max_consecutive_wins: z.number().optional(),
  max_consecutive_losses: z.number().optional(),
  avg_holding_period: z.number().optional(),
});

export type BacktestMetrics = z.infer<typeof BacktestMetricsSchema>;

/**
 * 权益曲线点 Schema
 */
export const EquityPointSchema = z.object({
  timestamp: z.number(),
  equity: z.number(),
  drawdown: z.number().optional(),
});

export type EquityPoint = z.infer<typeof EquityPointSchema>;

/**
 * 交易记录 Schema
 */
export const TradeSchema = z.object({
  id: z.string().optional(),
  timestamp: z.number(),
  side: z.enum(['buy', 'sell']),
  price: z.number(),
  quantity: z.number(),
  fee: z.number().optional(),
  pnl: z.number().optional(),
});

export type Trade = z.infer<typeof TradeSchema>;

/**
 * 回测结果 Schema
 */
export const BacktestResultSchema = z.object({
  metrics: BacktestMetricsSchema,
  equity_curve: z.array(EquityPointSchema),
  trades: z.array(TradeSchema),
  alpha: z.number().optional(),
  annualized_alpha: z.number().optional(),
  data_path: z.string().optional(),
});

export type BacktestResult = z.infer<typeof BacktestResultSchema>;

/**
 * 完整回测结果（包含基准数据）Schema
 */
export const FullBacktestResultSchema = z.object({
  result: BacktestResultSchema,
  benchmark_equity_curve: z.array(EquityPointSchema).optional(),
});

export type FullBacktestResult = z.infer<typeof FullBacktestResultSchema>;

// ============ WebSocket 消息类型 ============

/**
 * WebSocket 消息类型
 */
export const WsMessageTypeSchema = z.enum(['connected', 'status_update', 'final', 'error']);
export type WsMessageType = z.infer<typeof WsMessageTypeSchema>;

/**
 * WebSocket 消息 Schema
 */
export const WsMessageSchema = z.object({
  type: WsMessageTypeSchema,
  progress: z.number().optional(),
  status: TaskStatusSchema.optional(),
  message: z.string().optional(),
  error: z.string().optional(),
  data: z.any().optional(),
});

export type WsMessage = z.infer<typeof WsMessageSchema>;

// ============ 仪表板统计类型 ============

/**
 * 仪表板统计数据 Schema
 */
export const DashboardStatsSchema = z.object({
  total_tasks: z.number(),
  running_tasks: z.number(),
  completed_tasks: z.number(),
  failed_tasks: z.number(),
});

export type DashboardStats = z.infer<typeof DashboardStatsSchema>;
