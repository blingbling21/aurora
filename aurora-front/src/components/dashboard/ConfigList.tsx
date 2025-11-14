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

import { useEffect, useState } from 'react';
import { Button, Card } from '@/components/ui';
import { ConfigListItem } from '@/types/api';
import { configApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store';

/**
 * 配置列表组件属性
 */
interface ConfigListProps {
  // 选中配置时的回调
  onSelect?: (filename: string) => void;
  // 刷新触发器
  refreshTrigger?: number;
}

/**
 * 配置文件列表组件
 * 
 * 显示所有可用的配置文件,支持选择和删除操作
 */
export function ConfigList({ onSelect, refreshTrigger }: ConfigListProps) {
  // 状态管理
  const [configs, setConfigs] = useState<ConfigListItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedFilename, setSelectedFilename] = useState<string | null>(null);
  
  // 通知store
  const { addNotification } = useNotificationStore();

  /**
   * 加载配置列表
   */
  const loadConfigs = async (signal?: AbortSignal) => {
    setLoading(true);
    try {
      const response = await configApi.list();
      
      // 如果请求被取消，不更新状态
      if (signal?.aborted) {
        return;
      }
      
      if (response.success && response.data) {
        setConfigs(response.data);
      } else {
        throw new Error(response.error || '获取配置列表失败');
      }
    } catch (error) {
      // 如果请求被取消，不显示错误
      if (signal?.aborted) {
        return;
      }
      
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '获取配置列表失败',
      });
      setConfigs([]);
    } finally {
      if (!signal?.aborted) {
        setLoading(false);
      }
    }
  };

  /**
   * 删除配置文件
   */
  const handleDelete = async (filename: string, event: React.MouseEvent) => {
    // 阻止事件冒泡,避免触发选择
    event.stopPropagation();
    
    // 确认删除
    if (!confirm(`确定要删除配置文件 "${filename}" 吗?`)) {
      return;
    }

    try {
      const response = await configApi.delete(filename);
      
      if (response.success) {
        addNotification({
          type: 'success',
          message: `成功删除配置文件: ${filename}`,
        });
        
        // 如果删除的是当前选中的配置,清除选中状态
        if (selectedFilename === filename) {
          setSelectedFilename(null);
        }
        
        // 重新加载列表
        loadConfigs();
      } else {
        throw new Error(response.error || '删除配置文件失败');
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '删除配置文件失败',
      });
    }
  };

  /**
   * 选择配置文件
   */
  const handleSelect = (filename: string) => {
    setSelectedFilename(filename);
    onSelect?.(filename);
  };

  // 初始加载和响应刷新触发器
  useEffect(() => {
    // 创建 AbortController 用于取消请求
    const abortController = new AbortController();
    
    // 执行加载
    loadConfigs(abortController.signal);
    
    // 清理函数：组件卸载或依赖变化时取消请求
    return () => {
      abortController.abort();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshTrigger]);

  return (
    <Card title="配置文件列表" className="mt-6">
      {/* 操作按钮 */}
      <div className="flex justify-end mb-4">
        <Button 
          variant="secondary" 
          onClick={() => loadConfigs()}
          disabled={loading}
        >
          {loading ? '加载中...' : '🔄 刷新'}
        </Button>
      </div>

      {/* 配置列表 */}
      {loading ? (
        <div className="text-center py-8 text-gray-500">
          加载中...
        </div>
      ) : configs.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          暂无配置文件
        </div>
      ) : (
        <div className="space-y-3">
          {configs.map((config) => (
            <div
              key={config.filename}
              className={`p-4 border rounded-md hover:border-blue-500 hover:shadow-sm transition-all cursor-pointer ${
                selectedFilename === config.filename
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200'
              }`}
              onClick={() => handleSelect(config.filename)}
            >
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <h4 className="font-semibold text-gray-900">
                    {config.filename}
                  </h4>
                  <div className="flex gap-4 text-xs text-gray-500 mt-1">
                    <span>修改时间: {config.modified}</span>
                    <span>路径: {config.path}</span>
                  </div>
                </div>
                <div className="flex gap-2">
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={(e) => handleDelete(config.filename, e)}
                  >
                    🗑️ 删除
                  </Button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </Card>
  );
}
