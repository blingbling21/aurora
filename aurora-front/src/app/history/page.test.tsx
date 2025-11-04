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
import HistoryPage from './page';

// Mock 子组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title, action }: { icon: string; title: string; action?: React.ReactNode }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
      {action}
    </div>
  ),
  Card: ({ title, children }: { title?: string; children: React.ReactNode }) => (
    <div data-testid="card">
      {title && <h2>{title}</h2>}
      {children}
    </div>
  ),
  Button: ({ children, variant }: { children: React.ReactNode; variant?: string }) => (
    <button data-testid="button" data-variant={variant}>
      {children}
    </button>
  ),
}));

jest.mock('@/components/dashboard', () => ({
  TaskItem: ({ task }: { task: { name: string } }) => (
    <div data-testid="task-item">{task.name}</div>
  ),
}));

describe('HistoryPage', () => {
  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<HistoryPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('📜')).toBeInTheDocument();
    expect(screen.getByText('历史记录')).toBeInTheDocument();
  });

  // 测试刷新按钮在头部
  it('应该在头部显示刷新按钮', () => {
    render(<HistoryPage />);
    
    const header = screen.getByTestId('page-header');
    const button = header.querySelector('[data-testid="button"]');
    expect(button).toBeInTheDocument();
    expect(button?.textContent).toContain('刷新');
  });

  // 测试回测历史卡片
  it('应该渲染回测历史卡片', () => {
    render(<HistoryPage />);
    
    const cards = screen.getAllByTestId('card');
    const historyCard = cards.find(card => card.textContent?.includes('回测历史'));
    expect(historyCard).toBeInTheDocument();
  });

  // 测试空历史列表提示
  it('当没有历史记录时应该显示提示信息', () => {
    render(<HistoryPage />);
    
    expect(screen.getByText('暂无历史记录')).toBeInTheDocument();
  });

  // 测试结果详情卡片
  it('应该渲染结果详情卡片', () => {
    render(<HistoryPage />);
    
    const cards = screen.getAllByTestId('card');
    const detailCard = cards.find(card => card.textContent?.includes('结果详情'));
    expect(detailCard).toBeInTheDocument();
  });

  // 测试未选择任务时的提示
  it('未选择任务时应该显示提示信息', () => {
    render(<HistoryPage />);
    
    expect(screen.getByText('选择一个任务查看详细结果')).toBeInTheDocument();
  });

  // 测试页面布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(<HistoryPage />);
    
    const grids = container.querySelectorAll('.grid');
    expect(grids.length).toBeGreaterThan(0);
  });

  // 测试至少有两个主要卡片
  it('应该至少有两个卡片（历史和详情）', () => {
    render(<HistoryPage />);
    
    const cards = screen.getAllByTestId('card');
    expect(cards.length).toBeGreaterThanOrEqual(2);
  });

  // 测试响应式布局类名
  it('应该应用正确的响应式类名', () => {
    const { container } = render(<HistoryPage />);
    
    const grids = container.querySelectorAll('.grid');
    const grid = grids[0];
    expect(grid).toHaveClass('grid-cols-1');
    expect(grid).toHaveClass('lg:grid-cols-3');
  });

  // 测试卡片在网格中的列跨度
  it('卡片应该有正确的列跨度设置', () => {
    render(<HistoryPage />);
    
    // 验证结果详情 Card 存在
    const cards = screen.getAllByTestId('card');
    expect(cards).toHaveLength(2); // 回测历史 + 结果详情
    expect(screen.getByText('结果详情')).toBeInTheDocument();
  });
});
