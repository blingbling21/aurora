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
import { ConfigList } from './ConfigList';
import { configApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store';

// Mock dependencies
jest.mock('@/lib/api');
jest.mock('@/lib/store');

describe('ConfigList', () => {
  // Mock data
  const mockConfigs = [
    {
      filename: 'test1.toml',
      path: '/configs/test1.toml',
      modified: '2025-01-01 10:00:00',
    },
    {
      filename: 'test2.toml',
      path: '/configs/test2.toml',
      modified: '2025-01-02 11:00:00',
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
    it('应该在初始化时加载配置列表', async () => {
      // Mock API response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      render(<ConfigList />);

      // 应该显示加载状态 (按钮中也有"加载中...",所以使用getAllByText)
      expect(screen.getAllByText('加载中...').length).toBeGreaterThan(0);

      // 等待数据加载完成
      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
        expect(screen.getByText('test2.toml')).toBeInTheDocument();
      });

      expect(configApi.list).toHaveBeenCalledTimes(1);
    });

    it('应该在没有配置时显示空状态', async () => {
      // Mock empty response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: [],
      });

      render(<ConfigList />);

      await waitFor(() => {
        expect(screen.getByText('暂无配置文件')).toBeInTheDocument();
      });
    });

    it('应该在加载失败时显示错误通知', async () => {
      // Mock error response
      (configApi.list as jest.Mock).mockRejectedValue(
        new Error('Network error')
      );

      render(<ConfigList />);

      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith({
          type: 'error',
          message: 'Network error',
        });
      });
    });
  });

  describe('配置选择', () => {
    it('应该能够选择配置文件', async () => {
      // Mock API response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      render(<ConfigList onSelect={mockOnSelect} />);

      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
      });

      // 点击第一个配置
      fireEvent.click(screen.getByText('test1.toml'));

      // 应该调用onSelect回调
      expect(mockOnSelect).toHaveBeenCalledWith('test1.toml');

      // 应该显示选中状态 (找到包含文件名的卡片容器)
      const selectedCard = screen.getByText('test1.toml')
        .closest('.p-4');
      expect(selectedCard).toHaveClass('border-blue-500');
    });
  });

  describe('配置删除', () => {
    it('应该能够删除配置文件', async () => {
      // Mock API responses
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });
      (configApi.delete as jest.Mock).mockResolvedValue({
        success: true,
      });

      // Mock confirm dialog
      global.confirm = jest.fn(() => true);

      render(<ConfigList />);

      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
      });

      // 点击删除按钮
      const deleteButtons = screen.getAllByText('🗑️ 删除');
      fireEvent.click(deleteButtons[0]);

      // 应该显示确认对话框
      expect(global.confirm).toHaveBeenCalledWith(
        '确定要删除配置文件 "test1.toml" 吗?'
      );

      await waitFor(() => {
        // 应该调用删除API
        expect(configApi.delete).toHaveBeenCalledWith('test1.toml');
        
        // 应该显示成功通知
        expect(mockAddNotification).toHaveBeenCalledWith({
          type: 'success',
          message: '成功删除配置文件: test1.toml',
        });
        
        // 应该重新加载列表
        expect(configApi.list).toHaveBeenCalledTimes(2);
      });
    });

    it('应该在取消确认时不删除文件', async () => {
      // Mock API response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      // Mock confirm dialog to return false
      global.confirm = jest.fn(() => false);

      render(<ConfigList />);

      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
      });

      // 点击删除按钮
      const deleteButtons = screen.getAllByText('🗑️ 删除');
      fireEvent.click(deleteButtons[0]);

      // 应该不调用删除API
      expect(configApi.delete).not.toHaveBeenCalled();
    });

    it('应该在删除失败时显示错误通知', async () => {
      // Mock API responses
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });
      (configApi.delete as jest.Mock).mockRejectedValue(
        new Error('Delete failed')
      );

      // Mock confirm dialog
      global.confirm = jest.fn(() => true);

      render(<ConfigList />);

      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
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
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      render(<ConfigList />);

      await waitFor(() => {
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
      });

      // 点击刷新按钮
      fireEvent.click(screen.getByText('🔄 刷新'));

      // 应该再次调用API
      await waitFor(() => {
        expect(configApi.list).toHaveBeenCalledTimes(2);
      });
    });

    it('应该响应refreshTrigger属性变化', async () => {
      // Mock API response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      const { rerender } = render(<ConfigList refreshTrigger={1} />);

      await waitFor(() => {
        expect(configApi.list).toHaveBeenCalledTimes(1);
      });

      // 更新refreshTrigger
      rerender(<ConfigList refreshTrigger={2} />);

      await waitFor(() => {
        expect(configApi.list).toHaveBeenCalledTimes(2);
      });
    });
  });

  describe('显示信息', () => {
    it('应该显示配置文件的完整信息', async () => {
      // Mock API response
      (configApi.list as jest.Mock).mockResolvedValue({
        success: true,
        data: mockConfigs,
      });

      render(<ConfigList />);

      await waitFor(() => {
        // 应该显示文件名
        expect(screen.getByText('test1.toml')).toBeInTheDocument();
        
        // 应该显示修改时间
        expect(screen.getByText('修改时间: 2025-01-01 10:00:00')).toBeInTheDocument();
        
        // 应该显示路径
        expect(screen.getByText('路径: /configs/test1.toml')).toBeInTheDocument();
      });
    });
  });
});
