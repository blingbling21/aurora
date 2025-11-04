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
  StatCard: ({ label, value, icon }: { label: string; value: number; icon: string }) => (
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

  // 测试初始状态下的统计值
  it('初始状态下所有统计值应该为0', () => {
    render(<Home />);
    
    // 获取所有统计卡片
    const statCards = screen.getAllByTestId('stat-card');
    
    // 验证每个卡片都包含值0
    statCards.forEach((card) => {
      expect(card.textContent).toContain('0');
    });
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
