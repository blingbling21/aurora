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
 * 配置表单各个区块组件
 */

import React from 'react';
import { Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Switch } from '@/components/ui';
import type {
  DataSourceConfig,
  StrategyConfig,
  PortfolioConfig,
  LoggingConfig,
  BacktestSettings,
  LiveConfig,
  PricingMode,
  BenchmarkConfig,
} from '@/types/config-schema';
import { dataApi } from '@/lib/api';

// ==================== 数据源配置区块 ====================

interface DataSourceSectionProps {
  config: DataSourceConfig;
  onChange: (config: DataSourceConfig) => void;
}

export function DataSourceSection({ config, onChange }: DataSourceSectionProps) {
  const updateField = <K extends keyof DataSourceConfig>(key: K, value: DataSourceConfig[K]) => {
    onChange({ ...config, [key]: value });
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
        数据源配置
      </h4>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            数据提供商 *:
          </label>
          <Select value={config.provider} onValueChange={(value) => updateField('provider', value as DataSourceConfig['provider'])}>
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="binance">Binance</SelectItem>
              <SelectItem value="okx">OKX</SelectItem>
              <SelectItem value="bybit">Bybit</SelectItem>
              <SelectItem value="csv">CSV File</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            超时时间 (秒) *:
          </label>
          <Input
            type="number"
            value={config.timeout}
            onChange={(e) => updateField('timeout', parseInt(e.target.value) || 30)}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            最大重试次数 *:
          </label>
          <Input
            type="number"
            value={config.max_retries}
            onChange={(e) => updateField('max_retries', parseInt(e.target.value) || 3)}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            API密钥:
          </label>
          <Input
            type="password"
            value={config.api_key || ''}
            onChange={(e) => updateField('api_key', e.target.value || undefined)}
            placeholder="可选"
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            API密钥:
          </label>
          <Input
            type="password"
            value={config.api_secret || ''}
            onChange={(e) => updateField('api_secret', e.target.value || undefined)}
            placeholder="可选"
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            基础URL:
          </label>
          <Input
            type="url"
            value={config.base_url || ''}
            onChange={(e) => updateField('base_url', e.target.value || undefined)}
            placeholder="https://api.binance.com"
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            WebSocket URL:
          </label>
          <Input
            type="url"
            value={config.ws_url || ''}
            onChange={(e) => updateField('ws_url', e.target.value || undefined)}
            placeholder="wss://stream.binance.com:9443"
            className="w-full"
          />
        </div>
      </div>
    </div>
  );
}

// ==================== 策略配置区块 ====================

interface StrategiesSectionProps {
  strategies: StrategyConfig[];
  onChange: (strategies: StrategyConfig[]) => void;
}

