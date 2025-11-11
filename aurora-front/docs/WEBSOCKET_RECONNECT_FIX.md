# WebSocket 重连问题修复文档

## 问题描述

回测任务执行完成后,WebSocket 连接会不断重连,并重复显示"任务已完成"的提示,形成无限循环。

## 根本原因分析

1. **初始实现使用 `taskCompletedRef` 标志**,在收到 `final` 消息时设置为 `true`
2. 但这个标志在某些情况下无法有效阻止重连,特别是当 `event.wasClean` 为 `false` 时
3. 缺少从父组件传递任务完成状态的机制,导致 WebSocket 内部无法准确判断任务状态

## 解决方案

参考了数据下载页面 (`useDataDownloadWebSocket`) 的成功实践,采用以下方案:

### 1. 添加 `isTaskCompleted` 参数

在 `UseWebSocketOptions` 接口中添加新参数:

```typescript
export interface UseWebSocketOptions extends WsMessageHandlers {
  // ... 其他参数
  // 任务是否已完成(用于防止重连)
  isTaskCompleted?: boolean;
}
```

### 2. 使用 `manualDisconnectRef` 替代 `taskCompletedRef`

```typescript
// 旧实现
const taskCompletedRef = useRef(false);

// 新实现
const manualDisconnectRef = useRef(false);
```

`manualDisconnectRef` 更准确地反映了断开连接的意图,包括:
- 收到 `final` 消息后的主动断开
- 用户手动调用 `disconnect` 方法
- 任务完成后的正常关闭

### 3. 在连接逻辑中检查任务状态

```typescript
const connect = useCallback(() => {
  // 如果没有任务 ID 或任务已完成,不连接
  if (!taskId || isTaskCompleted) {
    return;
  }
  
  // 重置手动断开标志
  manualDisconnectRef.current = false;
  // ... 连接逻辑
}, [taskId, isTaskCompleted, ...]);
```

### 4. 在关闭事件中综合判断

```typescript
ws.onclose = (event) => {
  // 如果是手动断开或任务已完成,不进行重连
  if (manualDisconnectRef.current || isTaskCompleted) {
    console.log('✅ 任务已完成或手动断开,不再重连');
    return;
  }
  
  // 仅在非正常关闭且未达到最大重连次数时重连
  if (!event.wasClean && reconnectCountRef.current < maxReconnectAttempts) {
    // 重连逻辑
  }
};
```

### 5. 在收到 final 消息时标记手动断开

```typescript
case 'final':
  console.log('🏁 收到最终消息，准备关闭连接');
  // 标记为手动断开,避免触发重连逻辑
  manualDisconnectRef.current = true;
  onComplete?.(message.data);
  if (wsRef.current) {
    wsRef.current.close(1000, '任务已完成');
  }
  break;
```

### 6. 在页面组件中管理任务完成状态

```typescript
// 添加状态
const [isTaskCompleted, setIsTaskCompleted] = useState(false);

// 传递给 WebSocket Hook
useBacktestWebSocket(currentTaskId, {
  autoConnect: true,
  isTaskCompleted,
  // ...
});

// 在状态更新和完成回调中设置
onStatusUpdate: (progressValue, status) => {
  if (status === 'completed' || status === 'failed') {
    setIsTaskCompleted(true);
  }
},
onComplete: (data) => {
  setIsTaskCompleted(true);
},

// 启动新任务时重置
const handleStartBacktest = async () => {
  setIsTaskCompleted(false);
  // ...
};
```

## 修改的文件

1. **`src/lib/hooks/useBacktestWebSocket.ts`**
   - 添加 `isTaskCompleted` 参数
   - 使用 `manualDisconnectRef` 替代 `taskCompletedRef`
   - 在 `connect`、`onclose`、`final` 消息处理中正确处理任务完成状态

2. **`src/app/backtest/page.tsx`**
   - 添加 `isTaskCompleted` 状态
   - 在 WebSocket 回调中更新完成状态
   - 在启动新任务时重置完成状态

3. **`src/lib/hooks/useBacktestWebSocket.test.ts`**
   - 添加 `isTaskCompleted=true` 时不连接的测试
   - 更新现有测试用例以验证新逻辑

## 测试结果

✅ **24 个测试用例全部通过**
- 基本功能测试: 2/2
- 连接管理测试: 6/6  
- 消息处理测试: 6/6
- 发送消息测试: 2/2
- 心跳机制测试: 2/2
- 错误处理测试: 2/2
- **任务完成处理测试: 4/4** (包括新增测试)

✅ **代码覆盖率: 91.94%**

✅ **关键日志验证:**
```
✅ 任务已完成或手动断开,不再重连
```

## 关键改进点

1. **双重保险**: 同时使用 `manualDisconnectRef` 和 `isTaskCompleted` 确保任务完成后不重连
2. **状态传递**: 从父组件传递任务状态,避免 Hook 内部状态管理的局限性
3. **清晰语义**: `manualDisconnectRef` 比 `taskCompletedRef` 更准确地表达断开意图
4. **参考成功案例**: 完全遵循数据下载 WebSocket 的成功实践

## 后续建议

1. 考虑将 WebSocket 连接逻辑抽象为更通用的 Hook,统一管理所有 WebSocket 连接
2. 添加更详细的日志,帮助调试连接状态
3. 考虑添加连接超时机制

## 参考

- 数据下载 WebSocket 实现: `src/lib/hooks/useDataDownloadWebSocket.ts`
- 数据管理页面示例: `src/app/data/page.tsx`
