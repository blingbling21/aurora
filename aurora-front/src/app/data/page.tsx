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

import { useState, useCallback } from 'react';
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
import { dataApi } from '@/lib/api';
import { useDataDownloadStore } from '@/lib/store/dataDownloadStore';
import { useDataDownloadWebSocket } from '@/lib/hooks/useDataDownloadWebSocket';
import { useNotificationStore } from '@/lib/store/notificationStore';

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
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  // 下载状态管理
  const {
    activeTask,
    showProgressPanel,
    startDownload,
    updateProgress,
    completeDownload,
    failDownload,
  } = useDataDownloadStore();

  // 通知管理
  const { addNotification } = useNotificationStore();

  // 使用 useCallback 包裹回调函数，避免每次渲染都创建新函数
  const handleWebSocketConnected = useCallback(() => {
    console.log('WebSocket 已连接');
  }, []);

  const handleWebSocketProgress = useCallback((progress: {
    progress: number;
    status: string;
    progressMessage: string;
    downloadedCount: number;
    estimatedTotal: number | null;
  }) => {
    // 更新下载进度
    // status 需要转换为 DownloadStatus 类型
    const validStatus = ['Pending', 'Downloading', 'Completed', 'Failed'].includes(progress.status)
      ? progress.status as 'Pending' | 'Downloading' | 'Completed' | 'Failed'
      : 'Downloading';
    
    updateProgress(
      progress.progress,
      validStatus,
      progress.progressMessage,
      progress.downloadedCount,
      progress.estimatedTotal
    );
  }, [updateProgress]);

  const handleWebSocketComplete = useCallback((downloadedCount: number) => {
    // 下载完成
    completeDownload(downloadedCount);
    addNotification({
      type: 'success',
      message: `数据下载完成，共获取 ${downloadedCount} 条数据`,
    });
    // 刷新数据列表
    setRefreshTrigger((prev) => prev + 1);
  }, [completeDownload, addNotification]);

  const handleWebSocketError = useCallback((error: string) => {
    // 下载失败
    failDownload(error);
    addNotification({
      type: 'error',
      message: `数据下载失败: ${error}`,
    });
  }, [failDownload, addNotification]);

  // WebSocket 连接
  const { connectionStatus } = useDataDownloadWebSocket(activeTask?.taskId || null, {
    autoConnect: true,
    isTaskCompleted: activeTask?.status === 'Completed' || activeTask?.status === 'Failed',
    onConnected: handleWebSocketConnected,
    onProgress: handleWebSocketProgress,
    onComplete: handleWebSocketComplete,
    onError: handleWebSocketError,
  });

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
   * 处理下载表单提交
   */
  const handleDownloadSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // 验证必填字段
    if (!exchange || !symbol || !interval || !startDate || !endDate) {
      addNotification({
        type: 'error',
        message: '请填写所有必填字段',
      });
      return;
    }

    // 验证日期范围
    if (startDate >= endDate) {
      addNotification({
        type: 'error',
        message: '开始日期必须早于结束日期',
      });
      return;
    }

    try {
      // 构建请求参数
      const request = {
        exchange,
        symbol: symbol.toUpperCase(),
        interval,
        start_date: startDate.toISOString().split('T')[0],
        end_date: endDate.toISOString().split('T')[0],
        filename: filename || undefined,
      };

      // 发送下载请求
      const response = await dataApi.fetch(request);

      if (response.success && response.data) {
        // 开始下载任务
        startDownload(response.data.task_id, response.data.filename);
        addNotification({
          type: 'info',
          message: '数据下载任务已创建，正在连接...',
        });
      } else {
        throw new Error(response.error || '创建下载任务失败');
      }
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : '下载失败';
      addNotification({
        type: 'error',
        message: errorMsg,
      });
    }
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
        <form onSubmit={handleDownloadSubmit} className="space-y-4">
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

        {/* 下载进度显示 - 只在 showProgressPanel 为 true 时显示 */}
        {activeTask && showProgressPanel && (
          <div className="mt-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
            <div className="flex justify-between items-center mb-2">
              <div>
                <span className="font-medium text-gray-900">
                  {activeTask.status === 'Completed' ? '✅ ' : ''}
                  {activeTask.status === 'Failed' ? '❌ ' : ''}
                  {activeTask.status === 'Downloading' ? '📥 ' : ''}
                  {activeTask.progressMessage}
                </span>
                <div className="text-xs text-gray-500 mt-1">
                  {activeTask.estimatedTotal
                    ? `${activeTask.downloadedCount} / ${activeTask.estimatedTotal} 条数据`
                    : `${activeTask.downloadedCount} 条数据`}
                </div>
              </div>
              <span className="font-semibold text-blue-500">
                {Math.round(activeTask.progress)}%
              </span>
            </div>
            <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
              <div
                className={`h-full rounded-full transition-all duration-300 ${
                  activeTask.status === 'Completed'
                    ? 'bg-green-500'
                    : activeTask.status === 'Failed'
                    ? 'bg-red-500'
                    : 'bg-linear-to-r from-blue-500 to-green-500'
                }`}
                style={{ width: `${activeTask.progress}%` }}
              />
            </div>
            {activeTask.error && (
              <div className="mt-2 text-sm text-red-600">
                错误: {activeTask.error}
              </div>
            )}
            {connectionStatus !== 'connected' && activeTask.status === 'Downloading' && (
              <div className="mt-2 text-xs text-yellow-600">
                ⚠️ WebSocket 连接状态: {connectionStatus}
              </div>
            )}
          </div>
        )}
      </Card>

      {/* 数据文件列表 */}
      <DataList refreshTrigger={refreshTrigger} />
    </div>
  );
}
