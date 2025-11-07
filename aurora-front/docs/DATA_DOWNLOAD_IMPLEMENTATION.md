# 数据下载功能实现文档

## 概述

已成功实现数据管理页面的数据下载功能，包括发送请求、WebSocket 实时进度显示等完整流程。

## 实现的功能

### 1. 类型定义 (`types/api.ts`)

添加了以下类型：

- `DownloadTaskResponse`: 下载任务创建后的响应
- `DownloadStatus`: 下载状态枚举（Pending, Downloading, Completed, Failed）
- `DownloadProgressMessage`: WebSocket 进度消息类型

### 2. API 服务更新 (`lib/api/data.ts`)

更新了 `DataService.fetch()` 方法：
- 返回类型改为 `ApiResponse<DownloadTaskResponse>`
- 包含 `task_id`, `message`, `filename` 字段

### 3. WebSocket Hook (`lib/hooks/useDataDownloadWebSocket.ts`)

创建了专门的数据下载 WebSocket Hook：

**功能特性：**
- 自动连接和重连机制
- 实时接收下载进度更新
- 支持连接状态监控
- 错误处理和恢复

**使用示例：**

```typescript
const { progress, connectionStatus, connect, disconnect } = useDataDownloadWebSocket(
  taskId,
  {
    onProgress: (progress) => {
      console.log('进度:', progress.progress);
    },
    onComplete: (downloadedCount) => {
      console.log('完成:', downloadedCount);
    },
    onError: (error) => {
      console.error('错误:', error);
    },
  }
);
```

### 4. 状态管理 (`lib/store/dataDownloadStore.ts`)

创建了下载状态管理 Store：

**核心方法：**
- `startDownload(taskId, filename)`: 开始下载任务
- `updateProgress(...)`: 更新进度
- `completeDownload(count)`: 完成下载
- `failDownload(error)`: 标记失败
- `cancelDownload()`: 取消下载
- `getTask(taskId)`: 查询任务信息

**使用示例：**

```typescript
const {
  activeTask,
  startDownload,
  updateProgress,
  completeDownload,
  failDownload,
} = useDataDownloadStore();
```

### 5. 数据管理页面集成 (`app/data/page.tsx`)

完整实现了数据下载流程：

**流程说明：**

1. **用户填写表单**
   - 选择交易所、交易对、时间周期
   - 选择开始和结束日期
   - 可选填文件名（自动生成）

2. **提交下载请求**
   - 验证表单数据
   - 调用 API 创建下载任务
   - 获取 `task_id`

3. **建立 WebSocket 连接**
   - 使用 `task_id` 连接到 WebSocket
   - 自动开始接收进度更新

4. **实时进度显示**
   - 显示下载状态（准备中、下载中、完成、失败）
   - 显示进度百分比
   - 显示已下载/预估总数
   - 进度条动画效果

5. **下载完成**
   - 显示成功通知
   - 自动刷新数据列表
   - 3秒后隐藏进度面板

## 后端接口说明

### 1. 创建下载任务

**端点:** `POST /api/data/fetch`

**请求体:**
```json
{
  "exchange": "binance",
  "symbol": "BTCUSDT",
  "interval": "1h",
  "start_date": "2024-01-01",
  "end_date": "2024-12-31",
  "filename": "btc_1h_data.csv" // 可选
}
```

**响应:**
```json
{
  "success": true,
  "data": {
    "task_id": "uuid-string",
    "message": "数据下载任务已创建",
    "filename": "binance_btcusdt_1h_20240101_to_20241231.csv"
  }
}
```

### 2. WebSocket 进度推送

**端点:** `ws://localhost:8080/api/ws/data/{task_id}`

**消息类型:**

1. **连接成功:**
```json
{
  "type": "connected",
  "task_id": "uuid",
  "message": "已连接到数据下载进度推送"
}
```

2. **进度更新:**
```json
{
  "type": "progress",
  "task_id": "uuid",
  "status": "Downloading",
  "progress": 50.5,
  "progress_message": "已获取 500 / 1000 条数据",
  "downloaded_count": 500,
  "estimated_total": 1000
}
```

3. **下载完成:**
```json
{
  "type": "complete",
  "task_id": "uuid",
  "status": "Completed",
  "message": "下载完成，共获取 1000 条数据",
  "downloaded_count": 1000
}
```

4. **下载错误:**
```json
{
  "type": "error",
  "error": "下载失败: 网络错误"
}
```

## 测试

已创建单元测试：

1. **WebSocket Hook 测试** (`useDataDownloadWebSocket.test.ts`)
   - 连接和断开测试
   - 进度消息处理测试
   - 完成和错误处理测试

2. **Store 测试** (`dataDownloadStore.test.ts`)
   - 下载任务生命周期测试
   - 状态更新测试
   - 历史记录管理测试

运行测试：
```bash
npm test
```

## 使用指南

### 前端开发者

1. **启动前端开发服务器：**
```bash
cd aurora-front
npm run dev
```

2. **访问数据管理页面：**
   - 打开 `http://localhost:3000/data`
   - 填写表单并点击"开始下载"
   - 观察实时进度更新

### 后端开发者

确保后端服务器已启动并配置正确：

```bash
cd aurora-web
cargo run
```

后端应该监听在 `http://localhost:8080`

## 注意事项

1. **环境变量配置**
   - 确保 `.env.local` 中配置了正确的 `NEXT_PUBLIC_API_BASE_URL`
   - 默认值: `http://localhost:8080/api`

2. **WebSocket 连接**
   - WebSocket URL 自动从 HTTP URL 转换
   - 支持自动重连（最多5次，间隔3秒）

3. **错误处理**
   - 所有错误都会显示用户友好的提示
   - 通过通知系统显示错误消息

4. **性能考虑**
   - 进度更新频率: 每500ms
   - 历史记录限制: 最多保留10条
   - 完成后自动隐藏: 3秒延迟

## 下一步优化建议

1. **功能增强：**
   - 支持暂停/恢复下载
   - 支持批量下载
   - 下载进度持久化（刷新页面后恢复）

2. **用户体验：**
   - 添加下载速度显示
   - 添加剩余时间估算
   - 支持下载历史查看

3. **性能优化：**
   - 实现虚拟滚动（大量历史记录）
   - 优化 WebSocket 消息处理
   - 添加数据缓存机制

## 相关文件

### 新增文件
- `src/types/api.ts` - 类型定义更新
- `src/lib/hooks/useDataDownloadWebSocket.ts` - WebSocket Hook
- `src/lib/hooks/useDataDownloadWebSocket.test.ts` - Hook 测试
- `src/lib/store/dataDownloadStore.ts` - 状态管理
- `src/lib/store/dataDownloadStore.test.ts` - Store 测试

### 修改文件
- `src/app/data/page.tsx` - 数据管理页面
- `src/lib/api/data.ts` - API 服务
- `src/lib/hooks/index.ts` - Hook 导出
- `src/lib/store/index.ts` - Store 导出

## 技术栈

- **React 19**: UI 框架
- **Next.js 16**: 应用框架
- **TypeScript**: 类型安全
- **Zustand**: 状态管理
- **Zod**: 数据验证
- **WebSocket**: 实时通信
- **Jest**: 单元测试

---

**实现完成日期:** 2025-11-07
**遵循项目约定:** Apache 2.0 License, TDD, 高内聚低耦合
