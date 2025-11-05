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
 * TOML工具函数测试
 */

import {
  parseTOML,
  stringifyTOML,
  validateTOML,
  formatTOML,
} from './toml';
import { createDefaultAuroraConfig } from '@/types/config-schema';

describe('TOML Utils', () => {
  describe('parseTOML', () => {
    it('应该成功解析有效的TOML文本', async () => {
      const tomlText = `
[data_source]
provider = "binance"
timeout = 30
max_retries = 3

[[strategies]]
name = "MA交叉"
strategy_type = "ma-crossover"
enabled = true

[strategies.parameters]
short = 10
long = 30

[portfolio]
initial_cash = 10000.0
commission = 0.001
slippage = 0.0

[logging]
level = "info"
format = "pretty"
`;

      const config = await parseTOML(tomlText);
      
      expect(config.data_source.provider).toBe('binance');
      expect(config.strategies).toHaveLength(1);
      expect(config.strategies[0].name).toBe('MA交叉');
      expect(config.portfolio.initial_cash).toBe(10000.0);
    });

    it('应该拒绝无效的TOML文本', async () => {
      const invalidToml = `
[invalid
this is not valid toml
`;

      await expect(parseTOML(invalidToml)).rejects.toThrow();
    });
  });

  describe('stringifyTOML', () => {
    it('应该成功将配置对象转换为TOML文本', async () => {
      const config = createDefaultAuroraConfig();
      
      const tomlText = await stringifyTOML(config);
      
      expect(tomlText).toContain('[data_source]');
      expect(tomlText).toContain('[[strategies]]');
      expect(tomlText).toContain('[portfolio]');
      expect(tomlText).toContain('[logging]');
    });

    it('应该拒绝无效的配置对象', async () => {
      const invalidConfig = {
        data_source: {
          provider: 'invalid',
          timeout: -1,
        },
      };

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      await expect(stringifyTOML(invalidConfig as any)).rejects.toThrow();
    });
  });

  describe('validateTOML', () => {
    it('应该验证通过有效的TOML文本', async () => {
      const config = createDefaultAuroraConfig();
      const tomlText = await stringifyTOML(config);
      
      const result = await validateTOML(tomlText);
      
      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
      expect(result.config).toBeDefined();
    });

    it('应该验证失败无效的TOML文本', async () => {
      const invalidToml = `
[data_source]
provider = "invalid_provider"
`;

      const result = await validateTOML(invalidToml);
      
      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });

  describe('formatTOML', () => {
    it('应该格式化有效的TOML文本', async () => {
      const messyToml = `
[data_source]
provider="binance"
timeout=30
max_retries=3

[[strategies]]
name="MA交叉"
strategy_type="ma-crossover"
enabled=true
[strategies.parameters]
short=10
long=30

[portfolio]
initial_cash=10000.0
commission=0.001
slippage=0.0

[logging]
level="info"
format="pretty"
`;

      const formatted = await formatTOML(messyToml);
      
      expect(formatted).toContain('[data_source]');
      expect(formatted).toBeTruthy();
    });

    it('应该返回原文本如果解析失败', async () => {
      const invalidToml = 'invalid toml content';
      
      const result = await formatTOML(invalidToml);
      
      expect(result).toBe(invalidToml);
    });
  });
});
