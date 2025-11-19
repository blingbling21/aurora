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

import { useEffect, useRef, RefObject } from 'react';
import {
  createChart,
  LineSeries,
  type IChartApi,
  type ISeriesApi,
  type LineData,
  type DeepPartial,
  type ChartOptions,
  type LineSeriesPartialOptions,
} from 'lightweight-charts';

/**
 * 系列数据配置
 */
export interface SeriesConfig {
  // 系列数据
  data: LineData[];
  // 系列配置选项
  options?: DeepPartial<LineSeriesPartialOptions>;
}

/**
 * 多系列图表 Hook 选项
 */
export interface UseMultiSeriesChartOptions {
  // 图表配置
  chartOptions?: DeepPartial<ChartOptions>;
  // 是否自动调整大小
  autoResize?: boolean;
}

/**
 * 多系列图表 Hook 返回值
 */
export interface UseMultiSeriesChartReturn {
  // 图表容器 ref
  chartContainerRef: RefObject<HTMLDivElement | null>;
  // 图表实例（用于高级操作）
  chartRef: RefObject<IChartApi | null>;
}

/**
 * useMultiSeriesChart Hook
 * 
 * 用于创建支持多条线的图表（如策略净值 + 基准净值）
 * 
 * @param seriesConfigs - 系列配置数组
 * @param options - 图表配置选项
 * @returns Hook 返回值
 * 
 * @example
 * ```tsx
 * const { chartContainerRef } = useMultiSeriesChart([
 *   { data: strategyData, options: { color: '#10b981', lineWidth: 2 } },
 *   { data: benchmarkData, options: { color: '#6366f1', lineWidth: 1, lineStyle: 2 } },
 * ], {
 *   chartOptions: { layout: { background: { color: '#ffffff' } } },
 * });
 * ```
 */
export function useMultiSeriesChart(
  seriesConfigs: SeriesConfig[],
  options: UseMultiSeriesChartOptions = {}
): UseMultiSeriesChartReturn {
  // 图表容器引用
  const chartContainerRef = useRef<HTMLDivElement>(null);
  
  // 图表实例引用
  const chartRef = useRef<IChartApi | null>(null);
  
  // 系列实例引用数组
  const seriesListRef = useRef<ISeriesApi<'Line'>[]>([]);

  // 初始化图表和更新数据
  useEffect(() => {
    if (!chartContainerRef.current) return;

    // 创建图表实例
    const chart = createChart(chartContainerRef.current, {
      // 默认配置
      layout: {
        background: { color: '#ffffff' },
        textColor: '#6b7280',
      },
      grid: {
        vertLines: { color: '#f3f4f6' },
        horzLines: { color: '#f3f4f6' },
      },
      crosshair: {
        mode: 1, // Magnet 模式
      },
      rightPriceScale: {
        borderColor: '#e5e7eb',
      },
      timeScale: {
        borderColor: '#e5e7eb',
        timeVisible: true,
        secondsVisible: false,
      },
      // 应用用户自定义配置
      ...options.chartOptions,
    } as DeepPartial<ChartOptions>);

    chartRef.current = chart;

    // 获取主面板
    const pane = chart.panes()[0];

    // 清空之前的系列
    seriesListRef.current = [];

    // 创建所有系列
    seriesConfigs.forEach((config) => {
      if (config.data && config.data.length > 0) {
        // 使用 pane.addSeries 方法添加线系列
        const series = pane.addSeries(LineSeries, {
          color: '#2962FF',
          lineWidth: 2,
          ...config.options,
        }) as ISeriesApi<'Line'>;
        
        series.setData(config.data);
        seriesListRef.current.push(series);
      }
    });

    // 自动缩放以适应所有数据
    if (seriesConfigs.some(c => c.data && c.data.length > 0)) {
      chart.timeScale().fitContent();
    }

    // 自动调整大小处理函数
    const handleResize = () => {
      if (chartContainerRef.current) {
        chart.applyOptions({
          width: chartContainerRef.current.clientWidth,
          height: chartContainerRef.current.clientHeight,
        });
      }
    };

    if (options.autoResize !== false) {
      // 监听窗口大小变化
      window.addEventListener('resize', handleResize);
      // 初始化大小
      handleResize();
    }

    // 清理函数
    return () => {
      if (options.autoResize !== false) {
        window.removeEventListener('resize', handleResize);
      }
      chart.remove();
      chartRef.current = null;
      seriesListRef.current = [];
    };
  }, [seriesConfigs, options.autoResize, options.chartOptions]);

  return {
    chartContainerRef,
    chartRef,
  };
}
