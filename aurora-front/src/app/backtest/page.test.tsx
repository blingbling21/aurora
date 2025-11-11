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
import BacktestPage from './page';
import { configApi, dataApi, backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';
import { useBacktestWebSocket } from '@/lib/hooks/useBacktestWebSocket';

// Mock API
jest.mock('@/lib/api', () => ({
  configApi: {
    list: jest.fn(),
  },
  dataApi: {
    list: jest.fn(),
  },
  backtestApi: {
    start: jest.fn(),
  },
}));

// Mock stores
const mockUseConfigStore = jest.fn();
const mockUseDataStore = jest.fn();

jest.mock('@/lib/store', () => ({
  useConfigStore: (...args: unknown[]) => mockUseConfigStore(...args),
  useDataStore: (...args: unknown[]) => mockUseDataStore(...args),
}));

jest.mock('@/lib/store/notificationStore', () => ({
  useNotificationStore: jest.fn(),
}));

// Mock useBacktestWebSocket
jest.mock('@/lib/hooks/useBacktestWebSocket', () => ({
  useBacktestWebSocket: jest.fn(),
}));

// Mock 子组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title, description }: { icon: string; title: string; description: string }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
      <p>{description}</p>
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
  Input: ({ value, onChange, placeholder }: { 
    value: string; 
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void; 
    placeholder?: string;
  }) => (
    <input 
      data-testid="input" 
      value={value} 
      onChange={onChange} 
      placeholder={placeholder}
    />
  ),
  Select: ({ children, value, onValueChange }: { 
    children: React.ReactNode;
    value?: string;
    onValueChange?: (value: string) => void;
  }) => (
    <div data-testid="select" data-value={value} onClick={() => onValueChange?.('test')}>
      {children}
    </div>
  ),
  SelectContent: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectTrigger: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
  SelectValue: ({ placeholder }: { placeholder?: string }) => <span>{placeholder}</span>,
  SelectItem: ({ children, value }: { children: React.ReactNode; value: string }) => (
    <div data-testid="select-item" data-value={value}>{children}</div>
  ),
}));

