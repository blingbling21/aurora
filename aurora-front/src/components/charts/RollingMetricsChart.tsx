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
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { RollingMetricPoint } from '@/types';
import { smartSample } from '@/lib/utils/dataSampling';

/**
 * 滚动指标图组件属性
 */
export interface RollingMetricsChartProps {
  // 滚动指标数据
  data: RollingMetricPoint[];
  // 显示哪些指标
  metrics?: ('volatility' | 'sharpe' | 'return')[];
  // 图表高度
  height?: number;
}

/**
 * 滚动指标图组件
 * 
 * 展示策略波动率和夏普比率随时间的变化
 * - 识别策略失效期
 * - 评估策略稳定性
 * - 检测风险变化趋势
 * - 自动进行数据采样以优化大数据集的渲染性能
 */
export const RollingMetricsChart: React.FC<RollingMetricsChartProps> = ({
  data,
  metrics = ['volatility', 'sharpe'],
  height = 350,
}) => {
  // 使用 useMemo 优化数据处理和采样
  const chartData = useMemo(() => {
    // 数据验证
    if (!data || data.length === 0) {
      return [];
    }

    // 转换数据格式
    const formattedData = data.map((point) => ({
      time: point.time,
      volatility: point.volatility,
      sharpe: point.sharpe,
      return: point.return,
      displayTime: new Date(point.time).toLocaleDateString('zh-CN'),
    }));

    // 使用智能采样优化大数据集（最多 1000 个点）
    // 避免因数据量过大导致渲染性能问题或堆栈溢出
    return smartSample(formattedData, 1000);
  }, [data]);

  // 如果没有有效数据，显示提示
  if (chartData.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-400" style={{ height }}>
        暂无滚动指标数据
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height={height}>
      <LineChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
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
          yAxisId="left"
          tick={{ fontSize: 12 }}
          stroke="#6b7280"
          label={{ value: '波动率 / 收益率 (%)', angle: -90, position: 'insideLeft' }}
        />
        <YAxis
          yAxisId="right"
          orientation="right"
          tick={{ fontSize: 12 }}
          stroke="#6b7280"
          label={{ value: '夏普比率', angle: 90, position: 'insideRight' }}
        />
        <Tooltip
          contentStyle={{
            backgroundColor: 'rgba(255, 255, 255, 0.95)',
            border: '1px solid #e5e7eb',
            borderRadius: '6px',
            padding: '8px 12px',
          }}
          labelStyle={{ fontWeight: 600, marginBottom: '4px' }}
          formatter={(value, name) => {
            // 类型守卫：确保 value 是数字类型
            if (typeof value !== 'number') return ['-', name as string];
            
            if (name === 'volatility' || name === 'return') {
              return [`${value.toFixed(2)}%`, name === 'volatility' ? '波动率' : '收益率'];
            }
            return [value.toFixed(3), '夏普比率'];
          }}
          labelFormatter={(label) => `日期: ${label}`}
        />
        <Legend
          wrapperStyle={{ paddingTop: '10px' }}
          iconType="line"
        />
        
        {/* 波动率线 */}
        {metrics.includes('volatility') && (
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="volatility"
            stroke="#f59e0b"
            strokeWidth={2}
            dot={false}
            name="滚动波动率"
            animationDuration={300}
          />
        )}
        
        {/* 收益率线 */}
        {metrics.includes('return') && (
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="return"
            stroke="#10b981"
            strokeWidth={2}
            dot={false}
            name="滚动收益率"
            animationDuration={300}
          />
        )}
        
        {/* 夏普比率线 */}
        {metrics.includes('sharpe') && (
          <Line
            yAxisId="right"
            type="monotone"
            dataKey="sharpe"
            stroke="#6366f1"
            strokeWidth={2}
            dot={false}
            name="滚动夏普比率"
            animationDuration={300}
          />
        )}
      </LineChart>
    </ResponsiveContainer>
  );
};

export default RollingMetricsChart;
