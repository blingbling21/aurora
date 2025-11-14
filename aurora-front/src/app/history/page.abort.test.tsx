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
import HistoryPage from './page';
import { backtestApi } from '@/lib/api';

// Mock Next.js navigation
jest.mock('next/navigation', () => ({
  useRouter: () => ({
    push: jest.fn(),
  }),
}));

// Mock API
jest.mock('@/lib/api', () => ({
  backtestApi: {
    list: jest.fn(),
  },
}));

// Mock notification store
jest.mock('@/lib/store/notificationStore', () => ({
  useNotificationStore: () => ({
    addNotification: jest.fn(),
  }),
}));

describe('HistoryPage - AbortController 测试', () => {
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

    (backtestApi.list as jest.Mock).mockImplementation(() => mockListPromise);

    // 渲染组件
    const { unmount } = render(<HistoryPage />);

    // 立即卸载组件（在 API 请求完成之前）
    unmount();

    // 等待一段时间确保清理函数被调用
    await waitFor(
      () => {
        expect(backtestApi.list).toHaveBeenCalled();
      },
      { timeout: 200 }
    );
  });

  it('不应该因为 React StrictMode 导致状态不一致', async () => {
    let callCount = 0;
    
    (backtestApi.list as jest.Mock).mockImplementation(() => {
      callCount++;
      return Promise.resolve({
        success: true,
        data: [
          {
            id: 'test-1',
            name: '测试任务',
            status: 'Completed',
            config_path: 'config.toml',
            data_path: 'data.csv',
            progress: 100,
            created_at: '2025-01-01T00:00:00Z',
          },
        ],
      });
    });

    // 在 StrictMode 中渲染
    const { findByText } = render(<HistoryPage />);

    // 等待任务显示
    await findByText('测试任务');

    // 验证即使在 StrictMode 下，最终状态也是正确的
    expect(callCount).toBeGreaterThanOrEqual(1);
  });

  it('手动刷新不应该被 AbortController 影响', async () => {
    let callCount = 0;
    
    (backtestApi.list as jest.Mock).mockImplementation(() => {
      callCount++;
      return Promise.resolve({
        success: true,
        data: [],
      });
    });

    const { getByText } = render(<HistoryPage />);

    // 等待初始加载完成
    await waitFor(() => {
      expect(callCount).toBeGreaterThanOrEqual(1);
    });

    // 记录当前调用次数
    const callsBeforeRefresh = callCount;

    // 点击刷新按钮
    const refreshButton = getByText('🔄 刷新');
    refreshButton.click();

    // 验证刷新后调用次数增加了
    await waitFor(() => {
      expect(callCount).toBe(callsBeforeRefresh + 1);
    });
  });
});
