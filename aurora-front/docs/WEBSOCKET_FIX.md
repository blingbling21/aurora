# WebSocket 连接问题修复

## 问题描述

数据下载功能的 WebSocket 连接失败，显示错误：
```
WebSocket connection to 'ws://localhost:8080/api/ws/data/{task_id}' failed
WebSocket 错误: 1006 (异常关闭)
WebSocket 重连次数已达上限
```

但后端日志显示下载任务实际成功完成。

## 根本原因

**WebSocket URL 路径错误**

- **前端尝试连接**: `ws://localhost:8080/api/ws/data/{task_id}`
- **后端实际路由**: `ws://localhost:8080/ws/data/{task_id}`

### 问题分析

1. **环境变量配置**: `NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api`
2. **URL 构建逻辑**:
   ```typescript
   // 错误的实现
   const apiBaseUrl = 'http://localhost:8080/api';
   const wsBaseUrl = apiBaseUrl.replace(/^http/, 'ws'); 
   // => 'ws://localhost:8080/api'
   const wsUrl = `${wsBaseUrl}/ws/data/${taskId}`;
   // => 'ws://localhost:8080/api/ws/data/{taskId}' ❌
   ```

3. **后端路由配置** (`main.rs`):
   ```rust
   .nest("/api/config", api::config::routes())
   .nest("/api/backtest", api::backtest::routes())
   .nest("/api/data", api::data::routes())
   .nest("/ws", ws::routes())  // WebSocket 路由在根路径
   ```

## 解决方案

修改 `getWebSocketUrl` 函数，移除 `/api` 前缀：

```typescript
function getWebSocketUrl(taskId: string): string {
  // 从环境变量获取 API 基础 URL
  const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:8080/api';
  
  // 移除 /api 后缀，获取基础 URL
  const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
  
  // 将 HTTP(S) 协议转换为 WS(S) 协议
  const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
  
  // 构建 WebSocket URL（后端路由是 /ws/data/{id}）
  return `${wsBaseUrl}/ws/data/${taskId}`;
}
```

### 修复后的 URL 生成流程

```
输入: http://localhost:8080/api
步骤1: 移除 /api => http://localhost:8080
步骤2: 转换协议 => ws://localhost:8080
步骤3: 拼接路径 => ws://localhost:8080/ws/data/{taskId} ✅
```

## 修改的文件

### 1. `src/lib/hooks/useDataDownloadWebSocket.ts`
- 修改 `getWebSocketUrl` 函数
- 添加详细注释说明 URL 构建逻辑

### 2. `src/lib/hooks/useDataDownloadWebSocket.test.ts`
- 添加环境变量设置
- 新增测试用例验证 WebSocket URL 生成

### 3. `src/lib/hooks/websocket-url.test.ts` (新增)
- 专门的 URL 生成逻辑测试
- 覆盖多种场景（http/https、带/不带斜杠等）

## 测试验证

### 单元测试
```bash
npm test -- websocket-url.test.ts
```

测试场景：
- ✅ `http://localhost:8080/api` → `ws://localhost:8080/ws/data/{id}`
- ✅ `https://example.com/api` → `wss://example.com/ws/data/{id}`
- ✅ 带结尾斜杠的处理
- ✅ 不带 `/api` 的基础 URL

### 集成测试
1. 启动后端服务: `cargo run` (在 aurora-web 目录)
2. 启动前端服务: `npm run dev` (在 aurora-front 目录)
3. 访问 `http://localhost:3000/data`
4. 填写表单并提交下载请求
5. 观察 WebSocket 连接状态和进度更新

### 预期结果
- ✅ WebSocket 连接成功
- ✅ 实时显示下载进度
- ✅ 显示已下载/预估总数
- ✅ 下载完成后显示成功通知
- ✅ 自动刷新数据文件列表

## 参考实现

后端静态页面 (`static/js/data.js`) 的正确实现：
```javascript
const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
const wsUrl = `${protocol}//${window.location.host}/ws/data/${taskId}`;
```

这直接使用 `window.location.host`，避免了 `/api` 路径问题。

## 相关问题

### 回测 WebSocket 是否有同样问题？

检查 `src/lib/api/backtest.ts`:
```typescript
static getWebSocketUrl(taskId: string): string {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const host = window.location.host;
  return `${protocol}//${host}/ws/backtest/${encodeURIComponent(taskId)}`;
}
```

回测使用了 `window.location.host`，所以**没有这个问题**。

### 为什么不统一使用 window.location？

考虑因素：
1. **SSR 兼容性**: Next.js 可能在服务端渲染，`window` 不可用
2. **环境配置**: 环境变量允许灵活配置不同环境的后端地址
3. **跨域场景**: 前后端可能部署在不同域名

建议保持当前实现，但确保 URL 构建逻辑正确。

## 最佳实践

### 1. 统一的 URL 构建函数
考虑创建一个通用的 URL 构建工具：
```typescript
// lib/utils/url.ts
export function buildWebSocketUrl(path: string): string {
  const apiBaseUrl = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:8080/api';
  const baseUrl = apiBaseUrl.replace(/\/api\/?$/, '');
  const wsBaseUrl = baseUrl.replace(/^http/, 'ws');
  return `${wsBaseUrl}${path}`;
}

// 使用
const wsUrl = buildWebSocketUrl(`/ws/data/${taskId}`);
```

### 2. 环境变量文档
在 `.env.example` 中明确说明：
```bash
# API 基础 URL - 用于 HTTP 请求
# WebSocket 会自动移除 /api 并转换协议
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

### 3. 添加调试日志
在开发环境打印 WebSocket URL：
```typescript
if (process.env.NODE_ENV === 'development') {
  console.log('[WebSocket] Connecting to:', wsUrl);
}
```

## 总结

- ✅ **问题**: WebSocket URL 包含了错误的 `/api` 前缀
- ✅ **原因**: URL 构建逻辑未考虑后端路由结构
- ✅ **修复**: 移除 `/api` 前缀再构建 WebSocket URL
- ✅ **测试**: 添加单元测试验证 URL 生成逻辑
- ✅ **文档**: 更新实现文档说明修复内容

---

**修复日期**: 2025-11-07  
**影响范围**: 数据下载 WebSocket 连接  
**向后兼容**: 是 ✅
