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

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { NAV_MENU_ITEMS } from '@/constants';

/**
 * 侧边栏导航组件
 * 
 * 显示应用的主导航菜单,包含 logo 和导航菜单项
 */
export function Sidebar() {
  // 获取当前路径用于高亮激活状态
  const pathname = usePathname();

  return (
    <aside className="w-[260px] bg-white border-r border-gray-200 flex flex-col">
      {/* Logo 区域 */}
      <div className="px-6 py-6 border-b border-gray-200">
        <h1 className="text-2xl font-bold text-blue-500 mb-1">🌟 Aurora</h1>
        <p className="text-xs text-gray-500">量化交易回测平台</p>
      </div>

      {/* 导航菜单 */}
      <nav className="flex-1 py-4">
        <ul className="space-y-0">
          {NAV_MENU_ITEMS.map((item) => {
            // 判断是否为激活状态
            const isActive = pathname === item.href;

            return (
              <li key={item.id}>
                <Link
                  href={item.href}
                  className={`
                    flex items-center gap-3 px-6 py-3 transition-colors
                    ${
                      isActive
                        ? 'bg-gray-50 text-blue-500 border-r-[3px] border-blue-500'
                        : 'text-gray-500 hover:bg-gray-50 hover:text-gray-900'
                    }
                  `}
                >
                  <span className="text-xl">{item.icon}</span>
                  <span className="font-medium">{item.label}</span>
                </Link>
              </li>
            );
          })}
        </ul>
      </nav>
    </aside>
  );
}
