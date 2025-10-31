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

  return (
    <div
      className={cn(
        'fixed top-6 right-6 min-w-[300px] bg-white rounded-lg shadow-lg border-l-4 p-4 transition-transform duration-300 z-50',
        typeStyles[notification.type],
        isVisible ? 'translate-x-0' : 'translate-x-[400px]'
      )}
    >
      <div className="flex items-start justify-between">
        <p className="text-sm text-gray-900">{notification.message}</p>
        <button
          onClick={() => {
            setIsVisible(false);
            setTimeout(() => onClose(notification.id), 300);
          }}
          className="ml-4 text-gray-400 hover:text-gray-600"
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
 * 管理和显示多个通知
 */
export function NotificationContainer() {
  const [notifications, setNotifications] = useState<NotificationType[]>([]);

  // 移除通知
  const removeNotification = (id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  };

  return (
    <div className="fixed top-0 right-0 z-50">
      {notifications.map((notification, index) => (
        <div key={notification.id} style={{ top: `${index * 80}px` }}>
          <Notification
            notification={notification}
            onClose={removeNotification}
          />
        </div>
      ))}
    </div>
  );
}
