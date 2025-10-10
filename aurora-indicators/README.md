# Aurora Indicators - 技术指标库

Aurora Indicators 是一个高性能的 Rust 技术分析指标库，为量化交易策略提供常用的技术指标实现。

## 特性

- ✅ **高性能**: 使用 Rust 实现，O(1) 时间复杂度的指标更新
- ✅ **内存高效**: 使用滑动窗口避免存储过多历史数据
- ✅ **类型安全**: 利用 Rust 类型系统确保计算正确性
- ✅ **流式处理**: 支持实时数据流处理
- ✅ **全面测试**: 每个指标都有详细的单元测试和集成测试

## 已实现的指标

### 趋势指标

#### 1. MA (移动平均线)
简单移动平均线，用于识别趋势方向。

```rust
use aurora_indicators::MA;

let mut ma = MA::new(5);
ma.update(100.0);
ma.update(102.0);
ma.update(98.0);
ma.update(105.0);
let avg = ma.update(95.0).unwrap();
```

#### 2. EMA (指数移动平均线)
对近期数据赋予更高权重的移动平均线，响应更快。

```rust
use aurora_indicators::EMA;

let mut ema = EMA::new(10);
for i in 1..=20 {
    let result = ema.update(100.0 + i as f64);
    println!("EMA: {:.2}", result);
}
```

#### 3. MACD (移动平均收敛散度)
用于判断趋势变化和买卖时机，包含MACD线、信号线和柱状图。

```rust
use aurora_indicators::MACD;

let mut macd = MACD::default(); // 使用默认参数 (12, 26, 9)
let output = macd.update(100.0);
println!("MACD: {:.2}, Signal: {:.2}, Histogram: {:.2}", 
    output.macd, output.signal, output.histogram);
```

### 动量指标

#### 4. RSI (相对强弱指数)
衡量价格变动速度和幅度，用于判断超买超卖状态。

```rust
use aurora_indicators::RSI;

let mut rsi = RSI::new(14);
rsi.update(100.0);
if let Some(rsi_value) = rsi.update(105.0) {
    if rsi.is_overbought() {
        println!("市场超买! RSI = {:.2}", rsi_value);
    }
}
```

#### 5. Stochastic (随机震荡指标)
比较收盘价与价格区间的相对位置。

```rust
use aurora_indicators::Stochastic;

let mut stoch = Stochastic::default(); // 使用默认参数 (14, 3)
if let Some(output) = stoch.update(110.0, 90.0, 100.0) {
    println!("%K: {:.2}, %D: {:.2}", output.k, output.d);
}
```

#### 6. ROC (变动率指标)
衡量当前价格相对于N周期前价格的变化百分比。

```rust
use aurora_indicators::ROC;

let mut roc = ROC::new(10);
for _ in 0..10 {
    roc.update(100.0);
}
if let Some(roc_value) = roc.update(110.0) {
    println!("ROC: {:.2}%", roc_value); // 输出: ROC: 10.00%
}
```

#### 7. CCI (商品通道指数)
衡量价格相对于统计平均值的偏离程度。

```rust
use aurora_indicators::CCI;

let mut cci = CCI::new(20);
if let Some(cci_value) = cci.update(110.0, 90.0, 100.0) {
    if cci_value > 100.0 {
        println!("超买! CCI = {:.2}", cci_value);
    } else if cci_value < -100.0 {
        println!("超卖! CCI = {:.2}", cci_value);
    }
}
```

#### 8. Williams %R (威廉指标)
衡量收盘价在N周期高低区间中的相对位置。

```rust
use aurora_indicators::WilliamsR;

let mut wr = WilliamsR::new(14);
if let Some(wr_value) = wr.update(110.0, 90.0, 100.0) {
    if WilliamsR::is_overbought(wr_value, -20.0) {
        println!("超买! Williams %%R = {:.2}", wr_value);
    } else if WilliamsR::is_oversold(wr_value, -80.0) {
        println!("超卖! Williams %%R = {:.2}", wr_value);
    }
}
```

### 波动率指标

#### 9. Bollinger Bands (布林带)
基于标准差的价格通道，衡量价格波动范围。

```rust
use aurora_indicators::BollingerBands;

let mut bb = BollingerBands::default(); // 使用默认参数 (20, 2.0)
if let Some(bands) = bb.update(100.0) {
    println!("上轨: {:.2}, 中轨: {:.2}, 下轨: {:.2}", 
        bands.upper, bands.middle, bands.lower);
    
    // 计算%B指标
    if let Some(percent_b) = bb.percent_b(100.0) {
        println!("%B: {:.2}", percent_b);
    }
}
```

