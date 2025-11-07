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
 * 数据下载 WebSocket Hook
 * 用于监听数据下载任务的实时进度更新
 */

import { useEffect, useRef, useState, useCallback } from 'react';
import type { DownloadProgressMessage, DownloadStatus } from '@/types/api';

/**
 * WebSocket 连接状态
 */
export type DataDownloadWsConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

/**
 * 数据下载进度信息
 */
export interface DownloadProgress {
  // 下载状态
  status: DownloadStatus;
  // 进度百分比 (0-100)
  progress: number;
  // 进度消息
  progressMessage: string;
  // 已下载数量
  downloadedCount: number;
  // 预估总数
  estimatedTotal: number | null;
}

/**
 * WebSocket 消息处理器
 */
export interface DataDownloadWsHandlers {
  // 连接成功
  onConnected?: () => void;
  // 进度更新
  onProgress?: (progress: DownloadProgress) => void;
  // 下载完成
  onComplete?: (downloadedCount: number) => void;
  // 下载错误
  onError?: (error: string) => void;
}

/**
 * Hook 选项
 */
export interface UseDataDownloadWebSocketOptions extends DataDownloadWsHandlers {
  // 是否自动连接
  autoConnect?: boolean;
  // 重连间隔（毫秒）
  reconnectInterval?: number;
  // 最大重连次数
  maxReconnectAttempts?: number;
  // 任务是否已完成（用于防止重连）
  isTaskCompleted?: boolean;
}

/**
 * Hook 返回值
 */
export interface UseDataDownloadWebSocketReturn {
  // 连接状态
  connectionStatus: DataDownloadWsConnectionStatus;
  // 下载进度信息
  progress: DownloadProgress | null;
  // 连接
  connect: () => void;
  // 断开连接
  disconnect: () => void;
  // 是否已连接
  isConnected: boolean;
}

/**
 * 获取 WebSocket URL
 * 
 * @param taskId 任务ID
 * @returns WebSocket URL
 */
function getWebSocketUrl(taskId: string): string {
  // 从环境变量获取 API 基础 URL
  const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:8080/api';
  
  // 移除 /api 后缀，获取基础 URL
  const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
  
  // 将 HTTP(S) 协议转换为 WS(S) 协议
  const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
  
  // 构建 WebSocket URL（注意：后端路由是 /ws/data/{id}，不是 /api/ws/data/{id}）
  return `${wsBaseUrl}/ws/data/${taskId}`;
}

/**
 * 数据下载 WebSocket Hook
 * 
 * @param taskId 下载任务 ID
 * @param options Hook 选项
 * @returns WebSocket 连接状态和控制方法
 * 
 * @example
 * ```tsx
 * const { progress, connectionStatus, connect, disconnect } = useDataDownloadWebSocket(
 *   taskId,
 *   {
 *     onProgress: (progress) => console.log('进度:', progress),
 *     onComplete: (count) => console.log('完成:', count),
 *     onError: (error) => console.error('错误:', error),
 *   }
 * );
 * ```
 */
