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
import '@testing-library/jest-dom';
import { StrategiesSection, PortfolioSection } from './ConfigSections';
import type { StrategyConfig, PortfolioConfig } from '@/types/config-schema';

describe('StrategiesSection', () => {
  // 测试策略配置组件渲染
  it('should render strategy configuration section', () => {
    const mockStrategies: StrategyConfig[] = [
      {
        name: 'MA交叉策略',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: { short: 10, long: 30 },
      },
    ];
    const mockOnChange = jest.fn();

    render(<StrategiesSection strategies={mockStrategies} onChange={mockOnChange} />);

    // 检查标题
    expect(screen.getByText('策略配置')).toBeInTheDocument();
    
    // 检查策略名称输入框
    expect(screen.getByDisplayValue('MA交叉策略')).toBeInTheDocument();
    
    // 检查启用策略复选框
    expect(screen.getByText('启用策略')).toBeInTheDocument();
  });

  // 测试策略名称修改
  it('should update strategy name', () => {
    const mockStrategies: StrategyConfig[] = [
      {
        name: 'MA交叉策略',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: { short: 10, long: 30 },
      },
    ];
    const mockOnChange = jest.fn();

    render(<StrategiesSection strategies={mockStrategies} onChange={mockOnChange} />);

    const nameInput = screen.getByDisplayValue('MA交叉策略');
    fireEvent.change(nameInput, { target: { value: '新策略名称' } });

    expect(mockOnChange).toHaveBeenCalledWith([
      expect.objectContaining({
        name: '新策略名称',
      }),
    ]);
  });

  // 测试策略启用/禁用切换
  it('should toggle strategy enabled state', () => {
    const mockStrategies: StrategyConfig[] = [
      {
        name: 'MA交叉策略',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: { short: 10, long: 30 },
      },
    ];
    const mockOnChange = jest.fn();

    render(<StrategiesSection strategies={mockStrategies} onChange={mockOnChange} />);

    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(mockOnChange).toHaveBeenCalledWith([
      expect.objectContaining({
        enabled: false,
      }),
    ]);
  });

  // 测试策略类型下拉框加载
  it('should load strategy types', async () => {
    const mockStrategies: StrategyConfig[] = [
      {
        name: 'MA交叉策略',
        strategy_type: 'ma-crossover',
        enabled: true,
        parameters: { short: 10, long: 30 },
      },
    ];
    const mockOnChange = jest.fn();

    render(<StrategiesSection strategies={mockStrategies} onChange={mockOnChange} />);

    // 等待策略类型加载
    await waitFor(() => {
      expect(screen.getByText(/基于移动平均线/)).toBeInTheDocument();
    });
  });
});

describe('PortfolioSection', () => {
  // 测试投资组合配置基本渲染
  it('should render portfolio configuration section', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 检查标题
    expect(screen.getByText('投资组合配置')).toBeInTheDocument();
    
    // 检查初始资金输入框
    expect(screen.getByDisplayValue('10000')).toBeInTheDocument();
    
    // 检查手续费率输入框
    expect(screen.getByDisplayValue('0.001')).toBeInTheDocument();
    
    // 检查滑点率输入框
    expect(screen.getByDisplayValue('0')).toBeInTheDocument();
  });

  // 测试初始资金修改
  it('should update initial cash', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    const cashInput = screen.getByDisplayValue('10000');
    fireEvent.change(cashInput, { target: { value: '20000' } });

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        initial_cash: 20000,
      })
    );
  });

  // 测试风险管理配置启用
  it('should enable risk management', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 找到启用风险管理按钮
    const enableButton = screen.getByText(/启用风险管理/);
    fireEvent.click(enableButton);

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        risk_rules: {},
      })
    );
  });

  // 测试风险管理配置禁用
  it('should disable risk management', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
      risk_rules: {
        stop_loss_pct: 2.0,
        take_profit_pct: 5.0,
      },
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 找到禁用风险管理按钮
    const disableButton = screen.getByText(/禁用风险管理/);
    fireEvent.click(disableButton);

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        risk_rules: undefined,
      })
    );
  });

  // 测试止损百分比设置
  it('should update stop loss percentage', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
      risk_rules: {},
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 通过placeholder找到止损百分比输入框
    const stopLossInput = screen.getByPlaceholderText('2.0');
    fireEvent.change(stopLossInput, { target: { value: '3' } });

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        risk_rules: expect.objectContaining({
          stop_loss_pct: 3,
        }),
      })
    );
  });

  // 测试仓位管理配置启用
  it('should enable position sizing', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 找到启用仓位管理按钮
    const enableButton = screen.getByText(/启用仓位管理/);
    fireEvent.click(enableButton);

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        position_sizing: {
          strategy_type: 'fixed_percentage',
          percentage: 0.2,
        },
      })
    );
  });

  // 测试固定比例仓位策略参数修改
  it('should update fixed percentage position sizing', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
      position_sizing: {
        strategy_type: 'fixed_percentage',
        percentage: 0.2,
      },
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 通过displayValue找到百分比输入框
    const percentageInput = screen.getByDisplayValue('0.2');
    fireEvent.change(percentageInput, { target: { value: '0.3' } });

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        position_sizing: {
          strategy_type: 'fixed_percentage',
          percentage: 0.3,
        },
      })
    );
  });

  // 测试仓位管理策略类型切换
  it('should switch position sizing strategy type', () => {
    const mockConfig: PortfolioConfig = {
      initial_cash: 10000,
      commission: 0.001,
      slippage: 0,
      position_sizing: {
        strategy_type: 'fixed_percentage',
        percentage: 0.2,
      },
    };
    const mockOnChange = jest.fn();

    render(<PortfolioSection config={mockConfig} onChange={mockOnChange} />);

    // 找到策略类型下拉框(通过combobox角色)
    const strategySelect = screen.getByRole('combobox');
    fireEvent.click(strategySelect);

    // 选择Kelly准则策略(需要等待选项出现)
    const kellyOption = screen.getByText(/Kelly准则策略/);
    fireEvent.click(kellyOption);

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        position_sizing: expect.objectContaining({
          strategy_type: 'kelly_criterion',
        }),
      })
    );
  });
});
