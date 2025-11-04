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

/**
 * 数据管理 Store 测试
 */

import { renderHook, act } from '@testing-library/react';
import { useDataStore } from './dataStore';
import { DataFile } from '@/types/schemas';

describe('useDataStore', () => {
  // 在每个测试前重置 store 状态
  beforeEach(() => {
    const { result } = renderHook(() => useDataStore());
    act(() => {
      result.current.clearDataFiles();
    });
  });

  describe('初始状态', () => {
    it('应该有正确的初始状态', () => {
      const { result } = renderHook(() => useDataStore());

      expect(result.current.dataFiles).toEqual([]);
      expect(result.current.isDownloading).toBe(false);
      expect(result.current.downloadProgress).toBe(0);
      expect(result.current.currentDownloadFile).toBeNull();
      expect(result.current.isLoading).toBe(false);
      expect(result.current.error).toBeNull();
    });
  });

  describe('setDataFiles', () => {
    it('应该设置数据文件列表', () => {
      const mockFiles: DataFile[] = [
        {
          name: 'btc_usdt_1h.csv',
          path: '/data/btc_usdt_1h.csv',
          size: 1024000,
          lastModified: '2025-01-01T00:00:00Z',
        },
      ];

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.setDataFiles(mockFiles);
      });

      expect(result.current.dataFiles).toEqual(mockFiles);
      expect(result.current.error).toBeNull();
    });
  });

  describe('addDataFile', () => {
    it('应该添加新数据文件', () => {
      const newFile: DataFile = {
        name: 'eth_usdt_4h.csv',
        path: '/data/eth_usdt_4h.csv',
        size: 512000,
        lastModified: '2025-01-02T00:00:00Z',
      };

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.addDataFile(newFile);
      });

      expect(result.current.dataFiles).toHaveLength(1);
      expect(result.current.dataFiles[0]).toEqual(newFile);
    });
  });

  describe('updateDataFile', () => {
    it('应该更新指定的数据文件', () => {
      const file: DataFile = {
        name: 'test.csv',
        path: '/data/test.csv',
        size: 1000,
        lastModified: '2025-01-01T00:00:00Z',
      };

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.addDataFile(file);
      });

      act(() => {
        result.current.updateDataFile('test.csv', {
          size: 2000,
          lastModified: '2025-01-02T00:00:00Z',
        });
      });

      expect(result.current.dataFiles[0].size).toBe(2000);
      expect(result.current.dataFiles[0].lastModified).toBe('2025-01-02T00:00:00Z');
    });
  });

  describe('deleteDataFile', () => {
    it('应该删除指定的数据文件', () => {
      const file: DataFile = {
        name: 'to-delete.csv',
        path: '/data/to-delete.csv',
        size: 1000,
        lastModified: '2025-01-01T00:00:00Z',
      };

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.addDataFile(file);
      });

      expect(result.current.dataFiles).toHaveLength(1);

      act(() => {
        result.current.deleteDataFile('to-delete.csv');
      });

      expect(result.current.dataFiles).toHaveLength(0);
    });
  });

  describe('getDataFile', () => {
    it('应该根据名称获取数据文件', () => {
      const file: DataFile = {
        name: 'test.csv',
        path: '/data/test.csv',
        size: 1000,
        lastModified: '2025-01-01T00:00:00Z',
      };

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.addDataFile(file);
      });

      const foundFile = result.current.getDataFile('test.csv');

      expect(foundFile).toEqual(file);
    });

    it('应该在文件不存在时返回 undefined', () => {
      const { result } = renderHook(() => useDataStore());

      const foundFile = result.current.getDataFile('non-existent.csv');

      expect(foundFile).toBeUndefined();
    });
  });

  describe('下载状态管理', () => {
    it('应该设置下载状态', () => {
      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.setIsDownloading(true);
      });

      expect(result.current.isDownloading).toBe(true);
    });

    it('应该设置下载进度', () => {
      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.setDownloadProgress(50);
      });

      expect(result.current.downloadProgress).toBe(50);
    });

    it('应该设置当前下载文件', () => {
      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.setCurrentDownloadFile('downloading.csv');
      });

      expect(result.current.currentDownloadFile).toBe('downloading.csv');
    });

    it('startDownload 应该设置下载状态', () => {
      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.startDownload('test-file.csv');
      });

      expect(result.current.isDownloading).toBe(true);
      expect(result.current.downloadProgress).toBe(0);
      expect(result.current.currentDownloadFile).toBe('test-file.csv');
    });

    it('completeDownload 应该重置下载状态', () => {
      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.startDownload('test-file.csv');
      });

      expect(result.current.isDownloading).toBe(true);

      act(() => {
        result.current.completeDownload();
      });

      expect(result.current.isDownloading).toBe(false);
      expect(result.current.downloadProgress).toBe(0);
      expect(result.current.currentDownloadFile).toBeNull();
    });
  });

  describe('clearDataFiles', () => {
    it('应该清空所有数据文件和状态', () => {
      const file: DataFile = {
        name: 'test.csv',
        path: '/data/test.csv',
        size: 1000,
        lastModified: '2025-01-01T00:00:00Z',
      };

      const { result } = renderHook(() => useDataStore());

      act(() => {
        result.current.addDataFile(file);
        result.current.startDownload('downloading.csv');
        result.current.setError('Test error');
      });

      act(() => {
        result.current.clearDataFiles();
      });

      expect(result.current.dataFiles).toEqual([]);
      expect(result.current.isDownloading).toBe(false);
      expect(result.current.downloadProgress).toBe(0);
      expect(result.current.currentDownloadFile).toBeNull();
      expect(result.current.error).toBeNull();
    });
  });
});
