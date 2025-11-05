# 配置管理页面更新说明

## 更新日期
2025年11月5日

## 更新内容

### 1. 完整的配置类型定义和Schema (config-schema.ts)

基于 `complete_config.toml` 创建了完整的配置类型定义,包括:

- **数据源配置** (`DataSourceConfig`):
  - provider: 数据提供商(binance/okx/bybit/csv)
  - api_key/api_secret: API认证(可选)
  - base_url/ws_url: 自定义端点(可选)
  - timeout/max_retries: 连接配置

- **策略配置** (`StrategyConfig`):
  - name: 策略名称
  - strategy_type: 策略类型
  - enabled: 是否启用
  - parameters: 策略参数(灵活的键值对)

- **投资组合配置** (`PortfolioConfig`):
  - initial_cash: 初始资金
  - commission/slippage: 手续费和滑点
  - max_position_size/max_positions: 仓位限制(可选)
  - risk_rules: 风险管理规则(可选)
    - max_drawdown_pct: 最大回撤限制
    - max_daily_loss_pct: 单日最大亏损
    - stop_loss_pct/take_profit_pct: 止损止盈
  - position_sizing: 仓位管理策略(可选)
    - fixed_percentage: 固定比例
    - kelly_criterion: Kelly准则
    - pyramid: 金字塔加仓
    - fixed_amount: 固定金额
    - all_in: 全仓

- **日志配置** (`LoggingConfig`):
  - level: 日志级别(trace/debug/info/warn/error)
  - format: 日志格式(json/pretty)
  - output: 日志文件路径(可选)

- **回测配置** (`BacktestSettings`, 可选):
  - data_path: 历史数据文件路径
  - symbol/interval: 交易对和时间间隔
  - start_time/end_time: 回测时间范围

- **实时交易配置** (`LiveConfig`, 可选):
  - symbol: 交易对符号
  - interval: K线时间间隔
  - paper_trading: 是否为模拟交易

### 2. TOML解析和转换工具 (toml.ts)

实现了完整的TOML文件处理功能:

- `parseTOML(tomlText)`: 解析TOML文本为配置对象
- `stringifyTOML(config)`: 将配置对象转换为TOML文本
- `readTOMLFile(file)`: 从文件读取并解析TOML配置
- `validateTOML(tomlText)`: 验证TOML文本的有效性
- `formatTOML(tomlText)`: 格式化TOML文本

使用 `@iarna/toml` 库进行TOML解析,使用Zod进行配置验证。

### 3. 配置编辑器表单组件 (ConfigSections.tsx)

将配置编辑器拆分为多个独立的区块组件,每个组件负责一个配置部分:

- `DataSourceSection`: 数据源配置表单
- `StrategiesSection`: 策略配置表单
- `PortfolioSection`: 投资组合配置表单
- `LoggingSection`: 日志配置表单
- `BacktestSection`: 回测配置表单
- `LiveSection`: 实时交易配置表单

每个组件都支持:
- 表单字段验证
- 实时更新配置
- 合理的字段分组和布局
- 必填和可选字段的区分

### 4. 配置管理页面重构 (page.tsx)

完全重写了配置管理页面,新增功能:

#### 4.1 TOML导入功能修复

**问题**: 原有的导入按钮没有绑定任何事件处理函数,导入文件后什么也不会发生。

**解决方案**:
1. 使用 `useRef` 创建文件输入引用
2. 实现 `handleImportTOML` 函数处理文件导入:
   - 读取文件内容
   - 使用TOML解析库解析文件
   - 更新配置状态
   - 在文本模式下同步更新文本内容
   - 显示成功/错误通知
3. 正确绑定 `onChange` 事件到文件输入元素

#### 4.2 双模式编辑器

- **表单模式**: 结构化的表单编辑,分组展示各个配置项
- **文本模式**: 直接编辑TOML文本,适合高级用户
- 支持两种模式之间的无缝切换
- 切换时自动进行数据同步和验证

#### 4.3 配置验证

- 点击"验证"按钮实时验证配置
- 显示详细的错误信息
- 验证通过后自动更新配置对象

#### 4.4 用户体验优化

- 使用通知系统显示操作结果
- 加载状态指示
- 友好的错误提示
- 清晰的必填/可选字段标识

### 5. 完整的单元测试

为所有新增功能添加了详细的单元测试:

#### toml.test.ts
- TOML解析测试
- TOML生成测试
- TOML验证测试
- TOML格式化测试

#### config-schema.test.ts
- 各个配置Schema的验证测试
- 默认配置函数测试
- 边界值测试
- 错误情况测试

### 6. 依赖更新

新增依赖:
```json
{
  "dependencies": {
    "@iarna/toml": "^2.2.5"
  },
  "devDependencies": {
    "@types/iarna__toml": "^2.0.5"
  }
}
```

## 技术栈

- TypeScript: 类型安全
- Zod: Schema验证
- @iarna/toml: TOML解析
- React Hooks: 状态管理
- Zustand: 全局状态(通知)
- Jest: 单元测试

## 使用方法

### 新建配置
1. 点击"新建配置"按钮
2. 在表单模式下填写各项配置
3. 或切换到文本模式直接编辑TOML
4. 点击"验证"检查配置是否正确
5. 点击"保存"保存配置

### 导入配置
1. 点击"导入配置"或"导入TOML"按钮
2. 选择本地的.toml配置文件
3. 系统自动解析并填充表单
4. 可以继续编辑或直接保存

### 模式切换
1. 在表单模式点击"文本模式"切换到TOML编辑
2. 在文本模式点击"表单模式"切换回表单
3. 切换时会自动验证和同步数据

## 文件结构

```
aurora-front/src/
├── app/config/
│   ├── page.tsx              # 配置管理页面(新)
│   ├── page.old.tsx          # 原页面备份
│   └── ConfigSections.tsx    # 配置区块组件(新)
├── types/
│   ├── config-schema.ts      # 配置Schema定义(新)
│   └── config-schema.test.ts # Schema测试(新)
└── lib/utils/
    ├── toml.ts               # TOML工具函数(新)
    └── toml.test.ts          # 工具函数测试(新)
```

## 注意事项

1. 所有代码都包含Apache 2.0许可证头部
2. 遵循项目约定的测试驱动开发
3. 使用Zod进行类型定义和验证
4. 所有函数都有详细的JSDoc注释
5. 组件保持高内聚低耦合
6. 单个文件不超过300行(如需要则拆分)

## 后续改进建议

1. 添加配置文件列表功能(从后端API获取)
2. 实现配置保存到后端的功能
3. 支持多策略的添加、删除和管理
4. 为风险管理和仓位管理添加专门的UI
5. 添加配置预设模板
6. 支持配置导出功能
7. 添加配置历史版本管理
