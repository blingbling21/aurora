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

import { downsampleData, smartSample, uniformSample } from './dataSampling';

describe('dataSampling', () => {
  describe('downsampleData', () => {
    it('当数据量小于目标点数时，应返回原数据', () => {
      const data = [
        { time: 1, value: 10 },
        { time: 2, value: 20 },
        { time: 3, value: 30 },
      ];
      const result = downsampleData(data, 5);
      expect(result).toEqual(data);
    });

    it('当数据量等于目标点数时，应返回原数据', () => {
      const data = [
        { time: 1, value: 10 },
        { time: 2, value: 20 },
        { time: 3, value: 30 },
      ];
      const result = downsampleData(data, 3);
      expect(result).toEqual(data);
    });

    it('应正确采样大数据集', () => {
      const data = Array.from({ length: 1000 }, (_, i) => ({
        time: i,
        value: Math.sin(i / 10) * 100,
      }));
      const result = downsampleData(data, 100);
      
      // 采样后的数据点数应为目标点数
      expect(result.length).toBe(100);
      
      // 应保留第一个和最后一个点
      expect(result[0]).toEqual(data[0]);
      expect(result[result.length - 1]).toEqual(data[data.length - 1]);
    });

    it('应处理空数组', () => {
      const result = downsampleData([], 10);
      expect(result).toEqual([]);
    });

    it('应保持数据的时间顺序', () => {
      const data = Array.from({ length: 100 }, (_, i) => ({
        time: i,
        value: i * 2,
      }));
      const result = downsampleData(data, 20);
      
      // 检查时间是否递增
      for (let i = 1; i < result.length; i++) {
        expect(result[i].time).toBeGreaterThan(result[i - 1].time);
      }
    });

    it('应处理不同类型的数据对象', () => {
      const data = Array.from({ length: 50 }, (_, i) => ({
        timestamp: i * 1000,
        equity: 10000 + i * 100,
        drawdown: -i * 0.01,
      }));
      const result = downsampleData(data, 10);
      
      expect(result.length).toBe(10);
      expect(result[0]).toHaveProperty('timestamp');
      expect(result[0]).toHaveProperty('equity');
      expect(result[0]).toHaveProperty('drawdown');
    });
  });

  describe('smartSample', () => {
    it('当数据量小于最大点数时，应返回原数据', () => {
      const data = [
        { time: 1, value: 10 },
        { time: 2, value: 20 },
        { time: 3, value: 30 },
      ];
      const result = smartSample(data, 10);
      expect(result).toEqual(data);
    });

    it('当数据量超过最大点数时，应进行采样', () => {
      const data = Array.from({ length: 2000 }, (_, i) => ({
        time: i,
        value: i,
      }));
      const result = smartSample(data, 500);
      
      expect(result.length).toBe(500);
      expect(result[0]).toEqual(data[0]);
      expect(result[result.length - 1]).toEqual(data[data.length - 1]);
    });

    it('应使用默认的最大点数 1000', () => {
      const data = Array.from({ length: 5000 }, (_, i) => ({
        time: i,
        value: i,
      }));
      const result = smartSample(data);
      
      expect(result.length).toBe(1000);
    });

    it('应处理空数组', () => {
      const result = smartSample([]);
      expect(result).toEqual([]);
    });

    it('应正确处理边界情况：数据量刚好等于最大点数', () => {
      const data = Array.from({ length: 1000 }, (_, i) => ({
        time: i,
        value: i,
      }));
      const result = smartSample(data, 1000);
      
      expect(result).toEqual(data);
    });
  });

  describe('uniformSample', () => {
    it('当数据量小于目标点数时，应返回原数据', () => {
      const data = [1, 2, 3, 4, 5];
      const result = uniformSample(data, 10);
      expect(result).toEqual(data);
    });

    it('应均匀采样数据', () => {
      const data = Array.from({ length: 100 }, (_, i) => i);
      const result = uniformSample(data, 10);
      
      expect(result.length).toBe(10);
      expect(result[0]).toBe(0);
      expect(result[result.length - 1]).toBe(99);
    });

    it('应处理空数组', () => {
      const result = uniformSample([], 10);
      expect(result).toEqual([]);
    });

    it('应保证最后一个点被包含', () => {
      const data = Array.from({ length: 100 }, (_, i) => i);
      const result = uniformSample(data, 7);
      
      expect(result[result.length - 1]).toBe(data[data.length - 1]);
    });

    it('应处理对象数组', () => {
      const data = Array.from({ length: 50 }, (_, i) => ({
        id: i,
        name: `Item ${i}`,
      }));
      const result = uniformSample(data, 10);
      
      expect(result.length).toBe(10);
      expect(result[0]).toEqual({ id: 0, name: 'Item 0' });
      expect(result[result.length - 1]).toEqual({ id: 49, name: 'Item 49' });
    });

    it('采样后的点应该分布均匀', () => {
      const data = Array.from({ length: 100 }, (_, i) => i);
      const result = uniformSample(data, 10);
      
      // 检查采样间隔是否大致相等
      const intervals: number[] = [];
      for (let i = 1; i < result.length; i++) {
        intervals.push(result[i] - result[i - 1]);
      }
      
      // 验证大部分间隔是一致的
      const avgInterval = intervals.reduce((a, b) => a + b, 0) / intervals.length;
      // 至少80%的间隔应该接近平均值
      const closeToAvg = intervals.filter(i => Math.abs(i - avgInterval) < avgInterval * 0.2).length;
      expect(closeToAvg / intervals.length).toBeGreaterThan(0.8);
    });
  });
});
