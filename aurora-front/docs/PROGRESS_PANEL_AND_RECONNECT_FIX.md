# 数据下载进度显示和重连问题修复总结

## 问题描述

用户报告了两个关键问题：

1. **进度面板不消失**：数据下载完成后，下载进度显示没有消失，应该在完成后不久自动隐藏
2. **页面切换后重复通知**：在完成下载后切换到其他页面，再切换回数据管理页面时：
   - 页面会再次提示"数据下载成功"
   - 控制台出现 WebSocket 错误

## 问题根因分析

### 问题 1：进度面板不消失

**根本原因**：
- `page.tsx` 中的进度显示条件为：`{activeTask && ...}`
- `dataDownloadStore` 的 `completeDownload` 虽然设置了 3秒后隐藏 `showProgressPanel`
- 但页面组件并未使用 `showProgressPanel` 状态来控制显示
- 导致只要 `activeTask` 存在，进度面板就一直显示

**代码位置**：
```tsx
// 错误的显示条件
{activeTask && (
  <div className="mt-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
    {/* 进度显示 */}
  </div>
)}
```

### 问题 2：页面切换后重复通知和报错

**根本原因**：
1. 当用户切换页面后再返回时，React 组件重新挂载
2. `useDataDownloadWebSocket` Hook 的 `useEffect` 再次执行
3. 检测到 `activeTask.taskId` 存在，自动连接 WebSocket
4. 后端对于已完成的任务，立即发送 `complete` 消息
5. 前端 `onComplete` 回调再次触发，显示重复的成功通知
6. WebSocket 关闭导致控制台报错

**执行流程**：
```
页面切换回来
  → 组件重新挂载
  → useEffect 检测到 activeTask?.taskId
  → 自动连接 WebSocket
  → 后端发送已完成的 complete 消息
  → onComplete 再次调用
  → addNotification("数据下载成功") ← 重复通知
  → WebSocket 关闭 ← 控制台报错
```

**关键问题**：
- Hook 没有检查任务是否已完成就自动连接
- Store 中的 `activeTask` 在完成后一直保留
- 没有机制防止已完成任务的重连

## 修复方案

### 修复 1：使用 showProgressPanel 控制进度显示

**修改文件**：`src/app/data/page.tsx`

```tsx
// 修复后：使用 showProgressPanel 和 activeTask 共同控制
{activeTask && showProgressPanel && (
  <div className="mt-6 p-4 bg-gray-50 rounded-lg border border-gray-200">
    {/* 进度显示 */}
  </div>
)}
```

**效果**：
- 下载完成后 3 秒，`showProgressPanel` 自动设置为 `false`
- 进度面板自动消失，符合预期

### 修复 2：添加 isTaskCompleted 参数防止重连

**修改文件**：`src/lib/hooks/useDataDownloadWebSocket.ts`

#### 2.1 扩展 Hook 接口

```typescript
export interface UseDataDownloadWebSocketOptions extends DataDownloadWsHandlers {
  autoConnect?: boolean;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
  isTaskCompleted?: boolean;  // ← 新增：标记任务是否已完成
}
```

#### 2.2 在 connect() 中检查任务状态

```typescript
const connect = useCallback(() => {
  // 如果没有任务 ID 或任务已完成，不连接
  if (!taskId || isTaskCompleted) {
    return;
  }
  // ...连接逻辑
}, [taskId, isTaskCompleted, ...]);
```

#### 2.3 在 useEffect 中传递完成状态

```typescript
useEffect(() => {
  if (autoConnect && taskId && !isTaskCompleted) {
    connect();
  }
  return () => {
    disconnect();
  };
}, [taskId, autoConnect, isTaskCompleted, connect, disconnect]);
```

#### 2.4 在页面中传递任务状态

**修改文件**：`src/app/data/page.tsx`

```typescript
const { connectionStatus } = useDataDownloadWebSocket(activeTask?.taskId || null, {
  autoConnect: true,
  isTaskCompleted: activeTask?.status === 'Completed' || activeTask?.status === 'Failed',
  onConnected: () => { ... },
  onProgress: (progress) => { ... },
  onComplete: (downloadedCount) => { ... },
  onError: (error) => { ... },
});
```

