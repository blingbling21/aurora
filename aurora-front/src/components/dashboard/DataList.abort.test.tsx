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

import { render, waitFor } from '@testing-library/react';
import { DataList } from './DataList';
import { dataApi } from '@/lib/api';

// Mock API
jest.mock('@/lib/api', () => ({
  dataApi: {
    list: jest.fn(),
    delete: jest.fn(),
  },
}));

// Mock notification store
jest.mock('@/lib/store', () => ({
  useNotificationStore: () => ({
    addNotification: jest.fn(),
  }),
}));

describe('DataList - AbortController 测试', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('组件卸载时应该取消正在进行的请求', async () => {
    // 创建一个延迟的 Promise 来模拟慢速 API 请求
    const mockListPromise = new Promise((resolve) => {
      setTimeout(() => {
        resolve({
          success: true,
          data: [],
        });
      }, 100);
    });

    (dataApi.list as jest.Mock).mockImplementation(() => mockListPromise);

    // 渲染组件
    const { unmount } = render(<DataList />);

    // 立即卸载组件（在 API 请求完成之前）
    unmount();

    // 等待一段时间确保清理函数被调用
    await waitFor(
      () => {
        expect(dataApi.list).toHaveBeenCalled();
      },
      { timeout: 200 }
    );
  });

  it('refreshTrigger 变化时应该取消之前的请求', async () => {
    let requestCount = 0;
    
    (dataApi.list as jest.Mock).mockImplementation(() => {
      requestCount++;
      return Promise.resolve({
        success: true,
        data: [],
      });
    });

    // 首次渲染
    const { rerender } = render(<DataList refreshTrigger={0} />);

    // 等待第一次请求完成
    await waitFor(() => {
      expect(requestCount).toBe(1);
    });

    // 更新 refreshTrigger
    rerender(<DataList refreshTrigger={1} />);

    // 等待第二次请求完成
    await waitFor(() => {
      expect(requestCount).toBe(2);
    });
  });

  it('手动刷新不应该被 AbortController 影响', async () => {
    (dataApi.list as jest.Mock).mockResolvedValue({
      success: true,
      data: [
        {
          filename: 'test.csv',
          size: 2048,
          modified: '2025-01-01T00:00:00Z',
        },
      ],
    });

    const { getByText } = render(<DataList />);

    // 等待初始加载
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalledTimes(1);
    });

    // 点击刷新按钮
    const refreshButton = getByText('🔄 刷新');
    refreshButton.click();

    // 验证第二次调用
    await waitFor(() => {
      expect(dataApi.list).toHaveBeenCalledTimes(2);
    });
  });
});
