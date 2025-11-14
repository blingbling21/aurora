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

import { NavMenuItem } from '@/types';

// 导航菜单配置
export const NAV_MENU_ITEMS: NavMenuItem[] = [
  {
    id: 'dashboard',
    label: '仪表盘',
    icon: '📊',
    href: '/',
  },
  {
    id: 'config',
    label: '配置管理',
    icon: '⚙️',
    href: '/config',
  },
  {
    id: 'data',
    label: '数据管理',
    icon: '📁',
    href: '/data',
  },
  {
    id: 'backtest',
    label: '回测执行',
    icon: '🚀',
    href: '/backtest',
  },
  {
    id: 'history',
    label: '历史记录',
    icon: '📜',
    href: '/history',
  },
];

// 交易所选项
export const EXCHANGE_OPTIONS = [
  { value: 'binance', label: 'Binance' },
  { value: 'okx', label: 'OKX' },
  { value: 'bybit', label: 'Bybit' },
  { value: 'coinbase', label: 'Coinbase' },
];

// 时间周期选项
export const INTERVAL_OPTIONS = [
  { value: '1m', label: '1分钟' },
  { value: '5m', label: '5分钟' },
  { value: '15m', label: '15分钟' },
  { value: '30m', label: '30分钟' },
  { value: '1h', label: '1小时' },
  { value: '4h', label: '4小时' },
  { value: '1d', label: '1天' },
  { value: '1w', label: '1周' },
];

// 交易对选项
export const SYMBOL_OPTIONS = [
  { value: 'BTCUSDT', label: 'BTCUSDT - 比特币' },
  { value: 'ETHUSDT', label: 'ETHUSDT - 以太坊' },
  { value: 'BNBUSDT', label: 'BNBUSDT - 币安币' },
  { value: 'SOLUSDT', label: 'SOLUSDT - Solana' },
  { value: 'XRPUSDT', label: 'XRPUSDT - 瑞波币' },
  { value: 'ADAUSDT', label: 'ADAUSDT - 艾达币' },
  { value: 'DOGEUSDT', label: 'DOGEUSDT - 狗狗币' },
  { value: 'DOTUSDT', label: 'DOTUSDT - 波卡' },
  { value: 'MATICUSDT', label: 'MATICUSDT - Polygon' },
  { value: 'AVAXUSDT', label: 'AVAXUSDT - Avalanche' },
];

// 策略类型选项
export const STRATEGY_OPTIONS = [
  { value: 'ma-crossover', label: '均线交叉' },
  { value: 'rsi', label: 'RSI' },
  { value: 'macd', label: 'MACD' },
  { value: 'bollinger', label: '布林带' },
  { value: 'custom', label: '自定义' },
];

// 定价模式选项
export const PRICING_MODE_OPTIONS = [
  { value: 'close', label: '收盘价' },
  { value: 'open', label: '开盘价' },
  { value: 'high', label: '最高价' },
  { value: 'low', label: '最低价' },
  { value: 'vwap', label: '成交量加权平均价' },
  { value: 'bidask', label: '买卖价差' },
];

// 时区选项
export const TIMEZONE_OPTIONS = [
  { value: 'UTC', label: 'UTC (协调世界时)', offset: '+00:00' },
  { value: 'Asia/Shanghai', label: 'Asia/Shanghai (北京时间 UTC+8)', offset: '+08:00' },
  { value: 'Asia/Tokyo', label: 'Asia/Tokyo (东京时间 UTC+9)', offset: '+09:00' },
  { value: 'Asia/Hong_Kong', label: 'Asia/Hong_Kong (香港时间 UTC+8)', offset: '+08:00' },
  { value: 'Asia/Singapore', label: 'Asia/Singapore (新加坡时间 UTC+8)', offset: '+08:00' },
  { value: 'Europe/London', label: 'Europe/London (伦敦时间 UTC+0/+1)', offset: '+00:00' },
  { value: 'Europe/Paris', label: 'Europe/Paris (巴黎时间 UTC+1/+2)', offset: '+01:00' },
  { value: 'America/New_York', label: 'America/New_York (纽约时间 UTC-5/-4)', offset: '-05:00' },
  { value: 'America/Chicago', label: 'America/Chicago (芝加哥时间 UTC-6/-5)', offset: '-06:00' },
  { value: 'America/Los_Angeles', label: 'America/Los_Angeles (洛杉矶时间 UTC-8/-7)', offset: '-08:00' },
];

/**
 * 获取当前浏览器的时区
 * @returns IANA 时区标识符
 */
export function getCurrentTimezone(): string {
  return Intl.DateTimeFormat().resolvedOptions().timeZone;
}

// API 基础路径
export const API_BASE_URL = '/api';
