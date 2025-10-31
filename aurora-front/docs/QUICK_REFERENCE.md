# Aurora前端 - Zod & Zustand 快速参考

## 📦 导入

```typescript
// 导入类型和Schema
import { 
  BacktestTask, 
  BacktestTaskSchema,
  validateBacktestTask 
} from '@/types';

// 导入Store
import { 
  useBacktestTaskStore,
  useNotificationStore 
} from '@/lib/store';
```

## 🔍 Zod数据验证

### 验证方法1: safeParse (推荐)
```typescript
const result = BacktestTaskSchema.safeParse(data);

if (result.success) {
  const validData = result.data; // 类型安全的数据
} else {
  console.error(result.error.issues); // 错误详情
}
```

### 验证方法2: 使用验证函数
```typescript
const validation = validateBacktestTask(data);

if (validation.success) {
  const validData = validation.data;
} else {
  const errors = validation.errors; // 错误数组
}
```

### 可用的Schema
- `BacktestTaskSchema` - 回测任务
- `ConfigFileSchema` - 配置文件
- `DataFileSchema` - 数据文件
- `BacktestResultSchema` - 回测结果
- `TradeSchema` - 交易记录
- `NotificationSchema` - 通知消息
- `DataDownloadRequestSchema` - 数据下载请求
- `BacktestConfigSchema` - 回测配置

## 🎯 Zustand状态管理

### 基本使用
```typescript
function MyComponent() {
  // 获取状态和actions
  const { tasks, isLoading, addTask, updateTask } = useBacktestTaskStore();

  return (
    <div>
      {isLoading ? 'Loading...' : `Tasks: ${tasks.length}`}
    </div>
  );
}
```

### 使用选择器 (性能优化)
```typescript
// 只订阅需要的状态
const tasks = useBacktestTaskStore((state) => state.tasks);
const addTask = useBacktestTaskStore((state) => state.addTask);
```

### 可用的Store

#### useBacktestTaskStore
```typescript
const {
  tasks,              // 任务列表
  selectedTaskId,     // 选中的任务ID
  isLoading,          // 加载状态
  error,              // 错误信息
  setTasks,           // 设置任务列表
  addTask,            // 添加任务
  updateTask,         // 更新任务
  deleteTask,         // 删除任务
  selectTask,         // 选择任务
  getSelectedTask,    // 获取选中的任务
} = useBacktestTaskStore();
```

#### useBacktestResultStore
```typescript
const {
  results,            // 结果Map
  currentResultId,    // 当前结果ID
  setResult,          // 设置结果
  getResult,          // 获取结果
  deleteResult,       // 删除结果
  getCurrentResult,   // 获取当前结果
} = useBacktestResultStore();
```

#### useConfigStore
```typescript
const {
  configs,            // 配置列表
  currentConfig,      // 当前配置
  isEditing,          // 编辑状态
  editMode,           // 编辑模式
  addConfig,          // 添加配置
  updateConfig,       // 更新配置
  deleteConfig,       // 删除配置
} = useConfigStore();
```

#### useDataStore
```typescript
const {
  dataFiles,          // 数据文件列表
  isDownloading,      // 下载状态
  downloadProgress,   // 下载进度
  addDataFile,        // 添加数据文件
  startDownload,      // 开始下载
  completeDownload,   // 完成下载
} = useDataStore();
```

#### useNotificationStore
```typescript
const {
  notifications,      // 通知列表
  showSuccess,        // 成功通知
  showError,          // 错误通知
  showInfo,           // 信息通知
  showWarning,        // 警告通知
} = useNotificationStore();
```

## 🚀 实际使用示例

### 示例1: 表单提交与验证
```typescript
function BacktestForm() {
  const { addTask } = useBacktestTaskStore();
  const { showSuccess, showError } = useNotificationStore();

  const handleSubmit = (formData: unknown) => {
    // 验证数据
    const validation = validateBacktestTask(formData);

    if (!validation.success) {
      showError('数据验证失败');
      return;
    }

    // 添加到store
    addTask(validation.data);
    showSuccess('任务创建成功！');
  };

  return <form onSubmit={handleSubmit}>...</form>;
}
```

### 示例2: API数据验证
```typescript
async function fetchTasks() {
  const { setTasks, setError } = useBacktestTaskStore();

  try {
    const response = await fetch('/api/tasks');
    const data = await response.json();

    // 验证每个任务
    const validTasks = data
      .map((item: unknown) => BacktestTaskSchema.safeParse(item))
      .filter((result: any) => result.success)
      .map((result: any) => result.data);

    setTasks(validTasks);
  } catch (error) {
    setError('加载失败');
  }
}
```

### 示例3: 多Store协同
```typescript
function BacktestRunner() {
  const { updateTask } = useBacktestTaskStore();
  const { setResult } = useBacktestResultStore();
  const { showSuccess, showInfo } = useNotificationStore();

  const runBacktest = async (taskId: string) => {
    // 更新状态
    updateTask(taskId, { status: 'running' });
    showInfo('开始执行回测...');

    // 执行回测
    const result = await api.runBacktest(taskId);

    // 保存结果
    setResult(taskId, result);
    updateTask(taskId, { status: 'completed' });
    showSuccess('回测完成！');
  };

  return <button onClick={() => runBacktest('1')}>运行</button>;
}
```

## 📝 最佳实践

### Zod
✅ 使用`safeParse`避免异常  
✅ 提供清晰的错误消息  
✅ 使用类型推断`z.infer<typeof Schema>`  
✅ 组合Schema重用逻辑

### Zustand
✅ 使用选择器优化性能  
✅ 保持Store专注单一职责  
✅ 在actions中处理异步逻辑  
✅ 始终处理错误情况

## 📚 更多文档

- 完整文档: `docs/DATA_TYPES_AND_STATE_MANAGEMENT.md`
- 实现总结: `docs/IMPLEMENTATION_SUMMARY.md`
- 使用示例: `src/lib/store/examples.ts`
- 测试示例: `src/types/schemas.test.ts`, `src/lib/store/backtestTaskStore.test.ts`

## ✅ 测试覆盖率

```
All files: 97.78% Statements | 91.11% Branch | 90.47% Functions | 97.78% Lines
```

所有测试通过: ✅ 154 tests passed
