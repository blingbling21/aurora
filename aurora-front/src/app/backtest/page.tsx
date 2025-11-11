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

import { useState, useEffect } from 'react';
import {
  PageHeader,
  Button,
  Card,
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui';
import { useConfigStore, useDataStore } from '@/lib/store';
import { configApi, dataApi, backtestApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store/notificationStore';
import { useBacktestWebSocket } from '@/lib/hooks/useBacktestWebSocket';

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
  const [selectedConfig, setSelectedConfig] = useState('');
  const [selectedData, setSelectedData] = useState('');
  const [currentTaskId, setCurrentTaskId] = useState<string | null>(null);
  const [progressMessage, setProgressMessage] = useState('');
  // 任务完成状态,用于防止WebSocket重连
  const [isTaskCompleted, setIsTaskCompleted] = useState(false);

  // Zustand stores
  const { configs, setConfigs } = useConfigStore();
  const { dataFiles, setDataFiles } = useDataStore();
  const { addNotification } = useNotificationStore();

  // WebSocket连接用于接收回测进度更新
  useBacktestWebSocket(currentTaskId, {
    autoConnect: true,
    isTaskCompleted,
    onConnected: () => {
      console.log('WebSocket已连接,等待回测进度更新');
    },
    onStatusUpdate: (progressValue, status) => {
      // 更新进度条
      setProgress(progressValue);
      setProgressMessage(`状态: ${status}`);
      
      // 如果任务完成或失败,标记任务完成状态
      if (status === 'completed' || status === 'failed') {
        setIsRunning(false);
        setIsTaskCompleted(true);
        
        if (status === 'completed') {
          addNotification({
            type: 'success',
            message: '回测任务完成',
          });
        } else {
          addNotification({
            type: 'error',
            message: '回测任务失败',
          });
        }
      }
    },
    onComplete: (data) => {
      // 收到final消息,任务已结束
      console.log('回测任务已完成,数据:', data);
      setIsRunning(false);
      setIsTaskCompleted(true);
      setProgressMessage('任务已结束');
      
      addNotification({
        type: 'info',
        message: '任务已结束',
      });
    },
    onError: (error) => {
      console.error('WebSocket错误:', error);
      addNotification({
        type: 'error',
        message: `WebSocket连接错误: ${error}`,
      });
    },
  });

  // 加载配置文件列表和数据文件列表
  useEffect(() => {
    // 加载配置文件列表
    const loadConfigs = async () => {
      try {
        const response = await configApi.list();
        if (response.success && response.data) {
          // 将API返回的ConfigListItem转换为ConfigFile格式
          const configFiles = response.data.map((item) => ({
            name: item.filename,
            path: item.path,
            content: '', // API list接口不返回content，后续加载详情时获取
            lastModified: item.modified,
          }));
          setConfigs(configFiles);
        }
      } catch {
        addNotification({
          type: 'error',
          message: '加载配置文件列表失败',
        });
      }
    };

    // 加载数据文件列表
    const loadDataFiles = async () => {
      try {
        const response = await dataApi.list();
        if (response.success && response.data) {
          // 将API返回的DataFileItem转换为DataFile格式
          const files = response.data.map((item) => ({
            name: item.filename,
            path: '', // API不返回path，使用filename作为标识
            size: item.size,
            lastModified: item.modified,
          }));
          setDataFiles(files);
        }
      } catch {
        addNotification({
          type: 'error',
          message: '加载数据文件列表失败',
        });
      }
    };

    loadConfigs();
    loadDataFiles();
  }, [setConfigs, setDataFiles, addNotification]);

  /**
   * 处理启动回测
   */
  const handleStartBacktest = async (e: React.FormEvent) => {
    e.preventDefault();

    // 验证必填字段
    if (!taskName || !selectedConfig || !selectedData) {
      addNotification({
        type: 'error',
        message: '请填写所有必填字段',
      });
      return;
    }

    try {
      // 设置运行状态并重置完成标志
      setIsRunning(true);
      setIsTaskCompleted(false);
      setProgress(0);
      setProgressMessage('准备启动回测...');

      // 调用API启动回测任务
      const response = await backtestApi.start({
        name: taskName,
        config_path: selectedConfig,
        data_path: selectedData,
      });

      if (response.success && response.data) {
        // 从响应中提取task_id
        const taskId = typeof response.data === 'object' && 'task_id' in response.data
          ? String(response.data.task_id)
          : null;

        if (taskId) {
          setCurrentTaskId(taskId);
          setProgressMessage('回测任务已启动,等待进度更新...');
          addNotification({
            type: 'success',
            message: `回测任务已启动: ${taskName}`,
          });
        } else {
          throw new Error('未能获取任务ID');
        }
      } else {
        throw new Error(response.error || '启动回测失败');
      }
    } catch (error) {
      console.error('启动回测失败:', error);
      setIsRunning(false);
      setProgress(0);
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '启动回测失败',
      });
    }
  };

  /**
   * 处理停止回测
   */
  const handleStopBacktest = () => {
    setIsRunning(false);
    setProgress(0);
    setCurrentTaskId(null);
    setProgressMessage('');
    addNotification({
      type: 'info',
      message: '回测任务已停止',
    });
  };

  return (
    <div>
      {/* 页面头部 */}
      <PageHeader
        icon="🚀"
        title="回测执行"
        description="配置并启动新的回测任务"
      />

      <div className="grid grid-cols-1 gap-6">
        {/* 启动回测表单 */}
        <Card title="任务配置">
        <form
          onSubmit={handleStartBacktest}
          className="space-y-4"
        >
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              任务名称:
            </label>
            <Input
              type="text"
              required
              placeholder="例如: BTC MA交叉策略测试"
              value={taskName}
              onChange={(e) => setTaskName(e.target.value)}
              className="w-full"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              选择配置文件:
            </label>
            <Select 
              required 
              value={selectedConfig}
              onValueChange={setSelectedConfig}
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="-- 请选择 --" />
              </SelectTrigger>
              <SelectContent>
                {configs.length === 0 ? (
                  <div className="px-2 py-6 text-center text-sm text-gray-500">
                    暂无配置文件,请先创建配置
                  </div>
                ) : (
                  configs.map((config) => (
                    <SelectItem key={config.name} value={config.name}>
                      {config.name}
                    </SelectItem>
                  ))
                )}
              </SelectContent>
            </Select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              选择数据文件:
            </label>
            <Select 
              required
              value={selectedData}
              onValueChange={setSelectedData}
            >
              <SelectTrigger className="w-full">
                <SelectValue placeholder="-- 请选择 --" />
              </SelectTrigger>
              <SelectContent>
                {dataFiles.length === 0 ? (
                  <div className="px-2 py-6 text-center text-sm text-gray-500">
                    暂无数据文件,请先下载数据
                  </div>
                ) : (
                  dataFiles.map((file) => (
                    <SelectItem key={file.name} value={file.name}>
                      {file.name}
                    </SelectItem>
                  ))
                )}
              </SelectContent>
            </Select>
          </div>

          <div className="flex gap-3">
            <Button type="submit" disabled={isRunning}>
              🚀 开始回测
            </Button>
            <Button 
              type="button" 
              variant="secondary" 
              disabled={!isRunning}
              onClick={handleStopBacktest}
            >
              ⏹️ 停止
            </Button>
          </div>
        </form>
      </Card>

      {/* 执行结果 */}
      <Card title="执行结果" className="mt-6">
        {!isRunning ? (
          <div className="text-center py-12">
            <p className="text-gray-500 mb-4">点击&ldquo;开始回测&rdquo;按钮启动任务</p>
          </div>
        ) : (
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

            <p className="text-sm text-gray-600">
              {progressMessage || '准备中...'}
            </p>

            <Button
              variant="secondary"
              disabled={progress < 100}
              onClick={() => {
                // 后续实现查看结果
                if (currentTaskId) {
                  console.log('查看结果,任务ID:', currentTaskId);
                  addNotification({
                    type: 'info',
                    message: '结果查看功能即将实现',
                  });
                }
              }}
            >
              查看结果
            </Button>
          </div>
        )}
      </Card>
      </div>
    </div>
  );
}
