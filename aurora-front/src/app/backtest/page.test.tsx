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
import { render, screen, fireEvent } from '@testing-library/react';
import BacktestPage from './page';

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
  Button: ({ children, onClick, disabled, variant }: { 
    children: React.ReactNode; 
    onClick?: () => void; 
    disabled?: boolean;
    variant?: string;
  }) => (
    <button data-testid="button" onClick={onClick} disabled={disabled} data-variant={variant}>
      {children}
    </button>
  ),
  Input: ({ value, onChange, placeholder }: { 
    value: string; 
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void; 
    placeholder?: string;
  }) => (
    <input 
      data-testid="input" 
      value={value} 
      onChange={onChange} 
      placeholder={placeholder}
    />
  ),
  Select: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="select">{children}</div>
  ),
  SelectContent: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectTrigger: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectValue: ({ placeholder }: { placeholder?: string }) => <span>{placeholder}</span>,
}));

describe('BacktestPage', () => {
  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<BacktestPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('🚀')).toBeInTheDocument();
    expect(screen.getByText('回测执行')).toBeInTheDocument();
    expect(screen.getByText('配置并启动新的回测任务')).toBeInTheDocument();
  });

  // 测试任务配置区域
  it('应该渲染任务配置卡片', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    const configCard = cards.find(card => card.textContent?.includes('任务配置'));
    expect(configCard).toBeInTheDocument();
  });

  // 测试任务名称输入框
  it('应该显示任务名称输入框', () => {
    render(<BacktestPage />);
    
    const input = screen.getByTestId('input');
    expect(input).toBeInTheDocument();
  });

  // 测试任务名称输入
  it('应该能够输入任务名称', () => {
    render(<BacktestPage />);
    
    const input = screen.getByTestId('input') as HTMLInputElement;
    fireEvent.change(input, { target: { value: '测试任务' } });
    
    expect(input.value).toBe('测试任务');
  });

  // 测试配置选择器
  it('应该显示配置选择器', () => {
    render(<BacktestPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThan(0);
  });

  // 测试数据文件选择器
  it('应该显示数据文件选择器', () => {
    render(<BacktestPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThanOrEqual(2);
  });

  // 测试执行结果区域
  it('应该渲染执行结果卡片', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    const resultCard = cards.find(card => card.textContent?.includes('执行结果'));
    expect(resultCard).toBeInTheDocument();
  });

  // 测试未开始状态的提示
  it('未开始时应该显示提示信息', () => {
    render(<BacktestPage />);
    
    expect(screen.getByText(/点击.*开始回测.*按钮启动任务/)).toBeInTheDocument();
  });

  // 测试启动按钮
  it('应该显示启动回测按钮', () => {
    render(<BacktestPage />);
    
    const buttons = screen.getAllByTestId('button');
    const startButton = buttons.find(btn => btn.textContent?.includes('开始回测'));
    expect(startButton).toBeInTheDocument();
  });

  // 测试停止按钮
  it('应该显示停止按钮', () => {
    render(<BacktestPage />);
    
    const buttons = screen.getAllByTestId('button');
    const stopButton = buttons.find(btn => btn.textContent?.includes('停止'));
    expect(stopButton).toBeInTheDocument();
  });

  // 测试页面布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(<BacktestPage />);
    
    // 检查是否有网格布局
    const grids = container.querySelectorAll('.grid');
    expect(grids.length).toBeGreaterThan(0);
  });

  // 测试卡片数量
  it('应该至少有两个卡片（配置和结果）', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    expect(cards.length).toBeGreaterThanOrEqual(2);
  });
});
