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
import { MonthlyReturn } from '@/types';

/**
 * 月度收益热力图组件属性
 */
export interface MonthlyReturnsHeatmapProps {
  // 月度收益数据
  data: MonthlyReturn[];
  // 图表高度（可选）
  height?: number;
}

/**
 * 月度收益热力图组件
 * 
 * 以表格形式展示每一年、每一月的收益情况
 * - 用颜色深浅表示盈亏幅度
 * - 一眼看出策略的季节性效应
 * - 识别连续亏损的月份/年份
 */
export const MonthlyReturnsHeatmap: React.FC<MonthlyReturnsHeatmapProps> = ({
  data,
  height,
}) => {
  // 数据验证
  if (!data || data.length === 0) {
    return (
      <div className="flex items-center justify-center text-gray-400" style={{ minHeight: '200px' }}>
        暂无月度收益数据
      </div>
    );
  }

  // 月份名称
  const monthNames = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月'];

  // 组织数据：按年份分组
  const yearlyData = new Map<number, Map<number, number>>();
  data.forEach((item) => {
    if (!yearlyData.has(item.year)) {
      yearlyData.set(item.year, new Map());
    }
    yearlyData.get(item.year)!.set(item.month, item.return);
  });

  // 获取所有年份并排序
  const years = Array.from(yearlyData.keys()).sort((a, b) => a - b);

  // 计算最大和最小收益，用于颜色映射（避免使用扩展运算符导致堆栈溢出）
  const allReturns = data.map(d => d.return).filter(r => isFinite(r));
  const maxReturn = allReturns.length > 0 ? allReturns.reduce((max, r) => Math.max(max, r), -Infinity) : 0;
  const minReturn = allReturns.length > 0 ? allReturns.reduce((min, r) => Math.min(min, r), Infinity) : 0;

  /**
   * 根据收益值获取背景色
   */
  const getBackgroundColor = (returnValue: number): string => {
    if (returnValue === 0) return 'bg-gray-100';
    
    // 正收益：绿色渐变
    if (returnValue > 0) {
      const intensity = Math.min(returnValue / (maxReturn || 1), 1);
      if (intensity > 0.7) return 'bg-green-600';
      if (intensity > 0.4) return 'bg-green-500';
      if (intensity > 0.2) return 'bg-green-400';
      return 'bg-green-300';
    }
    
    // 负收益：红色渐变
    const intensity = Math.min(Math.abs(returnValue) / Math.abs(minReturn || 1), 1);
    if (intensity > 0.7) return 'bg-red-600';
    if (intensity > 0.4) return 'bg-red-500';
    if (intensity > 0.2) return 'bg-red-400';
    return 'bg-red-300';
  };

  /**
   * 根据背景色获取文字颜色
   */
  const getTextColor = (returnValue: number): string => {
    if (returnValue === 0) return 'text-gray-700';
    
    const absReturn = Math.abs(returnValue);
    const maxAbsReturn = Math.max(Math.abs(maxReturn), Math.abs(minReturn));
    const intensity = absReturn / (maxAbsReturn || 1);
    
    // 深色背景用白色文字
    return intensity > 0.4 ? 'text-white' : 'text-gray-900';
  };

  /**
   * 计算年度总收益
   */
  const getYearlyReturn = (year: number): number => {
    const monthlyReturns = yearlyData.get(year);
    if (!monthlyReturns) return 0;
    
    // 计算累计收益率（复利）
    let totalReturn = 1;
    for (let month = 1; month <= 12; month++) {
      const monthReturn = monthlyReturns.get(month) || 0;
      totalReturn *= (1 + monthReturn / 100);
    }
    return (totalReturn - 1) * 100;
  };

  return (
    <div
      className="overflow-x-auto"
      style={height ? { height, maxHeight: height } : undefined}
    >
      <table className="w-full border-collapse text-sm">
        <thead>
          <tr className="bg-gray-100 border-b border-gray-300">
            <th className="px-2 py-2 text-left font-semibold text-gray-700 sticky left-0 bg-gray-100">
              年份
            </th>
            {monthNames.map((month) => (
              <th
                key={month}
                className="px-2 py-2 text-center font-semibold text-gray-700 min-w-[60px]"
              >
                {month}
              </th>
            ))}
            <th className="px-2 py-2 text-center font-semibold text-gray-700 min-w-20">
              年度收益
            </th>
          </tr>
        </thead>
        <tbody>
          {years.map((year) => {
            const monthlyReturns = yearlyData.get(year)!;
            const yearlyReturn = getYearlyReturn(year);
            
            return (
              <tr key={year} className="border-b border-gray-200">
                <td className="px-2 py-2 font-semibold text-gray-900 sticky left-0 bg-white border-r border-gray-200">
                  {year}
                </td>
                {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((month) => {
                  const returnValue = monthlyReturns.get(month);
                  
                  if (returnValue === undefined) {
                    return (
                      <td key={month} className="px-2 py-2 text-center bg-gray-50">
                        <span className="text-gray-400">-</span>
                      </td>
                    );
                  }
                  
                  const bgColor = getBackgroundColor(returnValue);
                  const textColor = getTextColor(returnValue);
                  
                  return (
                    <td
                      key={month}
                      className={`px-2 py-2 text-center ${bgColor} ${textColor} font-medium transition-colors`}
                      title={`${year}年${month}月: ${returnValue.toFixed(2)}%`}
                    >
                      {returnValue.toFixed(1)}%
                    </td>
                  );
                })}
                <td
                  className={`px-2 py-2 text-center font-bold border-l border-gray-300 ${
                    getBackgroundColor(yearlyReturn)
                  } ${getTextColor(yearlyReturn)}`}
                >
                  {yearlyReturn.toFixed(2)}%
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default MonthlyReturnsHeatmap;
