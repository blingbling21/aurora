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

import { renderHook, act, waitFor } from '@testing-library/react';
import { useDataDownloadWebSocket } from './useDataDownloadWebSocket';

// 存储 WebSocket 实例以便测试中访问
let mockWebSocketInstance: MockWebSocket | null = null;

// Mock WebSocket
class MockWebSocket {
  public onopen: (() => void) | null = null;
  public onmessage: ((event: MessageEvent) => void) | null = null;
  public onerror: ((event: Event) => void) | null = null;
  public onclose: ((event: CloseEvent) => void) | null = null;
  public readyState: number = 0; // CONNECTING
  public static CONNECTING = 0;
  public static OPEN = 1;
  public static CLOSING = 2;
  public static CLOSED = 3;

  constructor(public url: string) {
    // 将当前实例保存到全局变量
    // eslint-disable-next-line @typescript-eslint/no-this-alias
    mockWebSocketInstance = this;
    // 模拟连接成功
    setTimeout(() => {
      this.readyState = 1; // OPEN
      this.onopen?.();
    }, 100);
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  send(_data: string): void {
    // Mock send - 空实现用于测试
  }

  close(): void {
    this.readyState = 3; // CLOSED
    this.onclose?.({ code: 1000, reason: 'Normal closure' } as CloseEvent);
  }
}

// 替换全局 WebSocket
(global as unknown as Record<string, unknown>).WebSocket = MockWebSocket;

describe('useDataDownloadWebSocket', () => {
  const mockTaskId = 'test-task-id-123';

  beforeEach(() => {
    jest.clearAllMocks();
    mockWebSocketInstance = null;
    // 设置环境变量
    process.env.NEXT_PUBLIC_API_BASE_URL = 'http://localhost:8080/api';
  });

  it('应该正确初始化', () => {
    const { result } = renderHook(() =>
      useDataDownloadWebSocket(null, { autoConnect: false })
    );

    expect(result.current.connectionStatus).toBe('disconnected');
    expect(result.current.progress).toBeNull();
    expect(result.current.isConnected).toBe(false);
  });

  it('应该生成正确的 WebSocket URL', async () => {
    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, { autoConnect: true })
    );

    // 等待连接开始
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connecting');
      },
      { timeout: 200 }
    );

    // 检查 WebSocket 实例的 URL
    expect(mockWebSocketInstance?.url).toBe('ws://localhost:8080/ws/data/test-task-id-123');
  });

  it('应该在提供 taskId 且 autoConnect=true 时自动连接', async () => {
    const onConnected = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        onConnected,
      })
    );

    // 初始状态应该是 connecting
    expect(result.current.connectionStatus).toBe('connecting');

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    expect(result.current.isConnected).toBe(true);
  });

  it('应该能够手动连接和断开', async () => {
    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, { autoConnect: false })
    );

    // 初始状态
    expect(result.current.connectionStatus).toBe('disconnected');

    // 手动连接
    act(() => {
      result.current.connect();
    });

    expect(result.current.connectionStatus).toBe('connecting');

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    // 手动断开
    act(() => {
      result.current.disconnect();
    });

    expect(result.current.connectionStatus).toBe('disconnected');
  });

  it('应该处理进度消息', async () => {
    const onProgress = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        onProgress,
      })
    );

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    // 模拟接收进度消息
    const progressMessage = {
      type: 'progress',
      status: 'Downloading',
      progress: 50,
      progress_message: '已下载 500 / 1000 条数据',
      downloaded_count: 500,
      estimated_total: 1000,
    };

    act(() => {
      // 使用存储的 WebSocket 实例触发消息
      if (mockWebSocketInstance?.onmessage) {
        mockWebSocketInstance.onmessage({
          data: JSON.stringify(progressMessage),
        } as MessageEvent);
      }
    });

    // 验证进度已更新
    expect(result.current.progress).toEqual({
      status: 'Downloading',
      progress: 50,
      progressMessage: '已下载 500 / 1000 条数据',
      downloadedCount: 500,
      estimatedTotal: 1000,
    });

    expect(onProgress).toHaveBeenCalledWith(
      expect.objectContaining({
        status: 'Downloading',
        progress: 50,
      })
    );
  });

  it('应该处理完成消息', async () => {
    const onComplete = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        onComplete,
      })
    );

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    // 模拟接收完成消息
    const completeMessage = {
      type: 'complete',
      downloaded_count: 1000,
      message: '下载完成',
    };

    act(() => {
      if (mockWebSocketInstance?.onmessage) {
        mockWebSocketInstance.onmessage({
          data: JSON.stringify(completeMessage),
        } as MessageEvent);
      }
    });

    // 验证完成回调被调用
    expect(onComplete).toHaveBeenCalledWith(1000);
  });

  it('应该处理错误消息', async () => {
    const onError = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        onError,
      })
    );

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    // 模拟接收错误消息
    const errorMessage = {
      type: 'error',
      error: '下载失败: 网络错误',
    };

    act(() => {
      if (mockWebSocketInstance?.onmessage) {
        mockWebSocketInstance.onmessage({
          data: JSON.stringify(errorMessage),
        } as MessageEvent);
      }
    });

    // 验证错误回调被调用
    expect(onError).toHaveBeenCalledWith('下载失败: 网络错误');
    expect(result.current.connectionStatus).toBe('error');
  });

  it('应该在任务已完成时不建立连接', async () => {
    const onConnected = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        isTaskCompleted: true,
        onConnected,
      })
    );

    // 等待一段时间
    await new Promise((resolve) => setTimeout(resolve, 200));

    // 验证未建立连接
    expect(result.current.connectionStatus).toBe('disconnected');
    expect(onConnected).not.toHaveBeenCalled();
    expect(mockWebSocketInstance).toBeNull();
  });

  it('应该在任务完成后设置手动断开标志', async () => {
    const onComplete = jest.fn();

    const { result } = renderHook(() =>
      useDataDownloadWebSocket(mockTaskId, {
        autoConnect: true,
        onComplete,
      })
    );

    // 等待连接成功
    await waitFor(
      () => {
        expect(result.current.connectionStatus).toBe('connected');
      },
      { timeout: 500 }
    );

    // 模拟接收完成消息
    const completeMessage = {
      type: 'complete',
      downloaded_count: 1000,
      message: '下载完成',
    };

    act(() => {
      if (mockWebSocketInstance?.onmessage) {
        mockWebSocketInstance.onmessage({
          data: JSON.stringify(completeMessage),
        } as MessageEvent);
      }
    });

    // 验证完成回调被调用
    expect(onComplete).toHaveBeenCalledWith(1000);

    // 等待自动关闭
    await waitFor(
      () => {
        expect(mockWebSocketInstance?.readyState).toBe(3); // CLOSED
      },
      { timeout: 300 }
    );
  });
});
