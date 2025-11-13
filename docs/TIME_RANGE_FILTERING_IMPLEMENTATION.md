# 时间范围过滤功能实现总结

## 概述

本次更新为 Aurora 量化回测系统添加了完整的时间范围过滤功能，允许用户在回测配置中指定开始和结束时间，系统会自动验证时间范围的有效性并提供详细的错误提示。

## 主要修改

### 1. Aurora-Backtester (后端引擎)

#### 新增模块：`time_utils.rs`

- **时间解析函数** `parse_date_to_timestamp`
  - 支持格式：`YYYY-MM-DD` 和 `YYYY-MM-DD HH:MM:SS`
  - 返回毫秒级时间戳
  
- **时间范围验证函数** `validate_time_range`
  - 检测配置时间与数据时间的关系
  - 返回详细的验证结果：
    - `Valid`: 时间范围有效
    - `NoOverlap`: 完全不重叠（错误）
    - `InvalidRange`: 开始时间晚于结束时间（错误）
    - `StartBeforeData`: 开始时间早于数据（警告）
    - `EndAfterData`: 结束时间晚于数据（警告）
  
- **时间格式化函数** `format_timestamp`
  - 将毫秒时间戳格式化为可读字符串

#### 修改：`engine.rs`

- **新函数** `load_klines_from_csv_with_filter`
  - 支持按时间范围过滤 K 线数据
  - 自动验证时间范围有效性
  - 提供详细的错误和警告信息
  
- **更新函数** `run_backtest_with_progress`
  - 新增 `start_time` 和 `end_time` 参数
  - 调用过滤函数加载数据

### 2. Aurora-Web (后端 API)

#### 修改：`src/api/backtest.rs`

- 从配置文件中读取 `start_time` 和 `end_time`
- 将时间范围参数传递给回测引擎
- 自动处理时间验证错误

### 3. Aurora-Front (前端)

#### 新增工具：`src/lib/utils/timeRange.ts`

- **时间解析和验证工具**
  - `parseDateToTimestamp`: 解析日期字符串
  - `formatTimestamp`: 格式化时间戳
  - `validateTimeRange`: 验证时间范围
  - `extractTimeRangeFromFilename`: 从文件名提取时间范围

#### 已有支持

- 配置 Schema (`config-schema.ts`) 已包含 `start_time` 和 `end_time` 字段
- 前端类型定义已完整支持时间范围参数

### 4. 配置文件更新

更新了示例配置文件，添加时间范围示例：

```toml
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"
start_time = "2024-01-01"
end_time = "2024-12-31"
```

## 错误处理机制

### 1. 完全不重叠

**场景**：配置时间范围与数据完全不重叠

**错误信息**：
```
配置的时间范围与数据完全不重叠！
配置范围: 2024-01-01 00:00:00 到 2024-12-31 00:00:00
数据范围: 2025-01-01 00:00:00 到 2025-11-13 00:00:00
```

**处理**：回测任务失败，返回错误

### 2. 无效时间范围

**场景**：开始时间晚于结束时间

**错误信息**：
```
无效的时间范围: 开始时间 2024-12-31 00:00:00 晚于结束时间 2024-01-01 00:00:00
```

**处理**：回测任务失败，返回错误

### 3. 部分重叠

**场景**：配置时间早于或晚于数据时间

**警告信息**：
```
警告: 配置的开始时间 2024-12-01 00:00:00 早于数据开始时间 2025-01-01 00:00:00，将使用数据开始时间
```

**处理**：回测继续执行，使用数据的实际时间范围

## 测试覆盖

### 后端测试 (aurora-backtester)

#### 单元测试 (8 个)
- ✅ 时间解析功能测试
- ✅ 时间格式化测试
- ✅ 时间范围验证 - 无配置
- ✅ 时间范围验证 - 有效范围
- ✅ 时间范围验证 - 完全不重叠
- ✅ 时间范围验证 - 开始时间早于数据
- ✅ 时间范围验证 - 结束时间晚于数据
- ✅ 时间范围验证 - 无效范围

#### 集成测试 (4 个)
- ✅ 时间范围过滤功能测试
- ✅ 完全不重叠错误处理测试
- ✅ 无效时间范围错误处理测试
- ✅ 时间解析集成测试

### 前端测试 (aurora-front)

#### 单元测试 (13 个)
- ✅ 日期解析 - YYYY-MM-DD 格式
- ✅ 日期解析 - YYYY-MM-DD HH:MM:SS 格式
- ✅ 日期解析 - 无效格式错误
- ✅ 时间戳格式化
- ✅ 时间范围验证 - 各种场景
- ✅ 从文件名提取时间范围

**测试覆盖率**: 96.59% (语句覆盖)

## 使用示例

### 配置文件方式

```toml
[backtest]
data_path = "binance_btcusdt_1m_20250101_to_20251113.csv"
symbol = "BTCUSDT"
interval = "1m"
start_time = "2025-01-01"
end_time = "2025-10-31"

[backtest.pricing_mode]
mode = "bid_ask"
spread_pct = 0.001
```

### 前端验证示例

```typescript
import { validateTimeRange, extractTimeRangeFromFilename } from '@/lib/utils/timeRange';

// 从文件名提取时间
const { start, end } = extractTimeRangeFromFilename(
  'binance_btcusdt_1m_20250101_to_20251113.csv'
);
// start: '2025-01-01', end: '2025-11-13'

// 验证时间范围
const validation = validateTimeRange(
  '2024-01-01',  // 配置开始时间
  '2024-12-31',  // 配置结束时间
  new Date('2025-01-01').getTime(),  // 数据开始时间
  new Date('2025-11-13').getTime()   // 数据结束时间
);

if (!validation.isValid) {
  console.error(validation.error);
} else if (validation.warning) {
  console.warn(validation.warning);
}
```

## 文件结构

### 新增文件

```
aurora-backtester/
└── src/
    └── time_utils.rs (217 行)

aurora-front/
└── src/
    └── lib/
        └── utils/
            ├── timeRange.ts (150 行)
            └── timeRange.test.ts (130 行)
```

### 修改文件

```
aurora-backtester/
├── src/
│   ├── lib.rs (添加 time_utils 模块导出)
│   ├── main.rs (添加 time_utils 模块声明)
│   └── engine.rs (添加时间过滤功能)
└── tests/
    └── integration_tests.rs (添加时间过滤测试)

aurora-web/
├── src/
│   └── api/
│       └── backtest.rs (传递时间参数)
└── configs/
    ├── backtest_bidask_config.toml (添加时间范围示例)
    └── backtest_bidask_config_2.toml (添加时间范围示例)
```

## 向后兼容性

✅ **完全向后兼容**

- 时间范围参数是可选的
- 如果不指定时间范围，系统行为与之前完全一致
- 现有配置文件无需修改即可继续使用

## 性能影响

- ✅ 时间过滤在数据加载阶段完成，不影响回测核心逻辑
- ✅ 过滤操作复杂度 O(n)，其中 n 为数据行数
- ✅ 对于大数据集，过滤可以减少后续处理的数据量，提升性能

## 注意事项

1. **时间格式**: 推荐使用 `YYYY-MM-DD` 格式，系统会自动将日期转换为当天的 00:00:00
2. **时间范围边界**: 使用闭区间 [start, end]，包含起始和结束时间点
3. **时区处理**: 所有时间默认使用 UTC 时区
4. **数据文件命名**: 建议数据文件名包含时间范围信息，方便识别

## 未来改进方向

1. 支持更多时间格式（如 ISO 8601）
2. 添加时区配置选项
3. 前端界面显示时间范围验证结果
4. 支持从数据文件自动检测时间范围
