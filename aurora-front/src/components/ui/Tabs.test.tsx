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
import '@testing-library/jest-dom';
import { Tabs, TabItem } from './Tabs';

describe('Tabs 组件', () => {
  // 测试用的 Tab 数据
  const mockTabs: TabItem[] = [
    {
      id: 'tab1',
      label: '标签1',
      icon: '📊',
      content: <div>内容1</div>,
    },
    {
      id: 'tab2',
      label: '标签2',
      icon: '📈',
      content: <div>内容2</div>,
    },
    {
      id: 'tab3',
      label: '标签3',
      content: <div>内容3</div>,
    },
  ];

  it('应该正确渲染所有 Tab 标签', () => {
    render(<Tabs tabs={mockTabs} />);

    expect(screen.getByText('标签1')).toBeInTheDocument();
    expect(screen.getByText('标签2')).toBeInTheDocument();
    expect(screen.getByText('标签3')).toBeInTheDocument();
  });

  it('应该显示 Tab 图标', () => {
    render(<Tabs tabs={mockTabs} />);

    expect(screen.getByText('📊')).toBeInTheDocument();
    expect(screen.getByText('📈')).toBeInTheDocument();
  });

  it('应该默认激活第一个 Tab', () => {
    render(<Tabs tabs={mockTabs} />);

    // 第一个 Tab 的内容应该显示
    expect(screen.getByText('内容1')).toBeInTheDocument();
    // 其他 Tab 的内容不应该显示
    expect(screen.queryByText('内容2')).not.toBeInTheDocument();
    expect(screen.queryByText('内容3')).not.toBeInTheDocument();
  });

  it('应该支持自定义默认激活的 Tab', () => {
    render(<Tabs tabs={mockTabs} defaultActiveId="tab2" />);

    // 第二个 Tab 的内容应该显示
    expect(screen.getByText('内容2')).toBeInTheDocument();
    // 其他 Tab 的内容不应该显示
    expect(screen.queryByText('内容1')).not.toBeInTheDocument();
    expect(screen.queryByText('内容3')).not.toBeInTheDocument();
  });

  it('应该支持 Tab 切换', () => {
    render(<Tabs tabs={mockTabs} />);

    // 初始状态：显示第一个 Tab
    expect(screen.getByText('内容1')).toBeInTheDocument();

    // 点击第二个 Tab
    const tab2Button = screen.getByRole('tab', { name: /标签2/i });
    fireEvent.click(tab2Button);

    // 应该显示第二个 Tab 的内容
    expect(screen.getByText('内容2')).toBeInTheDocument();
    expect(screen.queryByText('内容1')).not.toBeInTheDocument();
  });

  it('应该在 Tab 切换时调用回调函数', () => {
    const onTabChange = jest.fn();
    render(<Tabs tabs={mockTabs} onTabChange={onTabChange} />);

    // 点击第二个 Tab
    const tab2Button = screen.getByRole('tab', { name: /标签2/i });
    fireEvent.click(tab2Button);

    // 回调应该被调用，参数是新的 Tab ID
    expect(onTabChange).toHaveBeenCalledTimes(1);
    expect(onTabChange).toHaveBeenCalledWith('tab2');
  });

  it('应该正确应用激活状态的样式', () => {
    render(<Tabs tabs={mockTabs} />);

    const tab1Button = screen.getByRole('tab', { name: /标签1/i });
    const tab2Button = screen.getByRole('tab', { name: /标签2/i });

    // 第一个 Tab 应该有激活样式
    expect(tab1Button).toHaveClass('border-blue-600', 'text-blue-600');
    expect(tab1Button).toHaveAttribute('aria-selected', 'true');

    // 第二个 Tab 不应该有激活样式
    expect(tab2Button).toHaveClass('border-transparent', 'text-gray-600');
    expect(tab2Button).toHaveAttribute('aria-selected', 'false');

    // 点击第二个 Tab
    fireEvent.click(tab2Button);

    // 现在第二个 Tab 应该有激活样式
    expect(tab2Button).toHaveClass('border-blue-600', 'text-blue-600');
    expect(tab2Button).toHaveAttribute('aria-selected', 'true');

    // 第一个 Tab 不应该有激活样式
    expect(tab1Button).toHaveClass('border-transparent', 'text-gray-600');
    expect(tab1Button).toHaveAttribute('aria-selected', 'false');
  });

  it('应该处理空 Tab 列表', () => {
    const { container } = render(<Tabs tabs={[]} />);

    // 应该不渲染任何内容
    expect(container.firstChild).toBeNull();
  });

  it('应该支持自定义样式类名', () => {
    const { container } = render(<Tabs tabs={mockTabs} className="custom-class" />);

    const tabsContainer = container.querySelector('.custom-class');
    expect(tabsContainer).toBeInTheDocument();
  });
});
