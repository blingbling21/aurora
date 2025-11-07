# WebSocket 无限重连问题修复

## 问题描述

在数据下载完成后，WebSocket 连接会无限重连，导致前端不断显示"数据下载成功"的通知。

## 问题分析

### 根本原因

这是一个前后端配合的问题，有两个主要原因：

1. **前端问题**：在接收到 `complete` 或 `error` 消息后关闭 WebSocket 时，没有设置 `manualDisconnectRef.current = true`，导致 `onclose` 处理器认为这是异常断开，触发重连逻辑。

2. **后端问题**：后端在关闭 WebSocket 时没有指定关闭码，默认可能不是 1000（正常关闭），导致前端的 `event.code !== 1000` 条件判断触发重连。

### 问题流程

```
下载完成 → 后端发送 complete 消息 
  → 前端接收并关闭 WebSocket（未设置 manualDisconnectRef）
  → onclose 触发（code !== 1000 或 manualDisconnectRef 未设置）
  → 触发重连逻辑
  → 再次连接成功
  → 后端再次发送 complete 消息
  → 循环往复...
```

## 解决方案

### 前端修复

**文件**: `lib/hooks/useDataDownloadWebSocket.ts`

在 `complete` 和 `error` 消息处理中，关闭连接前设置 `manualDisconnectRef.current = true`：

```typescript
case 'complete':
  if (message.downloaded_count !== undefined) {
    onComplete?.(message.downloaded_count);
  }
  // 标记为手动断开，防止自动重连
  manualDisconnectRef.current = true;
  setTimeout(() => {
    if (wsRef.current) {
      wsRef.current.close();
    }
  }, 100);
  break;

case 'error':
  onError?.(message.message || '下载失败');
  // 同样标记为手动断开
  manualDisconnectRef.current = true;
  setTimeout(() => {
    if (wsRef.current) {
      wsRef.current.close();
    }
  }, 100);
  break;
```

**关键点**：
- `manualDisconnectRef.current = true` 告诉 `onclose` 处理器："这是计划内的断开，不需要重连"
- 必须在调用 `ws.close()` **之前**设置，否则 `onclose` 会先触发

### 后端修复

**文件**: `aurora-web/src/ws/data.rs`

后端在关闭 WebSocket 时显式指定关闭码 1000（正常关闭）：

```rust
// 使用正常关闭码1000
let _ = socket.send(Message::Close(Some(axum::extract::ws::CloseFrame {
    code: 1000,
    reason: "下载完成".into(),
}))).await;
```

**关键点**：
- WebSocket 规范定义了多种关闭码，1000 表示正常关闭
- 不指定关闭码时，不同实现可能使用不同的默认值
- 显式指定 1000 确保前后端对"正常关闭"的理解一致

## 重连逻辑设计

`onclose` 处理器的重连条件：

```typescript
ws.onclose = (event) => {
  wsRef.current = null;
  
  // 只在非手动断开且非正常关闭时尝试重连
  if (!manualDisconnectRef.current && event.code !== 1000) {
    if (reconnectAttemptsRef.current < maxReconnectAttempts) {
      reconnectAttemptsRef.current += 1;
      // ... 重连逻辑
    }
  }
  
  // 重置手动断开标志
  manualDisconnectRef.current = false;
};
```

**重连触发条件**：
1. `!manualDisconnectRef.current` - 不是手动断开
2. `event.code !== 1000` - 不是正常关闭码
3. `reconnectAttemptsRef.current < maxReconnectAttempts` - 未超过重试次数

## 测试验证

### 预期行为

1. 启动下载后，WebSocket 连接成功
2. 实时显示下载进度
3. 下载完成后：
   - 显示一次"数据下载成功"通知
   - WebSocket 连接关闭
   - **不再重连**
4. 刷新数据列表显示新下载的数据

### 测试步骤

1. 启动后端：
```bash
cd aurora-web
cargo run --release
```

2. 启动前端：
```bash
cd aurora-front
npm run dev
```

3. 访问 http://localhost:3000/data

4. 填写下载表单并提交

5. 观察：
   - 浏览器控制台：WebSocket 连接建立
   - 进度条实时更新
   - 下载完成后，控制台显示关闭码 1000
   - **不再有新的连接尝试**

### 调试日志

前端控制台日志：
```
[WebSocket] 连接已建立
[WebSocket] 收到消息: {"type":"connected",...}
[WebSocket] 收到消息: {"type":"progress",...}
[WebSocket] 收到消息: {"type":"complete",...}
[WebSocket] 连接已关闭，关闭码: 1000, 原因: 下载完成
```

后端日志：
```
INFO aurora_web: 数据下载WebSocket连接关闭: <task_id>
```

## 相关文件

- `aurora-front/lib/hooks/useDataDownloadWebSocket.ts` - WebSocket Hook
- `aurora-web/src/ws/data.rs` - 后端 WebSocket 处理
- `aurora-front/docs/WEBSOCKET_FIX.md` - WebSocket URL 修复文档

## 最佳实践

### 前端 WebSocket 断开

在任何**预期的**断开场景中，都应该设置 `manualDisconnectRef = true`：

```typescript
// ✅ 正确：主动断开
manualDisconnectRef.current = true;
ws.close();

// ❌ 错误：会触发重连
ws.close();
```

### 后端 WebSocket 关闭

总是显式指定关闭码，不依赖默认行为：

```rust
// ✅ 正确：指定关闭码
socket.send(Message::Close(Some(CloseFrame {
    code: 1000,
    reason: "正常关闭".into(),
}))).await;

// ❌ 不推荐：依赖默认关闭码
socket.close().await;
```

## 关闭码参考

常用 WebSocket 关闭码：

| 代码 | 含义 | 重连？ |
|------|------|--------|
| 1000 | Normal Closure | ❌ |
| 1001 | Going Away | ✅ |
| 1002 | Protocol Error | ❌ |
| 1003 | Unsupported Data | ❌ |
| 1006 | Abnormal Closure | ✅ |
| 1011 | Internal Error | ✅ |

本项目策略：**只在关闭码 !== 1000 时重连**

## 总结

这次修复展示了前后端协同工作的重要性：

1. **前端**负责正确标记预期断开
2. **后端**负责发送正确的关闭码
3. **两者配合**才能实现可靠的重连机制

修复后的系统能够：
- ✅ 在网络故障时自动重连（最多 5 次）
- ✅ 在任务完成时干净关闭，不重连
- ✅ 区分正常断开和异常断开
- ✅ 提供清晰的日志用于调试
