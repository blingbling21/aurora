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

'use client';

import React, { useMemo } from 'react';
import { useMultiSeriesChart } from '@/lib/hooks/useMultiSeriesChart';
import type { SeriesConfig } from '@/lib/hooks/useMultiSeriesChart';
import { EquityCurvePoint } from '@/types';
import { formatTimeForLightweightCharts } from '@/lib/utils/timeFormatting';
import type { LineData, UTCTimestamp } from 'lightweight-charts';

/**
 * 累计净值曲线图组件属性
 */
export interface EquityCurveChartProps {
  // 权益曲线数据
  data: EquityCurvePoint[];
  // 基准数据（可选，用于对比）
  benchmarkData?: EquityCurvePoint[];
  // 是否使用对数坐标
  useLogScale?: boolean;
  // 图表高度
  height?: number;
}

/**
 * 累计净值曲线图组件
 * 
 * 展示策略随时间变化的累计收益率曲线
 * - 支持基准对比，展示超额收益（Alpha）
 * - 支持对数坐标，便于观察长期复利效应
 * - 使用 lightweight-charts 实现高性能渲染，支持大数据量（10k+ 数据点）
 */
export const EquityCurveChart: React.FC<EquityCurveChartProps> = ({
  data,
  benchmarkData,
  useLogScale = false,
  height = 400,
}) => {
  // 转换策略数据格式
  const strategyData = useMemo((): LineData[] => {
    if (!data || data.length === 0) {
      return [];
    }

    return data.map((point) => {
      // 转换值（对数坐标处理）
      const value = useLogScale && point.value > 0 
        ? Math.log10(point.value) 
        : point.value;
      
      // 时间值：如果是数字直接使用（作为 UTCTimestamp），如果是字符串则转换为日期格式
      const time = typeof point.time === 'number' 
        ? (point.time as UTCTimestamp)
        : formatTimeForLightweightCharts(point.time);
      
      return {
        time,
        value,
      };
    });
  }, [data, useLogScale]);

  // 转换基准数据格式
  const benchmarkDataConverted = useMemo((): LineData[] => {
    if (!benchmarkData || benchmarkData.length === 0) {
      return [];
    }

    return benchmarkData.map((point) => {
      // 转换值（对数坐标处理）
      const value = useLogScale && point.value > 0 
        ? Math.log10(point.value) 
        : point.value;
      
      // 时间值：如果是数字直接使用，如果是字符串则转换为日期格式
      const time = typeof point.time === 'number' 
        ? (point.time as UTCTimestamp)
        : formatTimeForLightweightCharts(point.time);
      
      return {
        time,
        value,
      };
    });
  }, [benchmarkData, useLogScale]);

  // 构建系列配置
  const seriesConfigs = useMemo((): SeriesConfig[] => {
    const configs: SeriesConfig[] = [];
    
    // 策略净值线（主线，绿色，较粗）
    if (strategyData.length > 0) {
      configs.push({
        data: strategyData,
        options: {
          color: '#10b981', // emerald-500
          lineWidth: 2,
        },
      });
    }
    
    // 基准净值线（对比线，蓝色，虚线）
    if (benchmarkDataConverted.length > 0) {
      configs.push({
        data: benchmarkDataConverted,
        options: {
          color: '#6366f1', // indigo-500
          lineWidth: 1,
          lineStyle: 2, // 虚线
        },
      });
    }
    
    return configs;
  }, [strategyData, benchmarkDataConverted]);

  // 使用多系列图表 Hook
  const { chartContainerRef } = useMultiSeriesChart(seriesConfigs, {
    chartOptions: {
      layout: {
        background: { color: '#ffffff' },
        textColor: '#6b7280',
      },
      grid: {
        vertLines: { color: '#f3f4f6' },
        horzLines: { color: '#f3f4f6' },
      },
      rightPriceScale: {
        borderColor: '#e5e7eb',
      },
      timeScale: {
        borderColor: '#e5e7eb',
        timeVisible: true,
        secondsVisible: false,
      },
      crosshair: {
        mode: 1, // Magnet mode
      },
    },
    autoResize: true,
  });

  // 如果没有有效数据，显示提示
  if (!data || data.length === 0) {
    return (
      <div 
        className="flex items-center justify-center text-gray-400" 
        style={{ height: `${height}px` }}
      >
        暂无净值曲线数据
      </div>
    );
  }

  return (
    <div style={{ height: `${height}px`, width: '100%', position: 'relative' }}>
      <div ref={chartContainerRef} style={{ height: '100%', width: '100%' }} />
      {benchmarkData && benchmarkData.length > 0 && (
        <div 
          className="absolute top-2 left-2 text-xs text-gray-500 bg-white/90 px-2 py-1 rounded border border-gray-200"
        >
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1">
              <div className="w-4 h-0.5 bg-emerald-500" />
              <span>策略净值</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-4 h-0.5 bg-indigo-500 border-t-2 border-dashed border-indigo-500" />
              <span>基准净值</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default EquityCurveChart;
