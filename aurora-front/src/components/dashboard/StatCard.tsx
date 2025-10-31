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

interface StatCardProps {
  icon: string;
  value: number | string;
  label: string;
}

/**
 * 统计卡片组件
 * 
 * 显示统计数据,包括图标、数值和标签
 * 
 * @param {string} icon - 表情符号图标
 * @param {number | string} value - 统计数值
 * @param {string} label - 统计标签
 */
export function StatCard({ icon, value, label }: StatCardProps) {
  return (
    <div className="bg-white rounded-lg p-6 shadow-sm flex items-center gap-4">
      {/* 图标区域 */}
      <div className="flex items-center justify-center w-16 h-16 bg-gray-50 rounded-xl text-4xl">
        {icon}
      </div>

      {/* 数据区域 */}
      <div className="flex-1">
        <div className="text-3xl font-bold text-gray-900">{value}</div>
        <div className="text-sm text-gray-500 mt-1">{label}</div>
      </div>
    </div>
  );
}
