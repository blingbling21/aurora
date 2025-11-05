/**
 * Copyright 2025 blingbling21
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

# API 配置说明

## 问题描述

前端请求 API 时出现 404 错误，原因是前端尝试请求 Next.js 服务器（localhost:3000）的 API，但实际的 API 由 Rust 后端服务器（localhost:8080）提供。

## 架构说明

Aurora 项目采用前后端分离架构：

- **前端**: Next.js + React (端口: 3000)
  - 位置: `aurora-front/`
  - 负责: UI 渲染、用户交互

- **后端**: Rust + Axum (端口: 8080)
  - 位置: `aurora-web/`
  - 负责: API 服务、业务逻辑、数据处理

## 解决方案

### 1. 环境变量配置

已创建 `.env.local` 文件，配置后端 API 地址：

```env
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

### 2. Next.js 代理配置

已在 `next.config.ts` 中配置开发环境代理：

```typescript
async rewrites() {
  if (process.env.NODE_ENV === 'development') {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:8080/api/:path*',
      },
    ];
  }
  return [];
}
```

**优点**:
- 开发环境下可以使用相对路径 `/api`
- 自动处理 CORS 问题
- 无需修改代码即可切换环境

### 3. API 客户端

`src/lib/api/client.ts` 会自动读取环境变量：

```typescript
const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || '/api';
```

## 使用步骤

### 1. 启动 Rust 后端服务器

```bash
cd aurora-web
cargo run
```

服务器将在 `http://localhost:8080` 启动

### 2. 启动 Next.js 前端服务器

```bash
cd aurora-front
npm run dev
```

前端将在 `http://localhost:3000` 启动

### 3. 重启服务以加载配置

如果环境变量是新创建的，需要重启 Next.js 开发服务器：

```bash
# 按 Ctrl+C 停止当前服务器
# 然后重新启动
npm run dev
```

## API 路由映射

| 前端请求路径 | 代理后实际路径 | 后端处理路由 |
|------------|--------------|------------|
| `/api/config` | `http://localhost:8080/api/config` | `api::config::routes()` |
| `/api/config/validate` | `http://localhost:8080/api/config/validate` | `api::config::validate_config()` |
| `/api/backtest` | `http://localhost:8080/api/backtest` | `api::backtest::routes()` |
| `/api/data` | `http://localhost:8080/api/data` | `api::data::routes()` |

## 验证配置

### 检查后端是否运行

```bash
curl http://localhost:8080/api/config
```

应该返回配置文件列表（JSON 格式）

### 检查前端代理

1. 打开浏览器开发者工具（F12）
2. 访问 `http://localhost:3000`
3. 在配置管理页面点击"验证"或"保存"按钮
4. 在 Network 标签查看请求：
   - 请求 URL 应该是 `http://localhost:3000/api/config/...`
   - 实际会被代理到 `http://localhost:8080/api/config/...`
   - 状态码应该是 200（不是 404）

## 常见问题

### Q1: 仍然出现 404 错误

**原因**: 环境变量未生效或代理配置未生效

**解决方法**:
1. 确认 `.env.local` 文件已创建
2. 完全停止并重启 Next.js 开发服务器
3. 清除浏览器缓存或使用无痕模式

### Q2: CORS 错误

**原因**: 直接访问后端 API 而不是通过代理

**解决方法**:
1. 使用开发环境代理（已配置）
2. 或者在 Rust 后端添加 CORS 配置（已有 `CorsLayer::permissive()`）

### Q3: 后端端口冲突

**错误信息**: "Address already in use"

**解决方法**:
```bash
# Windows
netstat -ano | findstr :8080
taskkill /PID <进程ID> /F

# 或者修改 aurora-web/src/main.rs 中的端口号
```

## 生产环境配置

生产环境应该设置正确的 API 地址：

```env
# .env.production
NEXT_PUBLIC_API_BASE_URL=https://your-api-domain.com/api
```

或者使用 Nginx 反向代理将前后端部署在同一域名下。

## 测试

运行测试时，Mock 会自动处理 API 调用：

```bash
npm test
```

测试使用 Jest Mock，不会发起实际的网络请求。
