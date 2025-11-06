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
 * Aurora 配置文件完整的类型定义和Schema
 * 基于 complete_config.toml 规范
 */

import { z } from 'zod';

// ==================== 数据源配置 ====================

/**
 * 数据源配置Schema
 */
export const DataSourceConfigSchema = z.object({
  // 数据提供商名称
  provider: z.enum(['binance', 'okx', 'bybit', 'csv']).default('binance'),
  // API密钥(可选)
  api_key: z.string().optional(),
  // API密钥(可选)
  api_secret: z.string().optional(),
  // 基础URL(可选)
  base_url: z.string().url('基础URL格式不正确').optional(),
  // WebSocket URL(可选)
  ws_url: z.string().url('WebSocket URL格式不正确').optional(),
  // 连接超时时间(秒)
  timeout: z.number().int().positive('超时时间必须为正整数').default(30),
  // 最大重试次数
  max_retries: z.number().int().nonnegative('最大重试次数不能为负数').default(3),
});

export type DataSourceConfig = z.infer<typeof DataSourceConfigSchema>;

// ==================== 策略配置 ====================

/**
 * 策略参数Schema(灵活的键值对)
 */
export const StrategyParametersSchema = z.record(
  z.string(),
  z.union([z.string(), z.number(), z.boolean()])
);

/**
 * 单个策略配置Schema
 */
export const StrategyConfigSchema = z.object({
  // 策略名称
  name: z.string().min(1, '策略名称不能为空'),
  // 策略类型
  strategy_type: z.string().min(1, '策略类型不能为空'),
  // 是否启用
  enabled: z.boolean().default(true),
  // 策略参数
  parameters: StrategyParametersSchema,
});

export type StrategyConfig = z.infer<typeof StrategyConfigSchema>;

// ==================== 风险管理配置 ====================

/**
 * 风险规则配置Schema
 */
export const RiskRulesSchema = z.object({
  // 最大回撤限制(百分比)
  max_drawdown_pct: z.number().min(0).max(100).optional(),
  // 单日最大亏损限制(百分比)
  max_daily_loss_pct: z.number().min(0).max(100).optional(),
  // 连续亏损次数限制
  max_consecutive_losses: z.number().int().positive().optional(),
  // 单笔交易最大亏损限制(百分比)
  max_single_trade_loss_pct: z.number().min(0).max(100).optional(),
  // 账户最低权益要求
  min_equity: z.number().positive().optional(),
  // 止损百分比
  stop_loss_pct: z.number().min(0).max(100).optional(),
  // 止盈百分比
  take_profit_pct: z.number().positive().optional(),
}).optional();

export type RiskRules = z.infer<typeof RiskRulesSchema>;

// ==================== 仓位管理配置 ====================

/**
 * 固定比例仓位策略Schema
 */
export const FixedPercentagePositionSizingSchema = z.object({
  strategy_type: z.literal('fixed_percentage'),
  percentage: z.number().min(0).max(1, '百分比必须在0到1之间'),
});

/**
 * Kelly准则仓位策略Schema
 */
export const KellyCriterionPositionSizingSchema = z.object({
  strategy_type: z.literal('kelly_criterion'),
  win_rate: z.number().min(0).max(1, '胜率必须在0到1之间'),
  profit_loss_ratio: z.number().positive('盈亏比必须为正数'),
  kelly_fraction: z.number().min(0).max(1, 'Kelly分数必须在0到1之间').default(0.5),
});

/**
 * 金字塔加仓策略Schema
 */
export const PyramidPositionSizingSchema = z.object({
  strategy_type: z.literal('pyramid'),
  initial_percentage: z.number().min(0).max(1, '初始百分比必须在0到1之间'),
  profit_threshold: z.number().positive('盈利阈值必须为正数'),
  max_percentage: z.number().min(0).max(1, '最大百分比必须在0到1之间'),
  increment: z.number().min(0).max(1, '增量必须在0到1之间'),
});

/**
 * 固定金额仓位策略Schema
 */
export const FixedAmountPositionSizingSchema = z.object({
  strategy_type: z.literal('fixed_amount'),
  amount: z.number().positive('金额必须为正数'),
});

/**
 * 全仓策略Schema
 */
export const AllInPositionSizingSchema = z.object({
  strategy_type: z.literal('all_in'),
});

/**
 * 仓位管理配置Schema(联合类型)
 */
export const PositionSizingSchema = z.discriminatedUnion('strategy_type', [
  FixedPercentagePositionSizingSchema,
  KellyCriterionPositionSizingSchema,
  PyramidPositionSizingSchema,
  FixedAmountPositionSizingSchema,
  AllInPositionSizingSchema,
]).optional();

export type PositionSizing = z.infer<typeof PositionSizingSchema>;

// ==================== 投资组合配置 ====================

/**
 * 投资组合配置Schema
 */