export function StrategiesSection({ strategies, onChange }: StrategiesSectionProps) {
  // 只显示第一个策略的基本编辑(完整版本应支持多策略管理)
  const strategy = strategies[0];
  
  const updateStrategy = (updates: Partial<StrategyConfig>) => {
    onChange([{ ...strategy, ...updates }]);
  };

  // 动态导入策略类型定义
  const [strategyTypes, setStrategyTypes] = React.useState<Array<{
    type: string;
    name: string;
    description: string;
    fields: Array<{
      name: string;
      label: string;
      type: 'number' | 'text' | 'select' | 'checkbox';
      defaultValue?: string | number | boolean;
      placeholder?: string;
      description?: string;
      min?: number;
      max?: number;
      step?: number;
      required?: boolean;
    }>;
  }>>([]);

  React.useEffect(() => {
    // 动态加载策略类型
    import('@/types/strategy-types').then((module) => {
      setStrategyTypes(module.STRATEGY_TYPES);
    });
  }, []);

  // 获取当前策略的字段定义
  const currentStrategyDef = strategyTypes.find((def) => def.type === strategy.strategy_type);

  // 更新策略类型时,重置参数为默认值
  const handleStrategyTypeChange = (newType: string) => {
    const newDef = strategyTypes.find((def) => def.type === newType);
    if (newDef) {
      const defaultParams: Record<string, string | number | boolean> = {};
      newDef.fields.forEach((field) => {
        if (field.defaultValue !== undefined) {
          defaultParams[field.name] = field.defaultValue;
        }
      });
      updateStrategy({
        strategy_type: newType,
        parameters: defaultParams,
      });
    }
  };

  // 更新单个参数
  const updateParameter = (key: string, value: string | number | boolean) => {
    updateStrategy({
      parameters: {
        ...strategy.parameters,
        [key]: value,
      },
    });
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
        策略配置
      </h4>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            策略名称 *:
          </label>
          <Input
            type="text"
            value={strategy.name}
            onChange={(e) => updateStrategy({ name: e.target.value })}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            策略类型 *:
          </label>
          <Select 
            value={strategy.strategy_type} 
            onValueChange={handleStrategyTypeChange}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="选择策略类型" />
            </SelectTrigger>
            <SelectContent>
              {strategyTypes.map((def) => (
                <SelectItem key={def.type} value={def.type}>
                  {def.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {currentStrategyDef && (
            <p className="text-xs text-gray-500 mt-1">{currentStrategyDef.description}</p>
          )}
        </div>
        <div className="flex items-end">
          <label className="inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={strategy.enabled}
              onChange={(e) => updateStrategy({ enabled: e.target.checked })}
              className="form-checkbox h-5 w-5 text-blue-600"
            />
            <span className="ml-2 text-sm font-medium text-gray-700">启用策略</span>
          </label>
        </div>
      </div>
      
      {/* 动态策略参数表单 */}
      {currentStrategyDef && (
        <div className="mt-4">
          <h5 className="text-sm font-semibold text-gray-700 mb-3">
            策略参数 (JSON格式):
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {currentStrategyDef.fields.map((field) => (
              <div key={field.name}>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  {field.label} {field.required && <span className="text-red-500">*</span>}:
                </label>
                {field.type === 'number' ? (
                  <Input
                    type="number"
                    value={strategy.parameters[field.name] as number || ''}
                    onChange={(e) => updateParameter(field.name, parseFloat(e.target.value) || 0)}
                    placeholder={field.placeholder}
                    min={field.min}
                    max={field.max}
                    step={field.step}
                    className="w-full"
                  />
                ) : field.type === 'text' ? (
                  <Input
                    type="text"
                    value={strategy.parameters[field.name] as string || ''}
                    onChange={(e) => updateParameter(field.name, e.target.value)}
                    placeholder={field.placeholder}
                    className="w-full"
                  />
                ) : field.type === 'checkbox' ? (
                  <label className="inline-flex items-center">
                    <input
                      type="checkbox"
                      checked={strategy.parameters[field.name] as boolean || false}
                      onChange={(e) => updateParameter(field.name, e.target.checked)}
                      className="form-checkbox h-5 w-5 text-blue-600"
                    />
                    <span className="ml-2 text-sm text-gray-700">{field.description}</span>
                  </label>
                ) : null}
                {field.description && field.type !== 'checkbox' && (
                  <p className="text-xs text-gray-500 mt-1">{field.description}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

// ==================== 投资组合配置区块 ====================

interface PortfolioSectionProps {
  config: PortfolioConfig;
  onChange: (config: PortfolioConfig) => void;
}

export function PortfolioSection({ config, onChange }: PortfolioSectionProps) {
  const updateField = <K extends keyof PortfolioConfig>(key: K, value: PortfolioConfig[K]) => {
    onChange({ ...config, [key]: value });
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
        投资组合配置
      </h4>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            初始资金 *:
          </label>
          <Input
            type="number"
            step="0.01"
            value={config.initial_cash}
            onChange={(e) => updateField('initial_cash', parseFloat(e.target.value) || 10000)}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            手续费率 *:
          </label>
          <Input
            type="number"
            step="0.0001"
            value={config.commission}
            onChange={(e) => updateField('commission', parseFloat(e.target.value) || 0.001)}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            滑点率 *:
          </label>
          <Input
            type="number"
            step="0.0001"
            value={config.slippage}
            onChange={(e) => updateField('slippage', parseFloat(e.target.value) || 0)}
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            单笔最大持仓金额:
          </label>
          <Input
            type="number"
            step="0.01"
            value={config.max_position_size || ''}
            onChange={(e) => updateField('max_position_size', e.target.value ? parseFloat(e.target.value) : undefined)}
            placeholder="可选"
            className="w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            最大持仓数量:
          </label>
          <Input
            type="number"
            value={config.max_positions || ''}
            onChange={(e) => updateField('max_positions', e.target.value ? parseInt(e.target.value) : undefined)}
            placeholder="可选"
            className="w-full"
          />
        </div>
      </div>
      
      
      {/* 风险管理配置 */}
      <div className="mt-6">
        <h5 className="text-sm font-semibold text-gray-700 mb-3 flex items-center justify-between">
          <span>风险管理配置 (可选)</span>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">
              {config.risk_rules ? '已启用' : '已禁用'}
            </span>
            <Switch
              checked={!!config.risk_rules}
              onCheckedChange={(checked) => {
                if (checked) {
                  // 启用风险管理,设置默认值
                  updateField('risk_rules', {});
                } else {
                  // 禁用风险管理
                  updateField('risk_rules', undefined);
                }
              }}
            />
          </div>
        </h5>
        {config.risk_rules && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  止损百分比 (%):
                </label>
                <Input
                  type="number"
                  step="0.1"
                  value={config.risk_rules.stop_loss_pct || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      stop_loss_pct: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="2.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">价格跌破此百分比时自动止损</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  止盈百分比 (%):
                </label>
                <Input
                  type="number"
                  step="0.1"
                  value={config.risk_rules.take_profit_pct || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      take_profit_pct: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="5.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">价格涨至此百分比时自动止盈</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  最大回撤限制 (%):
                </label>
                <Input
                  type="number"
                  step="0.1"
                  value={config.risk_rules.max_drawdown_pct || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      max_drawdown_pct: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="15.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">达到此值时自动停止交易</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  单日最大亏损限制 (%):
                </label>
                <Input
                  type="number"
                  step="0.1"
                  value={config.risk_rules.max_daily_loss_pct || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      max_daily_loss_pct: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="5.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">防止单日损失过大</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  单笔最大亏损限制 (%):
                </label>
                <Input
                  type="number"
                  step="0.1"
                  value={config.risk_rules.max_single_trade_loss_pct || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      max_single_trade_loss_pct: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="3.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">限制单笔交易的风险敞口</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  连续亏损次数限制:
                </label>
                <Input
                  type="number"
                  value={config.risk_rules.max_consecutive_losses || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      max_consecutive_losses: e.target.value ? parseInt(e.target.value) : undefined,
                    })
                  }
                  placeholder="3"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">连续亏损达到此次数后停止</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  账户最低权益要求:
                </label>
                <Input
                  type="number"
                  step="0.01"
                  value={config.risk_rules.min_equity || ''}
                  onChange={(e) =>
                    updateField('risk_rules', {
                      ...config.risk_rules,
                      min_equity: e.target.value ? parseFloat(e.target.value) : undefined,
                    })
                  }
                  placeholder="5000.0"
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">账户权益低于此值时停止交易</p>
              </div>
            </div>
        )}
      </div>

      {/* 仓位管理配置 */}
      <div className="mt-6">
        <h5 className="text-sm font-semibold text-gray-700 mb-3 flex items-center justify-between">
          <span>仓位管理配置 (可选)</span>
          <div className="flex items-center gap-2">
            <span className="text-xs text-gray-500">
              {config.position_sizing ? '已启用' : '已禁用'}
            </span>
            <Switch
              checked={!!config.position_sizing}
              onCheckedChange={(checked) => {
                if (checked) {
                  // 启用仓位管理,设置默认策略
                  updateField('position_sizing', { strategy_type: 'fixed_percentage', percentage: 0.2 });
                } else {
                  // 禁用仓位管理
                  updateField('position_sizing', undefined);
                }
              }}
            />
          </div>
        </h5>
        {config.position_sizing && (
          <div>
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                仓位管理策略 *:
              </label>
              <Select
                value={config.position_sizing.strategy_type}
                onValueChange={(value) => {
                  // 根据不同策略类型设置默认参数
                  if (value === 'fixed_percentage') {
                    updateField('position_sizing', { strategy_type: 'fixed_percentage', percentage: 0.2 });
                  } else if (value === 'kelly_criterion') {
                    updateField('position_sizing', {
                      strategy_type: 'kelly_criterion',
                      win_rate: 0.6,
                      profit_loss_ratio: 2.0,
                      kelly_fraction: 0.5,
                    });
                  } else if (value === 'pyramid') {
                    updateField('position_sizing', {
                      strategy_type: 'pyramid',
                      initial_percentage: 0.1,
                      profit_threshold: 5.0,
                      max_percentage: 0.5,
                      increment: 0.1,
                    });
                  } else if (value === 'fixed_amount') {
                    updateField('position_sizing', { strategy_type: 'fixed_amount', amount: 1000.0 });
                  } else if (value === 'all_in') {
                    updateField('position_sizing', { strategy_type: 'all_in' });
                  }
                }}
              >
                <SelectTrigger className="w-full md:w-1/3">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="fixed_percentage">固定比例策略</SelectItem>
                  <SelectItem value="kelly_criterion">Kelly准则策略</SelectItem>
                  <SelectItem value="pyramid">金字塔加仓策略</SelectItem>
                  <SelectItem value="fixed_amount">固定金额策略</SelectItem>
                  <SelectItem value="all_in">全仓策略</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {/* 固定比例策略参数 */}
            {config.position_sizing.strategy_type === 'fixed_percentage' && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    账户权益百分比 *:
                  </label>
                  <Input
                    type="number"
                    step="0.01"
                    min="0"
                    max="1"
                    value={config.position_sizing.percentage}
                    onChange={(e) =>
                      updateField('position_sizing', {
                        strategy_type: 'fixed_percentage',
                        percentage: parseFloat(e.target.value) || 0,
                      })
                    }
                    placeholder="0.2"
                    className="w-full"
                  />
                  <p className="text-xs text-gray-500 mt-1">每次交易使用账户权益的比例 (0-1)</p>
                </div>
              </div>
            )}

            {/* Kelly准则策略参数 */}
            {config.position_sizing.strategy_type === 'kelly_criterion' && (() => {
              const kellySizing = config.position_sizing as { strategy_type: 'kelly_criterion'; win_rate: number; profit_loss_ratio: number; kelly_fraction: number };
              return (
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      历史胜率 *:
                    </label>
                    <Input
                      type="number"
                      step="0.01"
                      min="0"
                      max="1"
                      value={kellySizing.win_rate}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'kelly_criterion',
                          win_rate: parseFloat(e.target.value) || 0,
                          profit_loss_ratio: kellySizing.profit_loss_ratio,
                          kelly_fraction: kellySizing.kelly_fraction,
                        })
                      }
                      placeholder="0.6"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">历史胜率 (0-1)</p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      盈亏比 *:
                    </label>
                    <Input
                      type="number"
                      step="0.1"
                      min="0"
                      value={kellySizing.profit_loss_ratio}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'kelly_criterion',
                          win_rate: kellySizing.win_rate,
                          profit_loss_ratio: parseFloat(e.target.value) || 0,
                          kelly_fraction: kellySizing.kelly_fraction,
                        })
                      }
                      placeholder="2.0"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">平均盈利/平均亏损</p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Kelly分数 *:
                    </label>
                    <Input
                      type="number"
                      step="0.1"
                      min="0"
                      max="1"
                      value={kellySizing.kelly_fraction}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'kelly_criterion',
                          win_rate: kellySizing.win_rate,
                          profit_loss_ratio: kellySizing.profit_loss_ratio,
                          kelly_fraction: parseFloat(e.target.value) || 0,
                        })
                      }
                      placeholder="0.5"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">保守程度 (0.5为半凯利)</p>
                  </div>
                </div>
              );
            })()}

            {/* 金字塔加仓策略参数 */}
            {config.position_sizing.strategy_type === 'pyramid' && (() => {
              const pyramidSizing = config.position_sizing as { strategy_type: 'pyramid'; initial_percentage: number; profit_threshold: number; max_percentage: number; increment: number };
              return (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      初始仓位百分比 *:
                    </label>
                    <Input
                      type="number"
                      step="0.01"
                      min="0"
                      max="1"
                      value={pyramidSizing.initial_percentage}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'pyramid',
                          initial_percentage: parseFloat(e.target.value) || 0,
                          profit_threshold: pyramidSizing.profit_threshold,
                          max_percentage: pyramidSizing.max_percentage,
                          increment: pyramidSizing.increment,
                        })
                      }
                      placeholder="0.1"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">初始建仓比例 (0-1)</p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      盈利阈值 (%) *:
                    </label>
                    <Input
                      type="number"
                      step="0.1"
                      min="0"
                      value={pyramidSizing.profit_threshold}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'pyramid',
                          initial_percentage: pyramidSizing.initial_percentage,
                          profit_threshold: parseFloat(e.target.value) || 0,
                          max_percentage: pyramidSizing.max_percentage,
                          increment: pyramidSizing.increment,
                        })
                      }
                      placeholder="5.0"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">达到此盈利百分比时触发加仓</p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      最大仓位百分比 *:
                    </label>
                    <Input
                      type="number"
                      step="0.01"
                      min="0"
                      max="1"
                      value={pyramidSizing.max_percentage}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'pyramid',
                          initial_percentage: pyramidSizing.initial_percentage,
                          profit_threshold: pyramidSizing.profit_threshold,
                          max_percentage: parseFloat(e.target.value) || 0,
                          increment: pyramidSizing.increment,
                        })
                      }
                      placeholder="0.5"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">最大仓位比例 (0-1)</p>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      加仓增量 *:
                    </label>
                    <Input
                      type="number"
                      step="0.01"
                      min="0"
                      max="1"
                      value={pyramidSizing.increment}
                      onChange={(e) =>
                        updateField('position_sizing', {
                          strategy_type: 'pyramid',
                          initial_percentage: pyramidSizing.initial_percentage,
                          profit_threshold: pyramidSizing.profit_threshold,
                          max_percentage: pyramidSizing.max_percentage,
                          increment: parseFloat(e.target.value) || 0,
                        })
                      }
                      placeholder="0.1"
                      className="w-full"
                    />
                    <p className="text-xs text-gray-500 mt-1">每次加仓的比例 (0-1)</p>
                  </div>
                </div>
              );
            })()}

            {/* 固定金额策略参数 */}
            {config.position_sizing.strategy_type === 'fixed_amount' && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    固定金额 *:
                  </label>
                  <Input
                    type="number"
                    step="0.01"
                    min="0"
                    value={config.position_sizing.amount}
                    onChange={(e) =>
                      updateField('position_sizing', {
                        strategy_type: 'fixed_amount',
                        amount: parseFloat(e.target.value) || 0,
                      })
                    }
                    placeholder="1000.0"
                    className="w-full"
                  />
                  <p className="text-xs text-gray-500 mt-1">每次交易使用的固定金额</p>
                </div>
              </div>
            )}

            {/* 全仓策略提示 */}
            {config.position_sizing.strategy_type === 'all_in' && (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded">
                <p className="text-sm text-yellow-800">
                  ⚠️ 警告: 全仓策略风险极高,不推荐使用。将使用所有可用资金进行交易。
                </p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

// ==================== 日志配置区块 ====================

interface LoggingSectionProps {
  config: LoggingConfig;
  onChange: (config: LoggingConfig) => void;
}

export function LoggingSection({ config, onChange }: LoggingSectionProps) {
  const updateField = <K extends keyof LoggingConfig>(key: K, value: LoggingConfig[K]) => {
    onChange({ ...config, [key]: value });
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
        日志配置
      </h4>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            日志级别 *:
          </label>
          <Select value={config.level} onValueChange={(value) => updateField('level', value as LoggingConfig['level'])}>
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="trace">Trace</SelectItem>
              <SelectItem value="debug">Debug</SelectItem>
              <SelectItem value="info">Info</SelectItem>
              <SelectItem value="warn">Warn</SelectItem>
              <SelectItem value="error">Error</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            日志格式 *:
          </label>
          <Select value={config.format} onValueChange={(value) => updateField('format', value as LoggingConfig['format'])}>
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="json">JSON</SelectItem>
              <SelectItem value="pretty">Pretty</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            日志文件路径:
          </label>
          <Input
            type="text"
            value={config.output || ''}
            onChange={(e) => updateField('output', e.target.value || undefined)}
            placeholder="aurora.log"
            className="w-full"
          />
        </div>
      </div>
    </div>
  );
}

// ==================== 回测配置区块 ====================

interface BacktestSectionProps {
  config?: BacktestSettings;
  onChange: (config?: BacktestSettings) => void;
}

export function BacktestSection({ config, onChange }: BacktestSectionProps) {
  // 存储已下载的数据文件列表
  const [dataFiles, setDataFiles] = React.useState<string[]>([]);

  // 加载数据文件列表
  React.useEffect(() => {
    const loadDataFiles = async () => {
      try {
        const response = await dataApi.list();
        if (response.success && response.data) {
          const filenames = response.data.map(file => file.filename);
          setDataFiles(filenames);
        }
      } catch (error) {
        console.error('加载数据文件列表失败:', error);
      }
    };
    loadDataFiles();
  }, []);

  const updateField = <K extends keyof NonNullable<BacktestSettings>>(
    key: K,
    value: NonNullable<BacktestSettings>[K]
  ) => {
    onChange({ ...config, [key]: value } as BacktestSettings);
  };

  const updateBenchmarkField = <K extends keyof BenchmarkConfig>(
    key: K,
    value: BenchmarkConfig[K]
  ) => {
    const currentBenchmark = config?.benchmark || { enabled: false };
    const updatedBenchmark = { ...currentBenchmark, [key]: value };
    
    // 如果禁用基准，则将整个 benchmark 设置为 undefined
    if (key === 'enabled' && value === false) {
      updateField('benchmark', undefined);
    } else {
      updateField('benchmark', updatedBenchmark);
    }
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b flex items-center justify-between">
        <span>回测配置 (可选)</span>
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">
            {config !== undefined ? '已启用' : '已禁用'}
          </span>
          <Switch
            checked={config !== undefined}
            onCheckedChange={(checked) => {
              if (checked) {
                // 启用回测配置,设置默认值
                onChange({ data_path: '' });
              } else {
                // 禁用回测配置
                onChange(undefined);
              }
            }}
          />
        </div>
      </h4>
      {config !== undefined && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              数据文件路径 *:
            </label>
            <Input
              type="text"
              value={config.data_path || ''}
              onChange={(e) => updateField('data_path', e.target.value)}
              placeholder="data/btc_1h.csv"
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              交易对符号:
            </label>
            <Input
              type="text"
              value={config.symbol || ''}
              onChange={(e) => updateField('symbol', e.target.value || undefined)}
              placeholder="BTCUSDT"
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              时间间隔:
            </label>
            <Input
              type="text"
              value={config.interval || ''}
              onChange={(e) => updateField('interval', e.target.value || undefined)}
              placeholder="1h"
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              开始时间:
            </label>
            <Input
              type="text"
              value={config.start_time || ''}
              onChange={(e) => updateField('start_time', e.target.value || undefined)}
              placeholder="2024-01-01"
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              结束时间:
            </label>
            <Input
              type="text"
              value={config.end_time || ''}
              onChange={(e) => updateField('end_time', e.target.value || undefined)}
              placeholder="2024-12-31"
              className="w-full"
            />
          </div>

          {/* 定价模式配置 */}
          <div className="col-span-full">
            <h5 className="text-sm font-semibold text-gray-800 mb-3 mt-2">定价模式配置</h5>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  定价模式:
                </label>
                <Select
                  value={config.pricing_mode?.mode || 'none'}
                  onValueChange={(value) => {
                    if (value === 'none') {
                      // 移除定价模式配置
                      updateField('pricing_mode', undefined);
                    } else if (value === 'close') {
                      // 设置为收盘价模式
                      updateField('pricing_mode', { mode: 'close' } as PricingMode);
                    } else if (value === 'bid_ask') {
                      // 设置为买一卖一价模式，默认价差 0.1%
                      updateField('pricing_mode', { mode: 'bid_ask', spread_pct: 0.001 } as PricingMode);
                    }
                  }}
                >
                  <SelectTrigger className="w-full">
                    <SelectValue placeholder="选择定价模式" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="none">默认(不设置)</SelectItem>
                    <SelectItem value="close">收盘价模式</SelectItem>
                    <SelectItem value="bid_ask">买一卖一价模式</SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-xs text-gray-500 mt-1">
                  控制回测中交易价格的计算方式
                </p>
              </div>

              {/* 仅在选择买一卖一价模式时显示价差输入 */}
              {config.pricing_mode?.mode === 'bid_ask' && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    买卖价差百分比:
                  </label>
                  <Input
                    type="number"
                    step="0.0001"
                    min="0"
                    max="1"
                    value={config.pricing_mode.spread_pct}
                    onChange={(e) => {
                      const newValue = parseFloat(e.target.value) || 0;
                      updateField('pricing_mode', {
                        mode: 'bid_ask',
                        spread_pct: newValue,
                      } as PricingMode);
                    }}
                    placeholder="0.001"
                    className="w-full"
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    例如 0.001 表示 0.1% 的价差
                  </p>
                </div>
              )}
            </div>
          </div>

          {/* 基准配置 */}
          <div className="col-span-full">
            <h5 className="text-sm font-semibold text-gray-800 mb-3 mt-2 flex items-center justify-between">
              <span>基准配置</span>
              <div className="flex items-center gap-2">
                <span className="text-xs text-gray-500">
                  {config.benchmark?.enabled ? '已启用' : '已禁用'}
                </span>
                <Switch
                  checked={config.benchmark?.enabled || false}
                  onCheckedChange={(checked) => {
                    updateBenchmarkField('enabled', checked);
                    if (!checked) {
                      // 禁用时清除数据路径
                      updateBenchmarkField('data_path', undefined);
                    }
                  }}
                />
              </div>
            </h5>
            {config.benchmark?.enabled && (
              <div className="grid grid-cols-1 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    基准数据文件 *:
                  </label>
                  <Select
                    value={config.benchmark.data_path || ''}
                    onValueChange={(value) => updateBenchmarkField('data_path', value)}
                  >
                    <SelectTrigger className="w-full">
                      <SelectValue placeholder="选择基准数据文件" />
                    </SelectTrigger>
                    <SelectContent>
                      {dataFiles.length > 0 ? (
                        dataFiles.map((filename) => (
                          <SelectItem key={filename} value={filename}>
                            {filename}
                          </SelectItem>
                        ))
                      ) : (
                        <SelectItem value="" disabled>
                          暂无可用数据文件
                        </SelectItem>
                      )}
                    </SelectContent>
                  </Select>
                  <p className="text-xs text-gray-500 mt-1">
                    选择用于基准对比的数据文件
                  </p>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

// ==================== 实时交易配置区块 ====================

interface LiveSectionProps {
  config?: LiveConfig;
  onChange: (config?: LiveConfig) => void;
}

export function LiveSection({ config, onChange }: LiveSectionProps) {
  const updateField = <K extends keyof NonNullable<LiveConfig>>(
    key: K,
    value: NonNullable<LiveConfig>[K]
  ) => {
    onChange({ ...config, [key]: value } as LiveConfig);
  };

  return (
    <div>
      <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b flex items-center justify-between">
        <span>实时交易配置 (可选)</span>
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500">
            {config !== undefined ? '已启用' : '已禁用'}
          </span>
          <Switch
            checked={config !== undefined}
            onCheckedChange={(checked) => {
              if (checked) {
                // 启用实时交易配置,设置默认值
                onChange({ symbol: '', interval: '1m', paper_trading: true });
              } else {
                // 禁用实时交易配置
                onChange(undefined);
              }
            }}
          />
        </div>
      </h4>
      {config !== undefined && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              交易对符号 *:
            </label>
            <Input
              type="text"
              value={config.symbol}
              onChange={(e) => updateField('symbol', e.target.value)}
              placeholder="BTCUSDT"
              className="w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              K线间隔 *:
            </label>
            <Select value={config.interval} onValueChange={(value) => updateField('interval', value)}>
              <SelectTrigger className="w-full">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="1m">1分钟</SelectItem>
                <SelectItem value="5m">5分钟</SelectItem>
                <SelectItem value="15m">15分钟</SelectItem>
                <SelectItem value="1h">1小时</SelectItem>
                <SelectItem value="4h">4小时</SelectItem>
                <SelectItem value="1d">1天</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-end">
            <label className="inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={config.paper_trading}
                onChange={(e) => updateField('paper_trading', e.target.checked)}
                className="form-checkbox h-5 w-5 text-blue-600"
              />
              <span className="ml-2 text-sm font-medium text-gray-700">模拟交易</span>
            </label>
          </div>
        </div>
      )}
    </div>
  );
}
