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
  NAV_MENU_ITEMS,
  EXCHANGE_OPTIONS,
  INTERVAL_OPTIONS,
  SYMBOL_OPTIONS,
  STRATEGY_OPTIONS,
  PRICING_MODE_OPTIONS,
  API_BASE_URL,
} from './index';

describe('constants', () => {
  // 测试导航菜单项
  describe('NAV_MENU_ITEMS', () => {
    it('应该包含所有必需的导航项', () => {
      expect(NAV_MENU_ITEMS).toHaveLength(5);
      
      // 验证每个菜单项都有必需的字段
      NAV_MENU_ITEMS.forEach(item => {
        expect(item).toHaveProperty('id');
        expect(item).toHaveProperty('label');
        expect(item).toHaveProperty('icon');
        expect(item).toHaveProperty('href');
        expect(typeof item.id).toBe('string');
        expect(typeof item.label).toBe('string');
        expect(typeof item.icon).toBe('string');
        expect(typeof item.href).toBe('string');
      });
    });

    it('应该包含仪表盘菜单', () => {
      const dashboard = NAV_MENU_ITEMS.find(item => item.id === 'dashboard');
      expect(dashboard).toBeDefined();
      expect(dashboard?.href).toBe('/');
    });

    it('应该包含配置管理菜单', () => {
      const config = NAV_MENU_ITEMS.find(item => item.id === 'config');
      expect(config).toBeDefined();
      expect(config?.href).toBe('/config');
    });

    it('应该包含数据管理菜单', () => {
      const data = NAV_MENU_ITEMS.find(item => item.id === 'data');
      expect(data).toBeDefined();
      expect(data?.href).toBe('/data');
    });

    it('应该包含回测执行菜单', () => {
      const backtest = NAV_MENU_ITEMS.find(item => item.id === 'backtest');
      expect(backtest).toBeDefined();
      expect(backtest?.href).toBe('/backtest');
    });

    it('应该包含历史记录菜单', () => {
      const history = NAV_MENU_ITEMS.find(item => item.id === 'history');
      expect(history).toBeDefined();
      expect(history?.href).toBe('/history');
    });

    it('所有菜单ID应该是唯一的', () => {
      const ids = NAV_MENU_ITEMS.map(item => item.id);
      const uniqueIds = new Set(ids);
      expect(uniqueIds.size).toBe(ids.length);
    });
  });

  // 测试交易所选项
  describe('EXCHANGE_OPTIONS', () => {
    it('应该包含常用交易所', () => {
      expect(EXCHANGE_OPTIONS).toHaveLength(4);
      
      EXCHANGE_OPTIONS.forEach(option => {
        expect(option).toHaveProperty('value');
        expect(option).toHaveProperty('label');
        expect(typeof option.value).toBe('string');
        expect(typeof option.label).toBe('string');
      });
    });

    it('应该包含 Binance', () => {
      const binance = EXCHANGE_OPTIONS.find(opt => opt.value === 'binance');
      expect(binance).toBeDefined();
      expect(binance?.label).toBe('Binance');
    });

    it('应该包含 OKX', () => {
      const okx = EXCHANGE_OPTIONS.find(opt => opt.value === 'okx');
      expect(okx).toBeDefined();
    });

    it('所有value应该是小写', () => {
      EXCHANGE_OPTIONS.forEach(option => {
        expect(option.value).toBe(option.value.toLowerCase());
      });
    });
  });

  // 测试时间周期选项
  describe('INTERVAL_OPTIONS', () => {
    it('应该包含多种时间周期', () => {
      expect(INTERVAL_OPTIONS.length).toBeGreaterThan(0);
      
      INTERVAL_OPTIONS.forEach(option => {
        expect(option).toHaveProperty('value');
        expect(option).toHaveProperty('label');
      });
    });

    it('应该包含常用的时间周期', () => {
      const intervals = ['1m', '5m', '15m', '30m', '1h', '4h', '1d', '1w'];
      intervals.forEach(interval => {
        const option = INTERVAL_OPTIONS.find(opt => opt.value === interval);
        expect(option).toBeDefined();
      });
    });

    it('所有时间周期值都应该符合格式', () => {
      INTERVAL_OPTIONS.forEach(option => {
        // 应该是数字+单位的格式
        expect(option.value).toMatch(/^\d+[mhdw]$/);
      });
    });
  });

  // 测试交易对选项
  describe('SYMBOL_OPTIONS', () => {
    it('应该包含主流交易对', () => {
      expect(SYMBOL_OPTIONS.length).toBeGreaterThan(0);
      
      SYMBOL_OPTIONS.forEach(option => {
        expect(option).toHaveProperty('value');
        expect(option).toHaveProperty('label');
      });
    });

    it('应该包含 BTCUSDT', () => {
      const btc = SYMBOL_OPTIONS.find(opt => opt.value === 'BTCUSDT');
      expect(btc).toBeDefined();
      expect(btc?.label).toContain('比特币');
    });

    it('应该包含 ETHUSDT', () => {
      const eth = SYMBOL_OPTIONS.find(opt => opt.value === 'ETHUSDT');
      expect(eth).toBeDefined();
      expect(eth?.label).toContain('以太坊');
    });

    it('所有交易对值都应该是大写', () => {
      SYMBOL_OPTIONS.forEach(option => {
        expect(option.value).toBe(option.value.toUpperCase());
      });
    });

    it('所有交易对值都应该以USDT结尾', () => {
      SYMBOL_OPTIONS.forEach(option => {
        expect(option.value).toMatch(/USDT$/);
      });
    });
  });

  // 测试策略类型选项
  describe('STRATEGY_OPTIONS', () => {
    it('应该包含多种策略类型', () => {
      expect(STRATEGY_OPTIONS.length).toBeGreaterThan(0);
      
      STRATEGY_OPTIONS.forEach(option => {
        expect(option).toHaveProperty('value');
        expect(option).toHaveProperty('label');
      });
    });

    it('应该包含常见的技术指标策略', () => {
      const strategies = ['ma-crossover', 'rsi', 'macd', 'bollinger'];
      strategies.forEach(strategy => {
        const option = STRATEGY_OPTIONS.find(opt => opt.value === strategy);
        expect(option).toBeDefined();
      });
    });

    it('应该包含自定义策略选项', () => {
      const custom = STRATEGY_OPTIONS.find(opt => opt.value === 'custom');
      expect(custom).toBeDefined();
      expect(custom?.label).toBe('自定义');
    });
  });

  // 测试定价模式选项
  describe('PRICING_MODE_OPTIONS', () => {
    it('应该包含多种定价模式', () => {
      expect(PRICING_MODE_OPTIONS.length).toBeGreaterThan(0);
      
      PRICING_MODE_OPTIONS.forEach(option => {
        expect(option).toHaveProperty('value');
        expect(option).toHaveProperty('label');
      });
    });

    it('应该包含基本的价格类型', () => {
      const modes = ['open', 'high', 'low', 'close'];
      modes.forEach(mode => {
        const option = PRICING_MODE_OPTIONS.find(opt => opt.value === mode);
        expect(option).toBeDefined();
      });
    });

    it('应该包含高级定价模式', () => {
      const vwap = PRICING_MODE_OPTIONS.find(opt => opt.value === 'vwap');
      expect(vwap).toBeDefined();
      
      const bidask = PRICING_MODE_OPTIONS.find(opt => opt.value === 'bidask');
      expect(bidask).toBeDefined();
    });
  });

  // 测试 API 基础路径
  describe('API_BASE_URL', () => {
    it('应该定义API基础路径', () => {
      expect(API_BASE_URL).toBeDefined();
      expect(typeof API_BASE_URL).toBe('string');
    });

    it('应该以斜杠开头', () => {
      expect(API_BASE_URL).toMatch(/^\//);
    });

    it('应该不以斜杠结尾', () => {
      expect(API_BASE_URL).not.toMatch(/\/$/);
    });
  });

  // 通用测试：确保所有选项数组不为空
  describe('通用验证', () => {
    it('所有选项数组都不应该为空', () => {
      expect(NAV_MENU_ITEMS.length).toBeGreaterThan(0);
      expect(EXCHANGE_OPTIONS.length).toBeGreaterThan(0);
      expect(INTERVAL_OPTIONS.length).toBeGreaterThan(0);
      expect(SYMBOL_OPTIONS.length).toBeGreaterThan(0);
      expect(STRATEGY_OPTIONS.length).toBeGreaterThan(0);
      expect(PRICING_MODE_OPTIONS.length).toBeGreaterThan(0);
    });

    it('所有选项都应该有value和label字段', () => {
      const allOptions = [
        ...EXCHANGE_OPTIONS,
        ...INTERVAL_OPTIONS,
        ...SYMBOL_OPTIONS,
        ...STRATEGY_OPTIONS,
        ...PRICING_MODE_OPTIONS,
      ];

      allOptions.forEach(option => {
        expect(option.value).toBeTruthy();
        expect(option.label).toBeTruthy();
      });
    });
  });
});
