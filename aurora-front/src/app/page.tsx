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

import { PageHeader } from '@/components/ui';
import { StatCard, TaskItem } from '@/components/dashboard';
import { Card } from '@/components/ui';
import { BacktestTask } from '@/types';

/**
 * 仪表盘首页
 * 
 * 显示回测任务概览、统计数据和最近任务列表
 */
export default function Home() {
  // 模拟数据 - 后续会从 API 获取
  const stats = {
    total: 0,
    running: 0,
    completed: 0,
    failed: 0,
  };

  const recentTasks: BacktestTask[] = [];

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="📊"
        title="仪表盘"
        description="回测任务概览与实时监控"
      />

      {/* 统计卡片网格 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatCard icon="📈" value={stats.total} label="总任务数" />
        <StatCard icon="⏳" value={stats.running} label="运行中" />
        <StatCard icon="✅" value={stats.completed} label="已完成" />
        <StatCard icon="❌" value={stats.failed} label="失败" />
      </div>

      {/* 最近任务列表 */}
      <Card title="最近任务">
        {recentTasks.length === 0 ? (
          <p className="text-gray-500 text-center py-8">暂无任务记录</p>
        ) : (
          <div className="space-y-3">
            {recentTasks.map((task) => (
              <TaskItem
                key={task.id}
                task={task}
                onClick={() => {
                  // 后续实现跳转到任务详情
                  console.log('查看任务:', task.id);
                }}
              />
            ))}
          </div>
        )}
      </Card>
    </div>
  );
}
