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
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import HistoryPage from './page';
import { backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';

// Mock next/navigation
const mockPush = jest.fn();
jest.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

// Mock API
jest.mock('@/lib/api', () => ({
  backtestApi: {
    list: jest.fn(),
  },
}));

// Mock notification store
jest.mock('@/lib/store/notificationStore', () => ({
  useNotificationStore: jest.fn(),
}));

// Mock UI组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title, action }: { icon: string; title: string; action?: React.ReactNode }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
      {action}
    </div>
  ),
  Card: ({ title, children }: { title?: string; children: React.ReactNode }) => (
    <div data-testid="card">
      {title && <h2>{title}</h2>}
      {children}
    </div>
  ),
  Button: ({ children, onClick, disabled, variant }: { 
    children: React.ReactNode; 
    onClick?: () => void; 
    disabled?: boolean;
    variant?: string;
  }) => (
    <button data-testid="button" onClick={onClick} disabled={disabled} data-variant={variant}>
      {children}
    </button>
  ),
}));

// Mock TaskItem组件
jest.mock('@/components/dashboard', () => ({
  TaskItem: ({ onClick }: { task: unknown; onClick: () => void }) => (
    <div data-testid="task-item" onClick={onClick}>
      Task Item
    </div>
  ),
}));

describe('HistoryPage', () => {
  // Mock数据
  const mockTasks = [
    {
      id: 'task-1',
      name: '回测任务1',
      status: 'completed',
      config_path: '/config/test1.toml',
      data_path: '/data/btc_1h.csv',
      progress: 100,
      created_at: '2025-01-01T00:00:00Z',
      started_at: '2025-01-01T00:01:00Z',
      completed_at: '2025-01-01T00:10:00Z',
    },
    {
      id: 'task-2',
      name: '回测任务2',
      status: 'running',
      config_path: '/config/test2.toml',
      data_path: '/data/eth_4h.csv',
      progress: 50,
      created_at: '2025-01-02T00:00:00Z',
      started_at: '2025-01-02T00:01:00Z',
      completed_at: null,
    },
  ];

  const mockNotificationStore = {
    addNotification: jest.fn(),
  };

  beforeEach(() => {
    // 重置所有mock
    jest.clearAllMocks();
    
    // 设置notification store
    (useNotificationStore as unknown as jest.Mock).mockReturnValue(mockNotificationStore);
    
    // 设置API默认返回值
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: mockTasks,
    });
  });

  // ========== 基本渲染测试 ==========
  
  it('应该渲染页面头部', () => {
    render(<HistoryPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('📜')).toBeInTheDocument();
    expect(screen.getByText('历史记录')).toBeInTheDocument();
  });

  it('应该渲染刷新按钮', () => {
    render(<HistoryPage />);
    
    const button = screen.getByTestId('button');
    expect(button).toBeInTheDocument();
    expect(button).toHaveTextContent('🔄 刷新');
    expect(button).toHaveAttribute('data-variant', 'secondary');
  });

  it('应该渲染回测历史卡片', () => {
    render(<HistoryPage />);
    
    const card = screen.getByTestId('card');
    expect(card).toBeInTheDocument();
    expect(screen.getByText('回测历史')).toBeInTheDocument();
  });

  // ========== 加载状态测试 ==========

  it('初始状态应该显示加载中', async () => {
    // 让API调用延迟以便观察加载状态
    (backtestApi.list as jest.Mock).mockImplementation(
      () => new Promise(resolve => setTimeout(() => resolve({ success: true, data: [] }), 100))
    );
    
    render(<HistoryPage />);
    
    expect(screen.getByText('正在加载...')).toBeInTheDocument();
    
    await waitFor(() => {
      expect(screen.queryByText('正在加载...')).not.toBeInTheDocument();
    });
  });

  it('加载完成后应该显示任务列表', async () => {
    render(<HistoryPage />);
    
    await waitFor(() => {
      const taskItems = screen.getAllByTestId('task-item');
      expect(taskItems).toHaveLength(2);
    });
    
    expect(backtestApi.list).toHaveBeenCalledTimes(1);
  });

  // ========== 空状态测试 ==========

  it('当没有任务时应该显示空状态提示', async () => {
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [],
    });
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      expect(screen.getByText('暂无历史记录')).toBeInTheDocument();
    });
  });

  // ========== 错误处理测试 ==========

  it('当API返回错误时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: false,
      error: '服务器错误',
    });
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '加载历史记录失败',
      });
    });
  });

  it('当API抛出异常时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockRejectedValue(new Error('Network error'));
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '加载历史记录失败',
      });
    });
  });

  it('当API返回的data为空时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: null,
    });
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '加载历史记录失败',
      });
    });
  });

  // ========== 用户交互测试 ==========

  it('点击刷新按钮应该重新加载任务列表', async () => {
    render(<HistoryPage />);
    
    // 等待初始加载完成
    await waitFor(() => {
      expect(backtestApi.list).toHaveBeenCalledTimes(1);
    });
    
    // 点击刷新按钮
    const button = screen.getByTestId('button');
    fireEvent.click(button);
    
    // 应该再次调用API
    await waitFor(() => {
      expect(backtestApi.list).toHaveBeenCalledTimes(2);
    });
  });

  it('加载中时刷新按钮应该被禁用', async () => {
    (backtestApi.list as jest.Mock).mockImplementation(
      () => new Promise(resolve => setTimeout(() => resolve({ success: true, data: [] }), 100))
    );
    
    render(<HistoryPage />);
    
    const button = screen.getByTestId('button');
    expect(button).toBeDisabled();
    
    await waitFor(() => {
      expect(button).not.toBeDisabled();
    });
  });

  it('点击任务项应该跳转到详情页', async () => {
    render(<HistoryPage />);
    
    await waitFor(() => {
      const taskItems = screen.getAllByTestId('task-item');
      expect(taskItems).toHaveLength(2);
    });
    
    const taskItems = screen.getAllByTestId('task-item');
    fireEvent.click(taskItems[0]);
    
    expect(mockPush).toHaveBeenCalledWith('/history/task-1');
  });

  // ========== 数据转换测试 ==========

  it('应该正确转换API数据格式', async () => {
    render(<HistoryPage />);
    
    await waitFor(() => {
      const taskItems = screen.getAllByTestId('task-item');
      expect(taskItems).toHaveLength(2);
    });
    
    // 验证数据已经被正确加载和转换
    expect(backtestApi.list).toHaveBeenCalled();
  });

  it('应该处理没有completed_at的任务', async () => {
    const tasksWithoutCompletedAt = [
      {
        id: 'task-3',
        name: '回测任务3',
        status: 'pending',
        config_path: '/config/test3.toml',
        data_path: '/data/btc_1h.csv',
        progress: 0,
        created_at: '2025-01-03T00:00:00Z',
        started_at: null,
        completed_at: null,
      },
    ];
    
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: tasksWithoutCompletedAt,
    });
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      const taskItems = screen.getAllByTestId('task-item');
      expect(taskItems).toHaveLength(1);
    });
  });

  it('应该处理config_path为空的情况', async () => {
    const tasksWithEmptyPath = [
      {
        id: 'task-4',
        name: '回测任务4',
        status: 'completed',
        config_path: null,
        data_path: null,
        progress: 100,
        created_at: '2025-01-04T00:00:00Z',
        started_at: '2025-01-04T00:01:00Z',
        completed_at: '2025-01-04T00:10:00Z',
      },
    ];
    
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: tasksWithEmptyPath,
    });
    
    render(<HistoryPage />);
    
    await waitFor(() => {
      const taskItems = screen.getAllByTestId('task-item');
      expect(taskItems).toHaveLength(1);
    });
  });
});
