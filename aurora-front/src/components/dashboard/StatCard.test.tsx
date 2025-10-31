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

import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { StatCard } from './StatCard';

describe('StatCard 组件', () => {
  // 测试基础渲染
  it('应该正确渲染统计卡片', () => {
    render(<StatCard icon="📊" value={100} label="测试数据" />);
    
    // 验证图标
    expect(screen.getByText('📊')).toBeInTheDocument();
    
    // 验证数值
    expect(screen.getByText('100')).toBeInTheDocument();
    
    // 验证标签
    expect(screen.getByText('测试数据')).toBeInTheDocument();
  });

  // 测试数字类型的 value
  it('应该正确显示数字类型的 value', () => {
    render(<StatCard icon="💰" value={12345} label="总收益" />);
    expect(screen.getByText('12345')).toBeInTheDocument();
  });

  // 测试字符串类型的 value
  it('应该正确显示字符串类型的 value', () => {
    render(<StatCard icon="📈" value="99.5%" label="胜率" />);
    expect(screen.getByText('99.5%')).toBeInTheDocument();
  });

  // 测试小数值
  it('应该正确显示小数值', () => {
    render(<StatCard icon="📉" value={3.14159} label="夏普比率" />);
    expect(screen.getByText('3.14159')).toBeInTheDocument();
  });

  // 测试负数值
  it('应该正确显示负数值', () => {
    render(<StatCard icon="⚠️" value={-25.5} label="最大回撤" />);
    expect(screen.getByText('-25.5')).toBeInTheDocument();
  });

  // 测试零值
  it('应该正确显示零值', () => {
    render(<StatCard icon="⭕" value={0} label="待处理任务" />);
    expect(screen.getByText('0')).toBeInTheDocument();
  });

  // 测试不同的图标
  it('应该支持不同的表情符号图标', () => {
    const { rerender } = render(<StatCard icon="🚀" value={10} label="标签1" />);
    expect(screen.getByText('🚀')).toBeInTheDocument();

    rerender(<StatCard icon="⭐" value={20} label="标签2" />);
    expect(screen.getByText('⭐')).toBeInTheDocument();

    rerender(<StatCard icon="🎯" value={30} label="标签3" />);
    expect(screen.getByText('🎯')).toBeInTheDocument();
  });

  // 测试样式类名
  it('应该包含正确的样式类名', () => {
    const { container } = render(<StatCard icon="📊" value={100} label="测试" />);
    const card = container.firstChild as HTMLElement;
    
    // 验证卡片容器样式
    expect(card).toHaveClass('bg-white', 'rounded-lg', 'p-6', 'shadow-sm', 'flex', 'items-center', 'gap-4');
  });

  // 测试长标签文本
  it('应该正确显示长标签文本', () => {
    const longLabel = '这是一个非常长的标签文本用于测试组件的显示能力';
    render(<StatCard icon="📝" value={999} label={longLabel} />);
    expect(screen.getByText(longLabel)).toBeInTheDocument();
  });

  // 测试大数值
  it('应该正确显示大数值', () => {
    render(<StatCard icon="💎" value={1234567890} label="总资产" />);
    expect(screen.getByText('1234567890')).toBeInTheDocument();
  });

  // 测试带有特殊字符的字符串值
  it('应该正确显示带有特殊字符的字符串值', () => {
    render(<StatCard icon="📊" value="$1,234.56" label="金额" />);
    expect(screen.getByText('$1,234.56')).toBeInTheDocument();
  });
});
