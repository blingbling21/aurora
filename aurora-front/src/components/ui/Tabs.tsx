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

import React, { useState } from 'react';

/**
 * Tab 项配置
 */
export interface TabItem {
  // Tab 的唯一标识
  id: string;
  // Tab 显示的标签
  label: string;
  // Tab 图标（可选）
  icon?: string;
  // Tab 内容
  content: React.ReactNode;
}

/**
 * Tabs 组件属性
 */
export interface TabsProps {
  // Tab 项列表
  tabs: TabItem[];
  // 默认激活的 Tab ID
  defaultActiveId?: string;
  // Tab 切换回调
  onTabChange?: (tabId: string) => void;
  // 样式类名
  className?: string;
}

/**
 * Tabs 导航组件
 * 
 * 实现分 Tab 展示不同类型的内容
 * - 支持自定义 Tab 项
 * - 支持图标显示
 * - 响应式设计
 */
export const Tabs: React.FC<TabsProps> = ({
  tabs,
  defaultActiveId,
  onTabChange,
  className = '',
}) => {
  // 当前激活的 Tab
  const [activeTabId, setActiveTabId] = useState<string>(
    defaultActiveId || tabs[0]?.id || ''
  );

  /**
   * 处理 Tab 切换
   */
  const handleTabClick = (tabId: string) => {
    setActiveTabId(tabId);
    onTabChange?.(tabId);
  };

  // 获取当前激活的 Tab 内容
  const activeTab = tabs.find(tab => tab.id === activeTabId);

  if (tabs.length === 0) {
    return null;
  }

  return (
    <div className={`w-full ${className}`}>
      {/* Tab 导航栏 */}
      <div className="border-b border-gray-200 mb-6">
        <nav className="flex space-x-2 overflow-x-auto">
          {tabs.map((tab) => {
            const isActive = tab.id === activeTabId;
            
            return (
              <button
                key={tab.id}
                onClick={() => handleTabClick(tab.id)}
                className={`
                  flex items-center gap-2 px-4 py-3 text-sm font-medium whitespace-nowrap
                  border-b-2 transition-colors
                  ${
                    isActive
                      ? 'border-blue-600 text-blue-600'
                      : 'border-transparent text-gray-600 hover:text-gray-900 hover:border-gray-300'
                  }
                `}
                aria-selected={isActive}
                role="tab"
              >
                {tab.icon && <span className="text-lg">{tab.icon}</span>}
                {tab.label}
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab 内容区域 */}
      <div role="tabpanel" className="w-full">
        {activeTab?.content}
      </div>
    </div>
  );
};

export default Tabs;
