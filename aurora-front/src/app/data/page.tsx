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
import { EXCHANGE_OPTIONS, INTERVAL_OPTIONS, SYMBOL_OPTIONS } from '@/constants';
import { DataList } from '@/components/dashboard/DataList';
import { generateDataFilename } from '@/lib/utils/filename';

/**
 * 数据管理页面
 * 
 * 管理和下载历史市场数据
 */
export default function DataPage() {
  // 表单状态管理
  const [exchange, setExchange] = useState('');
  const [symbol, setSymbol] = useState('');
  const [interval, setInterval] = useState('');
  const [startDate, setStartDate] = useState<Date>();
  const [endDate, setEndDate] = useState<Date>();
  const [filename, setFilename] = useState('');
  
  // 下载进度状态
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  /**
   * 处理交易对下拉框变化
   * 当用户从下拉框选择交易对时，自动填充到输入框
   */
  const handleSymbolSelectChange = (value: string) => {
    if (value) {
      setSymbol(value);
      // 触发文件名更新
      updateFilename(exchange, value, interval, startDate, endDate);
    }
  };

  /**
   * 更新文件名
   * 根据表单输入自动生成文件名
   */
  const updateFilename = (
    ex: string,
    sym: string,
    int: string,
    start: Date | undefined,
    end: Date | undefined
  ) => {
    const generatedFilename = generateDataFilename(ex, sym, int, start, end);
    setFilename(generatedFilename);
  };

  /**
   * 处理表单字段变化，自动更新文件名
   */
  const handleExchangeChange = (value: string) => {
    setExchange(value);
    updateFilename(value, symbol, interval, startDate, endDate);
  };

  const handleSymbolChange = (value: string) => {
    setSymbol(value);
    updateFilename(exchange, value, interval, startDate, endDate);
  };

  const handleIntervalChange = (value: string) => {
    setInterval(value);
    updateFilename(exchange, symbol, value, startDate, endDate);
  };

  const handleStartDateChange = (date: Date | undefined) => {
    setStartDate(date);
    updateFilename(exchange, symbol, interval, date, endDate);
  };

  const handleEndDateChange = (date: Date | undefined) => {
    setEndDate(date);
    updateFilename(exchange, symbol, interval, startDate, date);
  };

  /**
   * 预览文件名
   */
  const handlePreviewFilename = () => {
    if (filename) {
      alert(`文件将保存为: ${filename}`);
    } else {
      alert('请先填写所有必填字段');
    }
  };

  /**
   * TODO: 实现数据下载完成后刷新列表
   * 在下载完成回调中调用: setRefreshTrigger(prev => prev + 1)
   */

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="📁"
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
            console.log('开始下载数据', { exchange, symbol, interval, startDate, endDate, filename });
          }}
          className="space-y-4"
        >
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                交易所:
              </label>
              <Select required value={exchange} onValueChange={handleExchangeChange}>
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
                <Select value={symbol} onValueChange={handleSymbolSelectChange}>
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
                  value={symbol}
                  onChange={(e) => handleSymbolChange(e.target.value.toUpperCase())}
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
              <Select required value={interval} onValueChange={handleIntervalChange}>
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
                onDateChange={handleStartDateChange}
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
                onDateChange={handleEndDateChange}
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
                value={filename}
                onChange={(e) => setFilename(e.target.value)}
                placeholder="自动生成"
                className="w-full"
              />
            </div>
          </div>

          <div className="flex gap-3">
            <Button type="submit">📥 开始下载</Button>
            <Button type="button" variant="secondary" onClick={handlePreviewFilename}>
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
      <DataList refreshTrigger={refreshTrigger} />
    </div>
  );
}
