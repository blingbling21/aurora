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

import { formatDateToYYYYMMDD, generateDataFilename } from './filename';

describe('formatDateToYYYYMMDD', () => {
  // 测试基本日期格式化
  it('应该将日期格式化为 YYYYMMDD 格式', () => {
    const date = new Date('2025-01-15');
    expect(formatDateToYYYYMMDD(date)).toBe('20250115');
  });

  // 测试单数字月份和日期的零填充
  it('应该为单数字月份和日期添加前导零', () => {
    const date = new Date('2025-03-05');
    expect(formatDateToYYYYMMDD(date)).toBe('20250305');
  });

  // 测试年末日期
  it('应该正确处理年末日期', () => {
    const date = new Date('2024-12-31');
    expect(formatDateToYYYYMMDD(date)).toBe('20241231');
  });

  // 测试年初日期
  it('应该正确处理年初日期', () => {
    const date = new Date('2025-01-01');
    expect(formatDateToYYYYMMDD(date)).toBe('20250101');
  });
});

describe('generateDataFilename', () => {
  // 测试完整的文件名生成
  it('应该生成正确格式的文件名', () => {
    const filename = generateDataFilename(
      'binance',
      'BTCUSDT',
      '1h',
      new Date('2025-01-01'),
      new Date('2025-01-31')
    );
    expect(filename).toBe('binance_btcusdt_1h_20250101_to_20250131.csv');
  });

  // 测试交易所和交易对转小写
  it('应该将交易所和交易对转为小写', () => {
    const filename = generateDataFilename(
      'BINANCE',
      'ETHUSDT',
      '5m',
      new Date('2025-02-01'),
      new Date('2025-02-28')
    );
    expect(filename).toBe('binance_ethusdt_5m_20250201_to_20250228.csv');
  });

  // 测试不同的时间周期
  it('应该正确处理不同的时间周期', () => {
    const filename = generateDataFilename(
      'okx',
      'BTCUSDT',
      '1d',
      new Date('2025-03-01'),
      new Date('2025-03-15')
    );
    expect(filename).toBe('okx_btcusdt_1d_20250301_to_20250315.csv');
  });

  // 测试缺失交易所参数
  it('当交易所参数缺失时应该返回空字符串', () => {
    const filename = generateDataFilename(
      '',
      'BTCUSDT',
      '1h',
      new Date('2025-01-01'),
      new Date('2025-01-31')
    );
    expect(filename).toBe('');
  });

  // 测试缺失交易对参数
  it('当交易对参数缺失时应该返回空字符串', () => {
    const filename = generateDataFilename(
      'binance',
      '',
      '1h',
      new Date('2025-01-01'),
      new Date('2025-01-31')
    );
    expect(filename).toBe('');
  });

  // 测试缺失时间周期参数
  it('当时间周期参数缺失时应该返回空字符串', () => {
    const filename = generateDataFilename(
      'binance',
      'BTCUSDT',
      '',
      new Date('2025-01-01'),
      new Date('2025-01-31')
    );
    expect(filename).toBe('');
  });

  // 测试缺失开始日期
  it('当开始日期缺失时应该返回空字符串', () => {
    const filename = generateDataFilename(
      'binance',
      'BTCUSDT',
      '1h',
      undefined,
      new Date('2025-01-31')
    );
    expect(filename).toBe('');
  });

  // 测试缺失结束日期
  it('当结束日期缺失时应该返回空字符串', () => {
    const filename = generateDataFilename(
      'binance',
      'BTCUSDT',
      '1h',
      new Date('2025-01-01'),
      undefined
    );
    expect(filename).toBe('');
  });

  // 测试特殊字符处理
  it('应该保持交易对中的特殊字符', () => {
    const filename = generateDataFilename(
      'bybit',
      'BTC-USDT',
      '1h',
      new Date('2025-01-01'),
      new Date('2025-01-31')
    );
    expect(filename).toBe('bybit_btc-usdt_1h_20250101_to_20250131.csv');
  });

  // 测试跨年日期范围
  it('应该正确处理跨年的日期范围', () => {
    const filename = generateDataFilename(
      'coinbase',
      'ETHUSDT',
      '1d',
      new Date('2024-12-15'),
      new Date('2025-01-15')
    );
    expect(filename).toBe('coinbase_ethusdt_1d_20241215_to_20250115.csv');
  });
});
