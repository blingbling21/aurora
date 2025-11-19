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
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MonthlyReturnsHeatmap } from './MonthlyReturnsHeatmap';
import { MonthlyReturn } from '@/types';

describe('MonthlyReturnsHeatmap 组件', () => {
  // 测试数据
  const mockData: MonthlyReturn[] = [
    { year: 2024, month: 1, return: 5.2 },
    { year: 2024, month: 2, return: -2.3 },
    { year: 2024, month: 3, return: 8.5 },
    { year: 2024, month: 4, return: 3.1 },
    { year: 2024, month: 5, return: -1.5 },
    { year: 2023, month: 10, return: 12.5 },
    { year: 2023, month: 11, return: -5.2 },
    { year: 2023, month: 12, return: 7.8 },
  ];

  it('应该正确渲染表格结构', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    // 表格应该存在
    const table = screen.getByRole('table');
    expect(table).toBeInTheDocument();

    // 表头应该包含所有月份
    expect(screen.getByText('1月')).toBeInTheDocument();
    expect(screen.getByText('2月')).toBeInTheDocument();
    expect(screen.getByText('12月')).toBeInTheDocument();
  });

  it('应该显示年份列', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    expect(screen.getByText('2023')).toBeInTheDocument();
    expect(screen.getByText('2024')).toBeInTheDocument();
  });

  it('应该显示收益数据', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    // 检查部分数据点是否显示
    expect(screen.getByText('5.2%')).toBeInTheDocument(); // 2024-01
    expect(screen.getByText('-2.3%')).toBeInTheDocument(); // 2024-02
    expect(screen.getByText('8.5%')).toBeInTheDocument(); // 2024-03
  });

  it('应该显示年度收益列', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    expect(screen.getByText('年度收益')).toBeInTheDocument();
  });

  it('应该为空数据单元格显示占位符', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    // 2024年6月到12月应该是空的（因为测试数据中没有）
    const cells = screen.getAllByText('-');
    expect(cells.length).toBeGreaterThan(0);
  });

  it('应该处理空数据', () => {
    const { getByText } = render(<MonthlyReturnsHeatmap data={[]} />);

    // 数据为空时，应该显示提示信息
    expect(getByText('暂无月度收益数据')).toBeInTheDocument();
  });

  it('应该支持自定义高度', () => {
    const customHeight = 500;
    const { container } = render(<MonthlyReturnsHeatmap data={mockData} height={customHeight} />);

    const wrapper = container.querySelector('.overflow-x-auto');
    expect(wrapper).toHaveStyle({ height: `${customHeight}px`, maxHeight: `${customHeight}px` });
  });

  it('应该按年份排序显示', () => {
    const unsortedData: MonthlyReturn[] = [
      { year: 2024, month: 1, return: 5.2 },
      { year: 2023, month: 1, return: 3.1 },
      { year: 2025, month: 1, return: 2.5 },
    ];

    render(<MonthlyReturnsHeatmap data={unsortedData} />);

    const yearCells = screen.getAllByText(/^20\d{2}$/);
    expect(yearCells[0]).toHaveTextContent('2023');
    expect(yearCells[1]).toHaveTextContent('2024');
    expect(yearCells[2]).toHaveTextContent('2025');
  });

  it('应该为正收益应用绿色样式', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    // 查找显示 8.5% 的单元格（正收益）
    const positiveCell = screen.getByText('8.5%').closest('td');
    expect(positiveCell).toHaveClass('bg-green-500');
  });

  it('应该为负收益应用红色样式', () => {
    render(<MonthlyReturnsHeatmap data={mockData} />);

    // 查找显示 -2.3% 的单元格（负收益）
    const negativeCell = screen.getByText('-2.3%').closest('td');
    // 根据颜色强度，-2.3% 会应用中等强度的红色
    expect(negativeCell).toHaveClass('bg-red-500');
  });
});
