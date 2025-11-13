// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
 * WebSocket 单一连接测试
 * 
 * 验证修复后只创建一个WebSocket连接
 */

import { describe, it, expect, jest, beforeEach, afterEach } from '@jest/globals';
import { renderHook, waitFor } from '@testing-library/react';
import { useBacktestWebSocket } from '@/lib/hooks/useBacktestWebSocket';

// Mock WebSocket
class MockWebSocket {
  static instances: MockWebSocket[] = [];
  
  readyState: number = 0; // CONNECTING
  url: string;
  onopen: ((ev: Event) => void) | null = null;
  onmessage: ((ev: MessageEvent) => void) | null = null;
  onerror: ((ev: Event) => void) | null = null;
  onclose: ((ev: CloseEvent) => void) | null = null;

  constructor(url: string) {
    this.url = url;
    MockWebSocket.instances.push(this);
    
    // 模拟异步连接
    setTimeout(() => {
      this.readyState = 1; // OPEN
      if (this.onopen) {
        this.onopen(new Event('open'));
      }
    }, 10);
  }

  send() {
    // Mock send
  }

  close(code?: number, reason?: string) {
    this.readyState = 2; // CLOSING
    setTimeout(() => {
      this.readyState = 3; // CLOSED
      if (this.onclose) {
        this.onclose(new CloseEvent('close', { code, reason }));
      }
    }, 10);
  }

  static reset() {
    MockWebSocket.instances = [];
  }

  static getInstanceCount() {
    return MockWebSocket.instances.length;
  }

  static getOpenInstances() {
    return MockWebSocket.instances.filter(
      (ws) => ws.readyState === 0 || ws.readyState === 1
    );
  }
}

describe('WebSocket 单一连接测试', () => {
  beforeEach(() => {
    // 替换全局WebSocket为Mock
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (global as any).WebSocket = MockWebSocket;
    MockWebSocket.reset();
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  it('应该只创建一个WebSocket连接', async () => {
    const { result } = renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: false,
      })
    );

    // 等待连接建立
    await waitFor(() => {
      expect(result.current.isConnected).toBe(true);
    }, { timeout: 1000 });

    // 验证只创建了一个WebSocket实例
    expect(MockWebSocket.getInstanceCount()).toBe(1);
  });

  it('应该在taskId改变时先关闭旧连接再创建新连接', async () => {
    const { rerender } = renderHook(
      ({ taskId }: { taskId: string | null }) =>
        useBacktestWebSocket(taskId, {
          autoConnect: true,
          isTaskCompleted: false,
        }),
      {
        initialProps: { taskId: 'task-1' },
      }
    );

    // 等待第一个连接建立
    await waitFor(() => {
      expect(MockWebSocket.getInstanceCount()).toBe(1);
      expect(MockWebSocket.instances[0].readyState).toBe(1); // OPEN
    }, { timeout: 1000 });

    const firstInstance = MockWebSocket.instances[0];

    // 改变taskId
    rerender({ taskId: 'task-2' });

    // 等待旧连接关闭和新连接建立
    await waitFor(() => {
      // 第一个连接应该被关闭
      expect(firstInstance.readyState).toBe(3); // CLOSED
      // 应该有第二个连接
      expect(MockWebSocket.getInstanceCount()).toBe(2);
      // 第二个连接应该是打开的
      expect(MockWebSocket.instances[1].readyState).toBe(1); // OPEN
    }, { timeout: 1000 });
  });

  it('应该在任务完成时不创建新连接', () => {
    renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: true, // 任务已完成
      })
    );

    // 验证没有创建WebSocket连接
    expect(MockWebSocket.getInstanceCount()).toBe(0);
  });

  it('应该在卸载时关闭连接', async () => {
    const { unmount } = renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: false,
      })
    );

    // 等待连接建立
    await waitFor(() => {
      expect(MockWebSocket.getInstanceCount()).toBe(1);
    });

    const instance = MockWebSocket.instances[0];

    // 卸载组件
    unmount();

    // 验证连接已关闭
    await waitFor(() => {
      expect(instance.readyState).toBe(3); // CLOSED
    });
  });

  it('应该在已有连接时不重复创建', async () => {
    const { result } = renderHook(() =>
      useBacktestWebSocket('task-123', {
        autoConnect: true,
        isTaskCompleted: false,
      })
    );

    // 等待连接建立
    await waitFor(() => {
      expect(result.current.isConnected).toBe(true);
    }, { timeout: 1000 });

    // 记录当前打开的连接数
    const openConnectionsBefore = MockWebSocket.getOpenInstances().length;

    // 等待一小段时间确保连接稳定
    await new Promise(resolve => setTimeout(resolve, 50));

    // 尝试在已连接时再次连接（应该被跳过）
    result.current.connect();
    
    // 等待一小段时间
    await new Promise(resolve => setTimeout(resolve, 50));

    // 验证：打开的连接数应该保持不变
    const openConnectionsAfter = MockWebSocket.getOpenInstances().length;
    expect(openConnectionsAfter).toBe(openConnectionsBefore);
    
    // 验证仍然是连接状态
    expect(result.current.isConnected).toBe(true);
  });
});
