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
 * 配置管理页面保存和验证功能测试
 */

import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import ConfigPage from './page';
import * as apiModule from '@/lib/api';
import * as tomlModule from '@/lib/utils/toml';

// Mock API 模块
jest.mock('@/lib/api', () => ({
  configApi: {
    create: jest.fn(),
    update: jest.fn(),
    validate: jest.fn(),
  },
}));

// Mock TOML 工具模块
jest.mock('@/lib/utils/toml', () => ({
  stringifyTOML: jest.fn(),
  validateTOML: jest.fn(),
  readTOMLFile: jest.fn(),
}));

// Mock 通知 store 的默认实现
const mockAddNotification = jest.fn();

// Mock 通知 store
jest.mock('@/lib/store', () => ({
  useNotificationStore: jest.fn(() => ({
    addNotification: mockAddNotification,
  })),
}));

// Mock 子组件
jest.mock('@/components/ui', () => ({
  PageHeader: ({ icon, title, action }: {
    icon: string;
    title: string;
    action?: React.ReactNode;
  }) => (
    <div data-testid="page-header">
      <span>{icon}</span>
      <h1>{title}</h1>
      {action}
    </div>
  ),
  Card: ({ title, children, className }: {
    title?: string;
    children: React.ReactNode;
    className?: string;
  }) => (
    <div data-testid="card" className={className}>
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
    <button
      data-testid="button"
      onClick={onClick}
      disabled={disabled}
      data-variant={variant}
    >
      {children}
    </button>
  ),
  Input: ({ value, onChange, placeholder, className }: {
    value?: string;
    onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
    placeholder?: string;
    className?: string;
  }) => (
    <input
      data-testid="input"
      value={value}
      onChange={onChange}
      placeholder={placeholder}
      className={className}
    />
  ),
  Textarea: ({ value, onChange, placeholder, rows, className }: {
    value?: string;
    onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
    placeholder?: string;
    rows?: number;
    className?: string;
  }) => (
    <textarea
      data-testid="textarea"
      value={value}
      onChange={onChange}
      placeholder={placeholder}
      rows={rows}
      className={className}
    />
  ),
}));

// Mock ConfigSections 组件
jest.mock('./ConfigSections', () => ({
  DataSourceSection: () => <div data-testid="data-source-section">DataSource</div>,
  StrategiesSection: () => <div data-testid="strategies-section">Strategies</div>,
  PortfolioSection: () => <div data-testid="portfolio-section">Portfolio</div>,
  LoggingSection: () => <div data-testid="logging-section">Logging</div>,
  BacktestSection: () => <div data-testid="backtest-section">Backtest</div>,
  LiveSection: () => <div data-testid="live-section">Live</div>,
}));

