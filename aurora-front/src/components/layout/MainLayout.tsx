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

import { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { NotificationContainer } from '@/components/ui';

interface MainLayoutProps {
  children: ReactNode;
}

/**
 * 主布局组件
 * 
 * 提供应用的主要布局结构,包含侧边栏、主内容区域和通知容器
 * 
 * @param {ReactNode} children - 主内容区域的子组件
 */
export function MainLayout({ children }: MainLayoutProps) {
  return (
    <div className="flex h-screen overflow-hidden">
      {/* 侧边栏 */}
      <Sidebar />

      {/* 主内容区 */}
      <main className="flex-1 overflow-y-auto bg-gray-50 p-8">
        {children}
      </main>

      {/* 全局通知容器 */}
      <NotificationContainer />
    </div>
  );
}
