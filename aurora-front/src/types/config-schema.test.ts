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
 * 配置Schema测试
 */

import {
  DataSourceConfigSchema,
  StrategyConfigSchema,
  PortfolioConfigSchema,
  LoggingConfigSchema,
  AuroraConfigSchema,
  createDefaultAuroraConfig,
  createDefaultDataSourceConfig,
  createDefaultStrategyConfig,
  createDefaultPortfolioConfig,
  createDefaultLoggingConfig,
} from './config-schema';

describe('Config Schema', () => {
  describe('DataSourceConfigSchema', () => {
    it('应该验证通过有效的数据源配置', () => {
      const config = {
        provider: 'binance' as const,
        timeout: 30,
        max_retries: 3,
      };

      const result = DataSourceConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该拒绝无效的provider', () => {
      const config = {
        provider: 'invalid',
        timeout: 30,
        max_retries: 3,
      };

      const result = DataSourceConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });

    it('应该使用默认值', () => {
      const config = {
        provider: 'binance' as const,
      };

      const result = DataSourceConfigSchema.parse(config);
      expect(result.timeout).toBe(30);
      expect(result.max_retries).toBe(3);
    });
  });

  describe('StrategyConfigSchema', () => {
    it('应该验证通过有效的策略配置', () => {
      const config = {
        name: 'MA交叉',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: {
          short: 10,
          long: 30,
        },
      };

      const result = StrategyConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该拒绝空的策略名称', () => {
      const config = {
        name: '',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: {},
      };

      const result = StrategyConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });
  });

  describe('PortfolioConfigSchema', () => {
    it('应该验证通过有效的投资组合配置', () => {
      const config = {
        initial_cash: 10000,
        commission: 0.001,
        slippage: 0,
      };

      const result = PortfolioConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该拒绝负数的初始资金', () => {
      const config = {
        initial_cash: -100,
        commission: 0.001,
        slippage: 0,
      };

      const result = PortfolioConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });

    it('应该拒绝超出范围的手续费率', () => {
      const config = {
        initial_cash: 10000,
        commission: 1.5,
        slippage: 0,
      };

      const result = PortfolioConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });
  });

  describe('LoggingConfigSchema', () => {
    it('应该验证通过有效的日志配置', () => {
      const config = {
        level: 'info' as const,
        format: 'pretty' as const,
      };

      const result = LoggingConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该拒绝无效的日志级别', () => {
      const config = {
        level: 'invalid',
        format: 'pretty',
      };

      const result = LoggingConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });
  });

  describe('AuroraConfigSchema', () => {
    it('应该验证通过完整的配置', () => {
      const config = createDefaultAuroraConfig();

      const result = AuroraConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该拒绝缺少必需字段的配置', () => {
      const config = {
        data_source: createDefaultDataSourceConfig(),
        // 缺少 strategies
        portfolio: createDefaultPortfolioConfig(),
        logging: createDefaultLoggingConfig(),
      };

      const result = AuroraConfigSchema.safeParse(config);
      expect(result.success).toBe(false);
    });

    it('应该接受带有回测配置的完整配置', () => {
      const config = {
        ...createDefaultAuroraConfig(),
        backtest: {
          data_path: 'data/btc_1h.csv',
          symbol: 'BTCUSDT',
          interval: '1h',
        },
      };

      const result = AuroraConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('应该接受带有实时交易配置的完整配置', () => {
      const config = {
        ...createDefaultAuroraConfig(),
        live: {
          symbol: 'BTCUSDT',
          interval: '1m',
          paper_trading: true,
        },
      };

      const result = AuroraConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });
  });

  describe('Default Config Functions', () => {
    it('createDefaultDataSourceConfig应该创建有效配置', () => {
      const config = createDefaultDataSourceConfig();
      
      const result = DataSourceConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('createDefaultStrategyConfig应该创建有效配置', () => {
      const config = createDefaultStrategyConfig();
      
      const result = StrategyConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('createDefaultPortfolioConfig应该创建有效配置', () => {
      const config = createDefaultPortfolioConfig();
      
      const result = PortfolioConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('createDefaultLoggingConfig应该创建有效配置', () => {
      const config = createDefaultLoggingConfig();
      
      const result = LoggingConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });

    it('createDefaultAuroraConfig应该创建有效配置', () => {
      const config = createDefaultAuroraConfig();
      
      const result = AuroraConfigSchema.safeParse(config);
      expect(result.success).toBe(true);
    });
  });
});
