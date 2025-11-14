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

/**
 * BacktestSection 日期选择器功能测试
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BacktestSection } from './ConfigSections';
import type { BacktestSettings } from '@/types/config-schema';

// Mock dataApi
jest.mock('@/lib/api', () => ({
  dataApi: {
    listFiles: jest.fn().mockResolvedValue({ files: [] }),
    list: jest.fn().mockResolvedValue({ files: [] }),
  },
  configApi: {
    list: jest.fn().mockResolvedValue({ configs: [] }),
  },
}));

describe('BacktestSection - DatePicker Integration', () => {
  const mockOnChange = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('应该渲染开始时间日期选择器', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      start_time: '2024-01-01',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 查找开始时间标签
    expect(screen.getByText('开始时间:')).toBeInTheDocument();
    
    // 查找日期选择器按钮（包含日期或占位文本）
    const datePickers = screen.getAllByRole('button');
    expect(datePickers.length).toBeGreaterThan(0);
  });

  it('应该渲染结束时间日期选择器', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      end_time: '2024-12-31',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 查找结束时间标签
    expect(screen.getByText('结束时间:')).toBeInTheDocument();
  });

  it('应该显示已选择的开始日期', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      start_time: '2024-01-01',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 日期选择器应该显示格式化的日期
    // DatePicker 使用 date-fns 的 PPP 格式，中文环境下会显示类似 "2024年1月1日" 的格式
    const dateText = screen.getByText(/2024/);
    expect(dateText).toBeInTheDocument();
  });

  it('应该显示已选择的结束日期', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      end_time: '2024-12-31',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 日期选择器应该显示格式化的日期
    const dateText = screen.getByText(/2024/);
    expect(dateText).toBeInTheDocument();
  });

  it('应该在未选择日期时显示占位文本', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 查找占位文本
    expect(screen.getByText('选择开始日期')).toBeInTheDocument();
    expect(screen.getByText('选择结束日期')).toBeInTheDocument();
  });

  it('应该正确处理日期格式转换', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      start_time: '2024-06-15',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 验证日期能够正确解析和显示
    // 2024-06-15 应该被解析为 Date 对象并显示
    const dateText = screen.getByText(/2024/);
    expect(dateText).toBeInTheDocument();
  });

  it('应该处理无效的日期字符串', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      start_time: 'invalid-date',
    };

    // 组件应该能够处理无效日期而不崩溃
    expect(() => {
      render(<BacktestSection config={config} onChange={mockOnChange} />);
    }).not.toThrow();
  });

  it('应该同时显示开始和结束日期选择器', () => {
    const config: BacktestSettings = {
      data_path: 'test.csv',
      start_time: '2024-01-01',
      end_time: '2024-12-31',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 两个日期都应该显示
    expect(screen.getByText('开始时间:')).toBeInTheDocument();
    expect(screen.getByText('结束时间:')).toBeInTheDocument();
  });

  it('应该在配置为undefined时仍能渲染日期选择器', async () => {
    const { container } = render(<BacktestSection config={undefined} onChange={mockOnChange} />);

    // 当配置为undefined时,整个表单被禁用(只显示标题和开关)
    expect(screen.getByText('回测配置 (可选)')).toBeInTheDocument();
    expect(screen.getByText('已禁用')).toBeInTheDocument();

    // 日期选择器不应该显示
    expect(screen.queryByText('开始时间:')).not.toBeInTheDocument();
    expect(screen.queryByText('结束时间:')).not.toBeInTheDocument();

    // 点击开关启用回测配置
    const switchElement = container.querySelector('button[role="switch"]');
    expect(switchElement).toBeInTheDocument();
    
    if (switchElement) {
      await userEvent.click(switchElement);
      
      // 验证onChange被调用,启用了配置
      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          data_path: '',
          timezone: expect.any(String), // getCurrentTimezone()
        })
      );
    }
  });
});
