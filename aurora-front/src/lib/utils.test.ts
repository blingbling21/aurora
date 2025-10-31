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

import { cn } from './utils';

describe('cn 工具函数', () => {
  // 测试单个类名
  it('应该正确处理单个类名', () => {
    expect(cn('text-red-500')).toBe('text-red-500');
  });

  // 测试多个类名
  it('应该正确合并多个类名', () => {
    const result = cn('text-red-500', 'bg-blue-500');
    expect(result).toContain('text-red-500');
    expect(result).toContain('bg-blue-500');
  });

  // 测试条件类名
  it('应该正确处理条件类名', () => {
    expect(cn('base', true && 'active')).toContain('base');
    expect(cn('base', true && 'active')).toContain('active');
    expect(cn('base', false && 'inactive')).not.toContain('inactive');
  });

  // 测试 undefined 和 null
  it('应该忽略 undefined 和 null', () => {
    expect(cn('text-red-500', undefined, null)).toBe('text-red-500');
  });

  // 测试空字符串
  it('应该忽略空字符串', () => {
    expect(cn('text-red-500', '')).toBe('text-red-500');
  });

  // 测试 Tailwind 冲突类名合并
  it('应该正确合并冲突的 Tailwind 类名', () => {
    // tailwind-merge 应该保留后面的类名
    const result = cn('text-red-500', 'text-blue-500');
    expect(result).toBe('text-blue-500');
  });

  // 测试 padding 冲突
  it('应该正确处理 padding 冲突', () => {
    const result = cn('p-4', 'p-8');
    expect(result).toBe('p-8');
  });

  // 测试 margin 冲突
  it('应该正确处理 margin 冲突', () => {
    const result = cn('m-2', 'm-6');
    expect(result).toBe('m-6');
  });

  // 测试不同方向的 padding
  it('应该保留不同方向的 padding', () => {
    const result = cn('px-4', 'py-2');
    expect(result).toContain('px-4');
    expect(result).toContain('py-2');
  });

  // 测试背景色冲突
  it('应该正确处理背景色冲突', () => {
    const result = cn('bg-red-500', 'bg-blue-500');
    expect(result).toBe('bg-blue-500');
  });

  // 测试文本颜色冲突
  it('应该正确处理文本颜色冲突', () => {
    const result = cn('text-gray-500', 'text-black');
    expect(result).toBe('text-black');
  });

  // 测试数组形式
  it('应该支持数组形式的类名', () => {
    const result = cn(['text-red-500', 'bg-blue-500']);
    expect(result).toContain('text-red-500');
    expect(result).toContain('bg-blue-500');
  });

  // 测试对象形式
  it('应该支持对象形式的条件类名', () => {
    const result = cn({
      'text-red-500': true,
      'bg-blue-500': true,
      'hidden': false,
    });
    expect(result).toContain('text-red-500');
    expect(result).toContain('bg-blue-500');
    expect(result).not.toContain('hidden');
  });

  // 测试混合形式
  it('应该支持混合形式的参数', () => {
    const result = cn(
      'base-class',
      ['array-class-1', 'array-class-2'],
      { 'object-class': true, 'disabled': false },
      true && 'conditional-class'
    );
    expect(result).toContain('base-class');
    expect(result).toContain('array-class-1');
    expect(result).toContain('array-class-2');
    expect(result).toContain('object-class');
    expect(result).not.toContain('disabled');
    expect(result).toContain('conditional-class');
  });

  // 测试复杂的样式组合
  it('应该正确处理复杂的样式组合', () => {
    const result = cn(
      'flex',
      'items-center',
      'justify-between',
      'p-4',
      'bg-white',
      'rounded-lg',
      'shadow-sm'
    );
    expect(result).toContain('flex');
    expect(result).toContain('items-center');
    expect(result).toContain('justify-between');
    expect(result).toContain('p-4');
    expect(result).toContain('bg-white');
    expect(result).toContain('rounded-lg');
    expect(result).toContain('shadow-sm');
  });

  // 测试响应式类名
  it('应该保留响应式类名', () => {
    const result = cn('md:flex', 'lg:grid', 'xl:block');
    expect(result).toContain('md:flex');
    expect(result).toContain('lg:grid');
    expect(result).toContain('xl:block');
  });

  // 测试伪类
  it('应该保留伪类样式', () => {
    const result = cn('hover:bg-blue-500', 'focus:ring-2', 'active:scale-95');
    expect(result).toContain('hover:bg-blue-500');
    expect(result).toContain('focus:ring-2');
    expect(result).toContain('active:scale-95');
  });

  // 测试暗黑模式
  it('应该保留暗黑模式类名', () => {
    const result = cn('bg-white', 'dark:bg-gray-900');
    expect(result).toContain('bg-white');
    expect(result).toContain('dark:bg-gray-900');
  });

  // 测试空参数
  it('应该处理空参数', () => {
    expect(cn()).toBe('');
  });

  // 测试只有 falsy 值
  it('应该处理只有 falsy 值的情况', () => {
    expect(cn(false, null, undefined, '')).toBe('');
  });

  // 测试重复类名
  it('应该去除重复的类名', () => {
    const result = cn('text-red-500', 'bg-blue-500', 'text-red-500');
    // 应该只包含一次 text-red-500
    const matches = result.match(/text-red-500/g);
    expect(matches?.length).toBe(1);
  });
});
