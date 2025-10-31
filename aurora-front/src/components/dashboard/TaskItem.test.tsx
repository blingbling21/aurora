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
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import { TaskItem } from './TaskItem';
import { BacktestTask } from '@/types';

describe('TaskItem 组件', () => {
  // 创建模拟任务数据
  const mockTaskPending: BacktestTask = {
    id: '1',
    name: '测试任务 - 待处理',
    status: 'pending',
    config: 'config1.toml',
    dataFile: 'btc_1h.csv',
    progress: 0,
    createdAt: '2025-01-01T10:00:00Z',
    updatedAt: '2025-01-01T10:00:00Z',
  };

  const mockTaskRunning: BacktestTask = {
    id: '2',
    name: '测试任务 - 运行中',
    status: 'running',
    config: 'config2.toml',
    dataFile: 'eth_4h.csv',
    progress: 45,
    createdAt: '2025-01-01T11:00:00Z',
    updatedAt: '2025-01-01T11:30:00Z',
  };

  const mockTaskCompleted: BacktestTask = {
    id: '3',
    name: '测试任务 - 已完成',
    status: 'completed',
    config: 'config3.toml',
    dataFile: 'bnb_1d.csv',
    progress: 100,
    createdAt: '2025-01-01T09:00:00Z',
    updatedAt: '2025-01-01T09:30:00Z',
  };

  const mockTaskFailed: BacktestTask = {
    id: '4',
    name: '测试任务 - 失败',
    status: 'failed',
    config: 'config4.toml',
    dataFile: 'sol_4h.csv',
    progress: 75,
    createdAt: '2025-01-01T08:00:00Z',
    updatedAt: '2025-01-01T08:45:00Z',
  };

  // 测试基础渲染
  it('应该正确渲染任务信息', () => {
    render(<TaskItem task={mockTaskPending} />);
    
    // 验证任务名称
    expect(screen.getByText('测试任务 - 待处理')).toBeInTheDocument();
    
    // 验证配置文件
    expect(screen.getByText(/config1.toml/)).toBeInTheDocument();
    
    // 验证数据文件
    expect(screen.getByText(/btc_1h.csv/)).toBeInTheDocument();
  });

  // 测试待处理状态
  it('应该正确显示待处理状态', () => {
    render(<TaskItem task={mockTaskPending} />);
    expect(screen.getByText('待处理')).toBeInTheDocument();
    expect(screen.getByText('待处理')).toHaveClass('bg-yellow-100', 'text-yellow-800');
  });

  // 测试运行中状态
  it('应该正确显示运行中状态', () => {
    render(<TaskItem task={mockTaskRunning} />);
    expect(screen.getByText('运行中')).toBeInTheDocument();
    expect(screen.getByText('运行中')).toHaveClass('bg-blue-100', 'text-blue-800');
  });

  // 测试已完成状态
  it('应该正确显示已完成状态', () => {
    render(<TaskItem task={mockTaskCompleted} />);
    expect(screen.getByText('已完成')).toBeInTheDocument();
    expect(screen.getByText('已完成')).toHaveClass('bg-green-100', 'text-green-800');
  });

  // 测试失败状态
  it('应该正确显示失败状态', () => {
    render(<TaskItem task={mockTaskFailed} />);
    expect(screen.getByText('失败')).toBeInTheDocument();
    expect(screen.getByText('失败')).toHaveClass('bg-red-100', 'text-red-800');
  });

  // 测试进度条显示（运行中）
  it('应该在运行中状态显示进度条', () => {
    render(<TaskItem task={mockTaskRunning} />);
    
    // 验证进度文本
    expect(screen.getByText('进度')).toBeInTheDocument();
    expect(screen.getByText('45%')).toBeInTheDocument();
  });

  // 测试进度条不显示（非运行中状态）
  it('应该在非运行中状态隐藏进度条', () => {
    const { rerender } = render(<TaskItem task={mockTaskPending} />);
    expect(screen.queryByText('进度')).not.toBeInTheDocument();

    rerender(<TaskItem task={mockTaskCompleted} />);
    expect(screen.queryByText('进度')).not.toBeInTheDocument();

    rerender(<TaskItem task={mockTaskFailed} />);
    expect(screen.queryByText('进度')).not.toBeInTheDocument();
  });

  // 测试点击事件
  it('应该触发 onClick 回调', async () => {
    const handleClick = jest.fn();
    const user = userEvent.setup();
    
    render(<TaskItem task={mockTaskPending} onClick={handleClick} />);
    
    const taskElement = screen.getByText('测试任务 - 待处理').closest('div');
    await user.click(taskElement!);
    
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  // 测试没有 onClick 回调的情况
  it('应该在没有 onClick 时正常渲染', () => {
    render(<TaskItem task={mockTaskPending} />);
    expect(screen.getByText('测试任务 - 待处理')).toBeInTheDocument();
  });

  // 测试悬停样式
  it('应该包含悬停样式类名', () => {
    const { container } = render(<TaskItem task={mockTaskPending} />);
    const taskElement = container.firstChild as HTMLElement;
    
    expect(taskElement).toHaveClass('hover:border-blue-500', 'hover:shadow-sm', 'cursor-pointer');
  });

  // 测试日期格式化
  it('应该正确格式化创建时间', () => {
    render(<TaskItem task={mockTaskPending} />);
    // 验证包含"创建:"文本（具体格式取决于本地化设置）
    expect(screen.getByText(/创建:/)).toBeInTheDocument();
  });

  // 测试进度为 0 的运行中任务
  it('应该正确显示进度为 0% 的运行中任务', () => {
    const runningTask: BacktestTask = {
      ...mockTaskRunning,
      progress: 0,
    };
    render(<TaskItem task={runningTask} />);
    expect(screen.getByText('0%')).toBeInTheDocument();
  });

  // 测试进度为 100 的运行中任务
  it('应该正确显示进度为 100% 的运行中任务', () => {
    const runningTask: BacktestTask = {
      ...mockTaskRunning,
      progress: 100,
    };
    render(<TaskItem task={runningTask} />);
    expect(screen.getByText('100%')).toBeInTheDocument();
  });

  // 测试多个元信息同时显示
  it('应该同时显示配置、数据和创建时间', () => {
    render(<TaskItem task={mockTaskPending} />);
    
    expect(screen.getByText(/配置: config1.toml/)).toBeInTheDocument();
    expect(screen.getByText(/数据: btc_1h.csv/)).toBeInTheDocument();
    expect(screen.getByText(/创建:/)).toBeInTheDocument();
  });
});
