# Aurora Front - 项目更新说明

## 更新日期
2025年11月3日

## 更新内容

本次更新为 Aurora Front 项目添加了完整的 API 客户端和动态功能，所有代码严格遵循项目约定。

## 新增文件

### API 相关
- `src/types/api.ts` - API 类型定义（使用 Zod）
- `src/lib/api/client.ts` - API 客户端基础设施
- `src/lib/api/config.ts` - 配置管理 API
- `src/lib/api/data.ts` - 数据管理 API
- `src/lib/api/backtest.ts` - 回测管理 API
- `src/lib/api/index.ts` - API 统一导出

### Hooks
- `src/lib/hooks/useBacktestWebSocket.ts` - WebSocket Hook
- `src/lib/hooks/index.ts` - Hooks 统一导出

### 工具函数
- `src/lib/utils/format.ts` - 格式化工具函数

### 示例和文档
- `src/components/examples/ApiExamples.tsx` - API 使用示例
- `docs/API_USAGE_GUIDE.md` - API 使用指南
- `docs/API_INTEGRATION_SUMMARY.md` - API 集成总结
- `docs/QUICK_START.md` - 快速开始指南

### 测试
- `src/lib/api/client.test.ts` - API 客户端测试
- `src/lib/utils/format.test.ts` - 格式化工具测试

## 修改文件

- `src/types/index.ts` - 添加了 API 类型导出

## 核心功能

### 1. API 客户端
- ✅ 类型安全的 HTTP 请求封装
- ✅ 统一的错误处理
- ✅ 请求超时控制
- ✅ Zod 类型验证

### 2. API 服务
- ✅ 配置管理（增删改查、验证）
- ✅ 数据管理（列表、下载、删除）
- ✅ 回测管理（启动、查询、结果获取）

### 3. WebSocket
- ✅ 自动连接和重连
- ✅ 心跳保持
- ✅ 消息类型分发
- ✅ 连接状态管理

### 4. 工具函数
- ✅ 文件大小格式化
- ✅ 日期时间格式化
- ✅ 百分比和货币格式化
- ✅ 任务状态处理
- ✅ 防抖和节流

## 使用方式

### 基础用法

```typescript
import { api } from '@/lib/api';

// 调用 API
const response = await api.config.list();
if (response.success && response.data) {
  console.log(response.data);
}
```

### WebSocket

```typescript
import { useBacktestWebSocket } from '@/lib/hooks';

const { status, lastMessage } = useBacktestWebSocket(taskId, {
  onStatusUpdate: (progress, status) => {
    console.log(`进度: ${progress}%`);
  },
});
```

### 格式化

```typescript
import { formatFileSize, formatDate } from '@/lib/utils/format';

formatFileSize(1024);  // "1.00 KB"
formatDate(dateString); // "2024/01/01 00:00"
```

## 环境配置

在 `.env.local` 中配置 API 基础 URL（可选）：

```bash
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

## 测试

运行测试：

```bash
npm test
```

运行特定测试：

```bash
npm test client.test
npm test format.test
```

## 与后端对接

所有 API 端点与 aurora-web 项目的后端路由一一对应：

- `/api/config` - 配置管理
- `/api/data` - 数据管理
- `/api/backtest` - 回测管理
- `/ws/backtest/:id` - WebSocket 实时更新

## 代码质量

- ✅ Apache 2.0 许可证头
- ✅ TypeScript 严格模式
- ✅ 详细的 JSDoc 注释
- ✅ 完整的单元测试
- ✅ ESLint 代码检查通过
- ✅ 高内聚低耦合设计

## 下一步建议

1. **集成到页面**：将 API 功能集成到现有的页面组件中
2. **添加缓存**：考虑使用 React Query 或 SWR
3. **添加通知**：集成 toast 通知系统
4. **添加拦截器**：实现请求/响应拦截器
5. **扩展测试**：添加更多集成测试

## 参考文档

- 📖 [快速开始](./docs/QUICK_START.md)
- 📖 [API 使用指南](./docs/API_USAGE_GUIDE.md)
- 📖 [API 集成总结](./docs/API_INTEGRATION_SUMMARY.md)
- 💻 [使用示例](./src/components/examples/ApiExamples.tsx)

## 版本信息

- 项目版本：0.1.0
- 更新内容：API 客户端和动态功能集成
- 测试覆盖：核心功能已覆盖

## 贡献者

- blingbling21

## 许可证

Apache License 2.0

---

**注意**：所有新增代码都已包含 Apache 2.0 许可证头，符合项目约定。
