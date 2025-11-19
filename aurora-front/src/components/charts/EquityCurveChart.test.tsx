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

import React from 'react';
import { render } from '@testing-library/react';
import '@testing-library/jest-dom';
import { EquityCurveChart } from './EquityCurveChart';
import { EquityCurvePoint } from '@/types';

// Mock lightweight-charts
jest.mock('lightweight-charts');

// Mock useMultiSeriesChart Hook
jest.mock('@/lib/hooks/useMultiSeriesChart', () => ({
  useMultiSeriesChart: jest.fn(() => ({
    chartContainerRef: { current: null },
    chartRef: { current: null },
  })),
}));

// 获取 mock 函数引用
import { useMultiSeriesChart } from '@/lib/hooks/useMultiSeriesChart';
const mockUseMultiSeriesChart = useMultiSeriesChart as jest.MockedFunction<typeof useMultiSeriesChart>;

describe('EquityCurveChart 组件', () => {
  const mockData: EquityCurvePoint[] = [
    { time: '2024-01-01', value: 10000 },
    { time: '2024-01-02', value: 10100 },
    { time: '2024-01-03', value: 10200 },
  ];

  const mockBenchmarkData: EquityCurvePoint[] = [
    { time: '2024-01-01', value: 10000 },
    { time: '2024-01-02', value: 10050 },
    { time: '2024-01-03', value: 10100 },
  ];

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('应该正确渲染图表容器', () => {
    const { container } = render(<EquityCurveChart data={mockData} />);

    // 应该有图表容器 div
    const chartContainer = container.querySelector('div[style*="height"]');
    expect(chartContainer).toBeInTheDocument();
  });

  it('应该使用正确的数据调用 useMultiSeriesChart Hook', () => {
    render(<EquityCurveChart data={mockData} />);

    // 验证 Hook 被调用
    expect(mockUseMultiSeriesChart).toHaveBeenCalled();
    
    // 获取调用参数
    const calls = mockUseMultiSeriesChart.mock.calls as unknown[][];
    const lastCall = calls[calls.length - 1];
    const seriesConfigs = lastCall?.[0] as Array<{ 
      data: Array<{ time: string | number; value: number }>; 
      options?: { color?: string; lineWidth?: number; lineStyle?: number };
    }>;
    
    // 验证传递了系列配置
    expect(seriesConfigs).toBeDefined();
    expect(seriesConfigs.length).toBe(1); // 只有策略线
    expect(seriesConfigs[0].data.length).toBe(mockData.length);
    expect(seriesConfigs[0].options?.color).toBe('#10b981');
  });

  it('应该在提供基准数据时显示图例', () => {
    const { getByText } = render(
      <EquityCurveChart data={mockData} benchmarkData={mockBenchmarkData} />
    );

    expect(getByText('策略净值')).toBeInTheDocument();
    expect(getByText('基准净值')).toBeInTheDocument();
  });

  it('应该在提供基准数据时传递两个系列', () => {
    render(<EquityCurveChart data={mockData} benchmarkData={mockBenchmarkData} />);

    // 获取调用参数
    const calls = mockUseMultiSeriesChart.mock.calls as unknown[][];
    const lastCall = calls[calls.length - 1];
    const seriesConfigs = lastCall?.[0] as Array<{ 
      data: Array<{ time: string | number; value: number }>; 
      options?: { color?: string; lineWidth?: number; lineStyle?: number };
    }>;
    
    // 应该有两个系列：策略和基准
    expect(seriesConfigs.length).toBe(2);
    expect(seriesConfigs[0].options?.color).toBe('#10b981'); // 策略线（绿色）
    expect(seriesConfigs[1].options?.color).toBe('#6366f1'); // 基准线（蓝色）
    expect(seriesConfigs[1].options?.lineStyle).toBe(2); // 虚线
  });

  it('应该在数据为空时显示提示信息', () => {
    const { getByText } = render(<EquityCurveChart data={[]} />);

    // 数据为空时，应该显示提示信息
    expect(getByText('暂无净值曲线数据')).toBeInTheDocument();
  });

  it('应该正确处理对数坐标选项', () => {
    render(<EquityCurveChart data={mockData} useLogScale={true} />);

    // 获取调用参数
    const calls = mockUseMultiSeriesChart.mock.calls as unknown[][];
    const lastCall = calls[calls.length - 1];
    const seriesConfigs = lastCall?.[0] as Array<{ 
      data: Array<{ time: string | number; value: number }>; 
    }>;
    
    // 对数坐标下，value 应该是 log10(原始值)
    expect(seriesConfigs[0].data[0].value).toBeCloseTo(Math.log10(10000), 2);
  });

  it('应该正确处理线性坐标', () => {
    render(<EquityCurveChart data={mockData} useLogScale={false} />);

    // 获取调用参数
    const calls = mockUseMultiSeriesChart.mock.calls as unknown[][];
    const lastCall = calls[calls.length - 1];
    const seriesConfigs = lastCall?.[0] as Array<{ 
      data: Array<{ time: string | number; value: number }>; 
    }>;
    
    // 线性坐标下，value 应该是原始值
    expect(seriesConfigs[0].data[0].value).toBe(10000);
  });

  it('应该处理大数据量', () => {
    // 生成 10000 个数据点
    const largeData: EquityCurvePoint[] = Array.from({ length: 10000 }, (_, i) => ({
      time: `2024-01-${String((i % 30) + 1).padStart(2, '0')}`,
      value: 10000 + i * 10,
    }));

    const { container } = render(<EquityCurveChart data={largeData} />);

    // 应该能够正常渲染
    expect(container.querySelector('div[style*="height"]')).toBeInTheDocument();
    
    // 验证数据被传递
    const calls = mockUseMultiSeriesChart.mock.calls as unknown[][];
    const lastCall = calls[calls.length - 1];
    const seriesConfigs = lastCall?.[0] as Array<{ 
      data: Array<{ time: string | number; value: number }>; 
    }>;
    
    expect(seriesConfigs[0].data.length).toBe(10000);
  });
});
