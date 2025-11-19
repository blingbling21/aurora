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

/**
 * 数据采样工具函数
 * 
 * 用于大数据集的可视化优化，避免因数据点过多导致渲染性能问题或堆栈溢出
 */

/**
 * 下采样数据点，保留关键特征
 * 
 * 使用 LTTB (Largest Triangle Three Buckets) 算法的简化版本
 * 该算法能够在保留数据趋势和关键特征的同时减少数据点数量
 * 
 * @param data - 原始数据数组
 * @param targetPoints - 目标数据点数量
 * @returns 采样后的数据数组
 */
export function downsampleData<T extends Record<string, unknown>>(
  data: T[],
  targetPoints: number
): T[] {
  // 如果数据量小于等于目标点数，直接返回
  if (!data || data.length <= targetPoints) {
    return data;
  }

  // 始终保留第一个和最后一个点
  const sampled: T[] = [data[0]];
  
  // 计算桶大小
  const bucketSize = (data.length - 2) / (targetPoints - 2);
  
  // 对于每个桶，选择一个代表性点
  for (let i = 0; i < targetPoints - 2; i++) {
    // 计算当前桶的范围
    const bucketStart = Math.floor(i * bucketSize) + 1;
    const bucketEnd = Math.floor((i + 1) * bucketSize) + 1;
    
    // 在桶内找到最具代表性的点（这里使用简化算法：选择中间点）
    const bucketMid = Math.floor((bucketStart + bucketEnd) / 2);
    
    // 确保索引不越界
    if (bucketMid < data.length) {
      sampled.push(data[bucketMid]);
    }
  }
  
  // 添加最后一个点
  sampled.push(data[data.length - 1]);
  
  return sampled;
}

/**
 * 智能采样：根据数据量自动决定是否需要采样
 * 
 * @param data - 原始数据数组
 * @param maxPoints - 最大允许的数据点数量（默认 1000）
 * @returns 采样后的数据数组
 */
export function smartSample<T extends Record<string, unknown>>(
  data: T[],
  maxPoints: number = 1000
): T[] {
  // 如果数据量不超过阈值，直接返回
  if (!data || data.length <= maxPoints) {
    return data;
  }
  
  // 否则进行下采样
  return downsampleData(data, maxPoints);
}

/**
 * 均匀采样：按固定间隔采样数据点
 * 
 * 这是最简单的采样方法，适用于数据分布均匀的场景
 * 
 * @param data - 原始数据数组
 * @param targetPoints - 目标数据点数量
 * @returns 采样后的数据数组
 */
export function uniformSample<T>(data: T[], targetPoints: number): T[] {
  // 如果数据量小于等于目标点数，直接返回
  if (!data || data.length <= targetPoints) {
    return data;
  }

  const sampled: T[] = [];
  const step = data.length / targetPoints;
  
  for (let i = 0; i < targetPoints; i++) {
    const index = Math.floor(i * step);
    sampled.push(data[index]);
  }
  
  // 确保包含最后一个点
  if (sampled[sampled.length - 1] !== data[data.length - 1]) {
    sampled[sampled.length - 1] = data[data.length - 1];
  }
  
  return sampled;
}
