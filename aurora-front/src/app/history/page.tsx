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
import { TaskItem } from '@/components/dashboard';
import { BacktestTask, BacktestResult } from '@/types';

/**
 * 历史记录页面
 * 
 * 查看历史回测任务和结果
 */
export default function HistoryPage() {
  // 状态管理
  const [tasks] = useState<BacktestTask[]>([]);
  const [selectedResult] = useState<BacktestResult | null>(null);

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="📜"
        title="历史记录"
        action={<Button variant="secondary">🔄 刷新</Button>}
      />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* 回测历史列表 */}
        <Card title="回测历史">
        {tasks.length === 0 ? (
          <p className="text-gray-500 text-center py-8">暂无历史记录</p>
        ) : (
          <div className="space-y-3">
            {tasks.map((task) => (
              <TaskItem
                key={task.id}
                task={task}
                onClick={() => {
                  // 后续实现加载结果
                  console.log('查看任务结果:', task.id);
                }}
              />
            ))}
          </div>
        )}
      </Card>

        {/* 回测结果查看器 */}
        <Card title="结果详情" className="lg:col-span-2">
          {!selectedResult ? (
            <div className="text-center py-12">
              <p className="text-gray-500 mb-4">选择一个任务查看详细结果</p>
            </div>
          ) : (
            <div className="space-y-6">
              {/* 结果摘要 */}
              <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">总收益率</p>
                <p
                  className={`text-xl font-semibold ${
                    selectedResult.metrics.totalReturn >= 0
                      ? 'text-green-600'
                      : 'text-red-600'
                  }`}
                >
                  {selectedResult.metrics.totalReturn.toFixed(2)}%
                </p>
              </div>

              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">年化收益率</p>
                <p
                  className={`text-xl font-semibold ${
                    selectedResult.metrics.annualizedReturn >= 0
                      ? 'text-green-600'
                      : 'text-red-600'
                  }`}
                >
                  {selectedResult.metrics.annualizedReturn.toFixed(2)}%
                </p>
              </div>

              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">最大回撤</p>
                <p className="text-xl font-semibold text-red-600">
                  {selectedResult.metrics.maxDrawdown.toFixed(2)}%
                </p>
              </div>

              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">夏普比率</p>
                <p className="text-xl font-semibold text-gray-900">
                  {selectedResult.metrics.sharpeRatio.toFixed(3)}
                </p>
              </div>

              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">总交易次数</p>
                <p className="text-xl font-semibold text-gray-900">
                  {selectedResult.metrics.totalTrades}
                </p>
              </div>

              <div className="p-4 bg-gray-50 rounded-lg border border-gray-200">
                <p className="text-xs text-gray-500 mb-2 font-medium">胜率</p>
                <p className="text-xl font-semibold text-gray-900">
                  {selectedResult.metrics.winRate.toFixed(2)}%
                </p>
              </div>
              </div>

              {/* 图表展示区域 - 后续添加图表组件 */}
              <div className="space-y-6">
                <div className="p-6 bg-white rounded-lg border border-gray-200">
                  <h4 className="text-base font-semibold text-gray-900 mb-4 pb-3 border-b-2 border-gray-200">
                    价格走势与交易点位
                  </h4>
                  <div className="h-[500px] flex items-center justify-center text-gray-400">
                    图表组件 - 待实现
                  </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <div className="p-4 bg-white rounded-lg border border-gray-200">
                    <h5 className="text-sm font-semibold text-gray-900 mb-3">
                      权益曲线
                    </h5>
                    <div className="h-[350px] flex items-center justify-center text-gray-400">
                      图表组件 - 待实现
                    </div>
                  </div>

                  <div className="p-4 bg-white rounded-lg border border-gray-200">
                    <h5 className="text-sm font-semibold text-gray-900 mb-3">
                      回撤曲线
                    </h5>
                    <div className="h-[350px] flex items-center justify-center text-gray-400">
                      图表组件 - 待实现
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </Card>
      </div>
    </div>
  );
}