#### 10. ATR (平均真实波幅)
衡量市场波动程度，常用于设置止损位置。

```rust
use aurora_indicators::ATR;

let mut atr = ATR::default(); // 使用默认周期 14
if let Some(atr_value) = atr.update(110.0, 90.0, 100.0) {
    println!("ATR: {:.2}", atr_value);
    
    // 计算止损价格
    if let Some(stop_loss) = atr.stop_loss(100.0, 2.0, true) {
        println!("多头止损: {:.2}", stop_loss);
    }
}
```

#### 11. StdDev (标准差)
衡量价格相对于平均值的离散程度，是布林带的基础。

```rust
use aurora_indicators::StdDev;

let mut stddev = StdDev::new(20);
if let Some(std_value) = stddev.update(100.0) {
    println!("标准差: {:.2}", std_value);
    
    // 获取平均值
    if let Some(mean) = stddev.mean() {
        println!("平均值: {:.2}", mean);
    }
}
```

### 成交量指标

#### 12. OBV (能量潮)
通过成交量变化预测价格趋势。

```rust
use aurora_indicators::OBV;

let mut obv = OBV::new();
obv.update(100.0, 1000.0);
let obv_value = obv.update(105.0, 1500.0);
println!("OBV: {:.2}", obv_value);
```

## 代码组织

遵循项目约定，代码按照以下结构组织：

```
aurora-indicators/
├── src/
│   ├── lib.rs              # 库入口和公开导出
│   ├── ma.rs               # 移动平均线
│   ├── ma/tests.rs         # MA 单元测试
│   ├── ema.rs              # 指数移动平均线
│   ├── ema/tests.rs        # EMA 单元测试
│   ├── macd.rs             # MACD
│   ├── macd/tests.rs       # MACD 单元测试
│   ├── adx.rs              # 平均动向指数
│   ├── adx/tests.rs        # ADX 单元测试
│   ├── rsi.rs              # 相对强弱指数
│   ├── rsi/tests.rs        # RSI 单元测试
│   ├── stochastic.rs       # 随机震荡指标
│   ├── stochastic/tests.rs # Stochastic 单元测试
│   ├── roc.rs              # 变动率指标
│   ├── cci.rs              # 商品通道指数
│   ├── williams_r.rs       # 威廉指标
│   ├── bollinger.rs        # 布林带
│   ├── bollinger/tests.rs  # Bollinger Bands 单元测试
│   ├── atr.rs              # 平均真实波幅
│   ├── atr/tests.rs        # ATR 单元测试
│   ├── stddev.rs           # 标准差
│   ├── obv.rs              # 能量潮
│   └── obv/tests.rs        # OBV 单元测试
└── tests/
    └── integration_tests.rs # 集成测试
```

每个指标文件：
- 不超过 500 行代码（符合项目约定）
- 包含完整的 Apache 2.0 许可证头
- 详细的文档注释（///）
- 独立的测试文件

## 测试覆盖

所有指标都经过详尽测试：

```bash
cargo test
```

测试覆盖：
- 基本功能测试
- 边界条件测试
- 极端值处理
- 精度验证
- 克隆和序列化
- 错误处理（panic 测试）

总计超过 189 个单元测试、26 个文档测试和 14 个集成测试。

## 设计原则

### 1. 高内聚、低耦合
- 每个指标独立封装
- 清晰的接口定义
- 最小化依赖

### 2. 内存效率
- 使用滑动窗口（VecDeque）避免存储全部历史数据
- 只保留计算所需的最少数据点

### 3. 计算效率
- 大多数指标 O(1) 时间复杂度
- 使用增量计算避免重复运算
- 适合实时流式数据处理

### 4. 类型安全
- 充分利用 Rust 的类型系统
- Option 类型处理可能缺失的值
- 编译时捕获错误

## 常用方法

所有指标都实现了以下通用方法：

- `new()` / `default()` - 创建新实例
- `update()` - 更新指标值
- `value()` - 获取当前值
- `reset()` - 重置状态
- `is_empty()` - 检查是否为空
- `is_ready()` - 检查是否准备好

## 许可证

Copyright 2025 blingbling21

Licensed under the Apache License, Version 2.0
