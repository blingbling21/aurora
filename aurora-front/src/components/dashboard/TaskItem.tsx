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

import { BacktestTask, TaskStatus } from '@/types';
import { cn } from '@/lib/utils';

interface TaskItemProps {
  task: BacktestTask;
  onClick?: () => void;
}

/**
 * 任务列表项组件
 * 
 * 显示单个回测任务的信息
 * 
 * @param {BacktestTask} task - 任务数据
 * @param {Function} onClick - 点击回调函数
 */
export function TaskItem({ task, onClick }: TaskItemProps) {
  // 状态样式映射
  const statusStyles: Record<TaskStatus, string> = {
    pending: 'bg-yellow-100 text-yellow-800',
    running: 'bg-blue-100 text-blue-800',
    completed: 'bg-green-100 text-green-800',
    failed: 'bg-red-100 text-red-800',
  };

  // 状态文本映射
  const statusText: Record<TaskStatus, string> = {
    pending: '待处理',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
  };

  return (
    <div
      className="p-4 border border-gray-200 rounded-md hover:border-blue-500 hover:shadow-sm transition-all cursor-pointer"
      onClick={onClick}
    >
      {/* 任务头部 */}
      <div className="flex items-center justify-between mb-2">
        <h4 className="font-semibold text-gray-900">{task.name}</h4>
        <span
          className={cn(
            'px-3 py-1 rounded-full text-xs font-medium',
            statusStyles[task.status]
          )}
        >
          {statusText[task.status]}
        </span>
      </div>

      {/* 任务元信息 */}
      <div className="flex gap-4 text-xs text-gray-500">
        <span>配置: {task.config}</span>
        <span>数据: {task.dataFile}</span>
        <span>创建: {new Date(task.createdAt).toLocaleString('zh-CN')}</span>
      </div>

      {/* 进度条(仅运行中时显示) */}
      {task.status === 'running' && (
        <div className="mt-3">
          <div className="flex justify-between text-xs text-gray-600 mb-1">
            <span>进度</span>
            <span>{task.progress}%</span>
          </div>
          <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
            <div
              className="h-full bg-linear-to-r from-blue-500 to-blue-600 transition-all duration-300"
              style={{ width: `${task.progress}%` }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
