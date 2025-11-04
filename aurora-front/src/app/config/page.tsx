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
import {
  PageHeader,
  Button,
  Card,
  Input,
  Textarea,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui';
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
          <div className="flex gap-3">
            <Button onClick={() => setIsEditing(true)}>+ 新建配置</Button>
            <Button variant="secondary">🔄 刷新</Button>
          </div>
        }
      />

      <div className="grid grid-cols-1 gap-6">
        {/* 配置文件列表 */}
        <Card title="配置列表">
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
      <Card title="配置编辑器" className="mt-6">
        {!isEditing ? (
          <div className="text-center py-12">
            <p className="text-gray-500 mb-4">选择或创建一个配置文件以开始编辑</p>
            <div className="flex gap-3 justify-center">
              <Button onClick={() => setIsEditing(true)}>+ 新建配置</Button>
              <Button variant="secondary">📁 导入配置</Button>
            </div>
          </div>
        ) : (
          <>
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
                {editMode === 'form' ? '📝 文本模式' : '📋 表单模式'}
              </Button>
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                文件名:
              </label>
              <Input
                type="text"
                placeholder="example.toml"
                className="w-full"
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
                      <Select defaultValue="binance">
                        <SelectTrigger className="w-full">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="binance">Binance</SelectItem>
                          <SelectItem value="okx">OKX</SelectItem>
                          <SelectItem value="bybit">Bybit</SelectItem>
                          <SelectItem value="csv">CSV File</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        超时时间 (秒):
                      </label>
                      <Input
                        type="number"
                        defaultValue={30}
                        className="w-full"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        最大重试次数:
                      </label>
                      <Input
                        type="number"
                        defaultValue={3}
                        className="w-full"
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
                      <Input
                        type="number"
                        defaultValue={10000}
                        className="w-full"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        手续费率 (%):
                      </label>
                      <Input
                        type="number"
                        step="0.01"
                        defaultValue={0.1}
                        className="w-full"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        滑点 (%):
                      </label>
                      <Input
                        type="number"
                        step="0.01"
                        defaultValue={0.05}
                        className="w-full"
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
                <Textarea
                  rows={20}
                  placeholder="在此输入TOML配置..."
                  className="w-full font-mono text-sm"
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
          </>
        )}
      </Card>
      </div>
    </div>
  );
}
