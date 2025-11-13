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
import Home from './page';
import { useDashboardStore } from '@/lib/store';

// Mock store
jest.mock('@/lib/store', () => ({
  useDashboardStore: jest.fn(),
}));

// Mock 子组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title, description }: { icon: string; title: string; description: string }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
      <p>{description}</p>
    </div>
  ),
  Card: ({ title, children }: { title?: string; children: React.ReactNode }) => (
    <div data-testid="card">
      {title && <h2>{title}</h2>}
      {children}
    </div>
  ),
}));

jest.mock('@/components/dashboard', () => ({
  StatCard: ({ label, value, icon }: { label: string; value: number | string; icon: string }) => (
    <div data-testid="stat-card">
      <span>{icon}</span>
      <span>{label}</span>
      <span>{value}</span>
    </div>
  ),
  TaskItem: ({ task }: { task: { name: string } }) => (
    <div data-testid="task-item">{task.name}</div>
  ),
}));

describe('Home Page', () => {
  // 默认 mock 实现
  const mockLoadData = jest.fn();
  const defaultMockStore = {
    stats: {
      total_tasks: 0,
      running_tasks: 0,
      completed_tasks: 0,
      failed_tasks: 0,
    },
    recentTasks: [],
    isLoading: false,
    error: null,
    loadData: mockLoadData,
  };

  beforeEach(() => {
    jest.clearAllMocks();
    (useDashboardStore as unknown as jest.Mock).mockReturnValue(defaultMockStore);
  });

  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<Home />);
    
    // 检查页面头部元素
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('📊')).toBeInTheDocument();
    expect(screen.getByText('仪表盘')).toBeInTheDocument();
    expect(screen.getByText('回测任务概览与实时监控')).toBeInTheDocument();
  });

  // 测试统计卡片渲染
  it('应该渲染四个统计卡片', () => {
    render(<Home />);
    
    // 检查统计卡片数量
    const statCards = screen.getAllByTestId('stat-card');
    expect(statCards).toHaveLength(4);
  });

  // 测试统计卡片内容
  it('应该显示正确的统计标签', () => {
    render(<Home />);
    
    // 检查各个统计指标
    expect(screen.getByText('总任务数')).toBeInTheDocument();
    expect(screen.getByText('运行中')).toBeInTheDocument();
    expect(screen.getByText('已完成')).toBeInTheDocument();
    expect(screen.getByText('失败')).toBeInTheDocument();
  });

  // 测试组件挂载时加载数据
  it('应该在组件挂载时调用 loadData', () => {
    render(<Home />);
    
    // 验证 loadData 被调用
    expect(mockLoadData).toHaveBeenCalled();
  });

  // 测试加载状态
  it('应该在加载时显示加载提示', () => {
    (useDashboardStore as unknown as jest.Mock).mockReturnValue({
      ...defaultMockStore,
      isLoading: true,
    });

    render(<Home />);
    
    // 验证显示加载提示
    expect(screen.getByText('加载中...')).toBeInTheDocument();
    // 统计值应该显示为 '-'
    const statCards = screen.getAllByTestId('stat-card');
    statCards.forEach((card) => {
      expect(card.textContent).toContain('-');
    });
  });

  // 测试错误状态
  it('应该显示错误信息', () => {
    (useDashboardStore as unknown as jest.Mock).mockReturnValue({
      ...defaultMockStore,
      error: '服务器错误',
    });

    render(<Home />);
    
    // 验证显示错误信息
    expect(screen.getByText('服务器错误')).toBeInTheDocument();
  });

  // 测试有数据时的显示
  it('应该显示统计数据', () => {
    (useDashboardStore as unknown as jest.Mock).mockReturnValue({
      ...defaultMockStore,
      stats: {
        total_tasks: 10,
        running_tasks: 2,
        completed_tasks: 7,
        failed_tasks: 1,
      },
    });

    render(<Home />);
    
    // 验证统计数据显示
    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('7')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument();
  });

  // 测试最近任务显示
  it('应该显示最近任务列表', () => {
    (useDashboardStore as unknown as jest.Mock).mockReturnValue({
      ...defaultMockStore,
      recentTasks: [
        {
          id: 'task-1',
          name: '测试任务1',
          status: 'completed',
          config: 'test1.toml',
          dataFile: 'test1.csv',
          progress: 100,
          createdAt: '2025-01-01T10:00:00Z',
          updatedAt: '2025-01-01T10:30:00Z',
        },
        {
          id: 'task-2',
          name: '测试任务2',
          status: 'running',
          config: 'test2.toml',
          dataFile: 'test2.csv',
          progress: 50,
          createdAt: '2025-01-01T11:00:00Z',
          updatedAt: '2025-01-01T11:15:00Z',
        },
      ],
    });

    render(<Home />);
    
    // 验证任务显示
    expect(screen.getByText('测试任务1')).toBeInTheDocument();
    expect(screen.getByText('测试任务2')).toBeInTheDocument();
  });

  // 测试最近任务卡片
  it('应该渲染最近任务卡片', () => {
    render(<Home />);
    
    // 查找最近任务卡片
    const cards = screen.getAllByTestId('card');
    const recentTasksCard = cards.find(card => 
      card.textContent?.includes('最近任务')
    );
    
    expect(recentTasksCard).toBeInTheDocument();
  });

  // 测试空任务列表提示
  it('当没有任务时应该显示提示信息', () => {
    render(<Home />);
    
    // 检查空状态提示
    expect(screen.getByText('暂无任务记录')).toBeInTheDocument();
  });

  // 测试页面结构完整性
  it('应该包含统计卡片网格和任务列表区域', () => {
    const { container } = render(<Home />);
    
    // 检查网格布局
    const grid = container.querySelector('.grid');
    expect(grid).toBeInTheDocument();
    expect(grid).toHaveClass('grid-cols-1', 'md:grid-cols-2', 'lg:grid-cols-4');
  });

  // 测试响应式布局类名
  it('应该应用正确的响应式类名', () => {
    const { container } = render(<Home />);
    
    // 检查响应式网格
    const grid = container.querySelector('.grid');
    expect(grid).toHaveClass('gap-6');
    expect(grid).toHaveClass('mb-8');
  });
});
