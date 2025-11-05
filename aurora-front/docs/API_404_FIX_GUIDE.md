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

# API 404 错误修复指南

## 问题现象

在配置管理页面点击"保存"或"验证"按钮时，浏览器控制台显示：
- `POST http://localhost:3000/api/config 404 Not Found`
- `POST http://localhost:3000/api/config/validate 404 Not Found`

## 根本原因

前端尝试请求 Next.js 服务器（localhost:3000）的 API，但实际的 API 由 Rust 后端服务器（localhost:8080）提供。

## 已实施的修复

### 1. 创建环境变量配置

文件: `.env.local`
```env
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api
```

### 2. 配置 Next.js 代理

文件: `next.config.ts`
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

## 如何应用修复

### 步骤 1: 确保后端服务器运行

```bash
# 在 aurora-web 目录
cd ../aurora-web
cargo run
```

看到输出：
```
🚀 启动 Aurora Web 服务器: http://127.0.0.1:8080
✅ Aurora Web 服务器已启动
```

### 步骤 2: 重启前端开发服务器

**重要**: 必须完全重启才能加载新的环境变量！

```bash
# 在当前终端按 Ctrl+C 停止服务器

# 然后重新启动
npm run dev
```

### 步骤 3: 清除浏览器缓存

在浏览器中：
1. 按 F12 打开开发者工具
2. 右键点击刷新按钮
3. 选择"清空缓存并硬性重新加载"

或者使用无痕模式（Ctrl+Shift+N）

### 步骤 4: 测试功能

1. 访问 http://localhost:3000/config
2. 点击"新建配置"
3. 点击"验证"按钮
4. 检查网络请求（F12 -> Network 标签）
   - 应该看到请求成功（状态码 200）
   - 不再是 404

## 验证修复是否生效

### 检查后端 API

在 PowerShell 中运行：
```powershell
Invoke-RestMethod -Uri http://localhost:8080/api/config
```

应该返回配置文件列表。

### 检查前端代理

1. 打开 http://localhost:3000
2. 按 F12 打开开发者工具
3. 切换到 Network 标签
4. 在配置页面点击"验证"
5. 查看请求详情：
   - 请求 URL: `http://localhost:3000/api/config/validate`
   - 状态: 200 OK
   - 响应: `{"success":true,"data":{"valid":true,...}}`

## 常见问题

### Q: 重启后仍然是 404

**检查清单**:
- [ ] 后端服务器是否在运行？（检查 8080 端口）
- [ ] 前端服务器是否完全重启？（不是热重载）
- [ ] 浏览器缓存是否清除？
- [ ] `.env.local` 文件是否存在？
- [ ] 控制台是否有其他错误？

### Q: 环境变量未生效

**原因**: Next.js 在启动时读取环境变量，运行中修改不会生效。

**解决**: 必须停止服务器（Ctrl+C）后重新启动（npm run dev）

### Q: 端口冲突

**症状**: `Port 3000 is in use`

**解决**:
```bash
# 方法1: 杀死占用端口的进程
netstat -ano | findstr :3000
taskkill /PID <进程ID> /F

# 方法2: 使用其他端口
npm run dev -- -p 3001
```

### Q: CORS 错误

**症状**: `Access-Control-Allow-Origin` 错误

**解决**: 
- 开发环境使用代理（已配置）
- 后端已配置 `CorsLayer::permissive()`
- 确保通过 localhost:3000 访问，不是直接访问 localhost:8080

## 测试命令

### 测试后端 API

```powershell
# 测试配置列表
Invoke-RestMethod -Uri http://localhost:8080/api/config

# 测试配置验证
$body = @{content='[data_source]
provider = "binance"'} | ConvertTo-Json
Invoke-RestMethod -Uri http://localhost:8080/api/config/validate -Method Post -Body $body -ContentType 'application/json'
```

### 测试前端代理

```powershell
# 应该返回相同的结果（通过代理）
Invoke-RestMethod -Uri http://localhost:3000/api/config
```

## 相关文档

- [API_CONFIGURATION.md](./API_CONFIGURATION.md) - 详细的 API 配置说明
- [QUICK_START.md](./QUICK_START.md) - 快速启动指南
- [前端项目约定.md](../前端项目约定.md) - 项目开发规范

## 成功标志

修复成功后，你应该看到：

1. **后端控制台**: 
   ```
   POST /api/config/validate
   配置验证成功
   ```

2. **前端控制台**: 
   ```
   配置验证通过!
   ```

3. **网络请求**: 
   - URL: http://localhost:3000/api/config/validate
   - Status: 200 OK
   - Response: {"success":true,"data":{"valid":true}}

4. **用户界面**:
   - 出现绿色成功通知
   - 显示"配置验证通过!"
