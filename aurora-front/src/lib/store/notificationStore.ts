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

import { create } from 'zustand';
import { Notification } from '@/types/schemas';

/**
 * 通知状态接口
 */
interface NotificationState {
  // 通知列表
  notifications: Notification[];
  // 最大通知数量
  maxNotifications: number;

  // Actions
  // 添加通知
  addNotification: (notification: Omit<Notification, 'id'>) => void;
  // 删除通知
  removeNotification: (id: string) => void;
  // 清空所有通知
  clearNotifications: () => void;
  // 显示成功通知
  showSuccess: (message: string, duration?: number) => void;
  // 显示错误通知
  showError: (message: string, duration?: number) => void;
  // 显示信息通知
  showInfo: (message: string, duration?: number) => void;
  // 显示警告通知
  showWarning: (message: string, duration?: number) => void;
}

/**
 * 生成唯一ID
 */
function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * 通知状态管理Store
 * 用于管理全局通知消息
 */
export const useNotificationStore = create<NotificationState>((set) => ({
  // 初始状态
  notifications: [],
  maxNotifications: 5,

  // 添加通知
  addNotification: (notification) =>
    set((state) => {
      const newNotification: Notification = {
        ...notification,
        id: generateId(),
        duration: notification.duration || 3000,
      };

      // 如果超过最大数量，移除最旧的通知
      const newNotifications = [...state.notifications, newNotification];
      if (newNotifications.length > state.maxNotifications) {
        newNotifications.shift();
      }

      return { notifications: newNotifications };
    }),

  // 删除通知
  removeNotification: (id) =>
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    })),

  // 清空所有通知
  clearNotifications: () => set({ notifications: [] }),

  // 显示成功通知
  showSuccess: (message, duration) =>
    set((state) => {
      state.addNotification({ type: 'success', message, duration });
      return state;
    }),

  // 显示错误通知
  showError: (message, duration) =>
    set((state) => {
      state.addNotification({ type: 'error', message, duration });
      return state;
    }),

  // 显示信息通知
  showInfo: (message, duration) =>
    set((state) => {
      state.addNotification({ type: 'info', message, duration });
      return state;
    }),

  // 显示警告通知
  showWarning: (message, duration) =>
    set((state) => {
      state.addNotification({ type: 'warning', message, duration });
      return state;
    }),
}));
