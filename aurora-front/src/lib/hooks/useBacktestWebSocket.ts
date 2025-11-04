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
 * WebSocket Hook
 * 用于回测任务实时进度更新
 */

import { useEffect, useRef, useState, useCallback } from 'react';
import { backtestApi } from '@/lib/api';
import type { WsMessage, TaskStatus } from '@/types/api';

/**
 * WebSocket 连接状态
 */
export type WsConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

/**
 * WebSocket 消息处理器
 */
export interface WsMessageHandlers {
  // 连接成功
  onConnected?: () => void;
  // 状态更新
  onStatusUpdate?: (progress: number, status: TaskStatus) => void;
  // 任务完成
  onComplete?: (data?: unknown) => void;
  // 错误
  onError?: (error: string) => void;
  // 原始消息
  onMessage?: (message: WsMessage) => void;
}

/**
 * WebSocket Hook 选项
 */
export interface UseWebSocketOptions extends WsMessageHandlers {
  // 是否自动连接
  autoConnect?: boolean;
  // 重连间隔（毫秒）
  reconnectInterval?: number;
  // 最大重连次数
  maxReconnectAttempts?: number;
  // 心跳间隔（毫秒）
  heartbeatInterval?: number;
}

/**
 * WebSocket Hook 返回值
 */
export interface UseWebSocketReturn {
  // 连接状态
  status: WsConnectionStatus;
  // 最后一条消息
  lastMessage: WsMessage | null;
  // 连接
  connect: () => void;
  // 断开连接
  disconnect: () => void;
  // 发送消息
  send: (message: unknown) => void;
  // 是否已连接
  isConnected: boolean;
}

/**
 * 回测 WebSocket Hook
 * 
 * @param taskId 回测任务 ID
 * @param options Hook 选项
 * @returns WebSocket 连接状态和控制方法
 */
export function useBacktestWebSocket(
  taskId: string | null,
  options: UseWebSocketOptions = {}
): UseWebSocketReturn {
  const {
    autoConnect = true,
    reconnectInterval = 3000,
    maxReconnectAttempts = 5,
    heartbeatInterval = 30000,
    onConnected,
    onStatusUpdate,
    onComplete,
    onError,
    onMessage,
  } = options;

  // WebSocket 实例引用
  const wsRef = useRef<WebSocket | null>(null);
  // 心跳定时器引用
  const heartbeatRef = useRef<NodeJS.Timeout | null>(null);
  // 重连计数器
  const reconnectCountRef = useRef(0);
  // 重连定时器
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // 连接状态
  const [status, setStatus] = useState<WsConnectionStatus>('disconnected');
  // 最后一条消息
  const [lastMessage, setLastMessage] = useState<WsMessage | null>(null);

  /**
   * 清理心跳定时器
   */
  const clearHeartbeat = useCallback(() => {
    if (heartbeatRef.current) {
      clearInterval(heartbeatRef.current);
      heartbeatRef.current = null;
    }
  }, []);

  /**
   * 启动心跳
   */
  const startHeartbeat = useCallback(() => {
    clearHeartbeat();
    if (heartbeatInterval > 0) {
      heartbeatRef.current = setInterval(() => {
        if (wsRef.current?.readyState === WebSocket.OPEN) {
          wsRef.current.send(JSON.stringify({ type: 'ping' }));
        }
      }, heartbeatInterval);
    }
  }, [heartbeatInterval, clearHeartbeat]);

  /**
   * 处理 WebSocket 消息
   */
  const handleMessage = useCallback(
    (event: MessageEvent) => {
      try {
        const message = JSON.parse(event.data) as WsMessage;
        setLastMessage(message);

        // 调用通用消息处理器
        onMessage?.(message);

        // 根据消息类型分发处理
        switch (message.type) {
          case 'connected':
            onConnected?.();
            break;
          case 'status_update':
            if (message.progress !== undefined && message.status) {
              onStatusUpdate?.(message.progress, message.status);
            }
            break;
          case 'final':
            onComplete?.(message.data);
            break;
          case 'error':
            onError?.(message.error || message.message || '未知错误');
            break;
        }
      } catch (error) {
        console.error('解析 WebSocket 消息失败:', error);
        onError?.('消息解析失败');
      }
    },
    [onMessage, onConnected, onStatusUpdate, onComplete, onError]
  );

  /**
   * 连接 WebSocket
   */
  const connect = useCallback(() => {
    // 如果没有任务 ID，不连接
    if (!taskId) {
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

    setStatus('connecting');

    // 内部递归重连函数
    const attemptConnect = () => {
      try {
        const url = backtestApi.getWebSocketUrl(taskId);
        const ws = new WebSocket(url);
        wsRef.current = ws;

        // 连接打开
        ws.onopen = () => {
          console.log('WebSocket 已连接');
          setStatus('connected');
          reconnectCountRef.current = 0;
          startHeartbeat();
        };

        // 接收消息
        ws.onmessage = handleMessage;

        // 连接错误
        ws.onerror = (event) => {
          console.error('WebSocket 错误:', event);
          setStatus('error');
          onError?.('连接错误');
        };

        // 连接关闭
        ws.onclose = (event) => {
          console.log('WebSocket 已关闭:', event.code, event.reason);
          setStatus('disconnected');
          clearHeartbeat();
          wsRef.current = null;

          // 尝试重连
          if (
            !event.wasClean &&
            reconnectCountRef.current < maxReconnectAttempts
          ) {
            reconnectCountRef.current++;
            console.log(
              `尝试重连 (${reconnectCountRef.current}/${maxReconnectAttempts})...`
            );
            reconnectTimeoutRef.current = setTimeout(() => {
              attemptConnect();
            }, reconnectInterval);
          }
        };
      } catch (error) {
        console.error('创建 WebSocket 失败:', error);
        setStatus('error');
        onError?.('创建连接失败');
      }
    };

    attemptConnect();
  }, [
    taskId,
    handleMessage,
    onError,
    startHeartbeat,
    clearHeartbeat,
    reconnectInterval,
    maxReconnectAttempts,
  ]);

  /**
   * 断开 WebSocket
   */
  const disconnect = useCallback(() => {
    // 清理重连定时器
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }

    // 清理心跳
    clearHeartbeat();

    // 关闭连接
    if (wsRef.current) {
      wsRef.current.close(1000, 'Client disconnect');
      wsRef.current = null;
    }

    setStatus('disconnected');
    reconnectCountRef.current = 0;
  }, [clearHeartbeat]);

  /**
   * 发送消息
   */
  const send = useCallback((message: unknown) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    } else {
      console.warn('WebSocket 未连接，无法发送消息');
    }
  }, []);

  // 自动连接
  useEffect(() => {
    if (autoConnect && taskId) {
      // 使用 setTimeout 避免在 effect 中同步调用 setState
      const timer = setTimeout(() => {
        connect();
      }, 0);

      // 清理函数
      return () => {
        clearTimeout(timer);
        disconnect();
      };
    } else {
      // 清理函数
      return () => {
        disconnect();
      };
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [taskId, autoConnect]);

  return {
    status,
    lastMessage,
    connect,
    disconnect,
    send,
    isConnected: status === 'connected',
  };
}
