# Aurora 前端数据类型和状态管理

本文档介绍Aurora前端项目中使用Zod进行数据验证和Zustand进行状态管理的方案。

## 目录

- [数据类型定义(Zod)](#数据类型定义zod)
- [状态管理(Zustand)](#状态管理zustand)
- [使用示例](#使用示例)
- [测试](#测试)

## 数据类型定义(Zod)

### 为什么使用Zod?

Zod是一个TypeScript优先的数据模式声明和验证库,提供了以下优势:

- **类型安全**: 自动推断TypeScript类型
- **运行时验证**: 在运行时验证数据结构
- **详细错误信息**: 提供清晰的验证错误消息
- **易于组合**: 可以组合复杂的数据模式

### 文件结构

```
src/types/
├── schemas.ts        # Zod schema定义
├── validators.ts     # 验证函数
├── index.ts         # 导出所有类型和验证函数
└── schemas.test.ts  # 测试文件
```

### 已定义的Schema

1. **BacktestTaskSchema** - 回测任务
2. **ConfigFileSchema** - 配置文件
3. **DataFileSchema** - 数据文件
4. **BacktestMetricsSchema** - 回测指标
5. **TradeSchema** - 交易记录
6. **BacktestResultSchema** - 回测结果
7. **NotificationSchema** - 通知消息
8. **DataDownloadRequestSchema** - 数据下载请求
9. **BacktestConfigSchema** - 回测配置

### 基本用法

#### 1. 导入Schema和类型

```typescript
import { 
  BacktestTaskSchema, 
  type BacktestTask 
} from '@/types';
```

#### 2. 验证数据

```typescript
// 方法1: 使用safeParse (推荐)
const result = BacktestTaskSchema.safeParse(data);
if (result.success) {
  // 数据有效
  const validData = result.data;
} else {
  // 数据无效
  console.error(result.error.issues);
}

// 方法2: 使用验证函数
import { validateBacktestTask } from '@/types';

const validation = validateBacktestTask(data);
if (validation.success) {
  const validData = validation.data;
} else {
  const errors = validation.errors;
}
```

#### 3. 类型推断

```typescript
import { type BacktestTask } from '@/types';

// TypeScript会自动推断类型
const task: BacktestTask = {
  id: '1',
  name: 'Test',
  status: 'pending',
  // ...
};
```

## 状态管理(Zustand)

### 为什么使用Zustand?

Zustand是一个轻量级的React状态管理库,提供:

- **简单API**: 易于学习和使用
- **TypeScript支持**: 完整的类型支持
- **无需Context**: 不需要Provider包裹
- **性能优化**: 只在必要时重新渲染

### 文件结构

```
src/lib/store/
├── backtestTaskStore.ts       # 回测任务状态
├── backtestResultStore.ts     # 回测结果状态
├── configStore.ts             # 配置管理状态
├── dataStore.ts               # 数据管理状态
├── notificationStore.ts       # 通知状态
├── index.ts                   # 导出所有store
├── examples.ts                # 使用示例
└── backtestTaskStore.test.ts  # 测试文件
```

### 可用的Store

#### 1. useBacktestTaskStore

管理回测任务的状态。

**状态:**
- `tasks`: 任务列表
- `selectedTaskId`: 当前选中的任务ID
- `isLoading`: 加载状态
- `error`: 错误信息

**Actions:**
- `setTasks`: 设置任务列表
- `addTask`: 添加任务
- `updateTask`: 更新任务
- `deleteTask`: 删除任务
- `selectTask`: 选择任务
- `setLoading`: 设置加载状态
- `setError`: 设置错误信息
- `clearTasks`: 清空所有任务
- `getSelectedTask`: 获取选中的任务

#### 2. useBacktestResultStore

管理回测结果的状态。

**状态:**
- `results`: 结果映射表
- `currentResultId`: 当前查看的结果ID
- `isLoading`: 加载状态
- `error`: 错误信息

**Actions:**
- `setResult`: 设置结果
- `getResult`: 获取结果
- `deleteResult`: 删除结果
- `setCurrentResult`: 设置当前查看的结果
- `getCurrentResult`: 获取当前查看的结果
- `setLoading`: 设置加载状态
- `setError`: 设置错误信息
- `clearResults`: 清空所有结果

#### 3. useConfigStore

管理配置文件的状态。

**状态:**
- `configs`: 配置列表
- `currentConfig`: 当前编辑的配置
- `isEditing`: 是否正在编辑
- `editMode`: 编辑模式('form' | 'text')
- `isLoading`: 加载状态
- `error`: 错误信息

**Actions:**
- `setConfigs`: 设置配置列表
- `addConfig`: 添加配置
- `updateConfig`: 更新配置
- `deleteConfig`: 删除配置
- `setCurrentConfig`: 设置当前编辑的配置
- `getConfig`: 获取配置
- `setIsEditing`: 设置编辑状态
- `setEditMode`: 设置编辑模式
- `setLoading`: 设置加载状态
- `setError`: 设置错误信息
- `clearConfigs`: 清空所有配置

#### 4. useDataStore

管理数据文件的状态。

**状态:**
- `dataFiles`: 数据文件列表
- `isDownloading`: 是否正在下载
- `downloadProgress`: 下载进度(0-100)
- `currentDownloadFile`: 当前下载的文件名
- `isLoading`: 加载状态
- `error`: 错误信息

**Actions:**
- `setDataFiles`: 设置数据文件列表
- `addDataFile`: 添加数据文件
- `updateDataFile`: 更新数据文件
- `deleteDataFile`: 删除数据文件
- `getDataFile`: 获取数据文件
- `setIsDownloading`: 设置下载状态
- `setDownloadProgress`: 设置下载进度
- `setCurrentDownloadFile`: 设置当前下载文件
- `startDownload`: 开始下载
- `completeDownload`: 完成下载
- `setLoading`: 设置加载状态
- `setError`: 设置错误信息
- `clearDataFiles`: 清空所有数据文件

#### 5. useNotificationStore

管理全局通知消息的状态。

**状态:**
- `notifications`: 通知列表
- `maxNotifications`: 最大通知数量

**Actions:**
- `addNotification`: 添加通知
- `removeNotification`: 删除通知
- `clearNotifications`: 清空所有通知
- `showSuccess`: 显示成功通知
- `showError`: 显示错误通知
- `showInfo`: 显示信息通知
- `showWarning`: 显示警告通知

### 基本用法

#### 1. 在组件中使用Store

```typescript
'use client';

import { useBacktestTaskStore } from '@/lib/store';

export function MyComponent() {
  // 获取状态和actions
  const { tasks, isLoading, addTask } = useBacktestTaskStore();

  // 使用状态
  return (
    <div>
      {isLoading ? 'Loading...' : `Tasks: ${tasks.length}`}
    </div>
  );
}
```

#### 2. 使用选择器优化性能

```typescript
// 只订阅需要的状态
const tasks = useBacktestTaskStore((state) => state.tasks);
const addTask = useBacktestTaskStore((state) => state.addTask);
```

#### 3. 多个Store协同工作

```typescript
import { useBacktestTaskStore, useNotificationStore } from '@/lib/store';

export function MyComponent() {
  const { addTask } = useBacktestTaskStore();
  const { showSuccess, showError } = useNotificationStore();

  const handleCreateTask = async () => {
    try {
      // 创建任务
      addTask(newTask);
      showSuccess('任务创建成功');
    } catch (error) {
      showError('任务创建失败');
    }
  };
}
```

## 使用示例

详细的使用示例请参考 `src/lib/store/examples.ts` 文件,包含:

1. 使用Zod进行数据验证
2. 在React组件中使用Zustand状态管理
3. 使用通知store显示消息
4. 结合Zod和Zustand进行表单验证
5. 多个Store协同工作
6. Store的选择器模式

## 测试

### 运行测试

```bash
npm test
```

### 测试覆盖率

```bash
npm test -- --coverage
```

### 测试文件

- `src/types/schemas.test.ts` - Schema验证测试
- `src/lib/store/backtestTaskStore.test.ts` - Store测试

## 最佳实践

### Zod

1. **始终使用safeParse**: 避免抛出异常
2. **提供清晰的错误消息**: 在schema中定义错误消息
3. **使用类型推断**: 使用`z.infer`自动推断类型
4. **组合Schema**: 使用`.extend()`、`.pick()`、`.omit()`等方法

### Zustand

1. **使用选择器**: 只订阅需要的状态,避免不必要的重渲染
2. **保持Store简单**: 每个Store专注于一个领域
3. **异步操作**: 在actions中处理异步逻辑
4. **错误处理**: 始终处理可能的错误情况

## 更新日志

### v1.0.0 (2025-01-31)

- ✅ 创建Zod schemas和验证函数
- ✅ 实现Zustand stores
- ✅ 添加测试文件
- ✅ 添加使用示例
- ✅ 添加文档

## 许可证

Apache 2.0 License - Copyright 2025 blingbling21
