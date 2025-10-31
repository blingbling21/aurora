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

import { useState } from 'react';
import { PageHeader, Button, Card } from '@/components/ui';

/**
 * 回测执行页面
 * 
 * 配置并启动新的回测任务
 */
export default function BacktestPage() {
  // 状态管理
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState(0);
  const [taskName, setTaskName] = useState('');

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="🚀"
        title="回测执行"
        description="配置并启动新的回测任务"
      />

      {/* 启动回测表单 */}
      <Card title="启动新回测">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            setIsRunning(true);
            setProgress(0);
            // 后续实现启动回测逻辑
            console.log('启动回测任务');
          }}
          className="space-y-4"
        >
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              任务名称:
            </label>
            <input
              type="text"
              required
              placeholder="例如: BTC MA交叉策略测试"
              value={taskName}
              onChange={(e) => setTaskName(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              选择配置文件:
            </label>
            <select
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">-- 请选择 --</option>
              {/* 后续从 API 加载配置列表 */}
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              选择数据文件:
            </label>
            <select
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">-- 请选择 --</option>
              {/* 后续从 API 加载数据文件列表 */}
            </select>
          </div>

          <Button type="submit">🚀 启动回测</Button>
        </form>
      </Card>

      {/* 回测进度 */}
      {isRunning && (
        <Card title="回测进度" className="mt-6">
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <span className="text-sm font-medium text-gray-700">
                {taskName || '回测任务'}
              </span>
              <span className="text-sm font-semibold text-blue-500">
                {progress}%
              </span>
            </div>

            <div className="h-6 bg-gray-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-linear-to-r from-blue-500 to-blue-600 transition-all duration-300 flex items-center justify-center"
                style={{ width: `${progress}%` }}
              >
                {progress > 10 && (
                  <span className="text-xs font-semibold text-white">
                    {progress}%
                  </span>
                )}
              </div>
            </div>

            <p className="text-sm text-gray-600">准备中...</p>

            <Button
              variant="secondary"
              disabled={progress < 100}
              onClick={() => {
                // 后续实现查看结果
                console.log('查看结果');
              }}
            >
              查看结果
            </Button>
          </div>
        </Card>
      )}
    </div>
  );
}
