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
import { Textarea } from './textarea';

describe('Textarea', () => {
  // 测试基本渲染
  describe('渲染', () => {
    it('应该正确渲染文本区域', () => {
      render(<Textarea />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toBeInTheDocument();
      expect(textarea.tagName).toBe('TEXTAREA');
    });

    it('应该支持自定义类名', () => {
      render(<Textarea className="custom-class" />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveClass('custom-class');
    });

    it('应该有data-slot属性', () => {
      render(<Textarea />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('data-slot', 'textarea');
    });
  });

  // 测试占位符
  describe('占位符', () => {
    it('应该显示占位符文本', () => {
      render(<Textarea placeholder="请输入多行文本" />);
      const textarea = screen.getByPlaceholderText('请输入多行文本');
      expect(textarea).toBeInTheDocument();
    });
  });

  // 测试值
  describe('值', () => {
    it('应该显示默认值', () => {
      render(<Textarea defaultValue="默认文本内容" />);
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      expect(textarea.value).toBe('默认文本内容');
    });

    it('应该支持受控组件', () => {
      const { rerender } = render(<Textarea value="初始值" onChange={() => {}} />);
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      expect(textarea.value).toBe('初始值');

      rerender(<Textarea value="新值" onChange={() => {}} />);
      expect(textarea.value).toBe('新值');
    });

    it('应该支持多行文本', () => {
      const multilineText = '第一行\n第二行\n第三行';
      render(<Textarea defaultValue={multilineText} />);
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      expect(textarea.value).toBe(multilineText);
    });
  });

  // 测试交互
  describe('用户交互', () => {
    it('应该响应用户输入', async () => {
      const user = userEvent.setup();
      const handleChange = jest.fn();
      render(<Textarea onChange={handleChange} />);
      
      const textarea = screen.getByRole('textbox');
      await user.type(textarea, 'test content');
      
      expect(handleChange).toHaveBeenCalled();
      expect((textarea as HTMLTextAreaElement).value).toBe('test content');
    });

    it('应该支持多行输入', async () => {
      const user = userEvent.setup();
      render(<Textarea />);
      
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      await user.type(textarea, '第一行{Enter}第二行{Enter}第三行');
      
      expect(textarea.value).toContain('\n');
    });

    it('应该支持清空输入', async () => {
      const user = userEvent.setup();
      render(<Textarea defaultValue="初始内容" />);
      
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      expect(textarea.value).toBe('初始内容');
      
      await user.clear(textarea);
      expect(textarea.value).toBe('');
    });
  });

  // 测试禁用状态
  describe('禁用状态', () => {
    it('应该支持禁用', () => {
      render(<Textarea disabled />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toBeDisabled();
    });

    it('禁用时不应该响应用户输入', async () => {
      const user = userEvent.setup();
      const handleChange = jest.fn();
      render(<Textarea disabled onChange={handleChange} />);
      
      const textarea = screen.getByRole('textbox');
      await user.type(textarea, 'test');
      
      expect(handleChange).not.toHaveBeenCalled();
    });
  });

  // 测试只读状态
  describe('只读状态', () => {
    it('应该支持只读', () => {
      render(<Textarea readOnly />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('readOnly');
    });

    it('只读时不应该响应用户输入', async () => {
      const user = userEvent.setup();
      render(<Textarea readOnly defaultValue="只读内容" />);
      
      const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;
      const initialValue = textarea.value;
      
      await user.type(textarea, 'new text');
      expect(textarea.value).toBe(initialValue);
    });
  });

  // 测试必填
  describe('必填验证', () => {
    it('应该支持必填属性', () => {
      render(<Textarea required />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toBeRequired();
    });
  });

  // 测试自动聚焦
  describe('自动聚焦', () => {
    it('应该支持自动聚焦', () => {
      render(<Textarea autoFocus />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveFocus();
    });
  });

  // 测试行列数
  describe('尺寸', () => {
    it('应该支持rows属性', () => {
      render(<Textarea rows={10} />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('rows', '10');
    });

    it('应该支持cols属性', () => {
      render(<Textarea cols={50} />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('cols', '50');
    });
  });

  // 测试名称和ID
  describe('名称和ID', () => {
    it('应该支持name属性', () => {
      render(<Textarea name="description" />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('name', 'description');
    });

    it('应该支持id属性', () => {
      render(<Textarea id="desc-input" />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('id', 'desc-input');
    });
  });

  // 测试长度限制
  describe('长度限制', () => {
    it('应该支持maxLength属性', () => {
      render(<Textarea maxLength={500} />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('maxLength', '500');
    });
  });

  // 测试可访问性
  describe('可访问性', () => {
    it('应该支持aria-label', () => {
      render(<Textarea aria-label="描述输入框" />);
      const textarea = screen.getByLabelText('描述输入框');
      expect(textarea).toBeInTheDocument();
    });

    it('应该支持aria-describedby', () => {
      render(<Textarea aria-describedby="help-text" />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('aria-describedby', 'help-text');
    });

    it('应该支持aria-invalid', () => {
      render(<Textarea aria-invalid />);
      const textarea = screen.getByRole('textbox');
      expect(textarea).toHaveAttribute('aria-invalid');
    });
  });

  // 测试样式类
  describe('样式', () => {
    it('应该包含基础样式类', () => {
      render(<Textarea />);
      const textarea = screen.getByRole('textbox');
      expect(textarea.className).toContain('rounded-md');
      expect(textarea.className).toContain('border');
      expect(textarea.className).toContain('min-h-16');
    });

    it('应该支持自定义样式与默认样式合并', () => {
      render(<Textarea className="my-custom-class" />);
      const textarea = screen.getByRole('textbox');
      expect(textarea.className).toContain('my-custom-class');
      expect(textarea.className).toContain('rounded-md');
    });
  });
});
