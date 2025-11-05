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
 * TOML配置文件解析和转换工具函数
 */

import { AuroraConfig, AuroraConfigSchema } from '@/types/config-schema';

/**
 * 解析TOML文本为配置对象
 * 注意: 由于TOML解析需要额外的库,这里先提供基础实现
 * 实际使用时需要安装 @iarna/toml 或其他TOML解析库
 * 
 * @param tomlText - TOML格式的配置文本
 * @returns 解析后的配置对象
 */
export async function parseTOML(tomlText: string): Promise<AuroraConfig> {
  try {
    // 动态导入TOML解析库
    const TOML = await import('@iarna/toml');
    const parsed = TOML.parse(tomlText);
    
    // 转换对象,移除Symbol类型的键
    const normalized = JSON.parse(JSON.stringify(parsed));
    
    // 使用Zod验证解析结果
    const validated = AuroraConfigSchema.parse(normalized);
    return validated;
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`TOML解析失败: ${error.message}`);
    }
    throw new Error('TOML解析失败: 未知错误');
  }
}

/**
 * 将配置对象转换为TOML文本
 * 
 * @param config - 配置对象
 * @returns TOML格式的文本
 */
export async function stringifyTOML(config: AuroraConfig): Promise<string> {
  try {
    // 先验证配置对象
    AuroraConfigSchema.parse(config);
    
    // 动态导入TOML序列化库
    const TOML = await import('@iarna/toml');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return TOML.stringify(config as any);
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`TOML生成失败: ${error.message}`);
    }
    throw new Error('TOML生成失败: 未知错误');
  }
}

/**
 * 从文件读取TOML配置
 * 
 * @param file - File对象
 * @returns 解析后的配置对象
 */
export async function readTOMLFile(file: File): Promise<AuroraConfig> {
  // 读取文件内容
  const text = await file.text();
  
  // 解析TOML
  return parseTOML(text);
}

/**
 * 验证TOML文本是否有效
 * 
 * @param tomlText - TOML格式的配置文本
 * @returns 验证结果对象
 */
export async function validateTOML(tomlText: string): Promise<{
  valid: boolean;
  errors: string[];
  config?: AuroraConfig;
}> {
  try {
    const config = await parseTOML(tomlText);
    return {
      valid: true,
      errors: [],
      config,
    };
  } catch (error) {
    return {
      valid: false,
      errors: [error instanceof Error ? error.message : '验证失败'],
    };
  }
}

/**
 * 格式化TOML文本(美化输出)
 * 
 * @param tomlText - TOML格式的配置文本
 * @returns 格式化后的TOML文本
 */
export async function formatTOML(tomlText: string): Promise<string> {
  try {
    const config = await parseTOML(tomlText);
    return stringifyTOML(config);
  } catch {
    // 如果解析失败,返回原文本
    return tomlText;
  }
}
