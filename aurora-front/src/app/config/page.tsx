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
import { ConfigFile } from '@/types';

/**
 * 配置管理页面
 * 
 * 管理和编辑回测配置文件
 */
export default function ConfigPage() {
  // 状态管理
  const [configs] = useState<ConfigFile[]>([]);
  const [isEditing, setIsEditing] = useState(false);
  const [editMode, setEditMode] = useState<'form' | 'text'>('form');

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="⚙️"
        title="配置管理"
        action={
          <Button onClick={() => setIsEditing(true)}>+ 新建配置</Button>
        }
      />

      {/* 配置文件列表 */}
      <Card title="配置文件列表">
        {configs.length === 0 ? (
          <p className="text-gray-500 text-center py-8">暂无配置文件</p>
        ) : (
          <div className="space-y-3">
            {configs.map((config) => (
              <div
                key={config.path}
                className="p-4 border border-gray-200 rounded-md hover:border-blue-500 hover:shadow-sm transition-all cursor-pointer"
                onClick={() => {
                  // 后续实现编辑功能
                  console.log('编辑配置:', config.name);
                }}
              >
                <div className="flex items-center justify-between">
                  <h4 className="font-semibold text-gray-900">{config.name}</h4>
                  <span className="text-xs text-gray-500">
                    {new Date(config.lastModified).toLocaleString('zh-CN')}
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>

      {/* 配置编辑器 */}
      {isEditing && (
        <Card title="编辑配置" className="mt-6">
          <div className="mb-4 flex gap-3">
            <input
              type="file"
              accept=".toml"
              className="hidden"
              id="config-import"
            />
            <Button
              variant="secondary"
              onClick={() => document.getElementById('config-import')?.click()}
            >
              📁 导入 TOML
            </Button>
            <Button
              variant="secondary"
              onClick={() => setEditMode(editMode === 'form' ? 'text' : 'form')}
            >
              🔄 切换模式
            </Button>
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              文件名:
            </label>
            <input
              type="text"
              placeholder="example.toml"
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          {editMode === 'form' ? (
            <div className="space-y-6">
              {/* 表单模式 - 后续会添加详细的表单字段 */}
              <div>
                <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
                  数据源配置
                </h4>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      数据提供商:
                    </label>
                    <select className="w-full px-3 py-2 border border-gray-300 rounded-md">
                      <option value="binance">Binance</option>
                      <option value="okx">OKX</option>
                      <option value="bybit">Bybit</option>
                      <option value="csv">CSV File</option>
                    </select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      超时时间 (秒):
                    </label>
                    <input
                      type="number"
                      defaultValue={30}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      最大重试次数:
                    </label>
                    <input
                      type="number"
                      defaultValue={3}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md"
                    />
                  </div>
                </div>
              </div>

              <div>
                <h4 className="text-base font-semibold text-blue-500 mb-3 pb-2 border-b">
                  投资组合配置
                </h4>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      初始资金:
                    </label>
                    <input
                      type="number"
                      defaultValue={10000}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      手续费率 (%):
                    </label>
                    <input
                      type="number"
                      step="0.01"
                      defaultValue={0.1}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      滑点 (%):
                    </label>
                    <input
                      type="number"
                      step="0.01"
                      defaultValue={0.05}
                      className="w-full px-3 py-2 border border-gray-300 rounded-md"
                    />
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                配置内容 (TOML):
              </label>
              <textarea
                rows={20}
                placeholder="在此输入TOML配置..."
                className="w-full px-3 py-2 border border-gray-300 rounded-md font-mono text-sm"
              />
            </div>
          )}

          <div className="mt-6 flex gap-3">
            <Button>💾 保存</Button>
            <Button variant="secondary">✓ 验证</Button>
            <Button variant="secondary" onClick={() => setIsEditing(false)}>
              ✕ 取消
            </Button>
          </div>
        </Card>
      )}
    </div>
  );
}
