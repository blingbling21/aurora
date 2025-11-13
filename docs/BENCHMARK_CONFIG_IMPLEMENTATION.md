# 基准配置功能实现总结

## 概述

本次更新为 Aurora 回测系统添加了基准配置功能，允许用户在回测执行时启用或禁用基准对比。

## 功能特性

### 1. 配置编辑器增强

在配置页面的回测配置区域新增了基准配置模块：

- **启用/禁用开关**：允许用户通过开关控制是否启用基准
- **基准数据文件选择**：启用后可从已下载的数据文件中选择基准数据
- **自动验证**：确保启用基准时必须选择数据文件

### 2. 回测执行页面增强

在回测执行页面新增基准选择下拉框：

- **显示配置默认值**：自动显示配置文件中的基准设置
- **动态选择**：可在执行时修改基准选择
- **禁用选项**：支持将基准设置为禁用
- **状态提示**：清晰显示当前基准启用状态

### 3. 数据类型定义

#### 前端 (TypeScript/Zod)

```typescript
export const BenchmarkConfigSchema = z.object({
  enabled: z.boolean().default(false),
  data_path: z.string().optional(),
}).refine(
  (data) => {
    if (data.enabled && !data.data_path) {
      return false;
    }
    return true;
  },
  {
    message: '启用基准时必须指定数据文件路径',
    path: ['data_path'],
  }
);
```

#### 后端 (Rust)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    #[serde(default)]
    pub enabled: bool,
    
    #[serde(default)]
    pub data_path: Option<String>,
}

impl BenchmarkConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled && self.data_path.is_none() {
            return Err("启用基准时必须指定数据文件路径".to_string());
        }
        Ok(())
    }
}
```

### 4. 回测配置更新

在 `BacktestConfig`/`BacktestSettings` 中新增 `benchmark` 字段：

```toml
[backtest]
data_path = "binance_btcusdt_1m_20250101_to_20251113.csv"
symbol = "BTCUSDT"
interval = "1m"

# 基准配置
[backtest.benchmark]
enabled = true
data_path = "binance_btcusdt_5m_20251001_to_20251111.csv"
```

## 修改文件列表

### 前端文件

1. **src/types/config-schema.ts**
   - 添加 `BenchmarkConfig` 类型和 Schema
   - 更新 `BacktestSettings` 包含基准配置

2. **src/types/config-schema.test.ts**
   - 添加基准配置验证测试
   - 添加回测配置与基准集成测试

3. **src/app/config/ConfigSections.tsx**
   - 在 `BacktestSection` 组件中添加基准配置 UI
   - 实现基准启用/禁用开关
   - 实现基准数据文件下拉选择

4. **src/app/backtest/page.tsx**
   - 添加基准选择状态管理
   - 实现配置文件变化时自动加载基准设置
   - 添加基准选择下拉框

### 后端文件

1. **aurora-config/src/types.rs**
   - 添加 `BenchmarkConfig` 结构体
   - 更新 `BacktestConfig` 包含基准配置
   - 实现基准配置验证方法

2. **aurora-config/src/types/tests.rs**
   - 添加基准配置单元测试
   - 添加基准配置序列化/反序列化测试
   - 更新现有测试以包含 benchmark 字段

3. **aurora-web/src/api/backtest.rs**
   - 在回测执行逻辑中添加基准配置检查
   - 添加基准配置日志输出

### 示例配置文件

1. **examples/backtest_with_benchmark_config.toml**
   - 启用基准的完整配置示例

2. **examples/backtest_without_benchmark_config.toml**
   - 禁用基准的配置示例

## 使用方法

### 在配置编辑器中配置基准

1. 打开配置管理页面
2. 编辑或创建配置文件
3. 在"回测配置"区域，找到"基准配置"模块
4. 启用基准开关
5. 从下拉框中选择基准数据文件
6. 保存配置

### 在回测执行时选择基准

1. 打开回测执行页面
2. 选择配置文件（自动加载配置中的基准设置）
3. 在"基准数据文件"下拉框中：
   - 保持配置文件中的默认值
   - 或选择其他数据文件
   - 或选择"禁用"来关闭基准
4. 启动回测

## 测试覆盖

### 前端测试

- ✅ 基准配置 Schema 验证
- ✅ 启用基准但无数据路径的错误处理
- ✅ 禁用基准的正常处理
- ✅ 回测配置与基准的集成

### 后端测试

- ✅ 基准配置结构创建
- ✅ 基准配置验证逻辑
- ✅ 基准配置序列化/反序列化
- ✅ 回测配置与基准的 TOML 解析

## 未来扩展

当前实现为基准配置提供了完整的 UI 和配置支持，但实际的基准回测逻辑需要在回测引擎中实现。未来可以：

1. 在回测引擎中实现基准数据加载
2. 计算策略相对于基准的表现指标
3. 在结果中展示基准对比图表
4. 支持多个基准同时对比

## 注意事项

1. **数据文件兼容性**：确保基准数据文件与主数据文件的时间范围和格式兼容
2. **验证规则**：启用基准时必须选择数据文件，否则配置验证会失败
3. **默认值**：基准配置默认为禁用状态

## 技术细节

### 前端状态管理

- 使用 React hooks 管理基准配置状态
- 使用 useEffect 监听配置文件变化并自动加载基准设置
- 使用 Zod 进行运行时类型验证

### 后端配置解析

- 使用 Serde 进行 TOML 序列化/反序列化
- 提供 validate() 方法进行配置验证
- 在回测执行时记录基准配置状态

### 数据流

```
配置编辑器 → TOML 配置文件 → 后端配置解析 → 回测引擎
     ↓
回测执行页面 → 基准选择 → 回测任务创建
```

## 版权信息

Copyright 2025 blingbling21

Licensed under the Apache License, Version 2.0
