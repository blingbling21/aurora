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
 * 定价模式配置示例
 * 
 * 该文件展示了如何在配置中使用定价模式功能
 */

// ==================== 示例 1: 不设置定价模式（使用默认） ====================

export const example1_noConfig = {
  backtest: {
    data_path: 'btc_1h.csv',
    symbol: 'BTCUSDT',
    interval: '1h',
    // pricing_mode 字段省略，将使用默认行为
  },
};

// ==================== 示例 2: 使用收盘价模式 ====================

export const example2_closeMode = {
  backtest: {
    data_path: 'btc_1h.csv',
    symbol: 'BTCUSDT',
    interval: '1h',
    pricing_mode: {
      mode: 'close',
    },
  },
};

// ==================== 示例 3: 使用买一卖一价模式 ====================

export const example3_bidAskMode = {
  backtest: {
    data_path: 'btc_1h.csv',
    symbol: 'BTCUSDT',
    interval: '1h',
    pricing_mode: {
      mode: 'bid_ask',
      spread_pct: 0.001, // 0.1% 价差
    },
  },
};

// ==================== 示例 4: 更大的价差（高流动性币种） ====================

export const example4_largerSpread = {
  backtest: {
    data_path: 'eth_4h.csv',
    symbol: 'ETHUSDT',
    interval: '4h',
    pricing_mode: {
      mode: 'bid_ask',
      spread_pct: 0.002, // 0.2% 价差
    },
  },
};

// ==================== 示例 5: 完整配置（包含定价模式） ====================

export const example5_fullConfig = {
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
    initial_cash: 10000.0,
    commission: 0.001,
    slippage: 0.0005,
    risk_rules: {
      max_drawdown_pct: 15.0,
      stop_loss_pct: 2.0,
      take_profit_pct: 5.0,
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
    pricing_mode: {
      mode: 'bid_ask',
      spread_pct: 0.001,
    },
  },
};

// ==================== 定价模式说明 ====================

/**
 * 定价模式类型说明：
 * 
 * 1. close 模式（收盘价）
 *    - 最简单的模式
 *    - 买入和卖出都使用 K 线收盘价
 *    - 适合快速测试和简单场景
 *    - 真实度较低，因为实际交易中存在买卖价差
 * 
 * 2. bid_ask 模式（买一卖一价）
 *    - 更真实的模式
 *    - 买入时使用卖一价（ask）
 *    - 卖出时使用买一价（bid）
 *    - 需要设置 spread_pct（价差百分比）
 *    - 适合更精确的回测
 * 
 * spread_pct 参数说明：
 *    - 范围：0 到 1
 *    - 0.001 表示 0.1% 的价差
 *    - 0.01 表示 1% 的价差
 *    - 一般主流币种价差在 0.01% - 0.1% 之间
 *    - 小币种或低流动性币种价差可能更大
 */

// ==================== TOML 配置示例 ====================

/**
 * 对应的 TOML 配置格式：
 * 
 * 收盘价模式：
 * ```toml
 * [backtest]
 * data_path = "btc_1h.csv"
 * symbol = "BTCUSDT"
 * interval = "1h"
 * 
 * [backtest.pricing_mode]
 * mode = "close"
 * ```
 * 
 * 买一卖一价模式：
 * ```toml
 * [backtest]
 * data_path = "btc_1h.csv"
 * symbol = "BTCUSDT"
 * interval = "1h"
 * 
 * [backtest.pricing_mode]
 * mode = "bid_ask"
 * spread_pct = 0.001
 * ```
 */
