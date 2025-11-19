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
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
  Cell,
} from 'recharts';
import { EquityCurvePoint, ReturnBucket } from '@/types';

/**
 * 收益分布图组件属性
 */
export interface ReturnsDistributionProps {
  // 权益曲线数据（用于计算日收益率）
  equityCurve?: EquityCurvePoint[];
  // 或者直接提供已经分桶的数据
  distributionData?: ReturnBucket[];
  // 分桶数量
  buckets?: number;
  // 图表高度
  height?: number;
}

/**
 * 收益分布直方图组件
 * 
 * 展示日收益率的频率分布
 * - 检查肥尾风险（Fat Tails）
 * - 识别偏度（Skewness）
 * - 评估极端收益事件的概率
 */
export const ReturnsDistribution: React.FC<ReturnsDistributionProps> = ({
  equityCurve,
  distributionData,
  buckets = 20,
  height = 350,
}) => {
  // 计算收益分布数据
  const chartData = useMemo(() => {
    // 如果直接提供了分布数据，使用它
    if (distributionData) {
      return distributionData.map(bucket => ({
        label: bucket.label,
        count: bucket.count,
        min: bucket.min,
        max: bucket.max,
      }));
    }

    // 否则从权益曲线计算
    if (!equityCurve || equityCurve.length < 2) {
      return [];
    }

    // 计算日收益率，过滤无效值
    const dailyReturns: number[] = [];
    for (let i = 1; i < equityCurve.length; i++) {
      const prevValue = equityCurve[i - 1].value;
      const currValue = equityCurve[i].value;
      
      // 验证数据有效性
      if (prevValue && currValue && prevValue !== 0 && isFinite(prevValue) && isFinite(currValue)) {
        const returnPct = ((currValue - prevValue) / prevValue) * 100;
        if (isFinite(returnPct)) {
          dailyReturns.push(returnPct);
        }
      }
    }

    // 检查是否有有效数据
    if (dailyReturns.length === 0) {
      return [];
    }

    // 计算收益范围（避免使用扩展运算符导致堆栈溢出）
    const minReturn = dailyReturns.reduce((min, r) => Math.min(min, r), Infinity);
    const maxReturn = dailyReturns.reduce((max, r) => Math.max(max, r), -Infinity);
    const range = maxReturn - minReturn;
    const bucketSize = range / buckets;

    // 初始化分桶
    const bucketCounts = new Array(buckets).fill(0);
    const bucketLabels: string[] = [];

    // 分配收益到各个桶
    dailyReturns.forEach(ret => {
      let bucketIndex = Math.floor((ret - minReturn) / bucketSize);
      // 处理边界情况
      if (bucketIndex >= buckets) bucketIndex = buckets - 1;
      if (bucketIndex < 0) bucketIndex = 0;
      bucketCounts[bucketIndex]++;
    });

    // 生成标签和数据
    for (let i = 0; i < buckets; i++) {
      const min = minReturn + i * bucketSize;
      bucketLabels.push(`${min.toFixed(1)}`);
    }

    return bucketCounts.map((count, index) => ({
      label: bucketLabels[index],
      count,
      min: minReturn + index * bucketSize,
      max: minReturn + (index + 1) * bucketSize,
    }));
  }, [equityCurve, distributionData, buckets]);

  // 计算统计信息
  const stats = useMemo(() => {
    if (!equityCurve || equityCurve.length < 2) {
      return { mean: 0, std: 0, skewness: 0 };
    }

    const dailyReturns: number[] = [];
    for (let i = 1; i < equityCurve.length; i++) {
      const returnPct = ((equityCurve[i].value - equityCurve[i - 1].value) / equityCurve[i - 1].value) * 100;
      dailyReturns.push(returnPct);
    }

    // 计算均值
    const mean = dailyReturns.reduce((sum, r) => sum + r, 0) / dailyReturns.length;

    // 计算标准差
    const variance = dailyReturns.reduce((sum, r) => sum + Math.pow(r - mean, 2), 0) / dailyReturns.length;
    const std = Math.sqrt(variance);

    // 计算偏度
    const skewness = dailyReturns.reduce((sum, r) => sum + Math.pow((r - mean) / std, 3), 0) / dailyReturns.length;

    return { mean, std, skewness };
  }, [equityCurve]);

  if (chartData.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-400">
        暂无数据
      </div>
    );
  }

  return (
    <div>
      {/* 统计信息 */}
      {equityCurve && equityCurve.length >= 2 && (
        <div className="mb-4 grid grid-cols-3 gap-4 text-sm">
          <div className="text-center">
            <p className="text-gray-500">均值</p>
            <p className="font-semibold text-gray-900">{stats.mean.toFixed(3)}%</p>
          </div>
          <div className="text-center">
            <p className="text-gray-500">标准差</p>
            <p className="font-semibold text-gray-900">{stats.std.toFixed(3)}%</p>
          </div>
          <div className="text-center">
            <p className="text-gray-500">偏度</p>
            <p className={`font-semibold ${stats.skewness < 0 ? 'text-red-600' : 'text-green-600'}`}>
              {stats.skewness.toFixed(3)}
            </p>
          </div>
        </div>
      )}

      {/* 直方图 */}
      <ResponsiveContainer width="100%" height={height}>
        <BarChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
          <XAxis
            dataKey="label"
            tick={{ fontSize: 11 }}
            stroke="#6b7280"
            label={{ value: '日收益率 (%)', position: 'insideBottom', offset: -5 }}
            tickFormatter={(value, index) => {
              // 只显示部分标签，避免拥挤
              if (chartData.length > 15 && index % 2 !== 0) {
                return '';
              }
              return value;
            }}
          />
          <YAxis
            tick={{ fontSize: 12 }}
            stroke="#6b7280"
            label={{ value: '频数', angle: -90, position: 'insideLeft' }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '6px',
              padding: '8px 12px',
            }}
            formatter={(value: number, name, props) => {
              const { min, max } = props.payload;
              return [
                `${value} 次`,
                `收益率: ${min.toFixed(2)}% ~ ${max.toFixed(2)}%`,
              ];
            }}
          />
          <ReferenceLine y={0} stroke="#6b7280" strokeDasharray="3 3" />
          <Bar dataKey="count" radius={[4, 4, 0, 0]}>
            {chartData.map((entry, index) => {
              // 根据收益率正负设置颜色
              const color = entry.min >= 0 ? '#10b981' : '#ef4444';
              return <Cell key={`cell-${index}`} fill={color} />;
            })}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
};

export default ReturnsDistribution;
