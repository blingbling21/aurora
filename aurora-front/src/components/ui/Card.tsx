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

import { ReactNode } from 'react';
import { cn } from '@/lib/utils';

interface CardProps {
  children?: ReactNode;
  className?: string;
  title?: string;
}

/**
 * 卡片组件
 * 
 * 提供一个带有白色背景、圆角和阴影的容器组件
 * 
 * @param {ReactNode} children - 卡片内容
 * @param {string} className - 额外的 CSS 类名
 * @param {string} title - 可选的卡片标题
 */
export function Card({ children, className, title }: CardProps) {
  return (
    <div className={cn('bg-white rounded-lg p-6 shadow-sm', className)}>
      {title && (
        <h3 className="text-lg font-semibold text-gray-900 mb-4">{title}</h3>
      )}
      {children}
    </div>
  );
}
