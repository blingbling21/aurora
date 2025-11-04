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
import { Popover, PopoverTrigger, PopoverContent, PopoverAnchor } from './popover';

// Mock Radix UI Popover 组件
jest.mock('@radix-ui/react-popover', () => ({
  Root: ({ children, ...props }: { children: React.ReactNode; [key: string]: unknown }) => (
    <div data-testid="popover-root" {...props}>{children}</div>
  ),
  Trigger: ({ children, ...props }: { children: React.ReactNode; [key: string]: unknown }) => (
    <button data-testid="popover-trigger" {...props}>{children}</button>
  ),
  Portal: ({ children, ...props }: { children: React.ReactNode; [key: string]: unknown }) => (
    <div data-testid="popover-portal" {...props}>{children}</div>
  ),
  Content: ({ children, className, ...props }: { children: React.ReactNode; className?: string; [key: string]: unknown }) => (
    <div data-testid="popover-content" className={className} {...props}>
      {children}
    </div>
  ),
  Anchor: ({ children, ...props }: { children: React.ReactNode; [key: string]: unknown }) => (
    <div data-testid="popover-anchor" {...props}>{children}</div>
  ),
}));

describe('Popover Components', () => {
  describe('Popover Root', () => {
    // 测试 Popover 根组件渲染
    it('应该渲染 Popover 根组件', () => {
      render(
        <Popover>
          <PopoverTrigger>打开</PopoverTrigger>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-root')).toBeInTheDocument();
    });

    // 测试 data-slot 属性
    it('应该有正确的 data-slot 属性', () => {
      render(
        <Popover>
          <PopoverTrigger>打开</PopoverTrigger>
        </Popover>
      );
      
      const root = screen.getByTestId('popover-root');
      expect(root).toHaveAttribute('data-slot', 'popover');
    });

    // 测试开关状态控制
    it('应该支持受控的开关状态', () => {
      const { rerender } = render(
        <Popover open={false}>
          <PopoverTrigger>打开</PopoverTrigger>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-root')).toBeInTheDocument();
      
      rerender(
        <Popover open={true}>
          <PopoverTrigger>打开</PopoverTrigger>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-root')).toBeInTheDocument();
    });
  });

  describe('PopoverTrigger', () => {
    // 测试触发器渲染
    it('应该渲染触发器按钮', () => {
      render(
        <Popover>
          <PopoverTrigger>点击打开</PopoverTrigger>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-trigger')).toBeInTheDocument();
      expect(screen.getByText('点击打开')).toBeInTheDocument();
    });

    // 测试 data-slot 属性
    it('应该有正确的 data-slot 属性', () => {
      render(
        <Popover>
          <PopoverTrigger>打开</PopoverTrigger>
        </Popover>
      );
      
      const trigger = screen.getByTestId('popover-trigger');
      expect(trigger).toHaveAttribute('data-slot', 'popover-trigger');
    });

    // 测试触发器内容
    it('应该能够包含自定义内容', () => {
      render(
        <Popover>
          <PopoverTrigger>
            <span>自定义</span>
            <span>内容</span>
          </PopoverTrigger>
        </Popover>
      );
      
      expect(screen.getByText('自定义')).toBeInTheDocument();
      expect(screen.getByText('内容')).toBeInTheDocument();
    });
  });

  describe('PopoverContent', () => {
    // 测试内容渲染
    it('应该渲染弹出内容', () => {
      render(
        <Popover>
          <PopoverTrigger>打开</PopoverTrigger>
          <PopoverContent>
            <div>弹出内容</div>
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-content')).toBeInTheDocument();
      expect(screen.getByText('弹出内容')).toBeInTheDocument();
    });

    // 测试 data-slot 属性
    it('应该有正确的 data-slot 属性', () => {
      render(
        <Popover>
          <PopoverContent>内容</PopoverContent>
        </Popover>
      );
      
      const content = screen.getByTestId('popover-content');
      expect(content).toHaveAttribute('data-slot', 'popover-content');
    });

    // 测试自定义类名
    it('应该支持自定义类名', () => {
      const customClass = 'custom-popover-content';
      render(
        <Popover>
          <PopoverContent className={customClass}>
            内容
          </PopoverContent>
        </Popover>
      );
      
      const content = screen.getByTestId('popover-content');
      expect(content).toHaveClass(customClass);
    });

    // 测试对齐方式
    it('应该支持不同的对齐方式', () => {
      render(
        <Popover>
          <PopoverContent align="start">
            内容
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-content')).toBeInTheDocument();
    });

    // 测试侧边偏移
    it('应该支持自定义侧边偏移', () => {
      render(
        <Popover>
          <PopoverContent sideOffset={10}>
            内容
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-content')).toBeInTheDocument();
    });

    // 测试内容通过 Portal 渲染
    it('应该通过 Portal 渲染内容', () => {
      render(
        <Popover>
          <PopoverContent>
            <div>Portal 内容</div>
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-portal')).toBeInTheDocument();
    });
  });

  describe('PopoverAnchor', () => {
    // 测试锚点渲染
    it('应该渲染锚点元素', () => {
      render(
        <Popover>
          <PopoverAnchor>
            <div>锚点</div>
          </PopoverAnchor>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-anchor')).toBeInTheDocument();
      expect(screen.getByText('锚点')).toBeInTheDocument();
    });

    // 测试 data-slot 属性
    it('应该有正确的 data-slot 属性', () => {
      render(
        <Popover>
          <PopoverAnchor>锚点</PopoverAnchor>
        </Popover>
      );
      
      const anchor = screen.getByTestId('popover-anchor');
      expect(anchor).toHaveAttribute('data-slot', 'popover-anchor');
    });
  });

  describe('完整 Popover 示例', () => {
    // 测试完整的 Popover 组合
    it('应该渲染完整的 Popover', () => {
      render(
        <Popover>
          <PopoverTrigger>打开弹窗</PopoverTrigger>
          <PopoverContent>
            <div>这是弹出的内容</div>
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByTestId('popover-root')).toBeInTheDocument();
      expect(screen.getByText('打开弹窗')).toBeInTheDocument();
      expect(screen.getByText('这是弹出的内容')).toBeInTheDocument();
    });

    // 测试带锚点的 Popover
    it('应该支持带锚点的 Popover', () => {
      render(
        <Popover>
          <PopoverAnchor>
            <div>锚点元素</div>
          </PopoverAnchor>
          <PopoverTrigger>打开</PopoverTrigger>
          <PopoverContent>
            <div>内容</div>
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByText('锚点元素')).toBeInTheDocument();
      expect(screen.getByText('打开')).toBeInTheDocument();
      expect(screen.getByText('内容')).toBeInTheDocument();
    });

    // 测试复杂内容的 Popover
    it('应该支持复杂的内容结构', () => {
      render(
        <Popover>
          <PopoverTrigger>打开</PopoverTrigger>
          <PopoverContent>
            <h3>标题</h3>
            <p>段落内容</p>
            <button>按钮</button>
          </PopoverContent>
        </Popover>
      );
      
      expect(screen.getByText('标题')).toBeInTheDocument();
      expect(screen.getByText('段落内容')).toBeInTheDocument();
      expect(screen.getByText('按钮')).toBeInTheDocument();
    });
  });
});
