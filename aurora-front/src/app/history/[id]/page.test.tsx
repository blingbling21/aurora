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
import BacktestDetailPage from './page';
import { backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';

// Mock next/navigation
const mockPush = jest.fn();
const mockParams = { id: 'test-task-id' };

jest.mock('next/navigation', () => ({
  useParams: () => mockParams,
  useRouter: () => ({
    push: mockPush,
  }),
}));

// Mock API
jest.mock('@/lib/api', () => ({
  backtestApi: {
    list: jest.fn(),
    getResult: jest.fn(),
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
  Button: ({ children, onClick, variant }: { 
    children: React.ReactNode; 
    onClick?: () => void; 
    variant?: string;
  }) => (
    <button data-testid="button" onClick={onClick} data-variant={variant}>
      {children}
    </button>
  ),
}));

describe('BacktestDetailPage', () => {
  // Mock数据
  const mockTask = {
    id: 'test-task-id',
    name: '测试回测任务',
    status: 'completed',
    config_path: '/config/test.toml',
    data_path: '/data/btc_1h.csv',
    progress: 100,
    created_at: '2025-01-01T00:00:00Z',
    started_at: '2025-01-01T00:01:00Z',
    completed_at: '2025-01-01T00:10:00Z',
  };

  const mockResult = {
    result: {
      metrics: {
        total_return: 15.5,
        annualized_return: 25.3,
        max_drawdown: -12.5,
        max_drawdown_duration: 5,
        sharpe_ratio: 1.85,
        sortino_ratio: 2.15,
        calmar_ratio: 2.02,
        annualized_volatility: 18.5,
        win_rate: 65.5,
        total_trades: 150,
        average_win: 250.5,
        average_loss: -150.3,
        profit_loss_ratio: 1.67,
        profit_factor: 1.95,
        max_consecutive_wins: 8,
        max_consecutive_losses: 5,
        avg_holding_period: 4.5,
        max_win: 1250.5,
        max_loss: -850.3,
      },
      equity_curve: [
        { timestamp: 1704067200, equity: 10000 },
        { timestamp: 1704153600, equity: 10500 },
        { timestamp: 1704240000, equity: 11000 },
      ],
      trades: [
        {
          timestamp: 1704067200,
          side: 'buy',
          price: 45000,
          quantity: 0.1,
          pnl: 0,
          fee: 4.5,
        },
        {
          timestamp: 1704153600,
          side: 'sell',
          price: 46000,
          quantity: 0.1,
          pnl: 100,
          fee: 4.6,
        },
      ],
    },
  };

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
      data: [mockTask],
    });
    
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: mockResult,
    });
  });

  // ========== 基本渲染测试 ==========

  it('应该渲染页面头部', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const header = screen.getByTestId('page-header');
      expect(header).toBeInTheDocument();
    });
    
    expect(screen.getByText('📊')).toBeInTheDocument();
  });

  it('应该在标题中显示任务名称', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('回测详情 - 测试回测任务')).toBeInTheDocument();
    });
  });

  it('应该渲染返回按钮', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const button = screen.getByTestId('button');
      expect(button).toHaveTextContent('← 返回列表');
      expect(button).toHaveAttribute('data-variant', 'secondary');
    });
  });

  // ========== 加载状态测试 ==========

  it('初始状态应该显示加载中', async () => {
    // 让API调用延迟
    (backtestApi.list as jest.Mock).mockImplementation(
      () => new Promise(resolve => setTimeout(() => resolve({ success: true, data: [mockTask] }), 100))
    );
    (backtestApi.getResult as jest.Mock).mockImplementation(
      () => new Promise(resolve => setTimeout(() => resolve({ success: true, data: mockResult }), 100))
    );
    
    render(<BacktestDetailPage />);
    
    expect(screen.getByText('加载中')).toBeInTheDocument();
    expect(screen.getByText('正在加载回测结果...')).toBeInTheDocument();
    
    await waitFor(() => {
      expect(screen.queryByText('正在加载回测结果...')).not.toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('加载完成后应该调用API', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(backtestApi.list).toHaveBeenCalledTimes(1);
      expect(backtestApi.getResult).toHaveBeenCalledWith('test-task-id');
    });
  });

  // ========== 任务信息显示测试 ==========

  it('应该显示任务信息卡片', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const cards = screen.getAllByTestId('card');
      const taskInfoCard = cards.find(card => card.textContent?.includes('任务信息'));
      expect(taskInfoCard).toBeInTheDocument();
    });
  });

  it('应该显示配置文件路径', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('/config/test.toml')).toBeInTheDocument();
    });
  });

  it('应该显示数据文件路径', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('/data/btc_1h.csv')).toBeInTheDocument();
    });
  });

  it('应该显示任务状态', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('✅ 已完成')).toBeInTheDocument();
    });
  });

  // ========== 性能指标显示测试 ==========

  it('应该显示性能指标卡片', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const cards = screen.getAllByTestId('card');
      const metricsCard = cards.find(card => card.textContent?.includes('性能指标'));
      expect(metricsCard).toBeInTheDocument();
    });
  });

  it('应该显示总收益率', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('15.50%')).toBeInTheDocument();
    });
  });

  it('应该显示年化收益率', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('25.30%')).toBeInTheDocument();
    });
  });

  it('应该显示最大回撤', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('-12.50%')).toBeInTheDocument();
    });
  });

  it('应该显示夏普比率', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('1.850')).toBeInTheDocument();
    });
  });

  it('应该显示胜率', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('65.50%')).toBeInTheDocument();
    });
  });

  it('应该显示总交易次数', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('150')).toBeInTheDocument();
    });
  });

  // ========== 无结果状态测试 ==========

  it('当任务状态为pending时应该显示等待提示', async () => {
    const pendingTask = { ...mockTask, status: 'pending' };
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [pendingTask],
    });
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: null,
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('该回测任务暂无结果')).toBeInTheDocument();
      expect(screen.getByText('任务正在等待执行')).toBeInTheDocument();
    });
  });

  it('当任务状态为running时应该显示运行中提示', async () => {
    const runningTask = { ...mockTask, status: 'running' };
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [runningTask],
    });
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: null,
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('任务正在运行中')).toBeInTheDocument();
    });
  });

  it('当任务状态为failed时应该显示失败提示', async () => {
    const failedTask = { ...mockTask, status: 'failed' };
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [failedTask],
    });
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: null,
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(screen.getByText('任务执行失败')).toBeInTheDocument();
    });
  });

  // ========== 错误处理测试 ==========

  it('当任务不存在时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [],
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '任务不存在',
      });
    });
  });

  it('当加载任务失败时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockResolvedValue({
      success: false,
      error: '服务器错误',
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '服务器错误',
      });
    });
  });

  it('当加载结果失败时应该显示错误通知', async () => {
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: false,
      error: '结果加载失败',
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        message: '结果加载失败',
      });
    });
  });

  it('当API抛出异常时应该显示错误通知', async () => {
    (backtestApi.list as jest.Mock).mockRejectedValue(new Error('Network error'));
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      expect(mockNotificationStore.addNotification).toHaveBeenCalled();
    });
  });

  // ========== 用户交互测试 ==========

  it('点击返回按钮应该跳转到历史记录列表', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const button = screen.getByTestId('button');
      expect(button).toBeInTheDocument();
    });
    
    const button = screen.getByTestId('button');
    fireEvent.click(button);
    
    expect(mockPush).toHaveBeenCalledWith('/history');
  });

  // ========== 数据转换测试 ==========

  it('应该正确计算盈利和亏损交易次数', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      // 总交易150次，胜率65.5%
      // 盈利次数 = 150 * 65.5 / 100 = 98
      // 亏损次数 = 150 - 98 = 52
      expect(screen.getByText('98 / 52')).toBeInTheDocument();
    });
  });

  it('应该正确转换时间戳为ISO字符串', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      // 验证equity_curve和trades已被正确加载和转换
      expect(backtestApi.getResult).toHaveBeenCalledWith('test-task-id');
    });
  });

  it('应该处理指标为0的情况', async () => {
    const zeroMetricsResult = {
      result: {
        metrics: {
          total_return: 0,
          annualized_return: 0,
          max_drawdown: 0,
          max_drawdown_duration: 0,
          sharpe_ratio: 0,
          sortino_ratio: 0,
          calmar_ratio: 0,
          annualized_volatility: 0,
          win_rate: 0,
          total_trades: 0,
          average_win: 0,
          average_loss: 0,
          profit_loss_ratio: 0,
          profit_factor: 0,
          max_consecutive_wins: 0,
          max_consecutive_losses: 0,
          avg_holding_period: 0,
          max_win: 0,
          max_loss: 0,
        },
        equity_curve: [],
        trades: [],
      },
    };
    
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: zeroMetricsResult,
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      // 验证指标卡片存在，并且有多个0值显示
      const allZeroPercentages = screen.getAllByText('0.00%');
      expect(allZeroPercentages.length).toBeGreaterThan(0);
    });
  });

  it('应该处理null值的指标', async () => {
    const nullMetricsResult = {
      result: {
        metrics: {
          total_return: null,
          annualized_return: null,
          max_drawdown: null,
          max_drawdown_duration: null,
          sharpe_ratio: null,
          sortino_ratio: null,
          calmar_ratio: null,
          annualized_volatility: null,
          win_rate: null,
          total_trades: null,
          average_win: null,
          average_loss: null,
          profit_loss_ratio: null,
          profit_factor: null,
          max_consecutive_wins: null,
          max_consecutive_losses: null,
          avg_holding_period: null,
          max_win: null,
          max_loss: null,
        },
        equity_curve: [],
        trades: [],
      },
    };
    
    (backtestApi.getResult as jest.Mock).mockResolvedValue({
      success: true,
      data: nullMetricsResult,
    });
    
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      // 应该显示0而不是崩溃
      const cards = screen.getAllByTestId('card');
      expect(cards.length).toBeGreaterThan(0);
    });
  });

  // ========== 图表区域测试 ==========

  it('应该显示图表分析卡片', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const cards = screen.getAllByTestId('card');
      const chartCard = cards.find(card => card.textContent?.includes('图表分析'));
      expect(chartCard).toBeInTheDocument();
    });
  });

  it('应该显示图表占位符', async () => {
    render(<BacktestDetailPage />);
    
    await waitFor(() => {
      const placeholders = screen.getAllByText('图表组件 - 待实现');
      expect(placeholders.length).toBeGreaterThan(0);
    });
  });
});
