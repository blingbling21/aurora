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

import { render, screen, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import { Notification } from './Notification';
import { Notification as NotificationType } from '@/types';

// 模拟定时器
jest.useFakeTimers();

describe('Notification 组件', () => {
  // 创建模拟通知数据
  const mockNotificationSuccess: NotificationType = {
    id: '1',
    type: 'success',
    message: '操作成功',
    duration: 3000,
  };

  const mockNotificationError: NotificationType = {
    id: '2',
    type: 'error',
    message: '操作失败',
    duration: 3000,
  };

  const mockNotificationInfo: NotificationType = {
    id: '3',
    type: 'info',
    message: '提示信息',
    duration: 3000,
  };

  const mockNotificationWarning: NotificationType = {
    id: '4',
    type: 'warning',
    message: '警告信息',
    duration: 3000,
  };

  const mockOnClose = jest.fn();

  // 每个测试前重置 mock
  beforeEach(() => {
    mockOnClose.mockClear();
    jest.clearAllTimers();
  });

  // 所有测试后恢复真实定时器
  afterAll(() => {
    jest.useRealTimers();
  });

  // 测试基础渲染
  it('应该正确渲染通知消息', () => {
    render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    expect(screen.getByText('操作成功')).toBeInTheDocument();
  });

  // 测试成功类型样式
  it('应该为成功通知应用正确的样式', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass('border-l-green-500');
  });

  // 测试错误类型样式
  it('应该为错误通知应用正确的样式', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationError}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass('border-l-red-500');
  });

  // 测试信息类型样式
  it('应该为信息通知应用正确的样式', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationInfo}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass('border-l-blue-500');
  });

  // 测试警告类型样式
  it('应该为警告通知应用正确的样式', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationWarning}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass('border-l-yellow-500');
  });

  // 测试关闭按钮
  it('应该包含关闭按钮', () => {
    render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    const closeButton = screen.getByText('✕');
    expect(closeButton).toBeInTheDocument();
  });

  // 测试手动关闭
  it('应该在点击关闭按钮时触发 onClose', async () => {
    const user = userEvent.setup({ delay: null });
    
    render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    const closeButton = screen.getByText('✕');
    await user.click(closeButton);
    
    // 等待动画延迟
    jest.advanceTimersByTime(300);
    
    expect(mockOnClose).toHaveBeenCalledWith('1');
  });

  // 测试自动关闭
  it('应该在指定时间后自动关闭', () => {
    render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    // 快进到自动关闭时间
    act(() => {
      jest.advanceTimersByTime(3000);
    });
    
    // 再快进动画时间
    act(() => {
      jest.advanceTimersByTime(300);
    });
    
    expect(mockOnClose).toHaveBeenCalledWith('1');
  });

  // 测试自定义持续时间
  it('应该支持自定义持续时间', () => {
    const customNotification: NotificationType = {
      ...mockNotificationSuccess,
      duration: 5000,
    };
    
    render(
      <Notification
        notification={customNotification}
        onClose={mockOnClose}
      />
    );
    
    // 快进 3 秒，不应该关闭
    act(() => {
      jest.advanceTimersByTime(3000);
    });
    expect(mockOnClose).not.toHaveBeenCalled();
    
    // 再快进 2 秒
    act(() => {
      jest.advanceTimersByTime(2000);
    });
    
    // 快进动画时间
    act(() => {
      jest.advanceTimersByTime(300);
    });
    
    expect(mockOnClose).toHaveBeenCalledWith('1');
  });

  // 测试没有 duration 时使用默认值
  it('应该在没有 duration 时使用默认值 3000ms', () => {
    const notificationWithoutDuration: NotificationType = {
      id: '5',
      type: 'info',
      message: '测试消息',
    };
    
    render(
      <Notification
        notification={notificationWithoutDuration}
        onClose={mockOnClose}
      />
    );
    
    // 快进默认时间
    act(() => {
      jest.advanceTimersByTime(3000);
      jest.advanceTimersByTime(300);
    });
    
    expect(mockOnClose).toHaveBeenCalledWith('5');
  });

  // 测试通知宽度
  it('应该有最小和最大宽度限制', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass('min-w-[320px]', 'max-w-md');
  });

  // 测试长消息
  it('应该正确显示长消息', () => {
    const longMessage = '这是一个非常长的通知消息，用于测试通知组件在处理较长文本时的显示效果和布局适应能力。';
    const longNotification: NotificationType = {
      ...mockNotificationSuccess,
      message: longMessage,
    };
    
    render(
      <Notification
        notification={longNotification}
        onClose={mockOnClose}
      />
    );
    
    expect(screen.getByText(longMessage)).toBeInTheDocument();
  });

  // 测试样式类名
  it('应该包含所有必要的样式类名', () => {
    const { container } = render(
      <Notification
        notification={mockNotificationSuccess}
        onClose={mockOnClose}
      />
    );
    
    const notification = container.firstChild as HTMLElement;
    expect(notification).toHaveClass(
      'bg-white',
      'rounded-lg',
      'shadow-xl',
      'border-l-4',
      'p-4',
      'transition-all',
      'duration-300'
    );
  });
});
