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

import { z } from 'zod';
import {
  BacktestTaskSchema,
  BacktestResultSchema,
  ConfigFileSchema,
  DataFileSchema,
  NotificationSchema,
  DataDownloadRequestSchema,
  BacktestConfigSchema,
} from './schemas';

/**
 * 验证回测任务数据
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateBacktestTask(data: unknown) {
  try {
    const validatedData = BacktestTaskSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证回测结果数据
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateBacktestResult(data: unknown) {
  try {
    const validatedData = BacktestResultSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证配置文件数据
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateConfigFile(data: unknown) {
  try {
    const validatedData = ConfigFileSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证数据文件信息
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateDataFile(data: unknown) {
  try {
    const validatedData = DataFileSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证通知消息数据
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateNotification(data: unknown) {
  try {
    const validatedData = NotificationSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证数据下载请求
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateDataDownloadRequest(data: unknown) {
  try {
    const validatedData = DataDownloadRequestSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 验证回测配置
 * @param data 待验证的数据
 * @returns 验证结果，包含成功标志和数据/错误信息
 */
export function validateBacktestConfig(data: unknown) {
  try {
    const validatedData = BacktestConfigSchema.parse(data);
    return { success: true as const, data: validatedData };
  } catch (error) {
    if (error instanceof z.ZodError) {
      return { success: false as const, errors: error.issues };
    }
    return { success: false as const, errors: [{ message: '未知验证错误' }] };
  }
}

/**
 * 安全解析数据
 * 使用safeParse而不是parse，不会抛出异常
 * @param schema Zod schema
 * @param data 待验证的数据
 * @returns 解析结果
 */
export function safeParseData<T extends z.ZodTypeAny>(schema: T, data: unknown) {
  return schema.safeParse(data);
}

/**
 * 格式化验证错误信息
 * @param errors Zod错误数组
 * @returns 格式化后的错误信息字符串
 */
export function formatValidationErrors(errors: z.ZodIssue[]): string {
  return errors
    .map((err) => {
      const path = err.path.join('.');
      return path ? `${path}: ${err.message}` : err.message;
    })
    .join('; ');
}
