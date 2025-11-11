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

import { useState, useEffect, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import { PageHeader, Button, Card } from '@/components/ui';
import { TaskItem } from '@/components/dashboard';
import { BacktestTask } from '@/types';
import { backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';

/**
 * 历史记录列表页面
 * 
 * 显示所有历史回测任务列表,点击任务跳转到详情页
 */
export default function HistoryPage() {
  // 状态管理
  const [tasks, setTasks] = useState<BacktestTask[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const { addNotification } = useNotificationStore();
  const router = useRouter();

  /**
   * 加载回测任务列表
   */
  const loadTasks = useCallback(async () => {
    setIsLoading(true);
    try {
      const response = await backtestApi.list();
      if (response.success && response.data) {
        // 转换API数据格式为前端期望的格式
        const convertedTasks: BacktestTask[] = response.data.map((task) => ({
          id: task.id,
          name: task.name,
          status: task.status,
          config: task.config_path || '',
          dataFile: task.data_path || '',
          progress: task.progress,
          createdAt: task.created_at,
          updatedAt: task.completed_at || task.started_at || task.created_at,
        }));
        setTasks(convertedTasks);
      } else {
        throw new Error(response.error || '加载失败');
      }
    } catch {
      addNotification({
        type: 'error',
        message: '加载历史记录失败',
      });
    } finally {
      setIsLoading(false);
    }
  }, [addNotification]);

  // 组件挂载时加载任务列表
  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  /**
   * 处理任务点击,跳转到详情页
   */
  const handleTaskClick = (task: BacktestTask) => {
    // 跳转到回测详情页面
    router.push(`/history/${task.id}`);
  };

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="📜"
        title="历史记录"
        action={
          <Button 
            variant="secondary" 
            onClick={() => loadTasks()}
            disabled={isLoading}
          >
            🔄 刷新
          </Button>
        }
      />

      {/* 回测历史列表 */}
      <Card title="回测历史">
        {isLoading ? (
          <div className="text-center py-12">
            <p className="text-gray-500">正在加载...</p>
          </div>
        ) : tasks.length === 0 ? (
          <div className="text-center py-12">
            <p className="text-gray-500">暂无历史记录</p>
          </div>
        ) : (
          <div className="space-y-3">
            {tasks.map((task) => (
              <TaskItem
                key={task.id}
                task={task}
                onClick={() => handleTaskClick(task)}
              />
            ))}
          </div>
        )}
      </Card>
    </div>
  );
}
