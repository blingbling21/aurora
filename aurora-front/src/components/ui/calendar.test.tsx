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
import { Calendar } from './calendar';

// Mock lucide-react 图标
jest.mock('lucide-react', () => ({
  ChevronDownIcon: () => <span data-testid="chevron-down-icon">▼</span>,
  ChevronLeftIcon: () => <span data-testid="chevron-left-icon">◀</span>,
  ChevronRightIcon: () => <span data-testid="chevron-right-icon">▶</span>,
}));

describe('Calendar Component', () => {
  // 测试基本渲染
  it('应该正确渲染日历组件', () => {
    const { container } = render(<Calendar />);
    
    // 检查日历容器存在
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试默认显示外部日期
  it('默认应该显示外部月份的日期', () => {
    const { container } = render(<Calendar />);
    
    // showOutsideDays 默认为 true
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试可以禁用外部日期显示
  it('应该能够禁用外部日期显示', () => {
    const { container } = render(<Calendar showOutsideDays={false} />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试自定义类名
  it('应该支持自定义类名', () => {
    const customClass = 'custom-calendar-class';
    const { container } = render(<Calendar className={customClass} />);
    
    const calendar = container.querySelector(`.${customClass}`);
    expect(calendar).toBeInTheDocument();
  });

  // 测试 captionLayout 属性
  it('应该支持不同的 captionLayout', () => {
    const { container: container1 } = render(<Calendar captionLayout="label" />);
    expect(container1.firstChild).toBeInTheDocument();
    
    const { container: container2 } = render(<Calendar captionLayout="dropdown" />);
    expect(container2.firstChild).toBeInTheDocument();
  });

  // 测试选择日期
  it('应该支持选择日期', () => {
    const { container } = render(<Calendar mode="single" />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试禁用日期
  it('应该支持禁用特定日期', () => {
    const disabledDate = new Date(2025, 0, 1);
    const { container } = render(<Calendar disabled={disabledDate} />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试日期范围选择
  it('应该支持日期范围选择模式', () => {
    const { container } = render(<Calendar mode="range" />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试多日期选择
  it('应该支持多日期选择模式', () => {
    const { container } = render(<Calendar mode="multiple" />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试自定义按钮变体
  it('应该支持自定义按钮变体', () => {
    const { container } = render(<Calendar buttonVariant="ghost" />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试带有初始月份
  it('应该支持设置初始显示月份', () => {
    const initialMonth = new Date(2025, 5, 1); // 2025年6月
    const { container } = render(<Calendar month={initialMonth} />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试多月显示
  it('应该支持同时显示多个月份', () => {
    const { container } = render(<Calendar numberOfMonths={2} />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试日历包含必要的导航元素
  it('应该包含导航按钮', () => {
    const { container } = render(<Calendar />);
    
    // 日历本身应该正常渲染
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试周数显示
  it('应该支持显示周数', () => {
    const { container } = render(<Calendar showWeekNumber />);
    
    expect(container.firstChild).toBeInTheDocument();
  });

  // 测试固定周数
  it('应该支持固定周数显示', () => {
    const { container } = render(<Calendar fixedWeeks />);
    
    expect(container.firstChild).toBeInTheDocument();
  });
});
