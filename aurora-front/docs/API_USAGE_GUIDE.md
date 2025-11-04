# Aurora Front - API 集成使用指南

本文档介绍如何在 Aurora Front 项目中使用 API 客户端和相关功能。

## 目录结构

```
src/
├── lib/
│   ├── api/                    # API 客户端
│   │   ├── client.ts          # 基础 HTTP 客户端
│   │   ├── config.ts          # 配置管理 API
│   │   ├── data.ts            # 数据管理 API
│   │   ├── backtest.ts        # 回测管理 API
│   │   └── index.ts           # 统一导出
│   ├── hooks/                 # 自定义 Hooks
│   │   ├── useBacktestWebSocket.ts  # WebSocket Hook
│   │   └── index.ts
│   └── utils/
│       └── format.ts          # 格式化工具函数
├── types/
│   └── api.ts                 # API 类型定义（使用 Zod）
└── components/
    └── examples/
        └── ApiExamples.tsx    # 使用示例
```

## 快速开始

### 1. 导入 API 客户端

```typescript
import { api } from '@/lib/api';
```

### 2. 调用 API

#### 配置管理

```typescript
// 获取配置列表
const response = await api.config.list();
if (response.success && response.data) {
  console.log(response.data);
}

// 获取单个配置
const configResponse = await api.config.get('config.toml');

// 创建配置
await api.config.create({
  filename: 'new_config.toml',
  content: '...'
});

// 更新配置
await api.config.update('config.toml', {
  content: '...'
});

// 删除配置
await api.config.delete('config.toml');

// 验证配置
const validateResponse = await api.config.validate('...');
```

#### 数据管理

```typescript
// 获取数据文件列表
const response = await api.data.list();

// 获取数据文件内容
const dataResponse = await api.data.get('data.csv');

// 删除数据文件
await api.data.delete('data.csv');

// 获取历史数据
await api.data.fetch({
  exchange: 'binance',
  symbol: 'BTCUSDT',
  interval: '1h',
  start_date: '2024-01-01',
  end_date: '2024-12-31',
});

// 生成文件名
const filename = api.data.generateFilename({
  exchange: 'binance',
  symbol: 'BTCUSDT',
  interval: '1h',
  startDate: '2024-01-01',
  endDate: '2024-12-31',
});
```

#### 回测管理

```typescript
// 获取任务列表
const response = await api.backtest.list();

// 启动回测任务
const startResponse = await api.backtest.start({
  name: '测试任务',
  config_path: 'config.toml',
  data_path: 'data.csv',
});

// 获取任务结果
const resultResponse = await api.backtest.getResult(taskId);

// 删除任务
await api.backtest.delete(taskId);
```

### 3. 使用 WebSocket 监听回测进度

```typescript
import { useBacktestWebSocket } from '@/lib/hooks';

function BacktestProgress({ taskId }: { taskId: string }) {
  const { status, lastMessage, isConnected } = useBacktestWebSocket(taskId, {
    autoConnect: true,
    onStatusUpdate: (progress, status) => {
      console.log(`进度: ${progress}%, 状态: ${status}`);
    },
    onComplete: (data) => {
      console.log('任务完成:', data);
    },
    onError: (error) => {
      console.error('错误:', error);
    },
  });

  return (
    <div>
      <p>连接状态: {status}</p>
      <p>是否已连接: {isConnected ? '是' : '否'}</p>
      {lastMessage && (
        <pre>{JSON.stringify(lastMessage, null, 2)}</pre>
      )}
    </div>
  );
}
```

## 在组件中使用

### 基础示例：加载配置列表

```typescript
'use client';

import { useEffect, useState } from 'react';
import { api } from '@/lib/api';
import type { ConfigListItem } from '@/types/api';

export function ConfigList() {
  const [configs, setConfigs] = useState<ConfigListItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadConfigs = async () => {
      setLoading(true);
      setError(null);
      
      try {
        const response = await api.config.list();
        if (response.success && response.data) {
          setConfigs(response.data);
        } else {
          setError(response.error || '加载失败');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : '加载失败');
      } finally {
        setLoading(false);
      }
    };

    loadConfigs();
  }, []);

  if (loading) return <div>加载中...</div>;
  if (error) return <div>错误: {error}</div>;

  return (
    <ul>
      {configs.map((config) => (
        <li key={config.filename}>{config.filename}</li>
      ))}
    </ul>
  );
}
```

