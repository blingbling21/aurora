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

import * as apiIndex from './index';
import { configApi } from './config';
import { dataApi } from './data';
import { backtestApi } from './backtest';

describe('API Index Module', () => {
  // 测试统一 API 对象导出
  describe('api 对象', () => {
    // 测试 api 对象存在
    it('应该导出 api 对象', () => {
      expect(apiIndex.api).toBeDefined();
      expect(typeof apiIndex.api).toBe('object');
    });

    // 测试 api.config 存在
    it('应该包含 config 服务', () => {
      expect(apiIndex.api.config).toBeDefined();
      expect(apiIndex.api.config).toBe(configApi);
    });

    // 测试 api.data 存在
    it('应该包含 data 服务', () => {
      expect(apiIndex.api.data).toBeDefined();
      expect(apiIndex.api.data).toBe(dataApi);
    });

    // 测试 api.backtest 存在
    it('应该包含 backtest 服务', () => {
      expect(apiIndex.api.backtest).toBeDefined();
      expect(apiIndex.api.backtest).toBe(backtestApi);
    });

    // 测试 api 对象结构完整性
    it('api 对象应该包含所有必要的服务', () => {
      const services = Object.keys(apiIndex.api);
      
      expect(services).toContain('config');
      expect(services).toContain('data');
      expect(services).toContain('backtest');
    });
  });

  // 测试各个服务模块的导出
  describe('服务模块导出', () => {
    // 测试 configApi 导出
    it('应该导出 configApi', () => {
      expect(apiIndex.configApi).toBeDefined();
      expect(apiIndex.configApi).toBe(configApi);
    });

    // 测试 dataApi 导出
    it('应该导出 dataApi', () => {
      expect(apiIndex.dataApi).toBeDefined();
      expect(apiIndex.dataApi).toBe(dataApi);
    });

    // 测试 backtestApi 导出
    it('应该导出 backtestApi', () => {
      expect(apiIndex.backtestApi).toBeDefined();
      expect(apiIndex.backtestApi).toBe(backtestApi);
    });
  });

  // 测试客户端基础设施导出
  describe('客户端基础设施', () => {
    // 测试是否导出了客户端相关的内容
    it('应该从 client 模块导出内容', () => {
      // 检查是否有 client 模块的导出
      const exports = Object.keys(apiIndex);
      
      // 至少应该包含 api 对象和各个服务
      expect(exports).toContain('api');
      expect(exports).toContain('configApi');
      expect(exports).toContain('dataApi');
      expect(exports).toContain('backtestApi');
    });
  });

  // 测试模块完整性
  describe('模块完整性', () => {
    // 测试所有导出都已定义
    it('所有导出都不应该是 undefined', () => {
      const exports = Object.values(apiIndex);
      
      exports.forEach(exportedItem => {
        expect(exportedItem).toBeDefined();
        expect(exportedItem).not.toBeNull();
      });
    });

    // 测试 api 对象的所有服务都可用
    it('api 对象的所有服务都应该可用', () => {
      expect(apiIndex.api.config).toBeDefined();
      expect(apiIndex.api.data).toBeDefined();
      expect(apiIndex.api.backtest).toBeDefined();
      
      // 验证服务是对象
      expect(typeof apiIndex.api.config).toBe('object');
      expect(typeof apiIndex.api.data).toBe('object');
      expect(typeof apiIndex.api.backtest).toBe('object');
    });
  });

  // 测试 API 方法可用性
  describe('API 方法可用性', () => {
    // 测试 config API 方法
    it('config API 应该有必要的方法', () => {
      expect(apiIndex.api.config).toHaveProperty('list');
      expect(typeof apiIndex.api.config.list).toBe('function');
    });

    // 测试 data API 方法
    it('data API 应该有必要的方法', () => {
      expect(apiIndex.api.data).toHaveProperty('list');
      expect(typeof apiIndex.api.data.list).toBe('function');
    });

    // 测试 backtest API 方法
    it('backtest API 应该有必要的方法', () => {
      expect(apiIndex.api.backtest).toHaveProperty('start');
      expect(typeof apiIndex.api.backtest.start).toBe('function');
    });
  });

  // 测试重新导出的一致性
  describe('重新导出一致性', () => {
    // 测试独立导出与 api 对象中的引用一致
    it('configApi 应该与 api.config 相同', () => {
      expect(apiIndex.configApi).toBe(apiIndex.api.config);
    });

    it('dataApi 应该与 api.data 相同', () => {
      expect(apiIndex.dataApi).toBe(apiIndex.api.data);
    });

    it('backtestApi 应该与 api.backtest 相同', () => {
      expect(apiIndex.backtestApi).toBe(apiIndex.api.backtest);
    });
  });

  // 测试使用场景
  describe('使用场景测试', () => {
    // 测试可以通过 api 对象访问服务
    it('应该能够通过 api 对象访问所有服务', () => {
      const { api } = apiIndex;
      
      // 验证可以访问
      expect(api.config).toBeDefined();
      expect(api.data).toBeDefined();
      expect(api.backtest).toBeDefined();
    });

    // 测试可以直接导入单个服务
    it('应该能够直接导入单个服务使用', () => {
      const { configApi, dataApi, backtestApi } = apiIndex;
      
      // 验证都可用
      expect(configApi).toBeDefined();
      expect(dataApi).toBeDefined();
      expect(backtestApi).toBeDefined();
    });
  });
});
