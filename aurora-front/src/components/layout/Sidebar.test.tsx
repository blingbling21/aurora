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

import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { Sidebar } from './Sidebar';
import { NAV_MENU_ITEMS } from '@/constants';

// Mock Next.js navigation
const mockUsePathname = jest.fn();
jest.mock('next/navigation', () => ({
  usePathname: () => mockUsePathname(),
}));

describe('Sidebar 组件', () => {
  // 每个测试前重置 mock
  beforeEach(() => {
    mockUsePathname.mockReturnValue('/');
  });

  // 测试基础渲染
  it('应该正确渲染侧边栏', () => {
    render(<Sidebar />);
    
    // 验证 logo
    expect(screen.getByText('🌟 Aurora')).toBeInTheDocument();
    expect(screen.getByText('量化交易回测平台')).toBeInTheDocument();
  });

  // 测试导航菜单项渲染
  it('应该渲染所有导航菜单项', () => {
    render(<Sidebar />);
    
    NAV_MENU_ITEMS.forEach((item) => {
      expect(screen.getByText(item.label)).toBeInTheDocument();
      expect(screen.getByText(item.icon)).toBeInTheDocument();
    });
  });

  // 测试首页激活状态
  it('应该在首页时高亮首页菜单', () => {
    mockUsePathname.mockReturnValue('/');
    render(<Sidebar />);
    
    const dashboardLink = screen.getByText('仪表盘').closest('a');
    expect(dashboardLink).toHaveClass('bg-gray-50', 'text-blue-500', 'border-r-[3px]', 'border-blue-500');
  });

  // 测试配置页激活状态
  it('应该在配置页时高亮配置菜单', () => {
    mockUsePathname.mockReturnValue('/config');
    render(<Sidebar />);
    
    const configLink = screen.getByText('配置管理').closest('a');
    expect(configLink).toHaveClass('bg-gray-50', 'text-blue-500', 'border-r-[3px]', 'border-blue-500');
  });

  // 测试数据页激活状态
  it('应该在数据页时高亮数据菜单', () => {
    mockUsePathname.mockReturnValue('/data');
    render(<Sidebar />);
    
    const dataLink = screen.getByText('数据管理').closest('a');
    expect(dataLink).toHaveClass('bg-gray-50', 'text-blue-500', 'border-r-[3px]', 'border-blue-500');
  });

  // 测试回测页激活状态
  it('应该在回测页时高亮回测菜单', () => {
    mockUsePathname.mockReturnValue('/backtest');
    render(<Sidebar />);
    
    const backtestLink = screen.getByText('回测执行').closest('a');
    expect(backtestLink).toHaveClass('bg-gray-50', 'text-blue-500', 'border-r-[3px]', 'border-blue-500');
  });

  // 测试历史页激活状态
  it('应该在历史页时高亮历史菜单', () => {
    mockUsePathname.mockReturnValue('/history');
    render(<Sidebar />);
    
    const historyLink = screen.getByText('历史记录').closest('a');
    expect(historyLink).toHaveClass('bg-gray-50', 'text-blue-500', 'border-r-[3px]', 'border-blue-500');
  });

  // 测试非激活菜单样式
  it('应该为非激活菜单项应用正确的样式', () => {
    mockUsePathname.mockReturnValue('/');
    render(<Sidebar />);
    
    const configLink = screen.getByText('配置管理').closest('a');
    expect(configLink).toHaveClass('text-gray-500', 'hover:bg-gray-50', 'hover:text-gray-900');
    expect(configLink).not.toHaveClass('text-blue-500');
  });

  // 测试菜单链接
  it('应该为每个菜单项设置正确的链接', () => {
    render(<Sidebar />);
    
    const dashboardLink = screen.getByText('仪表盘').closest('a');
    expect(dashboardLink).toHaveAttribute('href', '/');
    
    const configLink = screen.getByText('配置管理').closest('a');
    expect(configLink).toHaveAttribute('href', '/config');
    
    const dataLink = screen.getByText('数据管理').closest('a');
    expect(dataLink).toHaveAttribute('href', '/data');
    
    const backtestLink = screen.getByText('回测执行').closest('a');
    expect(backtestLink).toHaveAttribute('href', '/backtest');
    
    const historyLink = screen.getByText('历史记录').closest('a');
    expect(historyLink).toHaveAttribute('href', '/history');
  });

  // 测试侧边栏宽度
  it('应该有固定宽度', () => {
    const { container } = render(<Sidebar />);
    const sidebar = container.firstChild as HTMLElement;
    
    expect(sidebar).toHaveClass('w-[260px]');
  });

  // 测试侧边栏样式
  it('应该包含正确的样式类名', () => {
    const { container } = render(<Sidebar />);
    const sidebar = container.firstChild as HTMLElement;
    
    expect(sidebar).toHaveClass('bg-white', 'border-r', 'border-gray-200', 'flex', 'flex-col');
  });

  // 测试 logo 区域样式
  it('应该为 logo 区域应用正确的样式', () => {
    const { container } = render(<Sidebar />);
    
    const logoArea = container.querySelector('.px-6.py-6.border-b');
    expect(logoArea).toBeInTheDocument();
  });

  // 测试导航菜单顺序
  it('应该按正确顺序渲染导航菜单项', () => {
    render(<Sidebar />);
    
    const menuItems = screen.getAllByRole('link');
    
    // 第一个应该是仪表盘
    expect(menuItems[0]).toHaveTextContent('仪表盘');
    
    // 第二个应该是配置管理
    expect(menuItems[1]).toHaveTextContent('配置管理');
    
    // 第三个应该是数据管理
    expect(menuItems[2]).toHaveTextContent('数据管理');
    
    // 第四个应该是回测执行
    expect(menuItems[3]).toHaveTextContent('回测执行');
    
    // 第五个应该是历史记录
    expect(menuItems[4]).toHaveTextContent('历史记录');
  });

  // 测试菜单图标显示
  it('应该正确显示每个菜单项的图标', () => {
    render(<Sidebar />);
    
    expect(screen.getByText('📊')).toBeInTheDocument(); // 仪表盘
    expect(screen.getByText('⚙️')).toBeInTheDocument(); // 配置管理
    expect(screen.getByText('📁')).toBeInTheDocument(); // 数据管理
    expect(screen.getByText('🚀')).toBeInTheDocument(); // 回测执行
    expect(screen.getByText('📜')).toBeInTheDocument(); // 历史记录
  });

  // 测试当路径不匹配时
  it('应该在路径不匹配时不高亮任何菜单', () => {
    mockUsePathname.mockReturnValue('/unknown-path');
    render(<Sidebar />);
    
    NAV_MENU_ITEMS.forEach((item) => {
      const link = screen.getByText(item.label).closest('a');
      expect(link).not.toHaveClass('text-blue-500');
      expect(link).toHaveClass('text-gray-500');
    });
  });
});
