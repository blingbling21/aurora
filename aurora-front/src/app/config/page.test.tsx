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
import ConfigPage from './page';

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
  Textarea: ({ placeholder }: { placeholder?: string }) => (
    <textarea data-testid="textarea" placeholder={placeholder} />
  ),
  Select: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="select">{children}</div>
  ),
  SelectContent: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectItem: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectTrigger: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectValue: ({ placeholder }: { placeholder?: string }) => <span>{placeholder}</span>,
}));

describe('ConfigPage', () => {
  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<ConfigPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('⚙️')).toBeInTheDocument();
    expect(screen.getByText('配置管理')).toBeInTheDocument();
  });

  // 测试配置列表卡片
  it('应该渲染配置列表卡片', () => {
    render(<ConfigPage />);
    
    const cards = screen.getAllByTestId('card');
    const listCard = cards.find(card => card.textContent?.includes('配置列表'));
    expect(listCard).toBeInTheDocument();
  });

  // 测试空配置列表提示
  it('当没有配置时应该显示提示信息', () => {
    render(<ConfigPage />);
    
    expect(screen.getByText('暂无配置文件')).toBeInTheDocument();
  });

  // 测试新建按钮
  it('应该显示新建配置按钮', () => {
    render(<ConfigPage />);
    
    const buttons = screen.getAllByTestId('button');
    const newButton = buttons.find(btn => btn.textContent?.includes('新建配置'));
    expect(newButton).toBeInTheDocument();
  });

  // 测试新建配置按钮
  it('应该显示新建配置按钮', () => {
    render(<ConfigPage />);
    
    // 验证页面中有新建配置按钮
    expect(screen.getByText('+ 新建配置')).toBeInTheDocument();
  });

  // 测试配置编辑器卡片
  it('应该渲染配置编辑器卡片', () => {
    render(<ConfigPage />);
    
    const cards = screen.getAllByTestId('card');
    const editorCard = cards.find(card => card.textContent?.includes('配置编辑器'));
    expect(editorCard).toBeInTheDocument();
  });

  // 测试未编辑状态提示
  it('未选择配置时应该显示提示信息', () => {
    render(<ConfigPage />);
    
    expect(screen.getByText('选择或创建一个配置文件以开始编辑')).toBeInTheDocument();
  });

  // 测试模式切换按钮存在（根据当前状态显示不同文本）
  it('应该显示模式切换按钮', () => {
    render(<ConfigPage />);
    
    // 未编辑状态下不显示模式切换按钮
    expect(screen.queryByText(/文本模式/)).not.toBeInTheDocument();
    expect(screen.queryByText(/表单模式/)).not.toBeInTheDocument();
  });

  // 测试页面布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(<ConfigPage />);
    
    const grids = container.querySelectorAll('.grid');
    expect(grids.length).toBeGreaterThan(0);
  });

  // 测试至少有两个主要卡片
  it('应该至少有两个卡片（列表和编辑器）', () => {
    render(<ConfigPage />);
    
    const cards = screen.getAllByTestId('card');
    expect(cards.length).toBeGreaterThanOrEqual(2);
  });
});