describe('ConfigPage - 保存和验证功能', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockAddNotification.mockClear();
  });

  describe('保存功能', () => {
    it('点击新建配置按钮后应该显示保存按钮', async () => {
      render(<ConfigPage />);

      // 点击新建配置按钮
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      
      if (newButton) {
        fireEvent.click(newButton);
      }

      // 等待按钮出现
      await waitFor(() => {
        const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
        expect(saveButton).toBeInTheDocument();
      });
    });

    it('保存时如果文件名为空应该显示错误', async () => {

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
        expect(saveButton).toBeInTheDocument();
      });

      // 清空文件名输入框
      const filenameInput = screen.getByPlaceholderText('example.toml') as HTMLInputElement;
      fireEvent.change(filenameInput, { target: { value: '' } });

      // 点击保存按钮
      const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
      if (saveButton) {
        fireEvent.click(saveButton);
      }

      // 验证显示了错误通知
      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
            message: '请输入文件名',
          })
        );
      });
    });

    it('保存时如果文件名不以.toml结尾应该显示错误', async () => {
      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
        expect(saveButton).toBeInTheDocument();
      });

      // 输入不带.toml后缀的文件名
      const filenameInput = screen.getByPlaceholderText('example.toml') as HTMLInputElement;
      fireEvent.change(filenameInput, { target: { value: 'test.txt' } });

      // 点击保存按钮
      const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
      if (saveButton) {
        fireEvent.click(saveButton);
      }

      // 验证显示了错误通知
      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
            message: '文件名必须以.toml结尾',
          })
        );
      });
    });

    it('成功保存配置后应该显示成功通知', async () => {
      // Mock API 响应
      (apiModule.configApi.create as jest.Mock).mockResolvedValue({
        success: true,
        message: '配置已创建',
      });

      // Mock stringifyTOML
      (tomlModule.stringifyTOML as jest.Mock).mockResolvedValue('# TOML content');

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
        expect(saveButton).toBeInTheDocument();
      });

      // 输入文件名
      const filenameInput = screen.getByPlaceholderText('example.toml') as HTMLInputElement;
      fireEvent.change(filenameInput, { target: { value: 'test.toml' } });

      // 点击保存按钮
      const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
      if (saveButton) {
        fireEvent.click(saveButton);
      }

      // 验证调用了API
      await waitFor(() => {
        expect(apiModule.configApi.create).toHaveBeenCalledWith(
          expect.objectContaining({
            filename: 'test.toml',
            content: '# TOML content',
          })
        );

        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'success',
            message: '配置保存成功',
          })
        );
      });
    });

    it('保存配置失败应该显示错误通知', async () => {
      // Mock API 响应为失败
      (apiModule.configApi.create as jest.Mock).mockResolvedValue({
        success: false,
        error: '文件已存在',
      });

      // Mock stringifyTOML
      (tomlModule.stringifyTOML as jest.Mock).mockResolvedValue('# TOML content');

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
        expect(saveButton).toBeInTheDocument();
      });

      // 输入文件名
      const filenameInput = screen.getByPlaceholderText('example.toml') as HTMLInputElement;
      fireEvent.change(filenameInput, { target: { value: 'test.toml' } });

      // 点击保存按钮
      const saveButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('保存'));
      if (saveButton) {
        fireEvent.click(saveButton);
      }

      // 验证显示了错误通知
      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
            message: '文件已存在',
          })
        );
      });
    });
  });

  describe('验证功能', () => {
    it('点击新建配置按钮后应该显示验证按钮', async () => {
      render(<ConfigPage />);

      // 点击新建配置按钮
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      
      if (newButton) {
        fireEvent.click(newButton);
      }

      // 等待按钮出现
      await waitFor(() => {
        const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
        expect(validateButton).toBeInTheDocument();
      });
    });

    it('成功验证配置后应该显示成功通知', async () => {
      // Mock API 响应
      (apiModule.configApi.validate as jest.Mock).mockResolvedValue({
        success: true,
        data: {
          valid: true,
          errors: [],
        },
      });

      // Mock stringifyTOML
      (tomlModule.stringifyTOML as jest.Mock).mockResolvedValue('# TOML content');

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
        expect(validateButton).toBeInTheDocument();
      });

      // 点击验证按钮
      const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
      if (validateButton) {
        fireEvent.click(validateButton);
      }

      // 验证调用了API
      await waitFor(() => {
        expect(apiModule.configApi.validate).toHaveBeenCalledWith('# TOML content');

        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'success',
            message: '配置验证通过!',
          })
        );
      });
    });

    it('验证失败应该显示错误通知', async () => {
      // Mock API 响应为验证失败
      (apiModule.configApi.validate as jest.Mock).mockResolvedValue({
        success: true,
        data: {
          valid: false,
          errors: ['缺少必需字段: data_source', '策略配置无效'],
        },
      });

      // Mock stringifyTOML
      (tomlModule.stringifyTOML as jest.Mock).mockResolvedValue('# TOML content');

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
        expect(validateButton).toBeInTheDocument();
      });

      // 点击验证按钮
      const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
      if (validateButton) {
        fireEvent.click(validateButton);
      }

      // 验证显示了错误通知
      await waitFor(() => {
        expect(mockAddNotification).toHaveBeenCalledWith(
          expect.objectContaining({
            type: 'error',
            message: expect.stringContaining('缺少必需字段: data_source'),
          })
        );
      });
    });

    it('验证时按钮应该显示验证中状态', async () => {
      // Mock API 响应延迟
      (apiModule.configApi.validate as jest.Mock).mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve({
          success: true,
          data: { valid: true, errors: [] },
        }), 100))
      );

      // Mock stringifyTOML
      (tomlModule.stringifyTOML as jest.Mock).mockResolvedValue('# TOML content');

      render(<ConfigPage />);

      // 点击新建配置
      const newButtons = screen.getAllByTestId('button');
      const newButton = newButtons.find((btn) => btn.textContent?.includes('新建配置'));
      if (newButton) {
        fireEvent.click(newButton);
      }

      await waitFor(() => {
        const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
        expect(validateButton).toBeInTheDocument();
      });

      // 点击验证按钮
      const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证'));
      if (validateButton) {
        fireEvent.click(validateButton);
      }

      // 验证按钮文本变为"验证中..."
      await waitFor(() => {
        const validatingButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('验证中'));
        expect(validatingButton).toBeInTheDocument();
      });

      // 等待验证完成
      await waitFor(() => {
        const validateButton = screen.getAllByTestId('button').find((btn) => btn.textContent?.includes('✓ 验证'));
        expect(validateButton).toBeInTheDocument();
      }, { timeout: 3000 });
    });
  });
});
