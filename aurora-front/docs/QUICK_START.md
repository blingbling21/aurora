# Aurora Front - 快速开始使用 API

本指南帮助你快速开始在 Aurora Front 中使用 API 功能。

## 5 分钟快速上手

### 1. 导入 API 客户端

```typescript
import { api } from '@/lib/api';
```

### 2. 在组件中使用

```typescript
'use client';

import { useEffect, useState } from 'react';
import { api } from '@/lib/api';
import type { ConfigListItem } from '@/types/api';

export default function ConfigPage() {
  const [configs, setConfigs] = useState<ConfigListItem[]>([]);

  useEffect(() => {
    // 加载配置列表
    api.config.list().then((response) => {
      if (response.success && response.data) {
        setConfigs(response.data);
      }
    });
  }, []);

  return (
    <div>
      <h1>配置列表</h1>
      {configs.map((config) => (
        <div key={config.filename}>{config.filename}</div>
      ))}
    </div>
  );
}
```

### 3. 使用 WebSocket 监听回测进度

```typescript
import { useBacktestWebSocket } from '@/lib/hooks';

export default function BacktestPage() {
  const [taskId, setTaskId] = useState<string | null>(null);
  
  // 启动回测
  const handleStart = async () => {
    const response = await api.backtest.start({
      name: '我的回测',
      config_path: 'config.toml',
      data_path: 'data.csv',
    });
    
    if (response.success && response.data) {
      setTaskId(response.data.task_id);
    }
  };

  // 监听进度
  const { status, lastMessage } = useBacktestWebSocket(taskId, {
    onStatusUpdate: (progress, status) => {
      console.log(`进度: ${progress}%`);
    },
    onComplete: () => {
      alert('回测完成！');
    },
  });

  return (
    <div>
      <button onClick={handleStart}>启动回测</button>
      {taskId && <div>连接状态: {status}</div>}
    </div>
  );
}
```

## 常用 API

### 配置管理

```typescript
// 列出所有配置
const configs = await api.config.list();

// 创建配置
await api.config.create({
  filename: 'my_config.toml',
  content: '...',
});

// 验证配置
const result = await api.config.validate(content);
if (!result.data?.valid) {
  console.error('验证失败:', result.data?.errors);
}
```

### 数据管理

```typescript
// 列出数据文件
const files = await api.data.list();

// 下载历史数据
await api.data.fetch({
  exchange: 'binance',
  symbol: 'BTCUSDT',
  interval: '1h',
  start_date: '2024-01-01',
  end_date: '2024-12-31',
});
```

### 回测管理

```typescript
// 启动回测
const response = await api.backtest.start({
  name: '测试回测',
  config_path: 'config.toml',
  data_path: 'data.csv',
});

// 获取结果
const result = await api.backtest.getResult(taskId);
if (result.success && result.data) {
  console.log('回测结果:', result.data);
}
```

## 错误处理

```typescript
try {
  const response = await api.config.list();
  
  if (response.success && response.data) {
    // 成功
    setConfigs(response.data);
  } else {
    // 业务错误
    console.error(response.error);
  }
} catch (error) {
  // 网络错误
  console.error('请求失败:', error);
}
```

## 格式化工具

```typescript
import { formatFileSize, formatDate, formatPercent } from '@/lib/utils/format';

// 格式化文件大小
formatFileSize(1048576);  // "1.00 MB"

// 格式化日期
formatDate('2024-01-01T00:00:00Z');  // "2024/01/01 00:00"

// 格式化百分比
formatPercent(12.34);  // "12.34%"
```

## 环境配置

创建 `.env.local` 文件：

```bash
# 如果后端在不同端口
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

## 下一步

- 📖 阅读 [API 使用指南](./API_USAGE_GUIDE.md) 了解详细用法
- 📖 查看 [API 集成总结](./API_INTEGRATION_SUMMARY.md) 了解完整功能
- 💻 查看 `src/components/examples/ApiExamples.tsx` 获取更多示例
- 🧪 运行测试：`npm test`

## 需要帮助？

查看完整文档或搜索示例代码获取更多帮助。
