# Aurora Front - 前端项目

这是 Aurora 量化交易回测平台的 Next.js 前端项目。

## 项目结构

```
aurora-front/
├── src/
│   ├── app/                    # Next.js App Router 页面
│   │   ├── layout.tsx          # 根布局
│   │   ├── page.tsx            # 首页(仪表盘)
│   │   ├── config/             # 配置管理页面
│   │   ├── data/               # 数据管理页面
│   │   ├── backtest/           # 回测执行页面
│   │   └── history/            # 历史记录页面
│   ├── components/             # 组件目录
│   │   ├── layout/             # 布局组件
│   │   │   ├── Sidebar.tsx     # 侧边栏导航
│   │   │   ├── MainLayout.tsx  # 主布局
│   │   │   └── index.ts
│   │   ├── ui/                 # 基础 UI 组件
│   │   │   ├── Button.tsx      # 按钮组件
│   │   │   ├── Card.tsx        # 卡片组件
│   │   │   ├── PageHeader.tsx  # 页面头部
│   │   │   ├── Notification.tsx # 通知组件
│   │   │   └── index.ts
│   │   └── dashboard/          # 仪表盘组件
│   │       ├── StatCard.tsx    # 统计卡片
│   │       ├── TaskItem.tsx    # 任务列表项
│   │       └── index.ts
│   ├── types/                  # TypeScript 类型定义
│   │   └── index.ts
│   ├── constants/              # 常量定义
│   │   └── index.ts
│   └── lib/                    # 工具函数
│       └── utils.ts
├── public/                     # 静态资源
├── package.json
├── tsconfig.json
├── next.config.ts
├── tailwind.config.ts
└── README.md
```

## 已实现的页面

### 1. 仪表盘 (/)
- 显示统计卡片(总任务数、运行中、已完成、失败)
- 显示最近任务列表

### 2. 配置管理 (/config)
- 配置文件列表
- 配置编辑器(支持表单模式和 TOML 文本模式)
- 配置导入/导出功能

### 3. 数据管理 (/data)
- 历史数据下载表单
- 数据文件列表
- 下载进度显示

### 4. 回测执行 (/backtest)
- 回测任务启动表单
- 回测进度显示

### 5. 历史记录 (/history)
- 历史任务列表
- 回测结果查看
- 性能指标展示(总收益率、年化收益率、最大回撤等)

## 核心组件

### 布局组件
- **Sidebar**: 侧边栏导航,支持路由高亮
- **MainLayout**: 主布局容器,包含侧边栏和内容区

### UI 组件
- **Button**: 按钮组件,支持多种样式变体(primary, secondary, danger)
- **Card**: 卡片容器组件
- **PageHeader**: 页面头部组件
- **Notification**: 通知提示组件

### 业务组件
- **StatCard**: 统计数据卡片
- **TaskItem**: 任务列表项,显示任务状态和进度

## 技术栈

- **Next.js 16**: React 框架
- **TypeScript**: 类型安全
- **Tailwind CSS 4**: 样式框架
- **shadcn/ui**: UI 组件库(后续集成)
- **Zustand**: 状态管理(后续集成)
- **Zod**: 数据验证(后续集成)
- **Jest**: 测试框架

## 开发规范

遵循 `前端项目约定.md` 中的规范:
- 测试驱动开发
- 所有组件需要详细注释
- 可复用组件需要 JSDoc 文档注释
- 所有组件需要单元测试
- 单个文件不超过 300 行
- 遵循 Apache 2.0 许可证

## 下一步工作

1. ✅ 创建核心布局和页面结构
2. ⏳ 添加状态管理(Zustand)
3. ⏳ 集成 API 服务
4. ⏳ 添加图表组件(用于回测结果可视化)
5. ⏳ 编写组件单元测试
6. ⏳ 添加 WebSocket 支持(实时回测进度)
7. ⏳ 完善表单验证(Zod)
8. ⏳ 添加响应式设计优化

## 安装和运行

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建生产版本
npm run build

# 运行生产版本
npm start

# 运行测试
npm test
```

## 注意事项

- 当前所有页面都是静态页面,暂未连接后端 API
- 数据都是模拟数据,后续需要集成真实的 API 调用
- 图表组件待实现,需要选择合适的图表库(如 Chart.js 或 ECharts)
- 测试文件尚未创建,需要按照项目约定补充
