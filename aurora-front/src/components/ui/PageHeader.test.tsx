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
import { PageHeader } from './PageHeader';

describe('PageHeader 组件', () => {
  // 测试基础渲染
  it('应该正确渲染页面标题', () => {
    render(<PageHeader title="测试标题" />);
    expect(screen.getByText('测试标题')).toBeInTheDocument();
  });

  // 测试带图标的标题
  it('应该正确显示图标和标题', () => {
    render(<PageHeader icon="🚀" title="回测执行" />);
    
    expect(screen.getByText('🚀')).toBeInTheDocument();
    expect(screen.getByText('回测执行')).toBeInTheDocument();
  });

  // 测试没有图标的标题
  it('应该在没有图标时仅显示标题', () => {
    render(<PageHeader title="配置管理" />);
    
    expect(screen.getByText('配置管理')).toBeInTheDocument();
    // 确保没有 span 元素包含图标
    const heading = screen.getByText('配置管理').closest('h2');
    expect(heading?.querySelector('span')).not.toBeInTheDocument();
  });

  // 测试带描述的页头
  it('应该正确显示描述', () => {
    render(
      <PageHeader
        title="数据管理"
        description="管理和查看所有回测数据文件"
      />
    );
    
    expect(screen.getByText('数据管理')).toBeInTheDocument();
    expect(screen.getByText('管理和查看所有回测数据文件')).toBeInTheDocument();
  });

  // 测试没有描述
  it('应该在没有描述时不显示描述元素', () => {
    const { container } = render(<PageHeader title="标题" />);
    
    const description = container.querySelector('p');
    expect(description).not.toBeInTheDocument();
  });

  // 测试带操作按钮
  it('应该正确显示操作按钮', () => {
    const actionButton = <button>新建配置</button>;
    
    render(
      <PageHeader
        title="配置管理"
        description="管理配置文件"
        action={actionButton}
      />
    );
    
    expect(screen.getByText('新建配置')).toBeInTheDocument();
  });

  // 测试没有操作按钮
  it('应该在没有操作按钮时不显示操作区域', () => {
    const { container } = render(<PageHeader title="标题" />);
    
    // 查找包含操作按钮的 div
    const headerDiv = container.querySelector('.flex.items-start.justify-between');
    expect(headerDiv?.children.length).toBe(1); // 只有标题区域，没有操作区域
  });

  // 测试完整的页头
  it('应该正确显示所有元素', () => {
    const actionButton = (
      <button className="action-btn">创建任务</button>
    );
    
    render(
      <PageHeader
        icon="📊"
        title="仪表盘"
        description="查看回测任务和统计数据"
        action={actionButton}
      />
    );
    
    expect(screen.getByText('📊')).toBeInTheDocument();
    expect(screen.getByText('仪表盘')).toBeInTheDocument();
    expect(screen.getByText('查看回测任务和统计数据')).toBeInTheDocument();
    expect(screen.getByText('创建任务')).toBeInTheDocument();
  });

  // 测试标题样式
  it('应该为标题应用正确的样式', () => {
    render(<PageHeader title="测试" />);
    
    const title = screen.getByText('测试');
    expect(title.tagName).toBe('H2');
    expect(title).toHaveClass('text-3xl', 'font-bold', 'text-gray-900');
  });

  // 测试描述样式
  it('应该为描述应用正确的样式', () => {
    render(
      <PageHeader
        title="标题"
        description="这是描述文本"
      />
    );
    
    const description = screen.getByText('这是描述文本');
    expect(description.tagName).toBe('P');
    expect(description).toHaveClass('mt-1', 'text-sm', 'text-gray-500');
  });

  // 测试长标题
  it('应该正确显示长标题', () => {
    const longTitle = '这是一个非常长的页面标题用于测试组件的显示能力和布局适应性';
    render(<PageHeader title={longTitle} />);
    expect(screen.getByText(longTitle)).toBeInTheDocument();
  });

  // 测试长描述
  it('应该正确显示长描述', () => {
    const longDescription = '这是一个非常长的描述文本，用于测试页面头部组件在处理较长描述时的显示效果和布局适应能力。';
    render(
      <PageHeader
        title="标题"
        description={longDescription}
      />
    );
    expect(screen.getByText(longDescription)).toBeInTheDocument();
  });

  // 测试多个操作按钮
  it('应该支持多个操作按钮', () => {
    const actions = (
      <div className="flex gap-2">
        <button>按钮1</button>
        <button>按钮2</button>
      </div>
    );
    
    render(
      <PageHeader
        title="标题"
        action={actions}
      />
    );
    
    expect(screen.getByText('按钮1')).toBeInTheDocument();
    expect(screen.getByText('按钮2')).toBeInTheDocument();
  });

  // 测试不同类型的图标
  it('应该支持不同类型的图标', () => {
    const { rerender } = render(<PageHeader icon="🔧" title="配置" />);
    expect(screen.getByText('🔧')).toBeInTheDocument();

    rerender(<PageHeader icon="📁" title="数据" />);
    expect(screen.getByText('📁')).toBeInTheDocument();

    rerender(<PageHeader icon="📜" title="历史" />);
    expect(screen.getByText('📜')).toBeInTheDocument();
  });

  // 测试复杂的操作元素
  it('应该支持复杂的操作元素', () => {
    const complexAction = (
      <div>
        <span>当前用户: Admin</span>
        <button>退出</button>
      </div>
    );
    
    render(
      <PageHeader
        title="标题"
        action={complexAction}
      />
    );
    
    expect(screen.getByText('当前用户: Admin')).toBeInTheDocument();
    expect(screen.getByText('退出')).toBeInTheDocument();
  });
});