export function useDataDownloadWebSocket(
  taskId: string | null,
  options: UseDataDownloadWebSocketOptions = {}
): UseDataDownloadWebSocketReturn {
  const {
    autoConnect = true,
    reconnectInterval = 3000,
    maxReconnectAttempts = 5,
    isTaskCompleted = false,
    onConnected,
    onProgress,
    onComplete,
    onError,
  } = options;

  // WebSocket 实例引用
  const wsRef = useRef<WebSocket | null>(null);
  // 重连计数器
  const reconnectCountRef = useRef(0);
  // 重连定时器
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  // 是否手动断开连接
  const manualDisconnectRef = useRef(false);
  // 使用 ref 保存回调函数，避免闭包陷阱
  const onConnectedRef = useRef(onConnected);
  const onProgressRef = useRef(onProgress);
  const onCompleteRef = useRef(onComplete);
  const onErrorRef = useRef(onError);

  // 连接状态
  const [connectionStatus, setConnectionStatus] = useState<DataDownloadWsConnectionStatus>('disconnected');
  // 下载进度
  const [progress, setProgress] = useState<DownloadProgress | null>(null);

  // 更新回调函数的 ref
  useEffect(() => {
    onConnectedRef.current = onConnected;
    onProgressRef.current = onProgress;
    onCompleteRef.current = onComplete;
    onErrorRef.current = onError;
  }, [onConnected, onProgress, onComplete, onError]);

  /**
   * 清理重连定时器
   */
  const clearReconnectTimeout = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }
  }, []);

  /**
   * 断开 WebSocket 连接
   */
  const disconnect = useCallback(() => {
    manualDisconnectRef.current = true;
    clearReconnectTimeout();

    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    setConnectionStatus('disconnected');
  }, [clearReconnectTimeout]);

  /**
   * 连接 WebSocket
   */
  const connect = useCallback(() => {
    // 如果没有任务 ID 或任务已完成，不连接
    if (!taskId || isTaskCompleted) {
      return;
    }

    // 如果已经连接或正在连接，不重复连接
    if (
      wsRef.current &&
      (wsRef.current.readyState === WebSocket.CONNECTING ||
        wsRef.current.readyState === WebSocket.OPEN)
    ) {
      return;
    }

    // 清理旧的 WebSocket 连接
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    manualDisconnectRef.current = false;
    setConnectionStatus('connecting');

    try {
      const url = getWebSocketUrl(taskId);
      const ws = new WebSocket(url);
      wsRef.current = ws;

      // WebSocket 事件处理
      ws.onopen = () => {
        console.log('数据下载 WebSocket 连接成功');
        setConnectionStatus('connected');
        clearReconnectTimeout();
        reconnectCountRef.current = 0;  
      };

      ws.onmessage = (event: MessageEvent) => {
        try {
          const message = JSON.parse(event.data) as DownloadProgressMessage;

          // 根据消息类型分发处理
          switch (message.type) {
            case 'connected':
              // 连接成功
              reconnectCountRef.current = 0;
              onConnectedRef.current?.();
              break;

            case 'progress':
              // 进度更新
              if (message.status && message.progress !== undefined) {
                const progressData: DownloadProgress = {
                  status: message.status,
                  progress: message.progress,
                  progressMessage: message.progress_message || '',
                  downloadedCount: message.downloaded_count || 0,
                  estimatedTotal: message.estimated_total || null,
                };
                setProgress(progressData);
                onProgressRef.current?.(progressData);
              }
              break;

            case 'complete':
              // 下载完成
              if (message.downloaded_count !== undefined) {
                onCompleteRef.current?.(message.downloaded_count);
              }
              // 标记为手动断开，避免触发重连
              manualDisconnectRef.current = true;
              // 完成后自动断开连接
              setTimeout(() => {
                if (wsRef.current) {
                  wsRef.current.close();
                }
              }, 100);
              break;

            case 'error':
              // 下载错误
              const errorMsg = message.error || message.message || '未知错误';
              onErrorRef.current?.(errorMsg);
              setConnectionStatus('error');
              // 标记为手动断开，避免在错误后继续重连
              manualDisconnectRef.current = true;
              break;
          }
        } catch (error) {
          console.error('解析 WebSocket 消息失败:', error);
          onErrorRef.current?.('消息解析失败');
        }
      };

      ws.onerror = (error) => {
        console.error('WebSocket 错误:', error);
        setConnectionStatus('error');
      };

      ws.onclose = (event) => {
        console.log('WebSocket 连接关闭:', event.code, event.reason);
        setConnectionStatus('disconnected');
        wsRef.current = null;

        // 如果不是手动断开且不是正常关闭，尝试重连
        if (!manualDisconnectRef.current && event.code !== 1000 && !isTaskCompleted) {
          if (reconnectCountRef.current < maxReconnectAttempts) {
            reconnectCountRef.current += 1;
            console.log(`尝试重连 (${reconnectCountRef.current}/${maxReconnectAttempts})...`);

            reconnectTimeoutRef.current = setTimeout(() => {
              // 使用 taskId 检查而不是直接调用 connect
              // 这避免了在 callback 中的循环引用问题
              if (!manualDisconnectRef.current && taskId && !isTaskCompleted) {
                // 重新触发连接逻辑
                const retryUrl = getWebSocketUrl(taskId);
                const retryWs = new WebSocket(retryUrl);
                wsRef.current = retryWs;

                // 复用相同的事件处理器设置
                retryWs.onopen = ws.onopen;
                retryWs.onmessage = ws.onmessage;
                retryWs.onerror = ws.onerror;
                retryWs.onclose = ws.onclose;
              }
            }, reconnectInterval);
          } else {
            console.error('WebSocket 重连次数已达上限');
            setConnectionStatus('error');
            onErrorRef.current?.('连接失败: 已达最大重连次数');
          }
        }
      };
    } catch (error) {
      console.error('创建 WebSocket 连接失败:', error);
      setConnectionStatus('error');
      onErrorRef.current?.('无法创建 WebSocket 连接');
    }
  }, [taskId, isTaskCompleted, clearReconnectTimeout, maxReconnectAttempts, reconnectInterval]);

  /**
   * 自动连接和清理
   */
  useEffect(() => {
    // 如果启用自动连接且有任务ID且任务未完成，则连接
    if (autoConnect && taskId && !isTaskCompleted) {
      connect();
    }

    // 组件卸载时断开连接
    return () => {
      disconnect();
    };
    // 只依赖 taskId、autoConnect 和 isTaskCompleted
    // connect 和 disconnect 不应该作为依赖项，因为它们的变化会导致重新连接
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [taskId, autoConnect, isTaskCompleted]);

  return {
    connectionStatus,
    progress,
    connect,
    disconnect,
    isConnected: connectionStatus === 'connected',
  };
}
