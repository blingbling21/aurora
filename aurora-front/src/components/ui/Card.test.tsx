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
import { Card } from './Card';

describe('Card 组件', () => {
  // 测试基础渲染
  it('应该正确渲染卡片内容', () => {
    render(
      <Card>
        <p>这是卡片内容</p>
      </Card>
    );
    
    expect(screen.getByText('这是卡片内容')).toBeInTheDocument();
  });

  // 测试带标题的卡片
  it('应该正确显示卡片标题', () => {
    render(
      <Card title="卡片标题">
        <p>卡片内容</p>
      </Card>
    );
    
    expect(screen.getByText('卡片标题')).toBeInTheDocument();
    expect(screen.getByText('卡片内容')).toBeInTheDocument();
  });

  // 测试没有标题的卡片
  it('应该在没有标题时不显示标题元素', () => {
    const { container } = render(
      <Card>
        <p>卡片内容</p>
      </Card>
    );
    
    const heading = container.querySelector('h3');
    expect(heading).not.toBeInTheDocument();
  });

  // 测试自定义 className
  it('应该接受自定义 className', () => {
    const { container } = render(
      <Card className="custom-class">
        <p>内容</p>
      </Card>
    );
    
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('custom-class');
  });

  // 测试默认样式类名
  it('应该包含默认样式类名', () => {
    const { container } = render(
      <Card>
        <p>内容</p>
      </Card>
    );
    
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('bg-white', 'rounded-lg', 'p-6', 'shadow-sm');
  });

  // 测试自定义 className 与默认样式合并
  it('应该将自定义 className 与默认样式合并', () => {
    const { container } = render(
      <Card className="mt-4 border-2">
        <p>内容</p>
      </Card>
    );
    
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveClass('bg-white', 'rounded-lg', 'p-6', 'shadow-sm', 'mt-4', 'border-2');
  });

  // 测试嵌套多个子元素
  it('应该支持多个子元素', () => {
    render(
      <Card title="标题">
        <p>第一段</p>
        <p>第二段</p>
        <button>按钮</button>
      </Card>
    );
    
    expect(screen.getByText('第一段')).toBeInTheDocument();
    expect(screen.getByText('第二段')).toBeInTheDocument();
    expect(screen.getByText('按钮')).toBeInTheDocument();
  });

  // 测试复杂的子组件
  it('应该支持复杂的子组件', () => {
    render(
      <Card title="复杂内容">
        <div>
          <h4>子标题</h4>
          <ul>
            <li>项目 1</li>
            <li>项目 2</li>
          </ul>
        </div>
      </Card>
    );
    
    expect(screen.getByText('子标题')).toBeInTheDocument();
    expect(screen.getByText('项目 1')).toBeInTheDocument();
    expect(screen.getByText('项目 2')).toBeInTheDocument();
  });

  // 测试标题样式
  it('应该为标题应用正确的样式', () => {
    render(
      <Card title="测试标题">
        <p>内容</p>
      </Card>
    );
    
    const title = screen.getByText('测试标题');
    expect(title.tagName).toBe('H3');
    expect(title).toHaveClass('text-lg', 'font-semibold', 'text-gray-900', 'mb-4');
  });

  // 测试空子元素
  it('应该能够处理空子元素', () => {
    const { container } = render(<Card>{null}</Card>);
    const card = container.firstChild as HTMLElement;
    expect(card).toBeInTheDocument();
  });

  // 测试长标题
  it('应该正确显示长标题', () => {
    const longTitle = '这是一个非常长的标题用于测试卡片组件的标题显示能力';
    render(
      <Card title={longTitle}>
        <p>内容</p>
      </Card>
    );
    
    expect(screen.getByText(longTitle)).toBeInTheDocument();
  });

  // 测试仅包含标题的卡片
  it('应该支持仅包含标题的卡片', () => {
    render(<Card title="仅标题" />);
    expect(screen.getByText('仅标题')).toBeInTheDocument();
  });

  // 测试包含特殊字符的内容
  it('应该正确渲染包含特殊字符的内容', () => {
    render(
      <Card title="特殊字符 & <> 测试">
        <p>内容 & 符号 {'<>'} 测试</p>
      </Card>
    );
    
    expect(screen.getByText(/特殊字符/)).toBeInTheDocument();
    expect(screen.getByText(/内容 & 符号/)).toBeInTheDocument();
  });
});
