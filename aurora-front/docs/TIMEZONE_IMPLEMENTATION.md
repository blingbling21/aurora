# 时区支持实现总结

## 最新更新 (2025-01-14)

### 简化时区配置
根据用户反馈，简化了时区配置方案：
- ✅ **单一时区选择器**：不再区分开始时区和结束时区，使用统一的时区设置
- ✅ **前后端同步**：前端schema和后端Rust结构体已更新并通过测试
- ✅ **配置保存修复**：现在配置保存时会正确包含timezone字段

### 配置字段更改
**之前（已废弃）：**
```toml
[backtest]
start_time = "2025-01-01"
start_timezone = "Asia/Shanghai"
end_time = "2025-12-31"
end_timezone = "Asia/Shanghai"
```

**现在（推荐）：**
```toml
[backtest]
start_time = "2025-01-01"
end_time = "2025-12-31"
timezone = "Asia/Shanghai"  # 统一的时区设置，应用于开始和结束时间
```

## 已完成功能

### 1. 时区工具函数 (`src/lib/utils/timezone.ts`)

创建了完整的时区处理工具库，包含以下功能：

#### `convertToUTC(dateTimeStr, timezone)`
将指定时区的日期时间字符串转换为 UTC 时间字符串
- 支持格式：`YYYY-MM-DD` 或 `YYYY-MM-DD HH:mm:ss`
- 返回格式：`YYYY-MM-DD HH:mm:ss` (UTC)
- 自动处理夏令时转换
- 示例：`convertToUTC('2025-01-01 00:00:00', 'Asia/Shanghai')` → `'2024-12-31 16:00:00'`

#### `convertFromUTC(utcDateTimeStr, timezone)`
将 UTC 时间字符串转换为指定时区的日期时间字符串
- 支持往返转换（与 convertToUTC 配对使用）
- 自动处理夏令时
- 示例：`convertFromUTC('2024-12-31 16:00:00', 'Asia/Shanghai')` → `'2025-01-01 00:00:00'`

#### `getTimezoneOffset(timezone, date?)`
获取指定时区相对于 UTC 的偏移量（分钟）
- 返回正数表示东时区，负数表示西时区
- 可指定日期以正确处理夏令时
- 示例：`getTimezoneOffset('Asia/Shanghai')` → `480` (UTC+8)

#### `formatTimezoneOffset(offsetMinutes)`
格式化时区偏移量为字符串
- 示例：`formatTimezoneOffset(480)` → `'+08:00'`

### 2. 时区常量 (`src/constants/index.ts`)

#### `TIMEZONE_OPTIONS`
包含10个常用时区的选项数组：
- UTC (UTC+00:00)
- Asia/Shanghai (UTC+08:00 中国)
- Asia/Tokyo (UTC+09:00 日本)
- Asia/Hong_Kong (UTC+08:00 香港)
- Asia/Singapore (UTC+08:00 新加坡)
- Europe/London (UTC+00:00 / UTC+01:00)
- Europe/Paris (UTC+01:00 / UTC+02:00)
- America/New_York (UTC-05:00 / UTC-04:00)
- America/Los_Angeles (UTC-08:00 / UTC-07:00)
- Australia/Sydney (UTC+10:00 / UTC+11:00)

#### `getCurrentTimezone()`
获取用户浏览器当前时区的 IANA 标识符
- 使用 `Intl.DateTimeFormat().resolvedOptions().timeZone`
- 默认返回 'UTC' 如果无法检测

### 3. **配置文件架构更新** (`src/types/config-schema.ts`)

在 `BacktestSettingsSchema` 中添加了时区字段：
```typescript
timezone: z.string().optional(),  // 应用于 start_time 和 end_time 的统一时区
```

**注意**：`start_timezone` 和 `end_timezone` 已废弃，使用统一的 `timezone` 字段。

### 4. **UI组件更新** (`src/app/config/ConfigSections.tsx`)

在回测配置区块添加了时区选择器：

**时区选择器：**
- 位置：开始时间和结束时间输入框下方
- 默认值：当前浏览器时区（`getCurrentTimezone()`）
- 选项：`TIMEZONE_OPTIONS` 中的所有时区（10个常用时区）
- 说明文字："开始时间和结束时间所在的时区"
- 字段名：`timezone`（统一应用于两个时间字段）

### 5. 单元测试 (`src/lib/utils/timezone.test.ts`)

创建了32个全面的单元测试：

**convertToUTC 测试 (11个)**
- 北京时间转UTC（日期+时间、仅日期）
- 东京时间转UTC
- 纽约时间转UTC（标准时间、夏令时）
- 伦敦时间转UTC（标准时间、夏令时）
- UTC时区处理
- 跨月份转换
- 跨年份转换
- 无效格式错误处理

**convertFromUTC 测试 (7个)**
- UTC转北京时间（日期+时间、仅日期）
- UTC转东京时间
- UTC转纽约时间（标准时间、夏令时）
- UTC时区处理
- 无效格式错误处理

**往返转换测试 (3个)**
- 北京时间往返转换
- 纽约时间往返转换
- 伦敦时间往返转换