export const PortfolioConfigSchema = z.object({
  // 初始资金
  initial_cash: z.number().positive('初始资金必须为正数').default(10000.0),
  // 手续费率
  commission: z.number().min(0).max(1, '手续费率必须在0到1之间').default(0.001),
  // 滑点率
  slippage: z.number().min(0).max(1, '滑点率必须在0到1之间').default(0),
  // 单笔最大持仓金额(可选)
  max_position_size: z.number().positive().optional(),
  // 最大持仓数量(可选)
  max_positions: z.number().int().positive().optional(),
  // 风险管理配置(可选)
  risk_rules: RiskRulesSchema,
  // 仓位管理配置(可选)
  position_sizing: PositionSizingSchema,
});

export type PortfolioConfig = z.infer<typeof PortfolioConfigSchema>;

// ==================== 日志配置 ====================

/**
 * 日志配置Schema
 */
export const LoggingConfigSchema = z.object({
  // 日志级别
  level: z.enum(['trace', 'debug', 'info', 'warn', 'error']).default('info'),
  // 日志格式
  format: z.enum(['json', 'pretty']).default('pretty'),
  // 日志输出文件(可选)
  output: z.string().optional(),
});

export type LoggingConfig = z.infer<typeof LoggingConfigSchema>;

// ==================== 定价模式配置 ====================

/**
 * 收盘价定价模式Schema
 * 使用收盘价执行交易(简单模式)
 */
export const PricingModeCloseSchema = z.object({
  mode: z.literal('close'),
});

/**
 * 买一卖一价定价模式Schema
 * 使用买一卖一价执行交易(更真实的模式)
 */
export const PricingModeBidAskSchema = z.object({
  mode: z.literal('bid_ask'),
  // 买卖价差百分比(例如 0.001 表示 0.1% 的价差)
  spread_pct: z.number().min(0).max(1, '价差百分比必须在0到1之间'),
});

/**
 * 定价模式配置Schema(联合类型)
 * 用于控制回测中交易价格的计算方式
 */
export const PricingModeSchema = z.discriminatedUnion('mode', [
  PricingModeCloseSchema,
  PricingModeBidAskSchema,
]).optional();

export type PricingMode = z.infer<typeof PricingModeSchema>;

// ==================== 回测配置 ====================

/**
 * 回测配置Schema
 */
export const BacktestSettingsSchema = z.object({
  // 历史数据文件路径
  data_path: z.string().min(1, '数据文件路径不能为空'),
  // 交易对符号(可选)
  symbol: z.string().optional(),
  // 时间间隔(可选)
  interval: z.string().optional(),
  // 回测开始时间(可选)
  start_time: z.string().optional(),
  // 回测结束时间(可选)
  end_time: z.string().optional(),
  // 定价模式配置(可选)
  pricing_mode: PricingModeSchema,
}).optional();

export type BacktestSettings = z.infer<typeof BacktestSettingsSchema>;

// ==================== 实时交易配置 ====================

/**
 * 实时交易配置Schema
 */
export const LiveConfigSchema = z.object({
  // 交易对符号
  symbol: z.string().min(1, '交易对符号不能为空'),
  // K线时间间隔
  interval: z.string().min(1, 'K线时间间隔不能为空').default('1m'),
  // 是否为模拟交易
  paper_trading: z.boolean().default(true),
}).optional();

export type LiveConfig = z.infer<typeof LiveConfigSchema>;

// ==================== 完整配置 ====================

/**
 * Aurora 完整配置Schema
 */
export const AuroraConfigSchema = z.object({
  // 数据源配置
  data_source: DataSourceConfigSchema,
  // 策略配置数组
  strategies: z.array(StrategyConfigSchema).min(1, '至少需要一个策略配置'),
  // 投资组合配置
  portfolio: PortfolioConfigSchema,
  // 日志配置
  logging: LoggingConfigSchema,
  // 回测配置(可选)
  backtest: BacktestSettingsSchema,
  // 实时交易配置(可选)
  live: LiveConfigSchema,
});

export type AuroraConfig = z.infer<typeof AuroraConfigSchema>;

// ==================== 默认配置值 ====================

/**
 * 创建默认的数据源配置
 */
export const createDefaultDataSourceConfig = (): DataSourceConfig => ({
  provider: 'binance',
  timeout: 30,
  max_retries: 3,
});

/**
 * 创建默认的策略配置
 */
export const createDefaultStrategyConfig = (): StrategyConfig => ({
  name: 'MA交叉策略',
  strategy_type: 'ma-crossover',
  enabled: true,
  parameters: {
    short: 10,
    long: 30,
  },
});

/**
 * 创建默认的投资组合配置
 */
export const createDefaultPortfolioConfig = (): PortfolioConfig => ({
  initial_cash: 10000.0,
  commission: 0.001,
  slippage: 0,
});

/**
 * 创建默认的日志配置
 */
export const createDefaultLoggingConfig = (): LoggingConfig => ({
  level: 'info',
  format: 'pretty',
});

/**
 * 创建默认的完整配置
 */
export const createDefaultAuroraConfig = (): AuroraConfig => ({
  data_source: createDefaultDataSourceConfig(),
  strategies: [createDefaultStrategyConfig()],
  portfolio: createDefaultPortfolioConfig(),
  logging: createDefaultLoggingConfig(),
});
