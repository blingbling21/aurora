# Aurora 前端开发指南

## 项目概览

Aurora 前端是一个基于 Next.js 16 的现代化量化交易回测平台前端应用。本文档提供了项目的完整使用说明。

## 快速开始

### 前置要求

- Node.js 18+ 
- npm 或 yarn 或 pnpm

### 安装

```bash
cd aurora-front
npm install
```

### 开发

```bash
npm run dev
```

访问 http://localhost:3000 查看应用。

### 构建

```bash
npm run build
npm start
```

### 测试

```bash
npm test
```

## 项目特性

### ✅ 已完成的功能

1. **仪表盘页面** (/)
   - 统计卡片展示
   - 最近任务列表
   - 响应式布局

2. **配置管理页面** (/config)
   - 配置文件列表查看
   - 双模式编辑器(表单模式/TOML 文本模式)
   - 配置文件导入功能

3. **数据管理页面** (/data)
   - 历史数据下载表单
   - 交易所、交易对、时间周期选择
   - 下载进度显示
   - 数据文件列表管理

4. **回测执行页面** (/backtest)
   - 回测任务创建表单
   - 配置和数据文件选择
   - 实时进度显示

5. **历史记录页面** (/history)
   - 历史任务列表
   - 回测结果查看
   - 性能指标展示

### 🎨 UI 组件库

- **Button**: 多样式按钮组件
- **Card**: 卡片容器
- **PageHeader**: 页面头部
- **Notification**: 通知提示
- **StatCard**: 统计卡片
- **TaskItem**: 任务列表项

### 🏗️ 架构特点

- **组件化设计**: 所有 UI 元素都封装为可复用组件
- **类型安全**: 完整的 TypeScript 类型定义
- **响应式布局**: 适配移动端和桌面端
- **模块化结构**: 清晰的目录组织

## 页面功能详解

### 仪表盘

显示系统整体状态:
- 总任务数统计
- 运行中任务数量
- 完成的任务数量
- 失败的任务数量
- 最近 5 个任务的快速访问

### 配置管理

管理回测配置文件:
- **表单模式**: 通过表单界面配置参数
  - 数据源配置(提供商、超时、重试)
  - 投资组合配置(初始资金、手续费、滑点)
  - 策略配置
  - 风险管理参数
- **文本模式**: 直接编辑 TOML 配置文件
- 配置验证功能
- 配置导入/导出

### 数据管理

下载和管理市场数据:
- 支持多个交易所(Binance、OKX、Bybit、Coinbase)
- 常用交易对快速选择
- 多种时间周期(1分钟到1周)
- 自定义日期范围
- 实时下载进度
- 数据文件列表管理

### 回测执行

启动和监控回测任务:
- 任务命名
- 选择配置文件
- 选择数据文件
- 实时进度监控
- 结果快速查看

### 历史记录

查看和分析历史回测:
- 历史任务列表
- 详细的性能指标:
  - 总收益率
  - 年化收益率
  - 最大回撤
  - 夏普比率
  - 交易次数
  - 胜率
- 图表展示区域(待实现)

## 开发规范

### 文件组织

```
组件文件夹/
├── Component.tsx       # 组件实现
├── Component.test.tsx  # 测试文件
└── index.ts           # 导出文件
```

### 代码规范

1. **文件头部**: 每个文件必须包含 Apache 2.0 许可证声明
2. **注释**: 
   - 组件需要 JSDoc 文档注释
   - 复杂逻辑需要行内注释
3. **文件大小**: 单个文件不超过 300 行
4. **测试**: 所有组件必须有对应的测试文件

### 组件开发示例

```tsx
/**
 * Copyright 2025 blingbling21
 * [... 许可证声明 ...]
 */

interface MyComponentProps {
  title: string;
  onAction: () => void;
}

/**
 * 我的组件
 * 
 * 组件的详细说明
 * 
 * @param {string} title - 标题
 * @param {Function} onAction - 动作回调
 */
export function MyComponent({ title, onAction }: MyComponentProps) {
  return (
    <div>
      <h2>{title}</h2>
      <button onClick={onAction}>操作</button>
    </div>
  );
}
```

### 测试示例

参见 `src/components/ui/Button.test.tsx`

## 待开发功能

### 高优先级

1. **状态管理集成**
   - 使用 Zustand 管理全局状态
   - 实现任务状态同步

2. **API 集成**
   - 连接后端 REST API
   - WebSocket 实时更新

3. **图表组件**
   - K线图
   - 权益曲线图
   - 回撤曲线图

4. **表单验证**
   - 使用 Zod 进行表单验证
   - 错误提示优化

### 中优先级

5. **单元测试完善**
   - 为所有组件编写测试
   - 提高测试覆盖率

6. **国际化支持**
   - 多语言支持
   - 时区处理

7. **主题系统**
   - 深色模式
   - 自定义主题

### 低优先级

8. **性能优化**
   - 代码分割
   - 懒加载优化
   - 缓存策略

9. **可访问性**
   - ARIA 标签
   - 键盘导航

10. **PWA 支持**
    - 离线功能
    - 安装到桌面

## 技术栈详解

- **Next.js 16**: 使用 App Router,支持服务端渲染和静态生成
- **TypeScript**: 提供类型安全和更好的开发体验
- **Tailwind CSS 4**: 原子化 CSS 框架,快速构建 UI
- **Jest**: 单元测试框架
- **React Testing Library**: React 组件测试

## 常见问题

### Q: 如何添加新页面?

1. 在 `src/app/` 下创建新文件夹
2. 添加 `page.tsx` 文件
3. 在 `src/constants/index.ts` 中添加路由配置

### Q: 如何创建新组件?

1. 在相应的组件文件夹下创建组件文件
2. 添加类型定义
3. 编写组件实现
4. 添加 JSDoc 注释
5. 创建测试文件
6. 在 `index.ts` 中导出

### Q: 样式如何覆盖?

使用 Tailwind 的 `className` 属性,配合 `cn()` 工具函数:

```tsx
<Button className="custom-class">按钮</Button>
```

### Q: 如何调试?

1. 使用浏览器开发者工具
2. 在代码中添加 `console.log()`
3. 使用 React Developer Tools
4. VS Code 调试配置

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

Apache License 2.0 - 详见 LICENSE 文件

## 联系方式

项目维护者: blingbling21

---

**注意**: 当前版本为静态页面原型,所有数据都是模拟数据。后续版本将集成真实的后端 API。