**getTimezoneOffset 测试 (7个)**
- 各时区偏移量验证（北京、东京、UTC、纽约、伦敦）
- 夏令时偏移量验证

**formatTimezoneOffset 测试 (4个)**
- 正偏移量格式化
- 负偏移量格式化
- 零偏移量格式化
- 数字填充验证

**测试结果：32个测试全部通过 ✅**

## 工作原理

### 时区转换流程

1. **用户在前端选择日期时间和时区**
   - 例如：2025-01-01 00:00:00 (Asia/Shanghai)

2. **配置保存时**
   - 日期时间字段：`start_time: "2025-01-01 00:00:00"`
   - 时区字段：`start_timezone: "Asia/Shanghai"`
   - 两个字段分开存储在配置文件中

3. **发送给后端时**
   - 使用 `convertToUTC(start_time, start_timezone)` 转换为UTC
   - 后端接收：`"2024-12-31 16:00:00"` (UTC)

4. **从配置加载时**
   - 读取 `start_time` 和 `start_timezone` 字段
   - 如果需要显示本地时间，使用 `convertFromUTC(start_time, start_timezone)`

### 夏令时处理

使用 `Intl.DateTimeFormat` API 自动处理夏令时：
- 美国东部时间：冬季 UTC-5，夏季 UTC-4
- 欧洲伦敦时间：冬季 UTC+0，夏季 UTC+1
- 澳大利亚悉尼：夏季 UTC+11，冬季 UTC+10

工具函数会根据指定日期自动应用正确的偏移量。

## 后续待完成工作

### 前端部分
1. ✅ 时区工具函数
2. ✅ 时区常量定义
3. ✅ 配置文件架构更新（简化为单一timezone字段）
4. ✅ UI组件（单一时区选择器）
5. ✅ 单元测试
6. ⏳ 配置保存逻辑更新（使用 convertToUTC）
7. ⏳ 配置加载逻辑更新（处理时区字段）

### 后端部分
1. ✅ 更新 Rust BacktestConfig 结构体（添加 timezone 字段）
2. ✅ Rust 单元测试通过（56个测试全部通过）
3. ⏳ 添加 chrono-tz 依赖到 Cargo.toml
4. ⏳ 实现 Rust 时区解析和UTC转换
5. ⏳ 集成测试（前端→后端完整流程）

## 使用示例

### 在配置表单中使用
```typescript
// 用户选择北京时间 2025-01-01 00:00:00
const config = {
  start_time: '2025-01-01 00:00:00',
  start_timezone: 'Asia/Shanghai',
  // ...
};

// 发送给后端前转换为UTC
import { convertToUTC } from '@/lib/utils/timezone';
const utcTime = convertToUTC(config.start_time, config.start_timezone);
// utcTime = '2024-12-31 16:00:00'

// 发送给后端
await backtestApi.execute({
  ...config,
  start_time: utcTime, // 使用UTC时间
});
```

### 从配置加载
```typescript
// 从配置文件加载
const savedConfig = {
  start_time: '2025-01-01 00:00:00',
  start_timezone: 'Asia/Shanghai',
};

// 如果需要在其他时区显示
import { convertFromUTC, convertToUTC } from '@/lib/utils/timezone';

// 先转UTC
const utcTime = convertToUTC(
  savedConfig.start_time, 
  savedConfig.start_timezone
);

// 再转到目标时区（例如东京）
const tokyoTime = convertFromUTC(utcTime, 'Asia/Tokyo');
// tokyoTime = '2025-01-01 01:00:00' (比北京快1小时)
```

## 技术亮点

1. **完全基于标准API**：使用 `Intl.DateTimeFormat`，无需第三方库
2. **自动夏令时处理**：正确处理所有时区的夏令时转换
3. **往返转换精度**：支持 UTC ↔ 本地时间的无损往返转换
4. **全面测试覆盖**：32个测试覆盖各种边界情况
5. **类型安全**：完整的 TypeScript 类型定义
6. **用户友好**：自动检测浏览器时区作为默认值

## 配置文件示例

```toml
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"
start_time = "2025-01-01 00:00:00"
end_time = "2025-12-31 23:59:59"
timezone = "Asia/Shanghai"  # 统一时区，应用于开始和结束时间
```

后端将读取 `start_time`、`end_time` 和 `timezone`，并将其转换为 UTC 时间戳进行回测计算。

## 实际效果展示

### UI界面
配置管理页面的回测配置区域现在包含：
1. **开始时间**输入框（格式：YYYY-MM-DD）
2. **结束时间**输入框（格式：YYYY-MM-DD）
3. **时区**选择器（单个下拉框，应用于两个时间）
   - 默认值：自动检测用户浏览器时区
   - 10个常用时区选项，包含时区名称和UTC偏移量显示

### 配置保存
保存配置时，TOML文件中会包含：
```toml
[backtest]
# ... 其他字段 ...
start_time = "2025-01-01"
end_time = "2025-12-31"
timezone = "Asia/Shanghai"
```

**已修复**：之前版本保存配置时没有timezone字段，现在已正确保存。
