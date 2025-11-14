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
 * 时区字段保存测试
 * 验证配置保存时timezone字段是否正确包含
 */

import { stringifyTOML } from '@/lib/utils/toml';
import { AuroraConfig } from '@/types/config-schema';
import { getCurrentTimezone } from '@/constants';

describe('Timezone field in config save', () => {
  it('应该在配置中包含timezone字段', async () => {
    // 模拟配置对象（包含timezone）
    const config: AuroraConfig = {
      data_source: {
        provider: 'binance',
        timeout: 30,
        max_retries: 3,
      },
      strategies: [
        {
          name: 'MA交叉策略',
          strategy_type: 'ma-crossover',
          enabled: true,
          parameters: {
            short: 10,
            long: 30,
          },
        },
      ],
      portfolio: {
        initial_cash: 10000,
        commission: 0.001,
        slippage: 0.0005,
        risk_rules: {
          max_drawdown_pct: 15,
          max_daily_loss_pct: 5,
          max_consecutive_losses: 3,
          max_single_trade_loss_pct: 3,
          min_equity: 5000,
          stop_loss_pct: 2,
          take_profit_pct: 5,
        },
        position_sizing: {
          strategy_type: 'fixed_percentage',
          percentage: 0.2,
        },
      },
      logging: {
        level: 'info',
        format: 'pretty',
      },
      backtest: {
        data_path: 'btc_1h.csv',
        symbol: 'BTCUSDT',
        interval: '1h',
        start_time: '2024-01-01',
        end_time: '2024-12-31',
        timezone: 'Asia/Shanghai', // 显式设置时区
        pricing_mode: {
          mode: 'bid_ask',
          spread_pct: 0.001,
        },
      },
    };

    // 转换为TOML
    const tomlText = await stringifyTOML(config);

    // 验证TOML文本中包含timezone字段
    expect(tomlText).toContain('timezone');
    expect(tomlText).toContain('Asia/Shanghai');
  });

  it('应该在没有timezone时使用默认值', async () => {
    // 模拟配置对象（没有timezone）
    const configWithoutTimezone: AuroraConfig = {
      data_source: {
        provider: 'binance',
        timeout: 30,
        max_retries: 3,
      },
      strategies: [
        {
          name: 'MA交叉策略',
          strategy_type: 'ma-crossover',
          enabled: true,
          parameters: {
            short: 10,
            long: 30,
          },
        },
      ],
      portfolio: {
        initial_cash: 10000,
        commission: 0.001,
        slippage: 0.0005,
      },
      logging: {
        level: 'info',
        format: 'pretty',
      },
      backtest: {
        data_path: 'btc_1h.csv',
        symbol: 'BTCUSDT',
        interval: '1h',
        start_time: '2024-01-01',
        end_time: '2024-12-31',
        // 没有 timezone 字段
      },
    };

    // 模拟保存逻辑：添加默认timezone
    const configToSave = { ...configWithoutTimezone };
    if (configToSave.backtest && !configToSave.backtest.timezone) {
      configToSave.backtest = {
        ...configToSave.backtest,
        timezone: getCurrentTimezone(),
      };
    }

    // 转换为TOML
    const tomlText = await stringifyTOML(configToSave);

    // 验证TOML文本中包含timezone字段
    expect(tomlText).toContain('timezone');
    // 验证使用了默认时区
    const defaultTimezone = getCurrentTimezone();
    expect(tomlText).toContain(defaultTimezone);
  });

  it('应该保留已有的timezone值', async () => {
    const config: AuroraConfig = {
      data_source: {
        provider: 'binance',
        timeout: 30,
        max_retries: 3,
      },
      strategies: [
        {
          name: 'MA交叉策略',
          strategy_type: 'ma-crossover',
          enabled: true,
          parameters: {
            short: 10,
            long: 30,
          },
        },
      ],
      portfolio: {
        initial_cash: 10000,
        commission: 0.001,
        slippage: 0.0005,
      },
      logging: {
        level: 'info',
        format: 'pretty',
      },
      backtest: {
        data_path: 'btc_1h.csv',
        timezone: 'America/New_York', // 使用纽约时区
      },
    };

    // 模拟保存逻辑：如果已有timezone，不应该被覆盖
    const configToSave = { ...config };
    if (configToSave.backtest && !configToSave.backtest.timezone) {
      configToSave.backtest = {
        ...configToSave.backtest,
        timezone: getCurrentTimezone(),
      };
    }

    const tomlText = await stringifyTOML(configToSave);

    // 验证保留了原有的时区值
    expect(tomlText).toContain('timezone');
    expect(tomlText).toContain('America/New_York');
    // 验证没有被替换成默认时区
    if (getCurrentTimezone() !== 'America/New_York') {
      expect(tomlText).not.toContain(getCurrentTimezone());
    }
  });

  it('应该处理没有backtest配置的情况', async () => {
    const config: AuroraConfig = {
      data_source: {
        provider: 'binance',
        timeout: 30,
        max_retries: 3,
      },
      strategies: [
        {
          name: 'MA交叉策略',
          strategy_type: 'ma-crossover',
          enabled: true,
          parameters: {
            short: 10,
            long: 30,
          },
        },
      ],
      portfolio: {
        initial_cash: 10000,
        commission: 0.001,
        slippage: 0.0005,
      },
      logging: {
        level: 'info',
        format: 'pretty',
      },
      // 没有 backtest 配置
    };

    // 模拟保存逻辑
    const configToSave = { ...config };
    if (configToSave.backtest && !configToSave.backtest.timezone) {
      configToSave.backtest = {
        ...configToSave.backtest,
        timezone: getCurrentTimezone(),
      };
    }

    // 应该能正常转换，不会报错
    const tomlText = await stringifyTOML(configToSave);
    expect(tomlText).toBeDefined();
    // 没有backtest配置时，不应该包含timezone
    expect(tomlText).not.toContain('timezone');
  });
});
