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

import { renderHook, act } from '@testing-library/react';
import { useBacktestResultStore } from './backtestResultStore';
import { BacktestResult } from '@/types/schemas';

describe('useBacktestResultStore', () => {
  // 在每个测试之前重置store
  beforeEach(() => {
    const { result } = renderHook(() => useBacktestResultStore());
    act(() => {
      result.current.clearResults();
      result.current.setError(null);
      result.current.setLoading(false);
      result.current.setCurrentResult(null);
    });
  });

  // 测试初始状态
  describe('初始状态', () => {
    it('应该有正确的初始值', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      
      expect(result.current.results.size).toBe(0);
      expect(result.current.currentResultId).toBeNull();
      expect(result.current.isLoading).toBe(false);
      expect(result.current.error).toBeNull();
    });
  });

  // 创建模拟指标数据的辅助函数
  const createMockMetrics = (overrides = {}) => ({
    totalReturn: 15.5,
    annualizedReturn: 18.2,
    maxDrawdown: -8.2,
    maxDrawdownDuration: 5,
    sharpeRatio: 1.8,
    sortinoRatio: 2.1,
    calmarRatio: 2.2,
    annualizedVolatility: 12.5,
    winRate: 65,
    totalTrades: 100,
    winningTrades: 65,
    losingTrades: 35,
    averageWin: 150,
    averageLoss: -80,
    profitLossRatio: 1.875,
    profitFactor: 2.1,
    maxConsecutiveWins: 8,
    maxConsecutiveLosses: 4,
    avgHoldingPeriod: 24,
    maxWin: 500,
    maxLoss: -250,
    ...overrides,
  });

  // 测试 setResult
  describe('setResult', () => {
    it('应该能够添加新结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult);
      });

      expect(result.current.results.size).toBe(1);
      expect(result.current.results.get('task-1')).toEqual(mockResult);
      expect(result.current.error).toBeNull();
    });

    it('应该能够更新已存在的结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult1: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      const mockResult2: BacktestResult = {
        ...mockResult1,
        metrics: createMockMetrics({ totalReturn: 20.0 }),
      };

      act(() => {
        result.current.setResult('task-1', mockResult1);
        result.current.setResult('task-1', mockResult2);
      });

      expect(result.current.results.size).toBe(1);
      expect(result.current.results.get('task-1')?.metrics.totalReturn).toBe(20.0);
    });
  });

  // 测试 getResult
  describe('getResult', () => {
    it('应该能够获取存在的结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult);
      });

      const retrievedResult = result.current.getResult('task-1');
      expect(retrievedResult).toEqual(mockResult);
    });

    it('应该在结果不存在时返回 undefined', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      
      const retrievedResult = result.current.getResult('non-existent');
      expect(retrievedResult).toBeUndefined();
    });
  });

  // 测试 deleteResult
  describe('deleteResult', () => {
    it('应该能够删除指定的结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult);
        result.current.deleteResult('task-1');
      });

      expect(result.current.results.size).toBe(0);
      expect(result.current.getResult('task-1')).toBeUndefined();
    });

    it('删除当前查看的结果时应该清空 currentResultId', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult);
        result.current.setCurrentResult('task-1');
        result.current.deleteResult('task-1');
      });

      expect(result.current.currentResultId).toBeNull();
    });

    it('删除其他结果时不应该影响 currentResultId', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult1: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      const mockResult2: BacktestResult = {
        taskId: 'task-2',
        metrics: createMockMetrics({ totalReturn: 20.0 }),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult1);
        result.current.setResult('task-2', mockResult2);
        result.current.setCurrentResult('task-1');
        result.current.deleteResult('task-2');
      });

      expect(result.current.currentResultId).toBe('task-1');
    });
  });

  // 测试 setCurrentResult 和 getCurrentResult
  describe('setCurrentResult 和 getCurrentResult', () => {
    it('应该能够设置和获取当前结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult);
        result.current.setCurrentResult('task-1');
      });

      expect(result.current.currentResultId).toBe('task-1');
      expect(result.current.getCurrentResult()).toEqual(mockResult);
    });

    it('当没有设置当前结果时应该返回 null', () => {
      const { result } = renderHook(() => useBacktestResultStore());

      expect(result.current.getCurrentResult()).toBeNull();
    });

    it('当设置的结果不存在时应该返回 null', () => {
      const { result } = renderHook(() => useBacktestResultStore());

      act(() => {
        result.current.setCurrentResult('non-existent');
      });

      expect(result.current.getCurrentResult()).toBeNull();
    });
  });

  // 测试 setLoading
  describe('setLoading', () => {
    it('应该能够设置加载状态', () => {
      const { result } = renderHook(() => useBacktestResultStore());

      act(() => {
        result.current.setLoading(true);
      });

      expect(result.current.isLoading).toBe(true);

      act(() => {
        result.current.setLoading(false);
      });

      expect(result.current.isLoading).toBe(false);
    });
  });

  // 测试 setError
  describe('setError', () => {
    it('应该能够设置错误信息', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const errorMessage = '加载失败';

      act(() => {
        result.current.setError(errorMessage);
      });

      expect(result.current.error).toBe(errorMessage);
    });

    it('应该能够清空错误信息', () => {
      const { result } = renderHook(() => useBacktestResultStore());

      act(() => {
        result.current.setError('错误');
        result.current.setError(null);
      });

      expect(result.current.error).toBeNull();
    });
  });

  // 测试 clearResults
  describe('clearResults', () => {
    it('应该能够清空所有结果', () => {
      const { result } = renderHook(() => useBacktestResultStore());
      const mockResult1: BacktestResult = {
        taskId: 'task-1',
        metrics: createMockMetrics(),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      const mockResult2: BacktestResult = {
        taskId: 'task-2',
        metrics: createMockMetrics({ totalReturn: 20.0 }),
        equityCurve: [{ time: '2024-01-01T00:00:00Z', value: 10000 }],
        trades: [],
      };

      act(() => {
        result.current.setResult('task-1', mockResult1);
        result.current.setResult('task-2', mockResult2);
        result.current.clearResults();
      });

      expect(result.current.results.size).toBe(0);
    });
  });
});
