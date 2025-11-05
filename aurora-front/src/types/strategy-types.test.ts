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

import {
  STRATEGY_TYPES,
  getStrategyDefinition,
  getDefaultParameters,
  validateStrategyParameters,
  MACrossoverParametersSchema,
} from './strategy-types';

describe('strategy-types', () => {
  describe('STRATEGY_TYPES', () => {
    // 测试策略类型注册表不为空
    it('should have at least one strategy type', () => {
      expect(STRATEGY_TYPES.length).toBeGreaterThan(0);
    });

    // 测试每个策略类型定义的完整性
    it('should have valid structure for each strategy type', () => {
      STRATEGY_TYPES.forEach((strategy) => {
        expect(strategy.type).toBeTruthy();
        expect(strategy.name).toBeTruthy();
        expect(strategy.description).toBeTruthy();
        expect(strategy.parametersSchema).toBeDefined();
        expect(Array.isArray(strategy.fields)).toBe(true);
      });
    });

    // 测试MA交叉策略的存在性
    it('should include ma-crossover strategy', () => {
      const maCrossover = STRATEGY_TYPES.find((s) => s.type === 'ma-crossover');
      expect(maCrossover).toBeDefined();
      expect(maCrossover?.name).toBe('MA交叉策略');
    });
  });

  describe('MACrossoverParametersSchema', () => {
    // 测试有效的参数
    it('should validate valid MA crossover parameters', () => {
      const validParams = { short: 10, long: 30 };
      expect(() => MACrossoverParametersSchema.parse(validParams)).not.toThrow();
    });

    // 测试短期周期必须小于长期周期
    it('should reject when short >= long', () => {
      const invalidParams = { short: 30, long: 10 };
      expect(() => MACrossoverParametersSchema.parse(invalidParams)).toThrow();
    });

    // 测试负数参数
    it('should reject negative values', () => {
      const invalidParams = { short: -5, long: 30 };
      expect(() => MACrossoverParametersSchema.parse(invalidParams)).toThrow();
    });

    // 测试零值参数
    it('should reject zero values', () => {
      const invalidParams = { short: 0, long: 30 };
      expect(() => MACrossoverParametersSchema.parse(invalidParams)).toThrow();
    });

    // 测试缺失参数
    it('should reject missing parameters', () => {
      const invalidParams = { short: 10 };
      expect(() => MACrossoverParametersSchema.parse(invalidParams)).toThrow();
    });
  });

  describe('getStrategyDefinition', () => {
    // 测试获取存在的策略定义
    it('should return strategy definition for valid type', () => {
      const definition = getStrategyDefinition('ma-crossover');
      expect(definition).toBeDefined();
      expect(definition?.type).toBe('ma-crossover');
    });

    // 测试获取不存在的策略定义
    it('should return undefined for invalid type', () => {
      const definition = getStrategyDefinition('non-existent-strategy');
      expect(definition).toBeUndefined();
    });
  });

  describe('getDefaultParameters', () => {
    // 测试获取MA交叉策略的默认参数
    it('should return default parameters for ma-crossover', () => {
      const defaults = getDefaultParameters('ma-crossover');
      expect(defaults).toEqual({ short: 10, long: 30 });
    });

    // 测试获取不存在策略的默认参数
    it('should return empty object for invalid strategy type', () => {
      const defaults = getDefaultParameters('invalid-type');
      expect(defaults).toEqual({});
    });

    // 测试默认参数的有效性
    it('should return valid parameters that pass validation', () => {
      const defaults = getDefaultParameters('ma-crossover');
      expect(() => MACrossoverParametersSchema.parse(defaults)).not.toThrow();
    });
  });

  describe('validateStrategyParameters', () => {
    // 测试验证有效参数
    it('should validate correct parameters', () => {
      const result = validateStrategyParameters('ma-crossover', { short: 10, long: 30 });
      expect(result.success).toBe(true);
      expect(result.errors).toBeUndefined();
    });

    // 测试验证无效参数
    it('should return errors for invalid parameters', () => {
      const result = validateStrategyParameters('ma-crossover', { short: 30, long: 10 });
      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
    });

    // 测试验证不存在的策略类型
    it('should return error for unknown strategy type', () => {
      const result = validateStrategyParameters('unknown-strategy', { param: 'value' });
      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
      expect(result.errors?._).toBe('未知的策略类型');
    });

    // 测试验证缺失参数
    it('should return errors for missing required parameters', () => {
      const result = validateStrategyParameters('ma-crossover', { short: 10 });
      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
    });

    // 测试验证错误类型的参数
    it('should return errors for wrong type parameters', () => {
      const result = validateStrategyParameters('ma-crossover', { short: 'abc', long: 30 });
      expect(result.success).toBe(false);
      expect(result.errors).toBeDefined();
    });
  });

  describe('Strategy field definitions', () => {
    // 测试MA交叉策略的字段定义
    it('should have valid field definitions for ma-crossover', () => {
      const definition = getStrategyDefinition('ma-crossover');
      expect(definition?.fields).toHaveLength(2);

      // 检查short字段
      const shortField = definition?.fields.find((f) => f.name === 'short');
      expect(shortField).toBeDefined();
      expect(shortField?.type).toBe('number');
      expect(shortField?.required).toBe(true);
      expect(shortField?.defaultValue).toBe(10);

      // 检查long字段
      const longField = definition?.fields.find((f) => f.name === 'long');
      expect(longField).toBeDefined();
      expect(longField?.type).toBe('number');
      expect(longField?.required).toBe(true);
      expect(longField?.defaultValue).toBe(30);
    });
  });
});
