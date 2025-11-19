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
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
} from 'recharts';
import { DrawdownPoint } from '@/types';
import { smartSample } from '@/lib/utils/dataSampling';

/**
 * 回撤曲线图（潜水图）组件属性
 */
export interface DrawdownChartProps {
  // 回撤序列数据
  data: DrawdownPoint[];
  // 图表高度
  height?: number;
}

/**
 * 回撤曲线图（潜水图）组件
 * 
 * 展示资金从历史最高点回落的幅度百分比（Underwater Plot）
 * - 直观展示最大回撤的深度和持续时间
 * - 帮助评估投资者心理承受能力
 * - 自动进行数据采样以优化大数据集的渲染性能
 */
export const DrawdownChart: React.FC<DrawdownChartProps> = ({
  data,
  height = 350,
}) => {
  // 数据验证：过滤无效数据并进行采样优化
  const chartData = useMemo(() => {
    // 过滤无效数据
    if (!data || data.length === 0) {
      return [];
    }

    // 转换数据格式，过滤无效值
    const validData = data
      .filter((point) => {
        // 过滤掉 NaN、Infinity 等无效值
        return (
          point &&
          point.time &&
          typeof point.drawdown === 'number' &&
          isFinite(point.drawdown)
        );
      })
      .map((point) => ({
        time: point.time,
        drawdown: point.drawdown * 100, // 转换为百分比
        displayTime: new Date(point.time).toLocaleDateString('zh-CN'),
      }));

    // 如果没有有效数据，返回空数组
    if (validData.length === 0) {
      return [];
    }

    // 使用智能采样优化大数据集（最多 1000 个点）
    // 避免因数据量过大导致渲染性能问题或堆栈溢出
    return smartSample(validData, 1000);
  }, [data]);

  // 计算最大回撤点
  const maxDrawdown = useMemo(() => {
    if (chartData.length === 0) return 0;
    return chartData.reduce((min, d) => Math.min(min, d.drawdown), 0);
  }, [chartData]);

  // 如果没有有效数据，显示提示
  if (chartData.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-400" style={{ height }}>
        暂无回撤数据
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={height}>
      <AreaChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
        <defs>
          {/* 定义渐变色 */}
          <linearGradient id="drawdownGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#ef4444" stopOpacity={0.3} />
            <stop offset="95%" stopColor="#ef4444" stopOpacity={0.05} />
          </linearGradient>
        </defs>
        <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
        <XAxis
          dataKey="displayTime"
          tick={{ fontSize: 12 }}
          stroke="#6b7280"
          tickFormatter={(value, index) => {
            // 只显示部分标签，避免拥挤
            if (chartData.length > 20 && index % Math.ceil(chartData.length / 10) !== 0) {
              return '';
            }
            return value;
          }}
        />
        <YAxis
          tick={{ fontSize: 12 }}
          stroke="#6b7280"
          tickFormatter={(value) => `${value.toFixed(1)}%`}
          domain={[maxDrawdown * 1.1, 0]} // 稍微扩展一下范围
        />
        <Tooltip
          contentStyle={{
            backgroundColor: 'rgba(255, 255, 255, 0.95)',
            border: '1px solid #e5e7eb',
            borderRadius: '6px',
            padding: '8px 12px',
          }}
          labelStyle={{ fontWeight: 600, marginBottom: '4px' }}
          formatter={(value: number) => [`${value.toFixed(2)}%`, '回撤']}
          labelFormatter={(label) => `日期: ${label}`}
        />
        {/* 零线参考 */}
        <ReferenceLine
          y={0}
          stroke="#6b7280"
          strokeDasharray="3 3"
          strokeWidth={1}
        />
        <Area
          type="monotone"
          dataKey="drawdown"
          stroke="#ef4444"
          strokeWidth={2}
          fill="url(#drawdownGradient)"
          animationDuration={300}
        />
      </AreaChart>
    </ResponsiveContainer>
  );
};

export default DrawdownChart;
