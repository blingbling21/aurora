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
 * 策略类型定义和参数Schema
 * 定义所有可用的策略类型及其参数配置
 */

import { z } from 'zod';

// ==================== 策略类型定义 ====================

/**
 * MA交叉策略参数Schema
 */
export const MACrossoverParametersSchema = z.object({
  // 短期均线周期
  short: z.number().int().positive('短期周期必须为正整数'),
  // 长期均线周期
  long: z.number().int().positive('长期周期必须为正整数'),
}).refine(
  (data) => data.short < data.long,
  {
    message: '短期周期必须小于长期周期',
    path: ['short'],
  }
);

export type MACrossoverParameters = z.infer<typeof MACrossoverParametersSchema>;

// ==================== 策略类型注册表 ====================

/**
 * 策略类型定义接口
 */
export interface StrategyTypeDefinition {
  // 策略类型标识符
  type: string;
  // 策略显示名称
  name: string;
  // 策略描述
  description: string;
  // 参数Schema
  parametersSchema: z.ZodSchema;
  // 参数字段定义(用于动态表单生成)
  fields: StrategyFieldDefinition[];
}

/**
 * 策略字段定义接口
 */
export interface StrategyFieldDefinition {
  // 字段名称(对应参数key)
  name: string;
  // 字段显示标签
  label: string;
  // 字段类型
  type: 'number' | 'text' | 'select' | 'checkbox';
  // 默认值
  defaultValue?: string | number | boolean;
  // 占位符
  placeholder?: string;
  // 说明文本
  description?: string;
  // 最小值(用于number类型)
  min?: number;
  // 最大值(用于number类型)
  max?: number;
  // 步长(用于number类型)
  step?: number;
  // 选项列表(用于select类型)
  options?: { label: string; value: string }[];
  // 是否必填
  required?: boolean;
}

/**
 * 所有可用的策略类型
 */
export const STRATEGY_TYPES: StrategyTypeDefinition[] = [
  {
    type: 'ma-crossover',
    name: 'MA交叉策略',
    description: '基于移动平均线(MA)的金叉死叉信号进行交易',
    parametersSchema: MACrossoverParametersSchema,
    fields: [
      {
        name: 'short',
        label: '短期均线周期',
        type: 'number',
        defaultValue: 10,
        placeholder: '5-50',
        description: '较小的值对价格变化更敏感,常用5-20',
        min: 1,
        max: 200,
        step: 1,
        required: true,
      },
      {
        name: 'long',
        label: '长期均线周期',
        type: 'number',
        defaultValue: 30,
        placeholder: '20-200',
        description: '较大的值更平滑,适合判断趋势,常用20-200',
        min: 2,
        max: 500,
        step: 1,
        required: true,
      },
    ],
  },
  // 未来可以添加更多策略类型
  // {
  //   type: 'rsi-strategy',
  //   name: 'RSI策略',
  //   description: '基于相对强弱指标(RSI)的超买超卖信号',
  //   parametersSchema: RSIParametersSchema,
  //   fields: [...],
  // },
];

/**
 * 根据策略类型获取策略定义
 */
export function getStrategyDefinition(strategyType: string): StrategyTypeDefinition | undefined {
  return STRATEGY_TYPES.find((def) => def.type === strategyType);
}

/**
 * 获取策略类型的默认参数
 */
export function getDefaultParameters(strategyType: string): Record<string, unknown> {
  const definition = getStrategyDefinition(strategyType);
  if (!definition) {
    return {};
  }

  const defaultParams: Record<string, unknown> = {};
  definition.fields.forEach((field) => {
    if (field.defaultValue !== undefined) {
      defaultParams[field.name] = field.defaultValue;
    }
  });

  return defaultParams;
}

/**
 * 验证策略参数
 */
export function validateStrategyParameters(
  strategyType: string,
  parameters: Record<string, unknown>
): { success: boolean; errors?: Record<string, string> } {
  const definition = getStrategyDefinition(strategyType);
  if (!definition) {
    return { success: false, errors: { _: '未知的策略类型' } };
  }

  try {
    definition.parametersSchema.parse(parameters);
    return { success: true };
  } catch (error) {
    if (error instanceof z.ZodError) {
      const errors: Record<string, string> = {};
      error.issues.forEach((issue) => {
        const path = issue.path.join('.');
        errors[path] = issue.message;
      });
      return { success: false, errors };
    }
    return { success: false, errors: { _: '参数验证失败' } };
  }
}
