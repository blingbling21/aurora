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

'use client';

import { useEffect, useState } from 'react';
import { Notification as NotificationType } from '@/types';
import { cn } from '@/lib/utils';

interface NotificationProps {
  notification: NotificationType;
  onClose: (id: string) => void;
}

/**
 * 通知组件
 * 
 * 显示成功、错误、信息或警告消息的通知
 * 
 * @param {NotificationType} notification - 通知数据
 * @param {Function} onClose - 关闭通知的回调函数
 */
export function Notification({ notification, onClose }: NotificationProps) {
  const [isVisible, setIsVisible] = useState(false);

  // 通知类型样式映射
  const typeStyles = {
    success: 'border-l-green-500',
    error: 'border-l-red-500',
    info: 'border-l-blue-500',
    warning: 'border-l-yellow-500',
  };

  useEffect(() => {
    // 显示通知
    setTimeout(() => setIsVisible(true), 10);

    // 自动关闭
    const duration = notification.duration || 3000;
    const timer = setTimeout(() => {
      setIsVisible(false);
      setTimeout(() => onClose(notification.id), 300);
    }, duration);

    return () => clearTimeout(timer);
  }, [notification, onClose]);

  // 图标映射
  const icons = {
    success: '✓',
    error: '✕',
    info: 'ℹ',
    warning: '⚠',
  };

  // 图标颜色映射
  const iconColors = {
    success: 'text-green-600',
    error: 'text-red-600',
    info: 'text-blue-600',
    warning: 'text-yellow-600',
  };

  return (
    <div
      className={cn(
        'min-w-[320px] max-w-md bg-white rounded-lg shadow-xl border-l-4 p-4 transition-all duration-300',
        typeStyles[notification.type],
        isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-full'
      )}
    >
      <div className="flex items-start gap-3">
        {/* 图标 */}
        <div
          className={cn(
            'shrink-0 w-6 h-6 rounded-full flex items-center justify-center font-bold',
            iconColors[notification.type]
          )}
        >
          {icons[notification.type]}
        </div>

        {/* 消息内容 */}
        <p className="flex-1 text-sm text-gray-900 wrap-break-word">
          {notification.message}
        </p>

        {/* 关闭按钮 */}
        <button
          onClick={() => {
            setIsVisible(false);
            setTimeout(() => onClose(notification.id), 300);
          }}
          className="shrink-0 text-gray-400 hover:text-gray-600 transition-colors"
          aria-label="关闭通知"
        >
          ✕
        </button>
      </div>
    </div>
  );
}

/**
 * 通知容器组件
 * 
 * 管理和显示多个通知，连接到 zustand store
 */
export function NotificationContainer() {
  // 从 zustand store 获取通知列表和移除方法
  // 使用动态导入避免在测试中出现问题
  const [notifications, setNotifications] = useState<NotificationType[]>([]);
  const [removeNotification, setRemoveNotification] = useState<((id: string) => void) | null>(null);

  useEffect(() => {
    // 动态导入 store
    import('@/lib/store').then(({ useNotificationStore }) => {
      const store = useNotificationStore.getState();
      setNotifications(store.notifications);
      setRemoveNotification(() => store.removeNotification);

      // 订阅 store 变化
      const unsubscribe = useNotificationStore.subscribe((state) => {
        setNotifications(state.notifications);
        setRemoveNotification(() => state.removeNotification);
      });

      return () => unsubscribe();
    });
  }, []);

  return (
    <div className="fixed top-0 right-0 z-50 p-4">
      <div className="flex flex-col gap-3">
        {notifications.map((notification) => (
          <Notification
            key={notification.id}
            notification={notification}
            onClose={removeNotification || (() => {})}
          />
        ))}
      </div>
    </div>
  );
}
