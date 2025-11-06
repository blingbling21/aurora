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

import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { DataList } from './DataList';
import { dataApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store';

// Mock dependencies
jest.mock('@/lib/api');
jest.mock('@/lib/store');

describe('DataList', () => {
  // Mock data
  const mockDataFiles = [
    {
      filename: 'binance_btcusdt_1h_20250101_to_20250131.csv',
      size: 1048576, // 1 MB
      modified: '2025-01-01 10:00:00',
      record_count: 744,
    },
    {
      filename: 'binance_ethusdt_4h_20250101_to_20250131.csv',
      size: 524288, // 512 KB
      modified: '2025-01-02 11:00:00',
      record_count: 186,
    },
  ];

  const mockAddNotification = jest.fn();
  const mockOnSelect = jest.fn();

  beforeEach(() => {
    // Reset mocks
    jest.clearAllMocks();
    
    // Setup notification store mock
    (useNotificationStore as unknown as jest.Mock).mockReturnValue({
      addNotification: mockAddNotification,
    });
  });

  describe('渲染和数据加载', () => {
    it('应该在初始化时加载数据文件列表', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      render(<DataList />);

      // 应该显示加载状态 (按钮中也有"加载中...",所以使用getAllByText)
      expect(screen.getAllByText('加载中...').length).toBeGreaterThan(0);

      // 等待数据加载完成
      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
        expect(screen.getByText('binance_ethusdt_4h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      expect(dataApi.list).toHaveBeenCalledTimes(1);
    });

    it('应该在没有数据文件时显示空状态', async () => {
      // Mock empty response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: [],
      });

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('暂无数据文件')).toBeInTheDocument();
      });
    });

    it('应该在加载失败时显示错误通知', async () => {
      // Mock error response
      (dataApi.list as jest.Mock).mockRejectedValue(
        new Error('Network error')
      );

      render(<DataList />);

      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith({
          type: 'error',
          message: 'Network error',
        });
      });
    });
  });

  describe('数据文件选择', () => {
    it('应该能够选择数据文件', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      render(<DataList onSelect={mockOnSelect} />);

      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      // 点击第一个文件
      fireEvent.click(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv'));

      // 应该调用onSelect回调
      expect(mockOnSelect).toHaveBeenCalledWith('binance_btcusdt_1h_20250101_to_20250131.csv');

      // 应该显示选中状态 (找到包含文件名的卡片容器)
      const selectedCard = screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')
        .closest('.p-4');
      expect(selectedCard).toHaveClass('border-blue-500');
    });
  });

  describe('数据文件删除', () => {
    it('应该能够删除数据文件', async () => {
      // Mock API responses
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });
      (dataApi.delete as jest.Mock).mockResolvedValue({
        success: true,
      });

      // Mock confirm dialog
      global.confirm = jest.fn(() => true);

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      // 点击删除按钮
      const deleteButtons = screen.getAllByText('🗑️ 删除');
      fireEvent.click(deleteButtons[0]);

      // 应该显示确认对话框
      expect(global.confirm).toHaveBeenCalledWith(
        '确定要删除数据文件 "binance_btcusdt_1h_20250101_to_20250131.csv" 吗?'
      );

      await waitFor(() => {
        // 应该调用删除API
        expect(dataApi.delete).toHaveBeenCalledWith('binance_btcusdt_1h_20250101_to_20250131.csv');
        
        // 应该显示成功通知
        expect(mockAddNotification).toHaveBeenCalledWith({
          type: 'success',
          message: '成功删除数据文件: binance_btcusdt_1h_20250101_to_20250131.csv',
        });
        
        // 应该重新加载列表
        expect(dataApi.list).toHaveBeenCalledTimes(2);
      });
    });

    it('应该在取消确认时不删除文件', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      // Mock confirm dialog to return false
      global.confirm = jest.fn(() => false);

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      // 点击删除按钮
      const deleteButtons = screen.getAllByText('🗑️ 删除');
      fireEvent.click(deleteButtons[0]);

      // 应该不调用删除API
      expect(dataApi.delete).not.toHaveBeenCalled();
    });

    it('应该在删除失败时显示错误通知', async () => {
      // Mock API responses
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });
      (dataApi.delete as jest.Mock).mockRejectedValue(
        new Error('Delete failed')
      );

      // Mock confirm dialog
      global.confirm = jest.fn(() => true);

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      // 点击删除按钮
      const deleteButtons = screen.getAllByText('🗑️ 删除');
      fireEvent.click(deleteButtons[0]);

      await waitFor(() => {
        // 应该显示错误通知
        expect(mockAddNotification).toHaveBeenCalledWith({
          type: 'error',
          message: 'Delete failed',
        });
      });
    });
  });

  describe('刷新功能', () => {
    it('应该能够手动刷新列表', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
      });

      // 点击刷新按钮
      fireEvent.click(screen.getByText('🔄 刷新'));

      // 应该再次调用API
      await waitFor(() => {
        expect(dataApi.list).toHaveBeenCalledTimes(2);
      });
    });

    it('应该响应refreshTrigger属性变化', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      const { rerender } = render(<DataList refreshTrigger={1} />);

      await waitFor(() => {
        expect(dataApi.list).toHaveBeenCalledTimes(1);
      });

      // 更新refreshTrigger
      rerender(<DataList refreshTrigger={2} />);

      await waitFor(() => {
        expect(dataApi.list).toHaveBeenCalledTimes(2);
      });
    });
  });

  describe('文件信息显示', () => {
    it('应该显示数据文件的完整信息', async () => {
      // Mock API response
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockDataFiles,
      });

      render(<DataList />);

      await waitFor(() => {
        // 应该显示文件名
        expect(screen.getByText('binance_btcusdt_1h_20250101_to_20250131.csv')).toBeInTheDocument();
        
        // 应该显示文件大小
        expect(screen.getByText('大小: 1.00 MB')).toBeInTheDocument();
        
        // 应该显示修改时间
        expect(screen.getByText('修改时间: 2025-01-01 10:00:00')).toBeInTheDocument();
        
        // 应该显示记录数
        expect(screen.getByText('记录数: 744')).toBeInTheDocument();
      });
    });

    it('应该正确格式化不同大小的文件', async () => {
      // Mock API response with various file sizes
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: [
          { ...mockDataFiles[0], size: 1024 }, // 1 KB
          { ...mockDataFiles[0], filename: 'test2.csv', size: 1048576 }, // 1 MB
          { ...mockDataFiles[0], filename: 'test3.csv', size: 1073741824 }, // 1 GB
        ],
      });

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('大小: 1.00 KB')).toBeInTheDocument();
        expect(screen.getByText('大小: 1.00 MB')).toBeInTheDocument();
        expect(screen.getByText('大小: 1.00 GB')).toBeInTheDocument();
      });
    });

    it('应该在没有记录数时不显示记录数字段', async () => {
      // Mock API response without record_count
      (dataApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: [
          {
            filename: 'test.csv',
            size: 1024,
            modified: '2025-01-01 10:00:00',
          },
        ],
      });

      render(<DataList />);

      await waitFor(() => {
        expect(screen.getByText('test.csv')).toBeInTheDocument();
        expect(screen.queryByText(/记录数:/)).not.toBeInTheDocument();
      });
    });
  });
});
