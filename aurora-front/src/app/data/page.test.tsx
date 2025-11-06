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

// Mock DataList 组件
jest.mock('@/components/dashboard/DataList', () => ({
  DataList: () => (
    <div data-testid="card" className="mt-6">
      <h2>数据文件列表</h2>
      <div className="flex justify-end mb-4">
        <button data-testid="button" data-variant="secondary">
          🔄 刷新
        </button>
      </div>
      <div className="text-center py-8 text-gray-500">
        暂无数据文件
      </div>
    </div>
  ),
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
  Button: ({ children, variant, onClick, type }: { 
    children: React.ReactNode; 
    variant?: string; 
    onClick?: () => void; 
    type?: 'button' | 'submit' | 'reset' 
  }) => (
    <button data-testid="button" data-variant={variant} onClick={onClick} type={type}>
      {children}
    </button>
  ),
  Input: ({ placeholder, value, onChange, className, type, required }: { 
    placeholder?: string; 
    value?: string; 
    onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void; 
    className?: string;
    type?: string;
    required?: boolean;
  }) => (
    <input 
      data-testid="input" 
      placeholder={placeholder} 
      value={value}
      onChange={onChange}
      className={className}
      type={type}
      required={required}
    />
  ),
  DatePicker: ({ placeholder, onDateChange }: { 
    placeholder?: string; 
    date?: Date;
    onDateChange?: (date: Date | undefined) => void;
    required?: boolean;
    className?: string;
  }) => (
    <div data-testid="date-picker">
      <input 
        type="date" 
        placeholder={placeholder}
        onChange={(e) => onDateChange?.(e.target.value ? new Date(e.target.value) : undefined)}
      />
    </div>
  ),
  Select: ({ children, value, onValueChange, required }: { 
    children: React.ReactNode; 
    value?: string; 
    onValueChange?: (value: string) => void;
    required?: boolean;
  }) => (
    <div data-testid="select" onClick={() => onValueChange?.('test-value')}>
      <input type="hidden" value={value} />
      {children}
    </div>
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

  // 测试文件名输入框
  it('应该显示文件名输入框', () => {
    render(<DataPage />);
    
    const inputs = screen.getAllByTestId('input');
    const filenameInput = inputs.find(input => 
      input.getAttribute('placeholder') === '自动生成'
    );
    expect(filenameInput).toBeInTheDocument();
  });

  // 测试预览文件名按钮
  it('应该显示预览文件名按钮', () => {
    render(<DataPage />);
    
    const buttons = screen.getAllByTestId('button');
    const previewButton = buttons.find(btn => btn.textContent?.includes('预览文件名'));
    expect(previewButton).toBeInTheDocument();
  });

  // 测试文件名可以手动编辑
  it('文件名输入框应该可以手动编辑', () => {
    render(<DataPage />);
    
    const inputs = screen.getAllByTestId('input');
    const filenameInput = inputs.find(input => 
      input.getAttribute('placeholder') === '自动生成'
    ) as HTMLInputElement;
    
    expect(filenameInput).toBeInTheDocument();
    // 验证输入框不是只读的
    expect(filenameInput?.readOnly).toBeFalsy();
  });
});
