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
import DataPage from './page';

// Mock 常量
jest.mock('@/constants', () => ({
  EXCHANGE_OPTIONS: [
    { label: 'Binance', value: 'binance' },
    { label: 'OKX', value: 'okx' },
  ],
  INTERVAL_OPTIONS: [
    { label: '1分钟', value: '1m' },
    { label: '5分钟', value: '5m' },
  ],
  SYMBOL_OPTIONS: [
    { label: 'BTC/USDT', value: 'BTCUSDT' },
    { label: 'ETH/USDT', value: 'ETHUSDT' },
  ],
}));

// Mock 子组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title }: { icon: string; title: string }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
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
  Input: ({ placeholder }: { placeholder?: string }) => (
    <input data-testid="input" placeholder={placeholder} />
  ),
  DatePicker: ({ placeholder }: { placeholder?: string }) => (
    <div data-testid="date-picker">{placeholder}</div>
  ),
  Select: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="select">{children}</div>
  ),
  SelectContent: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectItem: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectTrigger: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectValue: ({ placeholder }: { placeholder?: string }) => <span>{placeholder}</span>,
}));

describe('DataPage', () => {
  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<DataPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    // 不直接匹配 emoji，因为在测试环境中可能显示为乱码
    expect(screen.getByText('数据管理')).toBeInTheDocument();
  });

  // 测试数据列表卡片
  it('应该渲染数据文件列表卡片', () => {
    render(<DataPage />);
    
    const cards = screen.getAllByTestId('card');
    const listCard = cards.find(card => card.textContent?.includes('数据文件列表'));
    expect(listCard).toBeInTheDocument();
  });

  // 测试空数据列表提示
  it('当没有数据文件时应该显示提示信息', () => {
    render(<DataPage />);
    
    expect(screen.getByText('暂无数据文件')).toBeInTheDocument();
  });

  // 测试刷新按钮
  it('应该显示刷新按钮', () => {
    render(<DataPage />);
    
    const buttons = screen.getAllByTestId('button');
    const refreshButton = buttons.find(btn => btn.textContent?.includes('刷新'));
    expect(refreshButton).toBeInTheDocument();
  });

  // 测试数据下载卡片
  it('应该渲染数据下载卡片', () => {
    render(<DataPage />);
    
    const cards = screen.getAllByTestId('card');
    const downloadCard = cards.find(card => card.textContent?.includes('下载数据'));
    expect(downloadCard).toBeInTheDocument();
  });

  // 测试交易所选择器
  it('应该显示交易所选择器', () => {
    render(<DataPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThan(0);
  });

  // 测试交易对选择器
  it('应该显示交易对选择器', () => {
    render(<DataPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThanOrEqual(2);
  });

  // 测试时间周期选择器
  it('应该显示时间周期选择器', () => {
    render(<DataPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThanOrEqual(3);
  });

  // 测试日期选择器
  it('应该显示开始和结束日期选择器', () => {
    render(<DataPage />);
    
    const datePickers = screen.getAllByTestId('date-picker');
    expect(datePickers.length).toBeGreaterThanOrEqual(2);
  });

  // 测试下载按钮
  it('应该显示下载按钮', () => {
    render(<DataPage />);
    
    const buttons = screen.getAllByTestId('button');
    const downloadButton = buttons.find(btn => btn.textContent?.includes('开始下载'));
    expect(downloadButton).toBeInTheDocument();
  });

  // 测试页面布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(<DataPage />);
    
    const grids = container.querySelectorAll('.grid');
    expect(grids.length).toBeGreaterThan(0);
  });

  // 测试至少有两个主要卡片
  it('应该至少有两个卡片（列表和下载）', () => {
    render(<DataPage />);
    
    const cards = screen.getAllByTestId('card');
    expect(cards.length).toBeGreaterThanOrEqual(2);
  });
});
