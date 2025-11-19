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
  AreaSeries,
  type IChartApi,
  type ISeriesApi,
  type LineData,
  type AreaData,
  type DeepPartial,
  type ChartOptions,
} from 'lightweight-charts';

/**
 * Lightweight Charts Hook 选项
 */
export interface UseLightweightChartOptions {
  // 图表配置
  chartOptions?: DeepPartial<ChartOptions>;
  // 线条系列配置（颜色、宽度等）
  seriesOptions?: {
    color?: string;
    lineWidth?: 1 | 2 | 3 | 4; // LineWidth 类型的有效值
    // 区域图专用
    topColor?: string;
    bottomColor?: string;
  };
  // 是否自动调整大小
  autoResize?: boolean;
}

/**
 * Lightweight Charts Hook 返回值
 */
export interface UseLightweightChartReturn {
  // 图表容器 ref (允许 null 因为初始时未挂载)
  chartContainerRef: RefObject<HTMLDivElement | null>;
}

/**
 * useLightweightChart Hook
 * 
 * 用于在 React 组件中集成 TradingView 的 lightweight-charts 库
 * 支持线图和区域图，适合大数据量的时间序列可视化
 * 
 * @param seriesType - 系列类型：'line' 或 'area'
 * @param data - 图表数据
 * @param options - 配置选项
 * @returns Hook 返回值，包含容器 ref
 * 
 * @example
 * ```tsx
 * const { chartContainerRef } = useLightweightChart('line', data, {
 *   chartOptions: { layout: { background: { color: '#ffffff' } } },
 *   seriesOptions: { color: '#2962FF', lineWidth: 2 },
 * });
 * 
 * return <div ref={chartContainerRef} style={{ height: '400px' }} />;
 * ```
 */
export function useLightweightChart(
  seriesType: 'line' | 'area',
  data: LineData[] | AreaData[],
  options: UseLightweightChartOptions = {}
): UseLightweightChartReturn {
  // 图表容器引用
  const chartContainerRef = useRef<HTMLDivElement>(null);
  
  // 图表实例引用
  const chartRef = useRef<IChartApi | null>(null);
  
  // 系列实例引用
  const seriesRef = useRef<ISeriesApi<'Line'> | ISeriesApi<'Area'> | null>(null);

  // 初始化图表和更新数据
  useEffect(() => {
    if (!chartContainerRef.current) return;

    // 创建图表实例
    const chart = createChart(chartContainerRef.current, {
      // 默认配置
      layout: {
        background: { color: '#ffffff' },
        textColor: '#333',
      },
      grid: {
        vertLines: { color: '#f0f0f0' },
        horzLines: { color: '#f0f0f0' },
      },
      crosshair: {
        mode: 1, // Magnet 模式
      },
      rightPriceScale: {
        borderColor: '#e0e0e0',
      },
      timeScale: {
        borderColor: '#e0e0e0',
        timeVisible: true,
        secondsVisible: false,
      },
      // 应用用户自定义配置
      ...options.chartOptions,
    });

    chartRef.current = chart;

    // 获取主面板
    const pane = chart.panes()[0];

    // 创建系列
    let series: ISeriesApi<'Line'> | ISeriesApi<'Area'>;
    
    if (seriesType === 'line') {
      series = pane.addSeries(LineSeries, {
        color: options.seriesOptions?.color || '#2962FF',
        lineWidth: options.seriesOptions?.lineWidth || 2,
      }) as ISeriesApi<'Line'>;
    } else {
      series = pane.addSeries(AreaSeries, {
        topColor: options.seriesOptions?.topColor || 'rgba(41, 98, 255, 0.4)',
        bottomColor: options.seriesOptions?.bottomColor || 'rgba(41, 98, 255, 0.0)',
        lineColor: options.seriesOptions?.color || '#2962FF',
        lineWidth: options.seriesOptions?.lineWidth || 2,
      }) as ISeriesApi<'Area'>;
    }

    seriesRef.current = series;

    // 设置数据
    if (data && data.length > 0) {
      series.setData(data);
      // 自动缩放以适应所有数据
      chart.timeScale().fitContent();
    }

    // 自动调整大小
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
      seriesRef.current = null;
    };
    // 依赖项包括所有用到的外部变量
  }, [seriesType, data, options.autoResize, options.chartOptions, options.seriesOptions]);

  return {
    chartContainerRef,
  };
}

