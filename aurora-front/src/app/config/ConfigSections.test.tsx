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
import { StrategiesSection, PortfolioSection, BacktestSection } from './ConfigSections';
import type { StrategyConfig, PortfolioConfig, BacktestSettings } from '@/types/config-schema';

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

    // 找到风险管理 Switch 开关 (第一个 switch 是风险管理)
    const switches = screen.getAllByRole('switch');
    const riskSwitch = switches[0]; // 第一个 switch 是风险管理
    fireEvent.click(riskSwitch);

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

    // 找到风险管理 Switch 开关 (第一个 switch 是风险管理)
    const switches = screen.getAllByRole('switch');
    const riskSwitch = switches[0]; // 第一个 switch 是风险管理
    fireEvent.click(riskSwitch);

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

    // 找到仓位管理 Switch 开关 (第二个 switch,第一个是风险管理)
    const switches = screen.getAllByRole('switch');
    const positionSizingSwitch = switches[1]; // 第二个 switch 是仓位管理
    fireEvent.click(positionSizingSwitch);

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

describe('BacktestSection', () => {
  // 测试回测配置组件渲染
  it('should render backtest configuration section', () => {
    const mockConfig: BacktestSettings = {
      data_path: 'btc_1h.csv',
      symbol: 'BTCUSDT',
      interval: '1h',
    };
    const mockOnChange = jest.fn();

    render(<BacktestSection config={mockConfig} onChange={mockOnChange} />);

    // 检查标题
    expect(screen.getByText('回测配置 (可选)')).toBeInTheDocument();
    
    // 检查数据文件路径输入框
    expect(screen.getByDisplayValue('btc_1h.csv')).toBeInTheDocument();
    
    // 检查交易对符号输入框
    expect(screen.getByDisplayValue('BTCUSDT')).toBeInTheDocument();
    
    // 检查时间间隔输入框
    expect(screen.getByDisplayValue('1h')).toBeInTheDocument();
  });

  // 测试未启用回测配置时的渲染
  it('should render disabled state when config is undefined', () => {
    const mockOnChange = jest.fn();

    render(<BacktestSection config={undefined} onChange={mockOnChange} />);

    // 检查 Switch 开关处于禁用状态
    const switchElement = screen.getByRole('switch');
    expect(switchElement).toHaveAttribute('data-state', 'unchecked');
  });

  // 测试启用回测配置
  it('should enable backtest configuration', () => {
    const mockOnChange = jest.fn();

    render(<BacktestSection config={undefined} onChange={mockOnChange} />);

    // 点击 Switch 开关启用回测配置
    const switchElement = screen.getByRole('switch');
    fireEvent.click(switchElement);

    expect(mockOnChange).toHaveBeenCalledWith({ 
      data_path: '',
      timezone: expect.any(String) // 默认时区会被自动设置
    });
  });

  // 测试定价模式选择为收盘价模式
  it('should select close pricing mode', () => {
    const mockConfig: BacktestSettings = {
      data_path: 'btc_1h.csv',
      symbol: 'BTCUSDT',
      interval: '1h',
    };
    const mockOnChange = jest.fn();

    render(<BacktestSection config={mockConfig} onChange={mockOnChange} />);

    // 找到定价模式下拉框
    const pricingModeSelect = screen.getAllByRole('combobox').find(
      (element) => element.closest('div')?.querySelector('label')?.textContent?.includes('定价模式')
    );
    
    expect(pricingModeSelect).toBeDefined();
    if (pricingModeSelect) {
      fireEvent.click(pricingModeSelect);

      // 选择收盘价模式
      const closeOption = screen.getByText('收盘价模式');
      fireEvent.click(closeOption);

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          pricing_mode: {
            mode: 'close',
          },
        })
      );
    }
  });

  // 测试定价模式选择为买一卖一价模式
  it('should select bid_ask pricing mode with default spread', () => {
    const mockConfig: BacktestSettings = {
      data_path: 'btc_1h.csv',
      symbol: 'BTCUSDT',
      interval: '1h',
    };
    const mockOnChange = jest.fn();

    render(<BacktestSection config={mockConfig} onChange={mockOnChange} />);

    // 找到定价模式下拉框
    const pricingModeSelect = screen.getAllByRole('combobox').find(
      (element) => element.closest('div')?.querySelector('label')?.textContent?.includes('定价模式')
    );
    
    expect(pricingModeSelect).toBeDefined();
    if (pricingModeSelect) {
      fireEvent.click(pricingModeSelect);

      // 选择买一卖一价模式
      const bidAskOption = screen.getByText('买一卖一价模式');
      fireEvent.click(bidAskOption);

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          pricing_mode: {
            mode: 'bid_ask',
            spread_pct: 0.001,
          },
        })
      );
    }
  });

  // 测试修改买卖价差百分比
  it('should update spread_pct for bid_ask pricing mode', () => {
    const mockConfig: BacktestSettings = {
      data_path: 'btc_1h.csv',
      symbol: 'BTCUSDT',
      interval: '1h',
      pricing_mode: {
        mode: 'bid_ask',
        spread_pct: 0.001,
      },
    };
    const mockOnChange = jest.fn();

    render(<BacktestSection config={mockConfig} onChange={mockOnChange} />);

    // 找到价差输入框
    const spreadInput = screen.getByDisplayValue('0.001');
    fireEvent.change(spreadInput, { target: { value: '0.002' } });

    expect(mockOnChange).toHaveBeenCalledWith(
      expect.objectContaining({
        pricing_mode: {
          mode: 'bid_ask',
          spread_pct: 0.002,
        },
      })
    );
  });

  // 测试取消定价模式设置
  it('should remove pricing mode when selecting none', () => {
    const mockConfig: BacktestSettings = {
      data_path: 'btc_1h.csv',
      symbol: 'BTCUSDT',
      interval: '1h',
      pricing_mode: {
        mode: 'close',
      },
    };
    const mockOnChange = jest.fn();

    render(<BacktestSection config={mockConfig} onChange={mockOnChange} />);

    // 找到定价模式下拉框
    const pricingModeSelect = screen.getAllByRole('combobox').find(
      (element) => element.closest('div')?.querySelector('label')?.textContent?.includes('定价模式')
    );
    
    expect(pricingModeSelect).toBeDefined();
    if (pricingModeSelect) {
      fireEvent.click(pricingModeSelect);

      // 选择默认(不设置)
      const noneOption = screen.getByText('默认(不设置)');
      fireEvent.click(noneOption);

      expect(mockOnChange).toHaveBeenCalledWith(
        expect.objectContaining({
          pricing_mode: undefined,
        })
      );
    }
  });

  // 测试spread_pct输入框仅在bid_ask模式下显示
  it('should only show spread_pct input for bid_ask mode', () => {
    const mockOnChange = jest.fn();

    // 测试收盘价模式
    const { rerender } = render(
      <BacktestSection 
        config={{
          data_path: 'btc_1h.csv',
          pricing_mode: { mode: 'close' },
        }} 
        onChange={mockOnChange} 
      />
    );

    // 不应显示价差输入框
    expect(screen.queryByText('买卖价差百分比:')).not.toBeInTheDocument();

    // 测试买一卖一价模式
    rerender(
      <BacktestSection 
        config={{
          data_path: 'btc_1h.csv',
          pricing_mode: { mode: 'bid_ask', spread_pct: 0.001 },
        }} 
        onChange={mockOnChange} 
      />
    );

    // 应显示价差输入框
    expect(screen.getByText('买卖价差百分比:')).toBeInTheDocument();
  });
});
