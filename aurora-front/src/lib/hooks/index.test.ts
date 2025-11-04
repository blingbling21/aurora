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

import * as hooksIndex from './index';

describe('Hooks Index Module', () => {
  // 测试模块导出
  it('应该导出 useBacktestWebSocket', () => {
    expect(hooksIndex).toHaveProperty('useBacktestWebSocket');
  });

  // 测试 useBacktestWebSocket 是一个函数
  it('useBacktestWebSocket 应该是一个函数', () => {
    expect(typeof hooksIndex.useBacktestWebSocket).toBe('function');
  });

  // 测试模块导出的完整性
  it('应该导出所有必要的 hooks', () => {
    const exports = Object.keys(hooksIndex);
    
    // 至少应该包含 useBacktestWebSocket
    expect(exports).toContain('useBacktestWebSocket');
  });

  // 测试没有意外的导出
  it('不应该有未定义的导出', () => {
    const exports = Object.values(hooksIndex);
    
    // 所有导出都不应该是 undefined
    exports.forEach(exportedItem => {
      expect(exportedItem).toBeDefined();
    });
  });

  // 测试重新导出的完整性
  it('重新导出应该与原始模块一致', async () => {
    // 动态导入原始模块
    const originalModule = await import('./useBacktestWebSocket');
    
    // 验证重新导出的内容
    expect(hooksIndex.useBacktestWebSocket).toBe(
      originalModule.useBacktestWebSocket
    );
  });
});
