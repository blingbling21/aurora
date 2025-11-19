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

import { renderHook } from '@testing-library/react';
import { useLightweightChart } from './useLightweightChart';
import { createChart } from 'lightweight-charts';

// 使用 __mocks__ 目录中的 mock
jest.mock('lightweight-charts');

describe('useLightweightChart', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  describe('Hook 基本功能', () => {
    it('应该返回 chartContainerRef', () => {
      const { result } = renderHook(() =>
        useLightweightChart('line', [], {})
      );

      expect(result.current.chartContainerRef).toBeDefined();
      expect(result.current.chartContainerRef.current).toBeNull(); // 初始时未挂载
    });
  });

  describe('边界情况', () => {
    it('应该处理空数据', () => {
      const { result } = renderHook(() =>
        useLightweightChart('line', [], {})
      );

      expect(result.current.chartContainerRef).toBeDefined();
    });

    it('应该在容器为 null 时不创建图表', () => {
      renderHook(() => useLightweightChart('line', [], {}));

      // 由于 chartContainerRef.current 为 null，createChart 不应被调用
      expect(createChart).not.toHaveBeenCalled();
    });

    it('应该支持线图类型', () => {
      const { result } = renderHook(() =>
        useLightweightChart('line', [], {})
      );

      expect(result.current).toHaveProperty('chartContainerRef');
    });

    it('应该支持区域图类型', () => {
      const { result } = renderHook(() =>
        useLightweightChart('area', [], {})
      );

      expect(result.current).toHaveProperty('chartContainerRef');
    });
  });
});