**效果**：
- 页面切换回来时，Hook 检测到 `isTaskCompleted = true`
- 不会重新连接 WebSocket
- 避免重复通知和报错

### 修复 3：自动清理已完成的任务

**修改文件**：`src/lib/store/dataDownloadStore.ts`

```typescript
completeDownload: (downloadedCount) => {
  // ...设置 completedTask

  // 3秒后自动隐藏进度面板
  setTimeout(() => {
    const currentTask = get().activeTask;
    if (currentTask?.taskId === completedTask.taskId) {
      set({ showProgressPanel: false });
    }
  }, 3000);

  // 10秒后自动清理活动任务，防止页面刷新时重连
  setTimeout(() => {
    const currentTask = get().activeTask;
    if (currentTask?.taskId === completedTask.taskId && currentTask.status === 'Completed') {
      set({ activeTask: null });
    }
  }, 10000);
},
```

**时间线**：
- 0s: 下载完成
- 3s: 隐藏进度面板
- 10s: 清理 `activeTask`

**效果**：
- 10 秒后 `activeTask` 被清空
- 即使用户在 10 秒后切换页面，也不会重连（因为没有 taskId）
- 同时保留了 `taskHistory` 中的记录

### 修复 4：修复 WebSocket 重连逻辑中的循环引用

**问题**：在 `connect()` 的 `onclose` 回调中直接调用 `connect()` 会产生循环依赖警告

**解决方案**：在 `onclose` 中直接创建新的 WebSocket 实例，复用事件处理器

```typescript
ws.onclose = (event) => {
  // ...
  if (!manualDisconnectRef.current && event.code !== 1000 && !isTaskCompleted) {
    if (reconnectCountRef.current < maxReconnectAttempts) {
      reconnectCountRef.current += 1;
      
      reconnectTimeoutRef.current = setTimeout(() => {
        if (!manualDisconnectRef.current && taskId && !isTaskCompleted) {
          // 直接创建新实例，避免调用 connect()
          const retryUrl = getWebSocketUrl(taskId);
          const retryWs = new WebSocket(retryUrl);
          wsRef.current = retryWs;
          
          // 复用相同的事件处理器
          retryWs.onopen = ws.onopen;
          retryWs.onmessage = ws.onmessage;
          retryWs.onerror = ws.onerror;
          retryWs.onclose = ws.onclose;
        }
      }, reconnectInterval);
    }
  }
};
```

## 测试更新

### 新增测试用例

**文件**：`src/lib/hooks/useDataDownloadWebSocket.test.ts`

```typescript
it('应该在任务已完成时不建立连接', async () => {
  const onConnected = jest.fn();

  const { result } = renderHook(() =>
    useDataDownloadWebSocket(mockTaskId, {
      autoConnect: true,
      isTaskCompleted: true,  // ← 标记为已完成
      onConnected,
    })
  );

  await new Promise((resolve) => setTimeout(resolve, 200));

  // 验证未建立连接
  expect(result.current.connectionStatus).toBe('disconnected');
  expect(onConnected).not.toHaveBeenCalled();
  expect(mockWebSocketInstance).toBeNull();
});
```

**文件**：`src/lib/store/dataDownloadStore.test.ts`

```typescript
describe('自动清理', () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it('应该在3秒后自动隐藏进度面板', () => {
    const { result } = renderHook(() => useDataDownloadStore());

    act(() => {
      result.current.startDownload('task-123', 'test.csv');
      result.current.completeDownload(1000);
    });

    expect(result.current.showProgressPanel).toBe(true);

    act(() => {
      jest.advanceTimersByTime(3000);
    });

    expect(result.current.showProgressPanel).toBe(false);
  });

  it('应该在10秒后自动清理已完成的任务', () => {
    const { result } = renderHook(() => useDataDownloadStore());

    act(() => {
      result.current.startDownload('task-123', 'test.csv');
      result.current.completeDownload(1000);
    });

    expect(result.current.activeTask).toBeDefined();

    act(() => {
      jest.advanceTimersByTime(10000);
    });

    expect(result.current.activeTask).toBeNull();
  });
});
```

### 测试结果