### 使用 Zustand Store（推荐）

如果你需要在多个组件间共享状态，建议使用 Zustand Store：

```typescript
// 在 store 中
import { create } from 'zustand';
import { api } from '@/lib/api';
import type { ConfigListItem } from '@/types/api';

interface ConfigState {
  configs: ConfigListItem[];
  loading: boolean;
  error: string | null;
  loadConfigs: () => Promise<void>;
}

export const useConfigStore = create<ConfigState>((set) => ({
  configs: [],
  loading: false,
  error: null,
  
  loadConfigs: async () => {
    set({ loading: true, error: null });
    try {
      const response = await api.config.list();
      if (response.success && response.data) {
        set({ configs: response.data, loading: false });
      } else {
        throw new Error(response.error || '加载失败');
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : '加载失败';
      set({ error: message, loading: false });
    }
  },
}));

// 在组件中使用
function ConfigList() {
  const { configs, loading, error, loadConfigs } = useConfigStore();

  useEffect(() => {
    loadConfigs();
  }, [loadConfigs]);

  // ...
}
```

## 格式化工具函数

```typescript
import { 
  formatFileSize, 
  formatDate, 
  formatPercent,
  formatCurrency,
  getTaskStatusText,
  getTaskStatusColor,
} from '@/lib/utils/format';

// 格式化文件大小
formatFileSize(1024);          // "1.00 KB"
formatFileSize(1048576);       // "1.00 MB"

// 格式化日期
formatDate('2024-01-01T00:00:00Z');  // "2024/01/01 00:00"

// 格式化百分比
formatPercent(12.345);         // "12.35%"

// 格式化货币
formatCurrency(1234.56);       // "$1234.56"

// 获取任务状态文本
getTaskStatusText('running');  // "运行中"

// 获取任务状态颜色
getTaskStatusColor('completed'); // "text-green-500"
```

## 错误处理

所有 API 调用都应该进行错误处理：

```typescript
try {
  const response = await api.config.list();
  
  if (response.success && response.data) {
    // 处理成功响应
    console.log(response.data);
  } else {
    // 处理业务错误
    console.error(response.error || response.message);
  }
} catch (error) {
  // 处理网络错误或其他异常
  if (error instanceof ApiError) {
    console.error('API 错误:', error.message, error.statusCode);
  } else {
    console.error('未知错误:', error);
  }
}
```

## 类型安全

所有 API 响应都有完整的 TypeScript 类型定义和 Zod 验证：

```typescript
import type { 
  ConfigListItem,
  DataFileItem,
  BacktestTask,
  BacktestResult,
} from '@/types/api';
```

## 环境配置

可以通过环境变量配置 API 基础 URL：

```bash
# .env.local
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

如果不设置，默认使用 `/api`（相对路径）。

## 测试

所有 API 客户端和 Hooks 都应该有对应的测试文件：

- `client.test.ts` - API 客户端基础设施测试
- `config.test.ts` - 配置 API 测试
- `data.test.ts` - 数据 API 测试
- `backtest.test.ts` - 回测 API 测试
- `useBacktestWebSocket.test.ts` - WebSocket Hook 测试

## 更多示例

查看 `src/components/examples/ApiExamples.tsx` 获取更多完整的使用示例。

## 注意事项

1. **客户端组件**: 使用 API 的组件需要添加 `'use client'` 指令
2. **错误处理**: 始终进行适当的错误处理
3. **加载状态**: 提供良好的加载反馈
4. **类型安全**: 利用 TypeScript 和 Zod 确保类型安全
5. **性能优化**: 考虑使用 React Query 或 SWR 进行数据缓存和重新验证
