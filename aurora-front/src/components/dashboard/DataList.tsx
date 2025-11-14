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
import { DataFileItem } from '@/types/api';
import { dataApi } from '@/lib/api';
import { useNotificationStore } from '@/lib/store';

/**
 * 数据列表组件属性
 */
interface DataListProps {
  // 选中数据文件时的回调
  onSelect?: (filename: string) => void;
  // 刷新触发器
  refreshTrigger?: number;
}

/**
 * 数据文件列表组件
 * 
 * 显示所有可用的数据文件,支持选择和删除操作
 */
export function DataList({ onSelect, refreshTrigger }: DataListProps) {
  // 状态管理
  const [dataFiles, setDataFiles] = useState<DataFileItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedFilename, setSelectedFilename] = useState<string | null>(null);
  
  // 通知store
  const { addNotification } = useNotificationStore();

  /**
   * 加载数据文件列表
   */
  const loadDataFiles = async (signal?: AbortSignal) => {
    setLoading(true);
    try {
      const response = await dataApi.list();
      
      // 如果请求被取消，不更新状态
      if (signal?.aborted) {
        return;
      }
      
      if (response.success && response.data) {
        setDataFiles(response.data);
      } else {
        throw new Error(response.error || '获取数据文件列表失败');
      }
    } catch (error) {
      // 如果请求被取消，不显示错误
      if (signal?.aborted) {
        return;
      }
      
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '获取数据文件列表失败',
      });
      setDataFiles([]);
    } finally {
      if (!signal?.aborted) {
        setLoading(false);
      }
    }
  };

  /**
   * 删除数据文件
   */
  const handleDelete = async (filename: string, event: React.MouseEvent) => {
    // 阻止事件冒泡,避免触发选择
    event.stopPropagation();
    
    // 确认删除
    if (!confirm(`确定要删除数据文件 "${filename}" 吗?`)) {
      return;
    }

    try {
      const response = await dataApi.delete(filename);
      
      if (response.success) {
        addNotification({
          type: 'success',
          message: `成功删除数据文件: ${filename}`,
        });
        
        // 如果删除的是当前选中的文件,清除选中状态
        if (selectedFilename === filename) {
          setSelectedFilename(null);
        }
        
        // 重新加载列表
        loadDataFiles();
      } else {
        throw new Error(response.error || '删除数据文件失败');
      }
    } catch (error) {
      addNotification({
        type: 'error',
        message: error instanceof Error ? error.message : '删除数据文件失败',
      });
    }
  };

  /**
   * 选择数据文件
   */
  const handleSelect = (filename: string) => {
    setSelectedFilename(filename);
    onSelect?.(filename);
  };

  /**
   * 格式化文件大小
   */
  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
  };

  // 初始加载和响应刷新触发器
  useEffect(() => {
    // 创建 AbortController 用于取消请求
    const abortController = new AbortController();
    
    // 执行加载
    loadDataFiles(abortController.signal);
    
    // 清理函数：组件卸载或依赖变化时取消请求
    return () => {
      abortController.abort();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [refreshTrigger]);

  return (
    <Card title="数据文件列表" className="mt-6">
      {/* 操作按钮 */}
      <div className="flex justify-end mb-4">
        <Button 
          variant="secondary" 
          onClick={() => loadDataFiles()}
          disabled={loading}
        >
          {loading ? '加载中...' : '🔄 刷新'}
        </Button>
      </div>

      {/* 数据文件列表 */}
      {loading ? (
        <div className="text-center py-8 text-gray-500">
          加载中...
        </div>
      ) : dataFiles.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          暂无数据文件
        </div>
      ) : (
        <div className="space-y-3">
          {dataFiles.map((file) => (
            <div
              key={file.filename}
              className={`p-4 border rounded-md hover:border-blue-500 hover:shadow-sm transition-all cursor-pointer ${
                selectedFilename === file.filename
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-200'
              }`}
              onClick={() => handleSelect(file.filename)}
            >
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <h4 className="font-semibold text-gray-900">
                    {file.filename}
                  </h4>
                  <div className="flex gap-4 text-xs text-gray-500 mt-1">
                    <span>大小: {formatFileSize(file.size)}</span>
                    <span>修改时间: {file.modified}</span>
                    {file.record_count !== undefined && (
                      <span>记录数: {file.record_count.toLocaleString()}</span>
                    )}
                  </div>
                </div>
                <div className="flex gap-2">
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={(e) => handleDelete(file.filename, e)}
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
