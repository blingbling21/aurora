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
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';
import { Input } from './input';

describe('Input', () => {
  // 测试基本渲染
  describe('渲染', () => {
    it('应该正确渲染输入框', () => {
      render(<Input />);
      const input = screen.getByRole('textbox');
      expect(input).toBeInTheDocument();
    });

    it('应该支持自定义类名', () => {
      render(<Input className="custom-class" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveClass('custom-class');
    });

    it('应该有data-slot属性', () => {
      render(<Input />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('data-slot', 'input');
    });
  });

  // 测试不同类型
  describe('类型', () => {
    it('应该支持text类型', () => {
      render(<Input type="text" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('type', 'text');
    });

    it('应该支持email类型', () => {
      render(<Input type="email" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('type', 'email');
    });

    it('应该支持password类型', () => {
      render(<Input type="password" />);
      const input = document.querySelector('input[type="password"]');
      expect(input).toBeInTheDocument();
      expect(input).toHaveAttribute('type', 'password');
    });

    it('应该支持number类型', () => {
      render(<Input type="number" />);
      const input = screen.getByRole('spinbutton');
      expect(input).toHaveAttribute('type', 'number');
    });
  });

  // 测试占位符
  describe('占位符', () => {
    it('应该显示占位符文本', () => {
      render(<Input placeholder="请输入内容" />);
      const input = screen.getByPlaceholderText('请输入内容');
      expect(input).toBeInTheDocument();
    });
  });

  // 测试值
  describe('值', () => {
    it('应该显示默认值', () => {
      render(<Input defaultValue="默认文本" />);
      const input = screen.getByRole('textbox') as HTMLInputElement;
      expect(input.value).toBe('默认文本');
    });

    it('应该支持受控组件', () => {
      const { rerender } = render(<Input value="初始值" onChange={() => {}} />);
      const input = screen.getByRole('textbox') as HTMLInputElement;
      expect(input.value).toBe('初始值');

      rerender(<Input value="新值" onChange={() => {}} />);
      expect(input.value).toBe('新值');
    });
  });

  // 测试交互
  describe('用户交互', () => {
    it('应该响应用户输入', async () => {
      const user = userEvent.setup();
      const handleChange = jest.fn();
      render(<Input onChange={handleChange} />);
      
      const input = screen.getByRole('textbox');
      await user.type(input, 'test');
      
      expect(handleChange).toHaveBeenCalled();
      expect((input as HTMLInputElement).value).toBe('test');
    });

    it('应该支持清空输入', async () => {
      const user = userEvent.setup();
      render(<Input defaultValue="初始内容" />);
      
      const input = screen.getByRole('textbox') as HTMLInputElement;
      expect(input.value).toBe('初始内容');
      
      await user.clear(input);
      expect(input.value).toBe('');
    });
  });

  // 测试禁用状态
  describe('禁用状态', () => {
    it('应该支持禁用', () => {
      render(<Input disabled />);
      const input = screen.getByRole('textbox');
      expect(input).toBeDisabled();
    });

    it('禁用时不应该响应用户输入', async () => {
      const user = userEvent.setup();
      const handleChange = jest.fn();
      render(<Input disabled onChange={handleChange} />);
      
      const input = screen.getByRole('textbox');
      await user.type(input, 'test');
      
      expect(handleChange).not.toHaveBeenCalled();
    });
  });

  // 测试只读状态
  describe('只读状态', () => {
    it('应该支持只读', () => {
      render(<Input readOnly />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('readOnly');
    });

    it('只读时不应该响应用户输入', async () => {
      const user = userEvent.setup();
      render(<Input readOnly defaultValue="只读内容" />);
      
      const input = screen.getByRole('textbox') as HTMLInputElement;
      const initialValue = input.value;
      
      await user.type(input, 'new text');
      expect(input.value).toBe(initialValue);
    });
  });

  // 测试必填
  describe('必填验证', () => {
    it('应该支持必填属性', () => {
      render(<Input required />);
      const input = screen.getByRole('textbox');
      expect(input).toBeRequired();
    });
  });

  // 测试自动聚焦
  describe('自动聚焦', () => {
    it('应该支持自动聚焦', () => {
      render(<Input autoFocus />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveFocus();
    });
  });

  // 测试名称和ID
  describe('名称和ID', () => {
    it('应该支持name属性', () => {
      render(<Input name="username" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('name', 'username');
    });

    it('应该支持id属性', () => {
      render(<Input id="user-input" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('id', 'user-input');
    });
  });

  // 测试aria属性
  describe('可访问性', () => {
    it('应该支持aria-label', () => {
      render(<Input aria-label="用户名输入框" />);
      const input = screen.getByLabelText('用户名输入框');
      expect(input).toBeInTheDocument();
    });

    it('应该支持aria-describedby', () => {
      render(<Input aria-describedby="help-text" />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('aria-describedby', 'help-text');
    });

    it('应该支持aria-invalid', () => {
      render(<Input aria-invalid />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('aria-invalid');
    });
  });

  // 测试maxLength
  describe('长度限制', () => {
    it('应该支持maxLength属性', () => {
      render(<Input maxLength={10} />);
      const input = screen.getByRole('textbox');
      expect(input).toHaveAttribute('maxLength', '10');
    });
  });

  // 测试样式类
  describe('样式', () => {
    it('应该包含基础样式类', () => {
      render(<Input />);
      const input = screen.getByRole('textbox');
      expect(input.className).toContain('rounded-md');
      expect(input.className).toContain('border');
    });

    it('应该支持自定义样式与默认样式合并', () => {
      render(<Input className="my-custom-class" />);
      const input = screen.getByRole('textbox');
      expect(input.className).toContain('my-custom-class');
      expect(input.className).toContain('rounded-md');
    });
  });
});
