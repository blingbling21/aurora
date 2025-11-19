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

import { z } from 'zod';

/**
 * 任务状态枚举Schema
 * 用于验证任务状态值
 */
export const TaskStatusSchema = z.enum(['pending', 'running', 'completed', 'failed']);

/**
 * 回测任务Schema
 * 用于验证和解析回测任务数据
 */
export const BacktestTaskSchema = z.object({
  id: z.string().min(1, '任务ID不能为空'),
  name: z.string().min(1, '任务名称不能为空').max(100, '任务名称不能超过100个字符'),
  status: TaskStatusSchema,
  config: z.string().min(1, '配置文件不能为空'),
  dataFile: z.string().min(1, '数据文件不能为空'),
  progress: z.number().min(0, '进度不能小于0').max(100, '进度不能大于100'),
  createdAt: z.string().datetime('创建时间格式不正确'),
  updatedAt: z.string().datetime('更新时间格式不正确'),
});

/**
 * 配置文件Schema
 * 用于验证配置文件数据
 */
export const ConfigFileSchema = z.object({
  name: z.string().min(1, '配置文件名不能为空'),
  path: z.string().min(1, '配置文件路径不能为空'),
  content: z.string(),
  lastModified: z.string().datetime('最后修改时间格式不正确'),
});

/**
 * 数据文件Schema
 * 用于验证数据文件信息
 */
export const DataFileSchema = z.object({
  name: z.string().min(1, '数据文件名不能为空'),
  path: z.string().min(1, '数据文件路径不能为空'),
  size: z.number().min(0, '文件大小不能为负数'),
  lastModified: z.string().datetime('最后修改时间格式不正确'),
});

/**
 * 回测结果指标Schema
 * 用于验证回测结果指标数据
 */
export const BacktestMetricsSchema = z.object({
  totalReturn: z.number(),
  annualizedReturn: z.number(),
  maxDrawdown: z.number(),
  maxDrawdownDuration: z.number().int('最大回撤持续时间必须为整数').min(0),
  sharpeRatio: z.number(),
  sortinoRatio: z.number(),
  calmarRatio: z.number(),
  annualizedVolatility: z.number().min(0, '年化波动率不能为负数'),
  winRate: z.number().min(0, '胜率不能小于0').max(100, '胜率不能大于100'),
  totalTrades: z.number().int('总交易次数必须为整数').min(0),
  winningTrades: z.number().int('盈利交易次数必须为整数').min(0),
  losingTrades: z.number().int('亏损交易次数必须为整数').min(0),
  averageWin: z.number(),
  averageLoss: z.number(),
  profitLossRatio: z.number().min(0, '盈亏比不能为负数'),
  profitFactor: z.number().min(0, '盈利因子不能为负数'),
  maxConsecutiveWins: z.number().int('最大连胜次数必须为整数').min(0),
  maxConsecutiveLosses: z.number().int('最大连亏次数必须为整数').min(0),
  avgHoldingPeriod: z.number().min(0, '平均持仓时间不能为负数'),
  maxWin: z.number(),
  maxLoss: z.number(),
});

/**
 * 交易Schema
 * 用于验证交易数据
 */
export const TradeSchema = z.object({
  id: z.string().min(1, '交易ID不能为空'),
  type: z.enum(['buy', 'sell'], '交易类型必须是buy或sell'),
  symbol: z.string().min(1, '交易标的不能为空'),
  price: z.number().positive('价格必须为正数'),
  quantity: z.number().positive('数量必须为正数'),
  time: z.union([z.string().datetime(), z.number()], {
    message: '交易时间必须是 ISO 字符串或 Unix 时间戳',
  }),
  pnl: z.number().optional(),
  commission: z.number().min(0).optional(),
});

/**
 * 权益曲线点Schema
 * 用于验证权益曲线数据点
 */
export const EquityCurvePointSchema = z.object({
  time: z.union([z.string().datetime(), z.number()]),
  value: z.number().min(0, '权益值不能为负数'),
});

/**
 * 回撤序列点Schema
 * 用于验证回撤数据点（潜水图）
 */
export const DrawdownPointSchema = z.object({
  time: z.string().datetime('时间格式不正确'),
  drawdown: z.number().max(0, '回撤值必须为负数或0'),
});

/**
 * 月度收益Schema
 * 用于验证月度收益数据
 */
export const MonthlyReturnSchema = z.object({
  year: z.number().int('年份必须为整数'),
  month: z.number().int('月份必须为整数').min(1).max(12),
  return: z.number(),
});

