# 时区配置修复总结

## 问题描述

用户报告了两个问题：

### 问题1: UI显示两个时区选择器
配置管理页面的回测配置区域显示了"开始时区"和"结束时区"两个独立的下拉框，导致界面冗余。用户希望简化为单一时区选择器。

### 问题2: 配置保存不包含时区字段
在配置管理页面保存配置后，生成的TOML文件中没有包含时区字段，导致时区设置丢失。

## 解决方案

### 1. 简化Schema设计

**修改前:**
```typescript
// 前端 (config-schema.ts)
BacktestSettingsSchema = z.object({
  start_time: z.string().optional(),
  start_timezone: z.string().optional(),  // 开始时区
  end_time: z.string().optional(),
  end_timezone: z.string().optional(),    // 结束时区
  // ...
});
```

**修改后:**
```typescript
// 前端 (config-schema.ts)
BacktestSettingsSchema = z.object({
  start_time: z.string().optional(),
  end_time: z.string().optional(),
  timezone: z.string().optional(),  // 统一时区，应用于两个时间
  // ...
});
```

### 2. 更新Rust后端结构体

**修改前:**
```rust
// 后端 (types.rs)
pub struct BacktestConfig {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    // 没有时区字段
}
```

**修改后:**
```rust
// 后端 (types.rs)
pub struct BacktestConfig {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    /// 时区(可选，默认 UTC)，应用于 start_time 和 end_time
    #[serde(default)]
    pub timezone: Option<String>,
    // ...
}
```

### 3. 简化UI组件

**修改前:** 两个独立的时区选择器
- "开始时区" 选择器
- "结束时区" 选择器

**修改后:** 单一时区选择器
- "时区" 选择器
- 说明文字："开始时间和结束时间所在的时区"
- 默认值：自动检测浏览器当前时区

## 文件修改清单

### 前端修改
1. ✅ `src/types/config-schema.ts`
   - 删除 `start_timezone` 和 `end_timezone` 字段
   - 添加 `timezone` 字段

2. ✅ `src/app/config/ConfigSections.tsx`
   - 移除"开始时区"选择器
   - 移除"结束时区"选择器
   - 添加统一的"时区"选择器
   - 绑定到 `config.timezone` 字段

### 后端修改
1. ✅ `aurora-config/src/types.rs`
   - BacktestConfig 添加 `timezone: Option<String>` 字段
   - 添加 `#[serde(default)]` 属性确保向后兼容

2. ✅ `aurora-config/src/types/tests.rs`
   - 更新测试用例添加 `timezone` 字段
   - 添加 `timezone` 断言验证

### 文档更新
1. ✅ `docs/TIMEZONE_IMPLEMENTATION.md`
   - 更新配置示例
   - 添加修复说明
   - 更新字段说明

2. ✅ `docs/TIMEZONE_FIX_SUMMARY.md` (本文档)
   - 记录问题和解决方案

## 测试结果

### 前端测试
```
Test Suites: 60 passed, 60 total
Tests:       925 passed, 925 total
```
所有925个前端测试全部通过 ✅

### 后端测试
```
running 56 tests
test result: ok. 56 passed; 0 failed; 0 ignored; 0 measured
```
所有56个配置模块测试全部通过 ✅

## 配置文件示例

### 保存的TOML格式
```toml
[backtest]
data_path = "btc_1h.csv"
symbol = "BTCUSDT"
interval = "1h"
start_time = "2025-01-01 00:00:00"
end_time = "2025-12-31 23:59:59"
timezone = "Asia/Shanghai"  # ✅ 现在会正确保存

[backtest.pricing_mode]
mode = "bid_ask"
spread_pct = 0.001

# ... 其他配置 ...
```

## 向后兼容性

### 旧配置文件处理
如果配置文件中没有 `timezone` 字段，系统行为：
- Rust后端：`timezone` 字段为 `None`（因为使用了 `#[serde(default)]`）
- 前端UI：自动使用浏览器当前时区作为默认值

### 迁移建议
不需要手动迁移旧配置文件。当用户在UI中加载并保存旧配置时，会自动添加 `timezone` 字段。

## 用户体验改进

### 修改前
1. 界面冗余：两个时区选择器占用更多空间
2. 配置不一致：大多数情况下开始和结束使用同一时区
3. 保存失败：时区设置无法持久化

### 修改后
1. ✅ 界面简洁：单一时区选择器，清晰明了
2. ✅ 配置一致：统一时区应用于所有时间字段
3. ✅ 正确保存：时区字段正确写入TOML文件
4. ✅ 智能默认：自动检测用户浏览器时区

## 技术细节

### Zod Schema验证
```typescript
BacktestSettingsSchema = z.object({
  // ...
  timezone: z.string().optional(),  // 可选字符串，IANA时区标识符
  // ...
}).optional();
```

### Rust Serde序列化
```rust
#[serde(default)]  // 如果字段不存在，使用默认值 None
pub timezone: Option<String>,
```

### UI绑定逻辑
```tsx
<Select
  value={config.timezone || getCurrentTimezone()}  // 默认浏览器时区
  onValueChange={(value) => updateField('timezone', value)}
>
```

## 时区选项

UI提供10个常用时区：
1. UTC (UTC+00:00)
2. Asia/Shanghai (UTC+08:00 中国)
3. Asia/Tokyo (UTC+09:00 日本)
4. Asia/Hong_Kong (UTC+08:00 香港)
5. Asia/Singapore (UTC+08:00 新加坡)
6. Europe/London (UTC+00:00 / UTC+01:00)
7. Europe/Paris (UTC+01:00 / UTC+02:00)
8. America/New_York (UTC-05:00 / UTC-04:00)
9. America/Los_Angeles (UTC-08:00 / UTC-07:00)
10. Australia/Sydney (UTC+10:00 / UTC+11:00)

## 后续工作

虽然当前修复已经解决了UI和配置保存问题，但完整的时区支持还需要：

1. ⏳ **配置加载逻辑**：从TOML加载时正确解析timezone字段
2. ⏳ **时区转换集成**：在发送给后端前使用 `convertToUTC()` 转换时间
3. ⏳ **后端时区处理**：Rust代码中实现timezone到UTC的转换
4. ⏳ **端到端测试**：验证前端→后端→回测的完整流程

## 总结

✅ **问题1已解决**：UI现在只显示一个时区选择器，界面更简洁  
✅ **问题2已解决**：配置保存时正确包含timezone字段  
✅ **所有测试通过**：前端925个测试、后端56个测试全部通过  
✅ **向后兼容**：旧配置文件仍可正常加载  
✅ **用户体验提升**：自动检测浏览器时区，操作更便捷  

---

**修复日期**: 2025-01-14  
**涉及模块**: aurora-front, aurora-config  
**测试状态**: ✅ 全部通过
