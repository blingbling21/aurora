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
import { MainLayout } from './MainLayout';

// Mock Next.js navigation
const mockUsePathname = jest.fn();
jest.mock('next/navigation', () => ({
  usePathname: () => mockUsePathname(),
}));

// Mock NotificationContainer
jest.mock('@/components/ui', () => ({
  ...jest.requireActual('@/components/ui'),
  NotificationContainer: () => <div data-testid="notification-container">NotificationContainer</div>,
}));

describe('MainLayout 组件', () => {
  // 每个测试前重置 mock
  beforeEach(() => {
    mockUsePathname.mockReturnValue('/');
  });

  // 测试基础渲染
  it('应该正确渲染布局组件', () => {
    render(
      <MainLayout>
        <div>测试内容</div>
      </MainLayout>
    );
    
    expect(screen.getByText('测试内容')).toBeInTheDocument();
  });

  // 测试侧边栏渲染
  it('应该渲染侧边栏', () => {
    render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    // 验证侧边栏的 logo
    expect(screen.getByText('🌟 Aurora')).toBeInTheDocument();
    expect(screen.getByText('量化交易回测平台')).toBeInTheDocument();
  });

  // 测试主内容区域
  it('应该在主内容区域渲染子组件', () => {
    render(
      <MainLayout>
        <h1>页面标题</h1>
        <p>页面内容</p>
      </MainLayout>
    );
    
    expect(screen.getByText('页面标题')).toBeInTheDocument();
    expect(screen.getByText('页面内容')).toBeInTheDocument();
  });

  // 测试多个子组件
  it('应该支持多个子组件', () => {
    render(
      <MainLayout>
        <div>组件1</div>
        <div>组件2</div>
        <div>组件3</div>
      </MainLayout>
    );
    
    expect(screen.getByText('组件1')).toBeInTheDocument();
    expect(screen.getByText('组件2')).toBeInTheDocument();
    expect(screen.getByText('组件3')).toBeInTheDocument();
  });

  // 测试复杂的子组件
  it('应该支持复杂的子组件结构', () => {
    render(
      <MainLayout>
        <div>
          <h1>标题</h1>
          <div>
            <p>段落1</p>
            <p>段落2</p>
            <button>按钮</button>
          </div>
        </div>
      </MainLayout>
    );
    
    expect(screen.getByText('标题')).toBeInTheDocument();
    expect(screen.getByText('段落1')).toBeInTheDocument();
    expect(screen.getByText('段落2')).toBeInTheDocument();
    expect(screen.getByText('按钮')).toBeInTheDocument();
  });

  // 测试布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const layout = container.firstChild as HTMLElement;
    expect(layout).toHaveClass('flex', 'h-screen', 'overflow-hidden');
  });

  // 测试主内容区域样式
  it('应该为主内容区域应用正确的样式', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const mainContent = container.querySelector('main');
    expect(mainContent).toHaveClass('flex-1', 'overflow-y-auto', 'bg-gray-50', 'p-8');
  });

  // 测试侧边栏在布局中的位置
  it('应该将侧边栏放在左侧', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const layout = container.firstChild as HTMLElement;
    const firstChild = layout.firstChild as HTMLElement;
    
    // 第一个子元素应该是侧边栏（包含 logo）
    expect(firstChild.querySelector('h1')?.textContent).toBe('🌟 Aurora');
  });

  // 测试主内容区域在布局中
  it('应该包含主内容区域', () => {
    const { container } = render(
      <MainLayout>
        <div className="test-content">测试内容</div>
      </MainLayout>
    );
    
    const mainContent = container.querySelector('main');
    
    // 应该包含 main 标签
    expect(mainContent).toBeInTheDocument();
    expect(mainContent?.textContent).toContain('测试内容');
  });

  // 测试通知容器的存在
  it('应该包含通知容器', () => {
    render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    // 应该渲染通知容器
    expect(screen.getByTestId('notification-container')).toBeInTheDocument();
  });

  // 测试空子元素
  it('应该能够处理空子元素', () => {
    const { container } = render(<MainLayout>{null}</MainLayout>);
    
    const mainContent = container.querySelector('main');
    expect(mainContent).toBeInTheDocument();
  });

  // 测试导航菜单项在布局中显示
  it('应该在侧边栏显示所有导航菜单项', () => {
    render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    expect(screen.getByText('仪表盘')).toBeInTheDocument();
    expect(screen.getByText('配置管理')).toBeInTheDocument();
    expect(screen.getByText('数据管理')).toBeInTheDocument();
    expect(screen.getByText('回测执行')).toBeInTheDocument();
    expect(screen.getByText('历史记录')).toBeInTheDocument();
  });

  // 测试响应式布局
  it('应该使用 flexbox 实现响应式布局', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const layout = container.firstChild as HTMLElement;
    expect(layout).toHaveClass('flex');
    
    const mainContent = container.querySelector('main');
    expect(mainContent).toHaveClass('flex-1');
  });

  // 测试内容区域滚动
  it('应该允许主内容区域滚动', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const mainContent = container.querySelector('main');
    expect(mainContent).toHaveClass('overflow-y-auto');
  });

  // 测试布局高度
  it('应该占满整个视口高度', () => {
    const { container } = render(
      <MainLayout>
        <div>内容</div>
      </MainLayout>
    );
    
    const layout = container.firstChild as HTMLElement;
    expect(layout).toHaveClass('h-screen');
  });

  // 测试长内容
  it('应该正确渲染长内容', () => {
    const longContent = '这是一段很长的内容。'.repeat(100);
    render(
      <MainLayout>
        <div>{longContent}</div>
      </MainLayout>
    );
    
    expect(screen.getByText(longContent)).toBeInTheDocument();
  });
});
