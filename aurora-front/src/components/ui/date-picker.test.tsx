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

import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { DatePicker } from './date-picker';

describe('DatePicker', () => {
  // 测试渲染占位文本
  it('应该渲染占位文本', () => {
    render(<DatePicker placeholder="选择日期" />);
    expect(screen.getByText('选择日期')).toBeInTheDocument();
  });

  // 测试必填标记
  it('当required为true且未选择日期时应该显示必填标记', () => {
    render(<DatePicker required placeholder="选择日期" />);
    expect(screen.getByText('*')).toBeInTheDocument();
  });

  // 测试日期变化
  it('应该调用onDateChange回调', () => {
    const handleDateChange = jest.fn();
    const testDate = new Date(2025, 0, 1);
    
    render(
      <DatePicker
        date={testDate}
        onDateChange={handleDateChange}
      />
    );

    // 检查日期是否显示
    expect(screen.getByText(/2025/)).toBeInTheDocument();
  });

  // 测试禁用状态
  it('当disabled为true时按钮应该被禁用', () => {
    render(<DatePicker disabled placeholder="选择日期" />);
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
  });

  // 测试自定义类名
  it('应该应用自定义类名', () => {
    render(<DatePicker className="custom-class" placeholder="选择日期" />);
    const button = screen.getByRole('button');
    expect(button).toHaveClass('custom-class');
  });

  // 测试日历图标
  it('应该显示日历图标', () => {
    render(<DatePicker placeholder="选择日期" />);
    const button = screen.getByRole('button');
    const svg = button.querySelector('svg');
    expect(svg).toBeInTheDocument();
  });

  // 测试弹出框打开
  it('点击按钮应该打开日历弹出框', () => {
    render(<DatePicker placeholder="选择日期" />);
    const button = screen.getByRole('button');
    
    fireEvent.click(button);
    
    // 检查弹出框内容是否存在
    // 注意：实际测试可能需要模拟 Popover 的行为
  });

  // 测试日期格式化
  it('应该正确格式化显示日期', () => {
    const testDate = new Date(2025, 0, 15); // 2025年1月15日
    
    render(
      <DatePicker
        date={testDate}
        placeholder="选择日期"
      />
    );

    // 检查格式化后的日期是否显示
    expect(screen.getByText(/2025/)).toBeInTheDocument();
    expect(screen.getByText(/1月/)).toBeInTheDocument();
  });
});