```
✅ 所有测试通过
Test Suites: 50 passed, 50 total
Tests:       772 passed, 772 total
```

## 文件修改清单

| 文件 | 修改内容 | 行数 |
|------|----------|------|
| `src/app/data/page.tsx` | 修复进度显示条件，传递 isTaskCompleted | ~2 行 |
| `src/lib/hooks/useDataDownloadWebSocket.ts` | 添加 isTaskCompleted 参数，修复重连逻辑 | ~30 行 |
| `src/lib/store/dataDownloadStore.ts` | 添加 10 秒自动清理逻辑 | ~8 行 |
| `src/lib/hooks/useDataDownloadWebSocket.test.ts` | 新增已完成任务测试用例 | ~50 行 |
| `src/lib/store/dataDownloadStore.test.ts` | 新增自动清理测试用例 | ~60 行 |

## 预期效果

### 场景 1：正常下载流程

1. ✅ 用户提交下载请求
2. ✅ 显示进度面板，实时更新进度
3. ✅ 下载完成，显示"数据下载成功"通知
4. ✅ 3 秒后进度面板自动消失
5. ✅ 10 秒后清理 activeTask

### 场景 2：下载完成后切换页面

1. ✅ 下载完成（状态为 Completed）
2. ✅ 用户切换到其他页面
3. ✅ 用户切换回数据页面
4. ✅ **不会**重新连接 WebSocket（因为 isTaskCompleted = true）
5. ✅ **不会**显示重复的成功通知
6. ✅ **不会**产生控制台报错

### 场景 3：10秒后切换页面

1. ✅ 下载完成
2. ✅ 等待 10 秒，activeTask 被清空
3. ✅ 用户切换页面后返回
4. ✅ 因为没有 activeTask，不会有任何 WebSocket 操作

## 设计亮点

### 1. 多层防护机制

- **第一层**：Hook 参数 `isTaskCompleted` 检查
- **第二层**：`connect()` 函数中的任务状态检查
- **第三层**：10 秒自动清理 `activeTask`

### 2. 优雅的生命周期管理

```
下载中 → 完成 → 3s 隐藏面板 → 10s 清理任务
```

### 3. 保留历史记录

- `activeTask` 清空不影响 `taskHistory`
- 用户仍可查看最近 10 次下载记录

### 4. 避免循环引用

- 不在 callback 中直接调用自身
- 使用内联 WebSocket 创建 + 事件处理器复用

## 注意事项

### 1. 时间窗口设计

- **3 秒隐藏面板**：给用户足够时间看到完成状态
- **10 秒清理任务**：平衡用户体验和防止重连

### 2. 边界情况处理

- ❌ 用户在 3 秒内手动关闭面板：需要添加关闭按钮（待实现）
- ✅ 用户在 10 秒内多次切换页面：`isTaskCompleted` 参数阻止重连
- ✅ 新任务创建前旧任务未清理：`taskId` 检查确保正确性

### 3. 性能考虑

- ✅ 使用 `setTimeout` 而非轮询
- ✅ 在 `completeDownload` 中使用闭包捕获 `taskId`，避免清理错误任务
- ✅ `useCallback` 优化 Hook 性能

## 相关文档

- [WEBSOCKET_FIX.md](./WEBSOCKET_FIX.md) - WebSocket URL 修复文档
- [WEBSOCKET_RECONNECTION_FIX.md](./WEBSOCKET_RECONNECTION_FIX.md) - WebSocket 重连修复文档
- [DATA_DOWNLOAD_IMPLEMENTATION.md](./DATA_DOWNLOAD_IMPLEMENTATION.md) - 数据下载功能实现文档

## 总结

本次修复解决了数据下载功能中的两个关键用户体验问题：

1. **进度面板消失** - 通过正确使用 `showProgressPanel` 状态实现
2. **防止重复通知** - 通过 `isTaskCompleted` 参数和自动清理机制实现

所有修改都遵循了项目约定：
- ✅ 符合 TDD 原则，先写测试再实现
- ✅ 使用 Zustand 管理全局状态
- ✅ 详细的代码注释和文档
- ✅ 高内聚低耦合的代码结构

修复后的系统更加健壮可靠，提供了更好的用户体验。
