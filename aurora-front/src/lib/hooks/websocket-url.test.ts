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
 * WebSocket URL 生成测试验证
 * 
 * 此文件用于验证 WebSocket URL 的生成逻辑是否正确
 */

describe('WebSocket URL 生成验证', () => {
  it('应该从 http://localhost:8080/api 生成 ws://localhost:8080/ws/data/{id}', () => {
    const apiBaseUrl = 'http://localhost:8080/api';
    const taskId = 'test-123';
    
    // 移除 /api 后缀
    const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
    expect(baseUrl).toBe('http://localhost:8080');
    
    // 转换为 ws 协议
    const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
    expect(wsBaseUrl).toBe('ws://localhost:8080');
    
    // 构建完整 URL
    const wsUrl = `${wsBaseUrl}/ws/data/${taskId}`;
    expect(wsUrl).toBe('ws://localhost:8080/ws/data/test-123');
  });

  it('应该从 https://example.com/api 生成 wss://example.com/ws/data/{id}', () => {
    const apiBaseUrl = 'https://example.com/api';
    const taskId = 'test-456';
    
    const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
    const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
    const wsUrl = `${wsBaseUrl}/ws/data/${taskId}`;
    
    expect(wsUrl).toBe('wss://example.com/ws/data/test-456');
  });

  it('应该处理带结尾斜杠的 API URL', () => {
    const apiBaseUrl = 'http://localhost:8080/api/';
    const taskId = 'test-789';
    
    const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
    const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
    const wsUrl = `${wsBaseUrl}/ws/data/${taskId}`;
    
    expect(wsUrl).toBe('ws://localhost:8080/ws/data/test-789');
  });

  it('应该处理不带 /api 的基础 URL', () => {
    const apiBaseUrl = 'http://localhost:8080';
    const taskId = 'test-000';
    
    const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
    const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
    const wsUrl = `${wsBaseUrl}/ws/data/${taskId}`;
    
    expect(wsUrl).toBe('ws://localhost:8080/ws/data/test-000');
  });
});
