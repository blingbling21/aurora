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

import React from 'react';
import {
  ScatterChart,
  Scatter,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
  ZAxis,
} from 'recharts';
import { Trade } from '@/types';
import { smartSample } from '@/lib/utils/dataSampling';

/**
 * 交易盈亏分布图组件属性
 */
export interface TradesPnLChartProps {
  // 交易数据
  trades: Trade[];
  // 图表高度
  height?: number;
}

/**
 * 交易盈亏分布图组件
 * 
 * 展示每笔交易的盈亏情况
 * - 判断策略是靠高胜率还是高盈亏比
 * - 识别异常交易
 * - 分析盈亏分布特征
 * - 自动进行数据采样以优化大数据集的渲染性能
 */
export const TradesPnLChart: React.FC<TradesPnLChartProps> = ({
  trades,
  height = 400,
}) => {
  // 过滤出有盈亏数据的交易
  const tradesWithPnL = React.useMemo(
    () => trades.filter(trade => trade.pnl !== undefined && trade.pnl !== null),
    [trades]
  );

  // 准备图表数据并进行采样优化
  const chartData = React.useMemo(() => {
    const formattedData = tradesWithPnL.map((trade, index) => ({
      index: index + 1,
      pnl: trade.pnl || 0,
      time: trade.time,
      displayTime: new Date(trade.time).toLocaleDateString('zh-CN'),
      type: trade.type,
      price: trade.price,
    }));

    // 使用智能采样优化大数据集（最多 2000 个点）
    // 散点图可以容纳更多点，但过多仍会影响性能
    return smartSample(formattedData, 2000);
  }, [tradesWithPnL]);

  // 计算统计信息
  const stats = React.useMemo(() => {
    if (tradesWithPnL.length === 0) {
      return {
        totalPnL: 0,
        avgWin: 0,
        avgLoss: 0,
        winCount: 0,
        lossCount: 0,
        maxWin: 0,
        maxLoss: 0,
      };
    }

    const pnls = tradesWithPnL.map(t => t.pnl || 0);
    const wins = pnls.filter(p => p > 0);
    const losses = pnls.filter(p => p < 0);

    return {
      totalPnL: pnls.reduce((sum, p) => sum + p, 0),
      avgWin: wins.length > 0 ? wins.reduce((sum, p) => sum + p, 0) / wins.length : 0,
      avgLoss: losses.length > 0 ? losses.reduce((sum, p) => sum + p, 0) / losses.length : 0,
      winCount: wins.length,
      lossCount: losses.length,
      // 避免使用扩展运算符导致堆栈溢出
      maxWin: wins.length > 0 ? wins.reduce((max, p) => Math.max(max, p), -Infinity) : 0,
      maxLoss: losses.length > 0 ? losses.reduce((min, p) => Math.min(min, p), Infinity) : 0,
    };
  }, [tradesWithPnL]);

  if (tradesWithPnL.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-400">
        暂无交易盈亏数据
      </div>
    );
  }

  return (
    <div>
      {/* 统计信息 */}
      <div className="mb-4 grid grid-cols-2 md:grid-cols-4 gap-3 text-sm">
        <div className="text-center p-2 bg-gray-50 rounded">
          <p className="text-gray-500">总盈亏</p>
          <p className={`font-semibold ${stats.totalPnL >= 0 ? 'text-green-600' : 'text-red-600'}`}>
            {stats.totalPnL.toFixed(2)}
          </p>
        </div>
        <div className="text-center p-2 bg-gray-50 rounded">
          <p className="text-gray-500">平均盈利</p>
          <p className="font-semibold text-green-600">{stats.avgWin.toFixed(2)}</p>
        </div>
        <div className="text-center p-2 bg-gray-50 rounded">
          <p className="text-gray-500">平均亏损</p>
          <p className="font-semibold text-red-600">{stats.avgLoss.toFixed(2)}</p>
        </div>
        <div className="text-center p-2 bg-gray-50 rounded">
          <p className="text-gray-500">盈亏比</p>
          <p className="font-semibold text-gray-900">
            {stats.avgLoss !== 0 ? (Math.abs(stats.avgWin / stats.avgLoss)).toFixed(2) : 'N/A'}
          </p>
        </div>
      </div>

      {/* 散点图 */}
      <ResponsiveContainer width="100%" height={height}>
        <ScatterChart margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
          <XAxis
            type="number"
            dataKey="index"
            name="交易序号"
            tick={{ fontSize: 12 }}
            stroke="#6b7280"
            label={{ value: '交易序号', position: 'insideBottom', offset: -5 }}
          />
          <YAxis
            type="number"
            dataKey="pnl"
            name="盈亏"
            tick={{ fontSize: 12 }}
            stroke="#6b7280"
            label={{ value: '盈亏', angle: -90, position: 'insideLeft' }}
          />
          <ZAxis range={[50, 200]} />
          <Tooltip
            cursor={{ strokeDasharray: '3 3' }}
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '6px',
              padding: '8px 12px',
            }}
            formatter={(value: number, name: string) => {
              if (name === 'pnl') {
                return [value.toFixed(2), '盈亏'];
              }
              return [value, name];
            }}
            labelFormatter={(label) => `第 ${label} 笔交易`}
          />
          <ReferenceLine y={0} stroke="#6b7280" strokeDasharray="3 3" strokeWidth={1} />
          <Scatter
            name="盈利交易"
            data={chartData.filter(d => d.pnl > 0)}
            fill="#10b981"
            opacity={0.6}
          />
          <Scatter
            name="亏损交易"
            data={chartData.filter(d => d.pnl < 0)}
            fill="#ef4444"
            opacity={0.6}
          />
          <Scatter
            name="持平交易"
            data={chartData.filter(d => d.pnl === 0)}
            fill="#6b7280"
            opacity={0.4}
          />
        </ScatterChart>
      </ResponsiveContainer>
    </div>
  );
};

export default TradesPnLChart;
