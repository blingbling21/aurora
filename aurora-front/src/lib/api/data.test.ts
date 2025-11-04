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
 * 数据管理 API 服务测试
 */

import { DataService, dataApi } from './data';
import * as client from './client';
import type { DataFileItem, FetchDataRequest, Kline } from '@/types/api';

// Mock client 模块
jest.mock('./client');

describe('DataService', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('list', () => {
    it('应该成功获取数据文件列表', async () => {
      // 准备测试数据
      const mockFiles: DataFileItem[] = [
        {
          filename: 'btc_usdt_1h.csv',
          size: 1024000,
          modified: '2025-01-01T00:00:00Z',
          record_count: 1000,
        },
        {
          filename: 'eth_usdt_4h.csv',
          size: 512000,
          modified: '2025-01-02T00:00:00Z',
          record_count: 500,
        },
      ];
      const mockResponse = {
        success: true,
        data: mockFiles,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.list();

      // 验证结果
      expect(client.get).toHaveBeenCalledWith('/data/list');
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockFiles);
      expect(result.data).toHaveLength(2);
    });

    it('应该处理空列表', async () => {
      // 模拟空列表响应
      const mockResponse = {
        success: true,
        data: [],
      };
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.list();

      // 验证结果
      expect(result.data).toEqual([]);
    });

    it('应该处理 API 错误', async () => {
      // 模拟 API 错误
      const mockError = {
        success: false,
        error: 'Failed to fetch data files',
      };
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.list();

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Failed to fetch data files');
    });
  });

  describe('get', () => {
    it('应该成功获取数据文件内容', async () => {
      // 准备测试数据
      const filename = 'btc_usdt_1h.csv';
      const mockContent = 'timestamp,open,high,low,close,volume\n1,100,110,90,105,1000';
      const mockResponse = {
        success: true,
        data: mockContent,
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.get(filename);

      // 验证结果
      expect(client.get).toHaveBeenCalledWith(`/data/${filename}`);
      expect(result).toEqual(mockResponse);
      expect(result.data).toBe(mockContent);
    });

    it('应该正确编码特殊字符', async () => {
      // 准备测试数据
      const filename = 'data with spaces.csv';
      const mockResponse = {
        success: true,
        data: '',
      };

      // 模拟 API 调用
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await DataService.get(filename);

      // 验证 URL 编码
      expect(client.get).toHaveBeenCalledWith(
        `/data/${encodeURIComponent(filename)}`
      );
    });

    it('应该处理文件不存在的情况', async () => {
      // 模拟文件不存在错误
      const mockError = {
        success: false,
        error: 'Data file not found',
      };
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.get('non-existent.csv');

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Data file not found');
    });
  });

  describe('delete', () => {
    it('应该成功删除数据文件', async () => {
      // 准备测试数据
      const filename = 'to-delete.csv';
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.del as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.delete(filename);

      // 验证结果
      expect(client.del).toHaveBeenCalledWith(`/data/${filename}`);
      expect(result).toEqual(mockResponse);
      expect(result.success).toBe(true);
    });

    it('应该正确编码文件名', async () => {
      // 准备测试数据
      const filename = 'file/with/slash.csv';
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.del as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      await DataService.delete(filename);

      // 验证 URL 编码
      expect(client.del).toHaveBeenCalledWith(
        `/data/${encodeURIComponent(filename)}`
      );
    });

    it('应该处理文件不存在的情况', async () => {
      // 模拟文件不存在错误
      const mockError = {
        success: false,
        error: 'Data file not found',
      };
      (client.del as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.delete('non-existent.csv');

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('fetch', () => {
    it('应该成功获取历史数据', async () => {
      // 准备测试数据
      const request: FetchDataRequest = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        start_date: '2025-01-01',
        end_date: '2025-01-31',
      };
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.fetch(request);

      // 验证结果
      expect(client.post).toHaveBeenCalledWith('/data/fetch', request);
      expect(result).toEqual(mockResponse);
      expect(result.success).toBe(true);
    });

    it('应该处理自定义文件名', async () => {
      // 准备测试数据
      const request: FetchDataRequest = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        start_date: '2025-01-01',
        end_date: '2025-01-31',
        filename: 'custom_btc_data.csv',
      };
      const mockResponse = {
        success: true,
        data: undefined,
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.fetch(request);

      // 验证结果
      expect(client.post).toHaveBeenCalledWith('/data/fetch', request);
      expect(result.success).toBe(true);
    });

    it('应该处理无效的交易所', async () => {
      // 准备测试数据
      const request: FetchDataRequest = {
        exchange: 'invalid-exchange',
        symbol: 'BTC/USDT',
        interval: '1h',
        start_date: '2025-01-01',
        end_date: '2025-01-31',
      };
      const mockError = {
        success: false,
        error: 'Invalid exchange',
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.fetch(request);

      // 验证结果
      expect(result.success).toBe(false);
      expect(result.error).toBe('Invalid exchange');
    });

    it('应该处理日期范围错误', async () => {
      // 准备测试数据
      const request: FetchDataRequest = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        start_date: '2025-12-31',
        end_date: '2025-01-01',
      };
      const mockError = {
        success: false,
        error: 'Invalid date range',
      };

      // 模拟 API 调用
      (client.post as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.fetch(request);

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('getKlines', () => {
    it('应该成功获取 K线数据', async () => {
      // 准备测试数据
      const params = {
        filename: 'btc_usdt_1h.csv',
        start: 1704067200000,
        end: 1704153600000,
        limit: 100,
      };
      const mockKlines: Kline[] = [
        {
          timestamp: 1704067200000,
          open: 42000,
          high: 43000,
          low: 41000,
          close: 42500,
          volume: 1000,
        },
        {
          timestamp: 1704070800000,
          open: 42500,
          high: 43500,
          low: 42000,
          close: 43000,
          volume: 1200,
        },
      ];
      const mockResponse = {
        success: true,
        data: mockKlines,
      };

      // 模拟 buildQueryString 和 API 调用
      (client.buildQueryString as jest.Mock).mockReturnValue(
        '?filename=btc_usdt_1h.csv&start=1704067200000&end=1704153600000&limit=100'
      );
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.getKlines(params);

      // 验证结果
      expect(client.buildQueryString).toHaveBeenCalledWith(params);
      expect(result).toEqual(mockResponse);
      expect(result.data).toEqual(mockKlines);
      expect(result.data).toHaveLength(2);
    });

    it('应该处理只有 filename 参数的情况', async () => {
      // 准备测试数据
      const params = {
        filename: 'btc_usdt_1h.csv',
      };
      const mockResponse = {
        success: true,
        data: [],
      };

      // 模拟 API 调用
      (client.buildQueryString as jest.Mock).mockReturnValue(
        '?filename=btc_usdt_1h.csv'
      );
      (client.get as jest.Mock).mockResolvedValue(mockResponse);

      // 执行测试
      const result = await DataService.getKlines(params);

      // 验证结果
      expect(client.buildQueryString).toHaveBeenCalledWith(params);
      expect(result.success).toBe(true);
    });

    it('应该处理数据文件不存在', async () => {
      // 准备测试数据
      const params = {
        filename: 'non-existent.csv',
      };
      const mockError = {
        success: false,
        error: 'Data file not found',
      };

      // 模拟 API 调用
      (client.buildQueryString as jest.Mock).mockReturnValue(
        '?filename=non-existent.csv'
      );
      (client.get as jest.Mock).mockResolvedValue(mockError);

      // 执行测试
      const result = await DataService.getKlines(params);

      // 验证结果
      expect(result.success).toBe(false);
    });
  });

  describe('generateFilename', () => {
    it('应该生成标准格式的文件名', () => {
      // 准备测试数据
      const params = {
        exchange: 'Binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      };

      // 执行测试
      const filename = DataService.generateFilename(params);

      // 验证结果
      expect(filename).toBe(
        'binance_btc/usdt_1h_20250101_to_20250131.csv'
      );
    });

    it('应该将交易所名称转换为小写', () => {
      // 准备测试数据
      const params = {
        exchange: 'BINANCE',
        symbol: 'BTC/USDT',
        interval: '1h',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      };

      // 执行测试
      const filename = DataService.generateFilename(params);

      // 验证结果
      expect(filename).toContain('binance_');
    });

    it('应该将交易对转换为小写', () => {
      // 准备测试数据
      const params = {
        exchange: 'binance',
        symbol: 'BTC/USDT',
        interval: '1h',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      };

      // 执行测试
      const filename = DataService.generateFilename(params);

      // 验证结果
      expect(filename).toContain('btc/usdt');
    });

    it('应该移除日期中的连字符', () => {
      // 准备测试数据
      const params = {
        exchange: 'binance',
        symbol: 'btc/usdt',
        interval: '1h',
        startDate: '2025-01-01',
        endDate: '2025-12-31',
      };

      // 执行测试
      const filename = DataService.generateFilename(params);

      // 验证结果
      expect(filename).toContain('20250101_to_20251231');
      expect(filename).not.toMatch(/2025-\d{2}-\d{2}/);
    });

    it('应该正确处理不同的时间间隔', () => {
      // 准备测试数据
      const params1 = {
        exchange: 'binance',
        symbol: 'btc/usdt',
        interval: '4h',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      };
      const params2 = {
        exchange: 'binance',
        symbol: 'btc/usdt',
        interval: '1d',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      };

      // 执行测试
      const filename1 = DataService.generateFilename(params1);
      const filename2 = DataService.generateFilename(params2);

      // 验证结果
      expect(filename1).toContain('_4h_');
      expect(filename2).toContain('_1d_');
    });
  });
});

describe('dataApi', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('应该暴露所有 DataService 方法', () => {
    // 验证所有方法都已导出
    expect(dataApi.list).toBeDefined();
    expect(dataApi.get).toBeDefined();
    expect(dataApi.delete).toBeDefined();
    expect(dataApi.fetch).toBeDefined();
    expect(dataApi.getKlines).toBeDefined();
    expect(dataApi.generateFilename).toBeDefined();
  });

  it('list 方法应该调用 DataService.list', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: [] };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    await dataApi.list();

    // 验证调用
    expect(client.get).toHaveBeenCalledWith('/data/list');
  });

  it('get 方法应该调用 DataService.get', async () => {
    // 模拟方法
    const mockResponse = { success: true, data: '' };
    (client.get as jest.Mock).mockResolvedValue(mockResponse);

    // 执行测试
    const filename = 'test.csv';
    await dataApi.get(filename);

    // 验证调用
    expect(client.get).toHaveBeenCalledWith(`/data/${filename}`);
  });

  it('generateFilename 方法应该调用 DataService.generateFilename', () => {
    // 准备测试数据
    const params = {
      exchange: 'binance',
      symbol: 'btc/usdt',
      interval: '1h',
      startDate: '2025-01-01',
      endDate: '2025-01-31',
    };

    // 执行测试
    const filename = dataApi.generateFilename(params);

    // 验证结果
    expect(filename).toBe('binance_btc/usdt_1h_20250101_to_20250131.csv');
  });
});
