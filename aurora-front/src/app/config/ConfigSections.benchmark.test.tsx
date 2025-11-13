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
 * BacktestSection 组件测试
 */

import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { BacktestSection } from './ConfigSections';
import { dataApi } from '@/lib/api';

// Mock dataApi
jest.mock('@/lib/api', () => ({
  dataApi: {
    list: jest.fn(),
  },
}));

describe('BacktestSection - Benchmark Configuration', () => {
  const mockOnChange = jest.fn();
  
  beforeEach(() => {
    jest.clearAllMocks();
    // Mock 数据文件列表
    (dataApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [
        { filename: 'btc_1h.csv', size: 1000, modified: '2025-01-01' },
        { filename: 'eth_4h.csv', size: 2000, modified: '2025-01-02' },
      ],
    });
  });

  it('应该能够启用基准配置', async () => {
    const config = {
      data_path: 'test.csv',
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 等待数据加载
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalled();
    });

    // 找到基准配置开关
    const switches = screen.getAllByRole('switch');
    const benchmarkSwitch = switches.find(s => 
      s.closest('div')?.textContent?.includes('基准配置')
    );

    expect(benchmarkSwitch).toBeDefined();

    // 启用基准配置
    if (benchmarkSwitch) {
      fireEvent.click(benchmarkSwitch);
    }

    // 验证 onChange 被调用，且包含 benchmark 配置
    await waitFor(() => {
      expect(mockOnChange).toHaveBeenCalled();
      const lastCall = mockOnChange.mock.calls[mockOnChange.mock.calls.length - 1][0];
      expect(lastCall.benchmark).toBeDefined();
      expect(lastCall.benchmark.enabled).toBe(true);
    });
  });

  it('应该能够禁用基准配置', async () => {
    const config = {
      data_path: 'test.csv',
      benchmark: {
        enabled: true,
        data_path: 'benchmark.csv',
      },
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 等待数据加载
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalled();
    });

    // 找到基准配置开关
    const switches = screen.getAllByRole('switch');
    const benchmarkSwitch = switches.find(s => 
      s.closest('div')?.textContent?.includes('基准配置')
    );

    // 禁用基准配置
    if (benchmarkSwitch) {
      fireEvent.click(benchmarkSwitch);
    }

    // 验证 onChange 被调用，且 benchmark 设置为 undefined
    await waitFor(() => {
      expect(mockOnChange).toHaveBeenCalled();
      const lastCall = mockOnChange.mock.calls[mockOnChange.mock.calls.length - 1][0];
      expect(lastCall.benchmark).toBeUndefined();
    });
  });

  it('启用基准后应该显示数据文件选择器', async () => {
    const config = {
      data_path: 'test.csv',
      benchmark: {
        enabled: true,
        data_path: undefined,
      },
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 等待数据加载
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalled();
    });

    // 应该显示基准数据文件标签
    expect(screen.getByText(/基准数据文件/)).toBeInTheDocument();
  });

  it('禁用基准后不应该显示数据文件选择器', async () => {
    const config = {
      data_path: 'test.csv',
      benchmark: {
        enabled: false,
      },
    };

    render(<BacktestSection config={config} onChange={mockOnChange} />);

    // 等待数据加载
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalled();
    });

    // 不应该显示基准数据文件标签（或者说不在文档中可见）
    expect(screen.queryByText(/基准数据文件/)).not.toBeInTheDocument();
  });
});
