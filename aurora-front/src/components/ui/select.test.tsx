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
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
  SelectGroup,
  SelectLabel,
} from './select';

// Mock Radix UI Select 组件
jest.mock('@radix-ui/react-select', () => ({
  Root: ({ children, ...props }: { children: React.ReactNode; [key: string]: unknown }) => (
    <div data-testid="select-root" {...props}>{children}</div>
  ),
  Trigger: ({ children, className, ...props }: { children: React.ReactNode; className?: string; [key: string]: unknown }) => (
    <button data-testid="select-trigger" className={className} {...props}>
      {children}
    </button>
  ),
  Value: ({ placeholder }: { placeholder?: string }) => (
    <span data-testid="select-value">{placeholder}</span>
  ),
  Icon: ({ children }: { children: React.ReactNode }) => (
    <span data-testid="select-icon">{children}</span>
  ),
  Portal: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="select-portal">{children}</div>
  ),
  Content: ({ children, className, ...props }: { children: React.ReactNode; className?: string; [key: string]: unknown }) => (
    <div data-testid="select-content" className={className} {...props}>
      {children}
    </div>
  ),
  Viewport: ({ children, className, ...props }: { children: React.ReactNode; className?: string; [key: string]: unknown }) => (
    <div data-testid="select-viewport" className={className} {...props}>
      {children}
    </div>
  ),
  Item: ({ children, className, ...props }: { children: React.ReactNode; className?: string; [key: string]: unknown }) => (
    <div data-testid="select-item" className={className} {...props}>
      {children}
    </div>
  ),
  ItemText: ({ children }: { children: React.ReactNode }) => (
    <span data-testid="select-item-text">{children}</span>
  ),
  ItemIndicator: ({ children }: { children: React.ReactNode }) => (
    <span data-testid="select-item-indicator">{children}</span>
  ),
  Group: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="select-group">{children}</div>
  ),
  Label: ({ children, className }: { children: React.ReactNode; className?: string }) => (
    <div data-testid="select-label" className={className}>
      {children}
    </div>
  ),
  Separator: ({ className }: { className?: string }) => (
    <div data-testid="select-separator" className={className} />
  ),
  ScrollUpButton: ({ className }: { className?: string }) => (
    <div data-testid="select-scroll-up" className={className} />
  ),
  ScrollDownButton: ({ className }: { className?: string }) => (
    <div data-testid="select-scroll-down" className={className} />
  ),
}));

// Mock lucide-react 图标
jest.mock('lucide-react', () => ({
  ChevronDownIcon: () => <span data-testid="chevron-down">▼</span>,
  ChevronUpIcon: () => <span data-testid="chevron-up">▲</span>,
  CheckIcon: () => <span data-testid="check-icon">✓</span>,
}));

describe('Select Components', () => {
  describe('Select Root', () => {
    // 测试 Select 根组件渲染
    it('应该渲染 Select 根组件', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue placeholder="选择选项" />
          </SelectTrigger>
        </Select>
      );
      
      expect(screen.getByTestId('select-root')).toBeInTheDocument();
    });

    // 测试 data-slot 属性
    it('应该有正确的 data-slot 属性', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
        </Select>
      );
      
      const root = screen.getByTestId('select-root');
      expect(root).toHaveAttribute('data-slot', 'select');
    });
  });

  describe('SelectTrigger', () => {
    // 测试触发器基本渲染
    it('应该渲染触发器按钮', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue placeholder="选择" />
          </SelectTrigger>
        </Select>
      );
      
      expect(screen.getByTestId('select-trigger')).toBeInTheDocument();
    });

    // 测试默认大小
    it('应该有默认大小', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
        </Select>
      );
      
      const trigger = screen.getByTestId('select-trigger');
      expect(trigger).toHaveAttribute('data-size', 'default');
    });

    // 测试小尺寸
    it('应该支持小尺寸', () => {
      render(
        <Select>
          <SelectTrigger size="sm">
            <SelectValue />
          </SelectTrigger>
        </Select>
      );
      
      const trigger = screen.getByTestId('select-trigger');
      expect(trigger).toHaveAttribute('data-size', 'sm');
    });

    // 测试自定义类名
    it('应该支持自定义类名', () => {
      const customClass = 'custom-trigger';
      render(
        <Select>
          <SelectTrigger className={customClass}>
            <SelectValue />
          </SelectTrigger>
        </Select>
      );
      
      const trigger = screen.getByTestId('select-trigger');
      expect(trigger).toHaveClass(customClass);
    });

    // 测试包含图标
    it('应该包含下拉箭头图标', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
        </Select>
      );
      
      expect(screen.getByTestId('chevron-down')).toBeInTheDocument();
    });
  });

  describe('SelectValue', () => {
    // 测试占位符显示
    it('应该显示占位符', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue placeholder="请选择" />
          </SelectTrigger>
        </Select>
      );
      
      expect(screen.getByText('请选择')).toBeInTheDocument();
    });
  });

  describe('SelectContent', () => {
    // 测试内容渲染
    it('应该渲染选择内容', () => {
      render(
        <Select>
          <SelectContent>
            <SelectItem value="1">选项1</SelectItem>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByTestId('select-content')).toBeInTheDocument();
    });

    // 测试包含视口
    it('应该包含视口元素', () => {
      render(
        <Select>
          <SelectContent>
            <SelectItem value="1">选项1</SelectItem>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByTestId('select-viewport')).toBeInTheDocument();
    });

    // 测试自定义类名
    it('应该支持自定义类名', () => {
      const customClass = 'custom-content';
      render(
        <Select>
          <SelectContent className={customClass}>
            <SelectItem value="1">选项1</SelectItem>
          </SelectContent>
        </Select>
      );
      
      const content = screen.getByTestId('select-content');
      expect(content).toHaveClass(customClass);
    });
  });

  describe('SelectItem', () => {
    // 测试选项渲染
    it('应该渲染选项', () => {
      render(
        <Select>
          <SelectContent>
            <SelectItem value="option1">选项1</SelectItem>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByTestId('select-item')).toBeInTheDocument();
    });

    // 测试选项文本
    it('应该显示选项文本', () => {
      render(
        <Select>
          <SelectContent>
            <SelectItem value="option1">测试选项</SelectItem>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByText('测试选项')).toBeInTheDocument();
    });
  });

  describe('SelectGroup 和 SelectLabel', () => {
    // 测试分组渲染
    it('应该渲染选项组', () => {
      render(
        <Select>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>分组标签</SelectLabel>
              <SelectItem value="1">选项1</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByTestId('select-group')).toBeInTheDocument();
    });

    // 测试分组标签
    it('应该显示分组标签', () => {
      render(
        <Select>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>测试分组</SelectLabel>
              <SelectItem value="1">选项1</SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByText('测试分组')).toBeInTheDocument();
    });
  });

  describe('完整 Select 示例', () => {
    // 测试完整的下拉选择器
    it('应该渲染完整的下拉选择器', () => {
      render(
        <Select>
          <SelectTrigger>
            <SelectValue placeholder="选择一个选项" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="option1">选项1</SelectItem>
            <SelectItem value="option2">选项2</SelectItem>
            <SelectItem value="option3">选项3</SelectItem>
          </SelectContent>
        </Select>
      );
      
      expect(screen.getByTestId('select-root')).toBeInTheDocument();
      expect(screen.getByText('选择一个选项')).toBeInTheDocument();
      expect(screen.getAllByTestId('select-item')).toHaveLength(3);
    });
  });
});
