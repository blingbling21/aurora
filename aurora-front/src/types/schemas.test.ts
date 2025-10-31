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
  BacktestTaskSchema,
  ConfigFileSchema,
  DataFileSchema,
  BacktestResultSchema,
  TradeSchema,
  NotificationSchema,
  DataDownloadRequestSchema,
  BacktestConfigSchema,
} from './schemas';

describe('Zod Schemas', () => {
  // 测试BacktestTaskSchema
  describe('BacktestTaskSchema', () => {
    it('should validate a valid backtest task', () => {
      const validTask = {
        id: '1',
        name: 'Test Task',
        status: 'pending',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 50,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };

      const result = BacktestTaskSchema.safeParse(validTask);
      expect(result.success).toBe(true);
    });

    it('should reject invalid task status', () => {
      const invalidTask = {
        id: '1',
        name: 'Test Task',
        status: 'invalid',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 50,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };

      const result = BacktestTaskSchema.safeParse(invalidTask);
      expect(result.success).toBe(false);
    });

    it('should reject progress out of range', () => {
      const invalidTask = {
        id: '1',
        name: 'Test Task',
        status: 'pending',
        config: 'config.toml',
        dataFile: 'data.csv',
        progress: 150,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };

      const result = BacktestTaskSchema.safeParse(invalidTask);
      expect(result.success).toBe(false);
    });
  });

  // 测试ConfigFileSchema
  describe('ConfigFileSchema', () => {
    it('should validate a valid config file', () => {
      const validConfig = {
        name: 'config.toml',
        path: '/configs/config.toml',
        content: '[strategy]\nname = "test"',
        lastModified: new Date().toISOString(),
      };

      const result = ConfigFileSchema.safeParse(validConfig);
      expect(result.success).toBe(true);
    });

    it('should reject empty name', () => {
      const invalidConfig = {
        name: '',
        path: '/configs/config.toml',
        content: '[strategy]\nname = "test"',
        lastModified: new Date().toISOString(),
      };

      const result = ConfigFileSchema.safeParse(invalidConfig);
      expect(result.success).toBe(false);
    });
  });

  // 测试DataFileSchema
  describe('DataFileSchema', () => {
    it('should validate a valid data file', () => {
      const validDataFile = {
        name: 'data.csv',
        path: '/data/data.csv',
        size: 1024,
        lastModified: new Date().toISOString(),
      };

      const result = DataFileSchema.safeParse(validDataFile);
      expect(result.success).toBe(true);
    });

    it('should reject negative file size', () => {
      const invalidDataFile = {
        name: 'data.csv',
        path: '/data/data.csv',
        size: -100,
        lastModified: new Date().toISOString(),
      };

      const result = DataFileSchema.safeParse(invalidDataFile);
      expect(result.success).toBe(false);
    });
  });

  // 测试TradeSchema
  describe('TradeSchema', () => {
    it('should validate a valid trade', () => {
      const validTrade = {
        id: '1',
        type: 'buy',
        symbol: 'BTC/USDT',
        price: 50000,
        quantity: 0.1,
        time: new Date().toISOString(),
        profit: 100,
      };

      const result = TradeSchema.safeParse(validTrade);
      expect(result.success).toBe(true);
    });

    it('should reject invalid trade type', () => {
      const invalidTrade = {
        id: '1',
        type: 'hold',
        symbol: 'BTC/USDT',
        price: 50000,
        quantity: 0.1,
        time: new Date().toISOString(),
      };

      const result = TradeSchema.safeParse(invalidTrade);
      expect(result.success).toBe(false);
    });

    it('should reject negative price', () => {
      const invalidTrade = {
        id: '1',
        type: 'buy',
        symbol: 'BTC/USDT',
        price: -50000,
        quantity: 0.1,
        time: new Date().toISOString(),
      };

      const result = TradeSchema.safeParse(invalidTrade);
      expect(result.success).toBe(false);
    });
  });

  // 测试NotificationSchema
  describe('NotificationSchema', () => {
    it('should validate a valid notification', () => {
      const validNotification = {
        id: '1',
        type: 'success',
        message: 'Operation successful',
        duration: 3000,
      };

      const result = NotificationSchema.safeParse(validNotification);
      expect(result.success).toBe(true);
    });

    it('should accept notification without duration', () => {
      const validNotification = {
        id: '1',
        type: 'info',
        message: 'Information message',
      };

      const result = NotificationSchema.safeParse(validNotification);
      expect(result.success).toBe(true);
    });
  });

  // 测试DataDownloadRequestSchema
  describe('DataDownloadRequestSchema', () => {
    it('should validate a valid download request', () => {
      const startDate = new Date('2024-01-01').toISOString();
      const endDate = new Date('2024-12-31').toISOString();

      const validRequest = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        startDate,
        endDate,
      };

      const result = DataDownloadRequestSchema.safeParse(validRequest);
      expect(result.success).toBe(true);
    });

    it('should reject when end date is before start date', () => {
      const startDate = new Date('2024-12-31').toISOString();
      const endDate = new Date('2024-01-01').toISOString();

      const invalidRequest = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        startDate,
        endDate,
      };

      const result = DataDownloadRequestSchema.safeParse(invalidRequest);
      expect(result.success).toBe(false);
    });
  });

  // 测试BacktestConfigSchema
  describe('BacktestConfigSchema', () => {
    it('should validate a valid backtest config', () => {
      const validConfig = {
        taskName: 'My Backtest',
        configFile: 'config.toml',
        dataFile: 'data.csv',
        description: 'Test backtest',
      };

      const result = BacktestConfigSchema.safeParse(validConfig);
      expect(result.success).toBe(true);
    });

    it('should reject task name exceeding 100 characters', () => {
      const invalidConfig = {
        taskName: 'a'.repeat(101),
        configFile: 'config.toml',
        dataFile: 'data.csv',
      };

      const result = BacktestConfigSchema.safeParse(invalidConfig);
      expect(result.success).toBe(false);
    });
  });

  // 测试BacktestResultSchema
  describe('BacktestResultSchema', () => {
    it('should validate a valid backtest result', () => {
      const validResult = {
        taskId: '1',
        metrics: {
          totalReturn: 0.25,
          annualizedReturn: 0.30,
          maxDrawdown: -0.15,
          maxDrawdownDuration: 10,
          sharpeRatio: 1.5,
          sortinoRatio: 2.0,
          calmarRatio: 1.2,
          annualizedVolatility: 0.20,
          winRate: 60,
          totalTrades: 100,
          winningTrades: 60,
          losingTrades: 40,
          averageWin: 200,
          averageLoss: -100,
          profitLossRatio: 2.0,
          profitFactor: 1.5,
          maxConsecutiveWins: 5,
          maxConsecutiveLosses: 3,
          avgHoldingPeriod: 24,
          maxWin: 1000,
          maxLoss: -500,
        },
        equityCurve: [
          { time: new Date().toISOString(), value: 10000 },
          { time: new Date().toISOString(), value: 11000 },
        ],
        trades: [
          {
            id: '1',
            type: 'buy',
            symbol: 'BTC/USDT',
            price: 50000,
            quantity: 0.1,
            time: new Date().toISOString(),
          },
        ],
      };

      const result = BacktestResultSchema.safeParse(validResult);
      expect(result.success).toBe(true);
    });

    it('should reject empty equity curve', () => {
      const invalidResult = {
        taskId: '1',
        metrics: {
          totalReturn: 0.25,
          annualizedReturn: 0.30,
          maxDrawdown: -0.15,
          maxDrawdownDuration: 10,
          sharpeRatio: 1.5,
          sortinoRatio: 2.0,
          calmarRatio: 1.2,
          annualizedVolatility: 0.20,
          winRate: 60,
          totalTrades: 100,
          winningTrades: 60,
          losingTrades: 40,
          averageWin: 200,
          averageLoss: -100,
          profitLossRatio: 2.0,
          profitFactor: 1.5,
          maxConsecutiveWins: 5,
          maxConsecutiveLosses: 3,
          avgHoldingPeriod: 24,
          maxWin: 1000,
          maxLoss: -500,
        },
        equityCurve: [],
        trades: [],
      };

      const result = BacktestResultSchema.safeParse(invalidResult);
      expect(result.success).toBe(false);
    });
  });
});