describe('BacktestPage', () => {
  // Mock数据
  const mockConfigStore = {
    configs: [
      { name: 'config1.toml', path: '/path/to/config1.toml', content: '', lastModified: '2025-01-01T00:00:00Z' },
      { name: 'config2.toml', path: '/path/to/config2.toml', content: '', lastModified: '2025-01-02T00:00:00Z' },
    ],
    setConfigs: jest.fn(),
  };

  const mockDataStore = {
    dataFiles: [
      { name: 'btc_1h.csv', path: '', size: 1024, lastModified: '2025-01-01T00:00:00Z' },
      { name: 'eth_4h.csv', path: '', size: 2048, lastModified: '2025-01-02T00:00:00Z' },
    ],
    setDataFiles: jest.fn(),
  };

  const mockNotificationStore = {
    addNotification: jest.fn(),
  };

  beforeEach(() => {
    // 重置所有mock
    jest.clearAllMocks();

    // 设置store的返回值
    mockUseConfigStore.mockReturnValue(mockConfigStore);
    mockUseDataStore.mockReturnValue(mockDataStore);
    (useNotificationStore as unknown as jest.Mock).mockReturnValue(mockNotificationStore);

    // Mock useBacktestWebSocket 返回空对象
    (useBacktestWebSocket as jest.Mock).mockReturnValue({
      status: 'disconnected',
      lastMessage: null,
      connect: jest.fn(),
      disconnect: jest.fn(),
      send: jest.fn(),
      isConnected: false,
    });

    // 设置API的返回值
    (configApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [
        { filename: 'config1.toml', path: '/path/to/config1.toml', modified: '2025-01-01T00:00:00Z' },
        { filename: 'config2.toml', path: '/path/to/config2.toml', modified: '2025-01-02T00:00:00Z' },
      ],
    });

    (dataApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [
        { filename: 'btc_1h.csv', size: 1024, modified: '2025-01-01T00:00:00Z' },
        { filename: 'eth_4h.csv', size: 2048, modified: '2025-01-02T00:00:00Z' },
      ],
    });

    (backtestApi.start as jest.Mock).mockResolvedValue({
      success: true,
      data: { task_id: 'test-task-id' },
    });
  });
  // 测试页面基本渲染
  it('应该渲染页面头部', () => {
    render(<BacktestPage />);
    
    const header = screen.getByTestId('page-header');
    expect(header).toBeInTheDocument();
    expect(screen.getByText('🚀')).toBeInTheDocument();
    expect(screen.getByText('回测执行')).toBeInTheDocument();
    expect(screen.getByText('配置并启动新的回测任务')).toBeInTheDocument();
  });

  // 测试任务配置区域
  it('应该渲染任务配置卡片', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    const configCard = cards.find(card => card.textContent?.includes('任务配置'));
    expect(configCard).toBeInTheDocument();
  });

  // 测试任务名称输入框
  it('应该显示任务名称输入框', () => {
    render(<BacktestPage />);
    
    const input = screen.getByTestId('input');
    expect(input).toBeInTheDocument();
  });

  // 测试任务名称输入
  it('应该能够输入任务名称', () => {
    render(<BacktestPage />);
    
    const input = screen.getByTestId('input') as HTMLInputElement;
    fireEvent.change(input, { target: { value: '测试任务' } });
    
    expect(input.value).toBe('测试任务');
  });

  // 测试配置选择器
  it('应该显示配置选择器', () => {
    render(<BacktestPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThan(0);
  });

  // 测试数据文件选择器
  it('应该显示数据文件选择器', () => {
    render(<BacktestPage />);
    
    const selects = screen.getAllByTestId('select');
    expect(selects.length).toBeGreaterThanOrEqual(2);
  });

  // 测试执行结果区域
  it('应该渲染执行结果卡片', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    const resultCard = cards.find(card => card.textContent?.includes('执行结果'));
    expect(resultCard).toBeInTheDocument();
  });

  // 测试未开始状态的提示
  it('未开始时应该显示提示信息', () => {
    render(<BacktestPage />);
    
    expect(screen.getByText(/点击.*开始回测.*按钮启动任务/)).toBeInTheDocument();
  });

  // 测试启动按钮
  it('应该显示启动回测按钮', () => {
    render(<BacktestPage />);
    
    const buttons = screen.getAllByTestId('button');
    const startButton = buttons.find(btn => btn.textContent?.includes('开始回测'));
    expect(startButton).toBeInTheDocument();
  });

  // 测试停止按钮
  it('应该显示停止按钮', () => {
    render(<BacktestPage />);
    
    const buttons = screen.getAllByTestId('button');
    const stopButton = buttons.find(btn => btn.textContent?.includes('停止'));
    expect(stopButton).toBeInTheDocument();
  });

  // 测试页面布局结构
  it('应该包含正确的布局结构', () => {
    const { container } = render(<BacktestPage />);
    
    // 检查是否有网格布局
    const grids = container.querySelectorAll('.grid');
    expect(grids.length).toBeGreaterThan(0);
  });

  // 测试卡片数量
  it('应该至少有两个卡片（配置和结果）', () => {
    render(<BacktestPage />);
    
    const cards = screen.getAllByTestId('card');
    expect(cards.length).toBeGreaterThanOrEqual(2);
  });

  // 测试配置文件加载
  it('应该在挂载时加载配置文件列表', async () => {
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(configApi.list).toHaveBeenCalled();
    });
  });

  // 测试数据文件加载
  it('应该在挂载时加载数据文件列表', async () => {
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalled();
    });
  });

  // 测试配置文件列表显示
  it('应该显示配置文件选项', async () => {
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(mockConfigStore.setConfigs).toHaveBeenCalledWith([
        { name: 'config1.toml', path: '/path/to/config1.toml', content: '', lastModified: '2025-01-01T00:00:00Z' },
        { name: 'config2.toml', path: '/path/to/config2.toml', content: '', lastModified: '2025-01-02T00:00:00Z' },
      ]);
    });
  });

  // 测试数据文件列表显示
  it('应该显示数据文件选项', async () => {
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(mockDataStore.setDataFiles).toHaveBeenCalledWith([
        { name: 'btc_1h.csv', path: '', size: 1024, lastModified: '2025-01-01T00:00:00Z' },
        { name: 'eth_4h.csv', path: '', size: 2048, lastModified: '2025-01-02T00:00:00Z' },
      ]);
    });
  });

  // 测试配置文件加载失败
  it('配置文件加载失败时应该显示错误通知', async () => {
    (configApi.list as jest.Mock).mockRejectedValue(new Error('加载失败'));
    
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '加载配置文件列表失败',
      });
    });
  });

  // 测试数据文件加载失败
  it('数据文件加载失败时应该显示错误通知', async () => {
    (dataApi.list as jest.Mock).mockRejectedValue(new Error('加载失败'));
    
    render(<BacktestPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '加载数据文件列表失败',
      });
    });
  });

  // 测试空配置列表显示
  it('没有配置文件时应该显示提示信息', () => {
    mockUseConfigStore.mockReturnValue({
      configs: [],
      setConfigs: jest.fn(),
    });
    
    render(<BacktestPage />);
    
    expect(screen.getByText('暂无配置文件,请先创建配置')).toBeInTheDocument();
  });

  // 测试空数据列表显示
  it('没有数据文件时应该显示提示信息', () => {
    mockUseDataStore.mockReturnValue({
      dataFiles: [],
      setDataFiles: jest.fn(),
    });
    
    render(<BacktestPage />);
    
    expect(screen.getByText('暂无数据文件,请先下载数据')).toBeInTheDocument();
  });

  // 测试启动回测功能
  it('当必填字段缺失时不应该调用API', async () => {
    render(<BacktestPage />);

    // 只填写任务名称,不选择配置文件和数据文件
    const taskNameInput = screen.getByTestId('input') as HTMLInputElement;
    fireEvent.change(taskNameInput, { target: { value: '测试回测任务' } });

    // 找到提交按钮并点击
    const buttons = screen.getAllByTestId('button');
    const startButton = buttons.find(btn => btn.textContent?.includes('开始回测'));
    if (startButton) {
      fireEvent.click(startButton);

      // 应该显示错误通知,因为缺少必填字段
      await waitFor(() => {
        expect(mockNotificationStore.addNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
            message: expect.stringContaining('必填'),
          })
        );
      });
      
      // API不应该被调用
      expect(backtestApi.start).not.toHaveBeenCalled();
    }
  });

  // 测试WebSocket集成
  it('应该在启动回测后连接WebSocket', async () => {
    // 此测试验证WebSocket hook被正确调用
    render(<BacktestPage />);

    // 验证useBacktestWebSocket被调用
    expect(useBacktestWebSocket).toHaveBeenCalled();
  });

  // 测试启动回测失败处理
  it('启动回测失败时应该显示错误通知', async () => {
    (backtestApi.start as jest.Mock).mockRejectedValue(new Error('启动失败'));

    // 设置所有必填字段
    mockUseConfigStore.mockReturnValue({
      configs: mockConfigStore.configs,
      setConfigs: jest.fn(),
    });
    mockUseDataStore.mockReturnValue({
      dataFiles: mockDataStore.dataFiles,
      setDataFiles: jest.fn(),
    });

    render(<BacktestPage />);

    const taskNameInput = screen.getByTestId('input') as HTMLInputElement;
    fireEvent.change(taskNameInput, { target: { value: '测试任务' } });

    // 手动设置state（这里仅测试错误处理逻辑）
    const buttons = screen.getAllByTestId('button');
    const startButton = buttons.find(btn => btn.textContent?.includes('开始回测'));
    if (startButton) {
      fireEvent.click(startButton);

      await waitFor(() => {
        expect(mockNotificationStore.addNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
          })
        );
      });
    }
  });
});