/**
 * 滚动指标点Schema
 * 用于验证滚动指标数据点
 */
export const RollingMetricPointSchema = z.object({
  time: z.string().datetime('时间格式不正确'),
  volatility: z.number().min(0).optional(),
  sharpe: z.number().optional(),
  return: z.number().optional(),
});

/**
 * 收益分布桶Schema
 * 用于验证收益分布直方图数据
 */
export const ReturnBucketSchema = z.object({
  min: z.number(),
  max: z.number(),
  count: z.number().int().min(0),
  label: z.string(),
});

/**
 * 回测结果Schema
 * 用于验证完整的回测结果数据
 */
export const BacktestResultSchema = z.object({
  taskId: z.string().min(1, '任务ID不能为空'),
  metrics: BacktestMetricsSchema,
  equityCurve: z.array(EquityCurvePointSchema).min(1, '权益曲线不能为空'),
  trades: z.array(TradeSchema),
  drawdownSeries: z.array(DrawdownPointSchema).optional(),
  monthlyReturns: z.array(MonthlyReturnSchema).optional(),
  rollingMetrics: z.array(RollingMetricPointSchema).optional(),
  returnsDistribution: z.array(ReturnBucketSchema).optional(),
});

/**
 * 导航菜单项Schema
 * 用于验证导航菜单项数据
 */
export const NavMenuItemSchema = z.object({
  id: z.string().min(1, '菜单ID不能为空'),
  label: z.string().min(1, '菜单标签不能为空'),
  icon: z.string(),
  href: z.string().min(1, '菜单链接不能为空'),
});

/**
 * 通知类型Schema
 * 用于验证通知类型
 */
export const NotificationTypeSchema = z.enum(['success', 'error', 'info', 'warning']);

/**
 * 通知消息Schema
 * 用于验证通知消息数据
 */
export const NotificationSchema = z.object({
  id: z.string().min(1, '通知ID不能为空'),
  type: NotificationTypeSchema,
  message: z.string().min(1, '通知消息不能为空'),
  duration: z.number().positive('通知持续时间必须为正数').optional(),
});

/**
 * 数据下载请求Schema
 * 用于验证数据下载请求参数
 */
export const DataDownloadRequestSchema = z.object({
  exchange: z.string().min(1, '交易所不能为空'),
  symbol: z.string().min(1, '交易对不能为空'),
  interval: z.string().min(1, '时间间隔不能为空'),
  startDate: z.string().datetime('开始日期格式不正确'),
  endDate: z.string().datetime('结束日期格式不正确'),
}).refine((data) => {
  // 验证结束日期必须晚于开始日期
  return new Date(data.endDate) > new Date(data.startDate);
}, {
  message: '结束日期必须晚于开始日期',
  path: ['endDate'],
});

/**
 * 回测配置Schema
 * 用于验证回测配置数据
 */
export const BacktestConfigSchema = z.object({
  taskName: z.string().min(1, '任务名称不能为空').max(100, '任务名称不能超过100个字符'),
  configFile: z.string().min(1, '配置文件不能为空'),
  dataFile: z.string().min(1, '数据文件不能为空'),
  description: z.string().max(500, '描述不能超过500个字符').optional(),
});

// 导出类型推断
export type TaskStatus = z.infer<typeof TaskStatusSchema>;
export type BacktestTask = z.infer<typeof BacktestTaskSchema>;
export type ConfigFile = z.infer<typeof ConfigFileSchema>;
export type DataFile = z.infer<typeof DataFileSchema>;
export type BacktestMetrics = z.infer<typeof BacktestMetricsSchema>;
export type Trade = z.infer<typeof TradeSchema>;
export type EquityCurvePoint = z.infer<typeof EquityCurvePointSchema>;
export type DrawdownPoint = z.infer<typeof DrawdownPointSchema>;
export type MonthlyReturn = z.infer<typeof MonthlyReturnSchema>;
export type RollingMetricPoint = z.infer<typeof RollingMetricPointSchema>;
export type ReturnBucket = z.infer<typeof ReturnBucketSchema>;
export type BacktestResult = z.infer<typeof BacktestResultSchema>;
export type NavMenuItem = z.infer<typeof NavMenuItemSchema>;
export type NotificationType = z.infer<typeof NotificationTypeSchema>;
export type Notification = z.infer<typeof NotificationSchema>;
export type DataDownloadRequest = z.infer<typeof DataDownloadRequestSchema>;
export type BacktestConfig = z.infer<typeof BacktestConfigSchema>;
