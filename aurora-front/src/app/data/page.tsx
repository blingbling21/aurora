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
  DatePicker,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui';
import { DataFile } from '@/types';
import { EXCHANGE_OPTIONS, INTERVAL_OPTIONS, SYMBOL_OPTIONS } from '@/constants';

/**
 * 数据管理页面
 * 
 * 管理和下载历史市场数据
 */
export default function DataPage() {
  // 状态管理
  const [dataFiles] = useState<DataFile[]>([]);
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [startDate, setStartDate] = useState<Date>();
  const [endDate, setEndDate] = useState<Date>();

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="�"
        title="数据管理"
        description="管理和下载历史市场数据"
      />

      {/* 数据下载表单 */}
      <Card title="下载数据">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            setIsDownloading(true);
            setDownloadProgress(0);
            // 后续实现下载逻辑
            console.log('开始下载数据');
          }}
          className="space-y-4"
        >
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                交易所:
              </label>
              <Select required>
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="-- 请选择 --" />
                </SelectTrigger>
                <SelectContent>
                  {EXCHANGE_OPTIONS.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                交易对:
              </label>
              <div className="flex gap-2">
                <Select>
                  <SelectTrigger className="flex-1">
                    <SelectValue placeholder="-- 选择或手动输入 --" />
                  </SelectTrigger>
                  <SelectContent>
                    {SYMBOL_OPTIONS.map((opt) => (
                      <SelectItem key={opt.value} value={opt.value}>
                        {opt.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input
                  type="text"
                  required
                  placeholder="例如: BTCUSDT"
                  className="flex-2 uppercase"
                />
              </div>
              <p className="text-xs text-gray-500 mt-1">
                💡 提示: 可以从下拉框选择常用交易对,或手动输入
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                时间周期:
              </label>
              <Select required>
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="-- 请选择 --" />
                </SelectTrigger>
                <SelectContent>
                  {INTERVAL_OPTIONS.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                开始日期:
              </label>
              <DatePicker
                date={startDate}
                onDateChange={setStartDate}
                placeholder="选择开始日期"
                required
                className="w-full"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                结束日期:
              </label>
              <DatePicker
                date={endDate}
                onDateChange={setEndDate}
                placeholder="选择结束日期"
                required
                className="w-full"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                保存文件名:
              </label>
              <Input
                type="text"
                placeholder="自动生成"
                readOnly
                className="w-full bg-gray-50"
              />
            </div>
          </div>

          <div className="flex gap-3">
            <Button type="submit">📥 开始下载</Button>
            <Button type="button" variant="secondary">
              👁️ 预览文件名
            </Button>
          </div>
        </form>

        {/* 下载进度 */}
        {isDownloading && (
          <div className="mt-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
            <div className="flex justify-between items-center mb-2 text-sm">
              <span className="font-medium text-gray-900">准备下载...</span>
              <span className="font-semibold text-blue-500">
                {downloadProgress}%
              </span>
            </div>
            <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-linear-to-r from-blue-500 to-green-500 rounded-full transition-all duration-300"
                style={{ width: `${downloadProgress}%` }}
              />
            </div>
          </div>
        )}
      </Card>

      {/* 数据文件列表 */}
      <Card title="数据文件列表" className="mt-6">
        <div className="flex justify-end mb-4">
          <Button variant="secondary">🔄 刷新</Button>
        </div>

        {dataFiles.length === 0 ? (
          <p className="text-gray-500 text-center py-8">暂无数据文件</p>
        ) : (
          <div className="space-y-3">
            {dataFiles.map((file) => (
              <div
                key={file.path}
                className="p-4 border border-gray-200 rounded-md hover:border-blue-500 hover:shadow-sm transition-all cursor-pointer"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <h4 className="font-semibold text-gray-900">{file.name}</h4>
                    <div className="flex gap-4 text-xs text-gray-500 mt-1">
                      <span>大小: {(file.size / 1024 / 1024).toFixed(2)} MB</span>
                      <span>
                        修改: {new Date(file.lastModified).toLocaleString('zh-CN')}
                      </span>
                    </div>
                  </div>
                  <Button variant="secondary" size="sm">
                    删除
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </Card>
    </div>
  );
}
