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

import * as React from 'react';
import { format } from 'date-fns';
import { zhCN } from 'date-fns/locale';
import { CalendarIcon } from 'lucide-react';

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/Button';
import { Calendar } from '@/components/ui/calendar';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';

/**
 * DatePicker 组件属性
 */
export interface DatePickerProps {
  /** 当前选中的日期 */
  date?: Date;
  /** 日期变化回调 */
  onDateChange?: (date: Date | undefined) => void;
  /** 占位文本 */
  placeholder?: string;
  /** 是否必填 */
  required?: boolean;
  /** 自定义类名 */
  className?: string;
  /** 是否禁用 */
  disabled?: boolean;
}

/**
 * 日期选择器组件
 * 
 * 基于 shadcn 的 Calendar 和 Popover 组件封装的日期选择器
 * 支持日期选择和格式化显示
 * 
 * @example
 * ```tsx
 * const [date, setDate] = useState<Date>();
 * 
 * <DatePicker
 *   date={date}
 *   onDateChange={setDate}
 *   placeholder="选择日期"
 *   required
 * />
 * ```
 */
export function DatePicker({
  date,
  onDateChange,
  placeholder = '选择日期',
  required = false,
  className,
  disabled = false,
}: DatePickerProps) {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="secondary"
          className={cn(
            'w-full justify-start text-left font-normal border border-gray-300 bg-white text-gray-900 hover:bg-gray-50',
            !date && 'text-gray-400',
            className
          )}
          disabled={disabled}
        >
          {/* 日历图标 */}
          <CalendarIcon className="mr-2 h-4 w-4" />
          
          {/* 显示选中的日期或占位文本 */}
          {date ? (
            format(date, 'PPP', { locale: zhCN })
          ) : (
            <span>{placeholder}</span>
          )}
          
          {/* 必填标记 */}
          {required && !date && <span className="ml-1 text-red-500">*</span>}
        </Button>
      </PopoverTrigger>
      
      <PopoverContent className="w-auto p-0" align="start">
        <Calendar
          mode="single"
          selected={date}
          onSelect={onDateChange}
          initialFocus
          locale={zhCN}
        />
      </PopoverContent>
    </Popover>
  );
}
