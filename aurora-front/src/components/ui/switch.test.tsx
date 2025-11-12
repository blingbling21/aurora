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
import { Switch } from './switch';

describe('Switch', () => {
  // 测试 Switch 组件渲染
  it('should render switch component', () => {
    render(<Switch />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toBeInTheDocument();
  });

  // 测试 Switch 默认状态为未选中
  it('should be unchecked by default', () => {
    render(<Switch />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('data-state', 'unchecked');
  });

  // 测试 Switch 受控组件 - checked 状态
  it('should render checked state when checked prop is true', () => {
    render(<Switch checked={true} />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('data-state', 'checked');
  });

  // 测试 Switch 受控组件 - unchecked 状态
  it('should render unchecked state when checked prop is false', () => {
    render(<Switch checked={false} />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('data-state', 'unchecked');
  });

  // 测试点击事件触发 onCheckedChange
  it('should call onCheckedChange when clicked', () => {
    const mockOnChange = jest.fn();
    render(<Switch onCheckedChange={mockOnChange} />);
    
    const switchElement = screen.getByRole('switch');
    fireEvent.click(switchElement);
    
    expect(mockOnChange).toHaveBeenCalledWith(true);
  });

  // 测试从 checked 到 unchecked 的切换
  it('should call onCheckedChange with false when toggling from checked to unchecked', () => {
    const mockOnChange = jest.fn();
    render(<Switch checked={true} onCheckedChange={mockOnChange} />);
    
    const switchElement = screen.getByRole('switch');
    fireEvent.click(switchElement);
    
    expect(mockOnChange).toHaveBeenCalledWith(false);
  });

  // 测试禁用状态
  it('should be disabled when disabled prop is true', () => {
    render(<Switch disabled={true} />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toBeDisabled();
  });

  // 测试禁用状态下不触发 onCheckedChange
  it('should not call onCheckedChange when disabled and clicked', () => {
    const mockOnChange = jest.fn();
    render(<Switch disabled={true} onCheckedChange={mockOnChange} />);
    
    const switchElement = screen.getByRole('switch');
    fireEvent.click(switchElement);
    
    expect(mockOnChange).not.toHaveBeenCalled();
  });

  // 测试自定义 className
  it('should apply custom className', () => {
    render(<Switch className="custom-class" />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveClass('custom-class');
  });

  // 测试可访问性 - aria 属性
  it('should support aria-label for accessibility', () => {
    render(<Switch aria-label="Toggle setting" />);
    
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('aria-label', 'Toggle setting');
  });
});
