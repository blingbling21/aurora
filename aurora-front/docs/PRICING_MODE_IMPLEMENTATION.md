# 定价模式功能实现说明

## 概述

根据 Aurora 回测引擎的需求，在前端配置管理中添加了定价模式（Pricing Mode）配置功能。该功能允许用户在回测配置中选择不同的交易价格计算方式。

## 实现的功能

### 1. 类型定义 (config-schema.ts)

添加了完整的定价模式类型定义：

- **PricingModeCloseSchema**: 收盘价定价模式
  - 使用 K 线收盘价执行交易（简单模式）
  
- **PricingModeBidAskSchema**: 买一卖一价定价模式
  - 使用买一卖一价执行交易（更真实的模式）
  - 包含 `spread_pct` 字段，表示买卖价差百分比（0-1之间）

- **PricingModeSchema**: 联合类型，支持可选配置

### 2. UI 组件 (ConfigSections.tsx)

在 `BacktestSection` 组件中添加了定价模式表单：

#### 表单项包括：

1. **定价模式选择器**
   - 默认(不设置)
   - 收盘价模式
   - 买一卖一价模式

2. **价差输入框**（仅在选择买一卖一价模式时显示）
   - 输入买卖价差百分比
   - 范围：0-1
   - 默认值：0.001 (0.1%)
   - 步长：0.0001

#### 特性：

- 动态显示/隐藏价差输入框
- 提供说明文字帮助用户理解
- 支持取消定价模式设置

### 3. 单元测试

#### config-schema.test.ts

添加了以下测试用例：

- ✅ 收盘价定价模式验证
- ✅ 买一卖一价定价模式验证
- ✅ 缺少 spread_pct 时的错误处理
- ✅ spread_pct 负数验证
- ✅ spread_pct 大于 1 时的验证
- ✅ 可选字段支持
- ✅ 回测配置中的 pricing_mode 集成测试

#### ConfigSections.test.tsx

添加了以下测试用例：

- ✅ 基础渲染测试
- ✅ 选择收盘价模式
- ✅ 选择买一卖一价模式（带默认价差）
- ✅ 修改买卖价差百分比
- ✅ 取消定价模式设置
- ✅ 条件渲染价差输入框

所有测试全部通过 ✅

## 与后端的对应关系

### Rust 后端定义 (aurora-config/src/types.rs)

```rust
pub enum PricingModeConfig {
    Close,
    BidAsk {
        spread_pct: f64,
    },
}
```

### TypeScript 前端定义 (config-schema.ts)

```typescript
export const PricingModeCloseSchema = z.object({
  mode: z.literal('close'),
});

export const PricingModeBidAskSchema = z.object({
  mode: z.literal('bid_ask'),
  spread_pct: z.number().min(0).max(1),
});
```

### 配置文件示例 (TOML)

```toml
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"

# 收盘价模式
[backtest.pricing_mode]
mode = "close"

# 或者买一卖一价模式
[backtest.pricing_mode]
mode = "bid_ask"
spread_pct = 0.001
```

## 验证

1. **类型安全**: 通过 Zod Schema 确保数据类型正确
2. **边界验证**: spread_pct 限制在 0-1 范围内
3. **可选配置**: 支持不设置定价模式（使用默认行为）
4. **UI 反馈**: 提供清晰的表单说明和提示文字
5. **测试覆盖**: 完整的单元测试确保功能正确性

## 使用方法

1. 在配置管理页面 (`/config`)，启用回测配置
2. 向下滚动到"定价模式配置"区域
3. 从下拉菜单中选择定价模式：
   - 默认(不设置): 不指定定价模式
   - 收盘价模式: 使用 K 线收盘价
   - 买一卖一价模式: 使用买一卖一价，并设置价差
4. 如果选择买一卖一价模式，输入价差百分比（例如 0.001 表示 0.1%）
5. 保存配置

## 技术规范遵循

- ✅ 遵循 Apache 2.0 许可证
- ✅ 所有文件添加了完整的许可证头部
- ✅ 详细的代码注释（使用 // 和 JSDoc）
- ✅ 完整的单元测试覆盖
- ✅ 使用 Zod 进行类型定义和验证
- ✅ 高内聚、低耦合的组件设计

## 测试结果

```
Test Suites: 46 passed, 46 total
Tests:       725 passed, 725 total
Snapshots:   0 total
```

所有相关测试全部通过，包括：
- config-schema.test.ts: 13 个测试
- ConfigSections.test.tsx: 8 个新增测试

## 相关文件

- `src/types/config-schema.ts`: 类型定义和 Schema
- `src/types/config-schema.test.ts`: 类型测试
- `src/app/config/ConfigSections.tsx`: UI 组件
- `src/app/config/ConfigSections.test.tsx`: UI 测试

## 更新日期

2025-11-06
