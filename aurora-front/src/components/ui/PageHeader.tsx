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

interface PageHeaderProps {
  icon?: string;
  title: string;
  description?: string;
  action?: ReactNode;
}

/**
 * 页面头部组件
 * 
 * 显示页面标题、描述和操作按钮
 * 
 * @param {string} icon - 可选的表情符号图标
 * @param {string} title - 页面标题
 * @param {string} description - 页面描述
 * @param {ReactNode} action - 可选的操作按钮或元素
 */
export function PageHeader({ icon, title, description, action }: PageHeaderProps) {
  return (
    <div className="flex items-start justify-between mb-8">
      <div>
        <h2 className="text-3xl font-bold text-gray-900">
          {icon && <span className="mr-2">{icon}</span>}
          {title}
        </h2>
        {description && (
          <p className="mt-1 text-sm text-gray-500">{description}</p>
        )}
      </div>
      {action && <div>{action}</div>}
    </div>
  );
}
