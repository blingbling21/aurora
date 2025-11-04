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

import { renderHook, act } from '@testing-library/react';
import { useNotificationStore } from './notificationStore';

describe('useNotificationStore', () => {
  // 在每个测试之前重置store
  beforeEach(() => {
    const { result } = renderHook(() => useNotificationStore());
    act(() => {
      result.current.clearNotifications();
    });
  });

  // 测试初始状态
  describe('初始状态', () => {
    it('应该有正确的初始值', () => {
      const { result } = renderHook(() => useNotificationStore());
      
      expect(result.current.notifications).toEqual([]);
      expect(result.current.maxNotifications).toBe(5);
    });
  });

  // 测试 addNotification
  describe('addNotification', () => {
    it('应该能够添加新通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({
          type: 'success',
          message: '操作成功',
        });
      });

      expect(result.current.notifications).toHaveLength(1);
      expect(result.current.notifications[0]).toMatchObject({
        type: 'success',
        message: '操作成功',
        duration: 3000, // 默认值
      });
      expect(result.current.notifications[0].id).toBeDefined();
    });

    it('应该使用自定义持续时间', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({
          type: 'info',
          message: '信息提示',
          duration: 5000,
        });
      });

      expect(result.current.notifications[0].duration).toBe(5000);
    });

    it('应该为每个通知生成唯一ID', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({ type: 'success', message: '消息1' });
        result.current.addNotification({ type: 'success', message: '消息2' });
      });

      const ids = result.current.notifications.map(n => n.id);
      expect(new Set(ids).size).toBe(2); // 所有ID都是唯一的
    });

    it('超过最大数量时应该移除最旧的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        // 添加6个通知，超过最大数量5
        for (let i = 1; i <= 6; i++) {
          result.current.addNotification({
            type: 'info',
            message: `消息${i}`,
          });
        }
      });

      expect(result.current.notifications).toHaveLength(5);
      // 第一个消息应该被移除，现在第一个应该是"消息2"
      expect(result.current.notifications[0].message).toBe('消息2');
      expect(result.current.notifications[4].message).toBe('消息6');
    });
  });

  // 测试 removeNotification
  describe('removeNotification', () => {
    it('应该能够删除指定的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({
          type: 'success',
          message: '测试消息',
        });
      });

      expect(result.current.notifications).toHaveLength(1);
      const notificationId = result.current.notifications[0].id;

      act(() => {
        result.current.removeNotification(notificationId);
      });

      expect(result.current.notifications).toHaveLength(0);
    });

    it('删除不存在的通知不应该产生错误', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({
          type: 'success',
          message: '测试消息',
        });
      });

      expect(result.current.notifications).toHaveLength(1);

      act(() => {
        result.current.removeNotification('non-existent-id');
      });

      // 应该仍然保留原有通知
      expect(result.current.notifications).toHaveLength(1);
    });

    it('应该只删除指定的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({ type: 'success', message: '消息1' });
      });

      act(() => {
        result.current.addNotification({ type: 'info', message: '消息2' });
      });

      act(() => {
        result.current.addNotification({ type: 'warning', message: '消息3' });
      });

      expect(result.current.notifications).toHaveLength(3);
      const id1 = result.current.notifications[0].id;
      const id2 = result.current.notifications[1].id;
      const id3 = result.current.notifications[2].id;

      act(() => {
        result.current.removeNotification(id2);
      });

      expect(result.current.notifications).toHaveLength(2);
      expect(result.current.notifications.find(n => n.id === id1)).toBeDefined();
      expect(result.current.notifications.find(n => n.id === id2)).toBeUndefined();
      expect(result.current.notifications.find(n => n.id === id3)).toBeDefined();
    });
  });

  // 测试 clearNotifications
  describe('clearNotifications', () => {
    it('应该能够清空所有通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.addNotification({ type: 'success', message: '消息1' });
        result.current.addNotification({ type: 'info', message: '消息2' });
        result.current.addNotification({ type: 'warning', message: '消息3' });
      });

      expect(result.current.notifications).toHaveLength(3);

      act(() => {
        result.current.clearNotifications();
      });

      expect(result.current.notifications).toHaveLength(0);
    });
  });

  // 测试 showSuccess
  describe('showSuccess', () => {
    it('应该添加成功类型的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showSuccess('操作成功');
      });

      expect(result.current.notifications).toHaveLength(1);
      expect(result.current.notifications[0]).toMatchObject({
        type: 'success',
        message: '操作成功',
      });
    });

    it('应该支持自定义持续时间', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showSuccess('操作成功', 5000);
      });

      expect(result.current.notifications[0].duration).toBe(5000);
    });
  });

  // 测试 showError
  describe('showError', () => {
    it('应该添加错误类型的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showError('操作失败');
      });

      expect(result.current.notifications).toHaveLength(1);
      expect(result.current.notifications[0]).toMatchObject({
        type: 'error',
        message: '操作失败',
      });
    });

    it('应该支持自定义持续时间', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showError('操作失败', 10000);
      });

      expect(result.current.notifications[0].duration).toBe(10000);
    });
  });

  // 测试 showInfo
  describe('showInfo', () => {
    it('应该添加信息类型的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showInfo('提示信息');
      });

      expect(result.current.notifications).toHaveLength(1);
      expect(result.current.notifications[0]).toMatchObject({
        type: 'info',
        message: '提示信息',
      });
    });

    it('应该支持自定义持续时间', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showInfo('提示信息', 4000);
      });

      expect(result.current.notifications[0].duration).toBe(4000);
    });
  });

  // 测试 showWarning
  describe('showWarning', () => {
    it('应该添加警告类型的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showWarning('警告信息');
      });

      expect(result.current.notifications).toHaveLength(1);
      expect(result.current.notifications[0]).toMatchObject({
        type: 'warning',
        message: '警告信息',
      });
    });

    it('应该支持自定义持续时间', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showWarning('警告信息', 6000);
      });

      expect(result.current.notifications[0].duration).toBe(6000);
    });
  });

  // 测试多种通知类型混合
  describe('多种通知类型', () => {
    it('应该能够同时显示多种类型的通知', () => {
      const { result } = renderHook(() => useNotificationStore());

      act(() => {
        result.current.showSuccess('成功消息');
        result.current.showError('错误消息');
        result.current.showInfo('信息消息');
        result.current.showWarning('警告消息');
      });

      expect(result.current.notifications).toHaveLength(4);
      expect(result.current.notifications[0].type).toBe('success');
      expect(result.current.notifications[1].type).toBe('error');
      expect(result.current.notifications[2].type).toBe('info');
      expect(result.current.notifications[3].type).toBe('warning');
    });
  });
});
