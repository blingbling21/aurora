# Aurora Indicators

Aurora 技术指标库 - 为量化交易策略提供全面的技术分析指标

## 概述

`aurora-indicators` 是 Aurora 量化交易框架的技术指标库，提供了 20+ 种常用的技术分析指标实现。所有指标都采用流式计算方式，支持增量更新，适用于实时数据处理和历史数据回测。

## 设计特点

### 🎯 核心特性

- **流式计算**: 支持逐条数据增量更新，无需存储完整历史数据
- **内存高效**: 使用滑动窗口技术，内存占用固定
- **状态管理**: 每个指标维护自身状态，支持多策略并发
- **类型安全**: 充分利用 Rust 类型系统，编译期错误检查
- **零依赖**: 除核心库外无外部依赖，轻量级实现

### 📊 统一接口

所有指标都遵循统一的接口设计：

```rust
pub struct Indicator {
    // 指标状态
}

impl Indicator {
    pub fn new(period: usize) -> Self { }        // 创建指标
    pub fn update(&mut self, price: f64) -> Option<Output> { }  // 更新并返回值
    pub fn value(&self) -> Option<Output> { }    // 获取当前值
    pub fn reset(&mut self) { }                  // 重置状态
    pub fn is_ready(&self) -> bool { }           // 是否准备好
}
```

## 支持的指标

### 📈 趋势指标 (Trend Indicators)

用于识别和跟踪市场趋势方向。

#### MA - 移动平均线 (Moving Average)

最基础的趋势跟踪指标，计算指定周期内价格的算术平均值。

```rust
use aurora_indicators::MA;

let mut ma = MA::new(20);  // 20周期移动平均线

for price in prices {
    if let Some(avg) = ma.update(price) {
        println!("MA(20): {:.2}", avg);
    }
}
```

**用途**: 识别趋势方向、支撑阻力位、交叉信号

#### EMA - 指数移动平均线 (Exponential Moving Average)

对近期数据赋予更高权重的移动平均线。

```rust
use aurora_indicators::EMA;

let mut ema = EMA::new(12);  // 12周期EMA

for price in prices {
    let value = ema.update(price);  // 始终返回值
    println!("EMA(12): {:.2}", value);
}
```

**用途**: 快速响应价格变化、MACD计算基础

#### MACD - 移动平均收敛散度 (Moving Average Convergence Divergence)

通过快慢均线差异判断趋势变化和买卖时机。

```rust
use aurora_indicators::MACD;

let mut macd = MACD::new(12, 26, 9);  // 标准MACD参数
// 或使用默认值
let mut macd = MACD::default();

for price in prices {
    let output = macd.update(price);
    println!("MACD: {:.2}, Signal: {:.2}, Histogram: {:.2}",
        output.macd, output.signal, output.histogram);
}
```

**输出**:
- `macd`: MACD线 (快线-慢线)
- `signal`: 信号线 (MACD的EMA)
- `histogram`: 柱状图 (MACD-信号线)

**用途**: 趋势反转、买卖信号、背离分析

#### ADX - 平均动向指数 (Average Directional Index)

衡量趋势强度但不判断方向的指标。

```rust
use aurora_indicators::ADX;

let mut adx = ADX::new(14);

for kline in klines {
    if let Some(output) = adx.update(kline.high, kline.low, kline.close) {
        println!("ADX: {:.2}, +DI: {:.2}, -DI: {:.2}",
            output.adx, output.plus_di, output.minus_di);
    }
}
```

**输出**:
- `adx`: ADX值 (0-100)
- `plus_di`: 上升动向指标
- `minus_di`: 下降动向指标

**用途**: 趋势强度判断、过滤震荡行情

#### PSAR - 抛物线转向指标 (Parabolic SAR)

用于确定止损位和趋势反转点。

```rust
use aurora_indicators::PSAR;

let mut psar = PSAR::new(0.02, 0.2);  // 加速因子，最大加速

for kline in klines {
    if let Some(output) = psar.update(kline.high, kline.low) {
        println!("SAR: {:.2}, Trend: {:?}", output.sar, output.trend);
    }
}
```

**用途**: 止损位设置、趋势跟踪

#### Ichimoku - 一目均衡表 (Ichimoku Cloud)

综合性趋势系统，提供多维度市场信息。

```rust
use aurora_indicators::Ichimoku;

let mut ichimoku = Ichimoku::new(9, 26, 52);

for kline in klines {
    if let Some(output) = ichimoku.update(kline.high, kline.low, kline.close) {
        println!("转换线: {:.2}, 基准线: {:.2}", 
            output.tenkan_sen, output.kijun_sen);
        println!("先行带A: {:.2}, 先行带B: {:.2}",
            output.senkou_span_a, output.senkou_span_b);
    }
}
```

**输出**: 转换线、基准线、先行带A、先行带B、迟行带

**用途**: 多时间框架分析、支撑阻力、趋势云

### 🔥 动量指标 (Momentum Indicators)

衡量价格变动的速度和强度。

#### RSI - 相对强弱指数 (Relative Strength Index)

衡量价格变动速度和幅度，用于判断超买超卖状态。

```rust
use aurora_indicators::RSI;

let mut rsi = RSI::new(14);  // 14周期RSI

for price in prices {
    if let Some(value) = rsi.update(price) {
        println!("RSI: {:.2}", value);
        
        if value > 70.0 {
            println!("超买区域");
        } else if value < 30.0 {
            println!("超卖区域");
        }
    }
}
```

**范围**: 0-100
- **超买**: > 70
- **超卖**: < 30

**用途**: 超买超卖判断、背离分析、趋势确认

#### Stochastic - 随机震荡指标 (Stochastic Oscillator)

比较收盘价与价格区间的相对位置。

```rust
use aurora_indicators::Stochastic;

let mut stoch = Stochastic::new(14, 3, 3);  // %K周期, %K平滑, %D平滑

for kline in klines {
    if let Some(output) = stoch.update(kline.high, kline.low, kline.close) {
        println!("%%K: {:.2}, %%D: {:.2}", output.k, output.d);
    }
}
```

**输出**:
- `k`: %K值 (快线)
- `d`: %D值 (慢线)

**用途**: 超买超卖、交叉信号、背离

#### ROC - 变动率指标 (Rate of Change)

衡量当前价格相对于N周期前的变化百分比。

```rust
use aurora_indicators::ROC;

let mut roc = ROC::new(12);

for price in prices {
    if let Some(value) = roc.update(price) {
        println!("ROC: {:.2}%", value);
    }
}
```

**用途**: 动量强度、趋势确认

#### CCI - 商品通道指数 (Commodity Channel Index)

衡量价格相对于统计平均值的偏离程度。

```rust
use aurora_indicators::CCI;

let mut cci = CCI::new(20);

for kline in klines {
    if let Some(value) = cci.update(kline.high, kline.low, kline.close) {
        println!("CCI: {:.2}", value);
    }
}
```

**范围**: 通常在 -100 到 +100 之间
- **超买**: > +100
- **超卖**: < -100

**用途**: 识别超买超卖、趋势判断

#### Williams %R - 威廉指标 (Williams Percent Range)

衡量收盘价在N周期高低区间中的相对位置。

```rust
use aurora_indicators::WilliamsR;

let mut williams = WilliamsR::new(14);

for kline in klines {
    if let Some(value) = williams.update(kline.high, kline.low, kline.close) {
        println!("Williams %%R: {:.2}", value);
    }
}
```

**范围**: -100 到 0
- **超买**: > -20
- **超卖**: < -80

**用途**: 超买超卖、反转信号

### 📊 波动率指标 (Volatility Indicators)

衡量市场波动程度和价格范围。

#### Bollinger Bands - 布林带 (Bollinger Bands)

基于标准差的价格通道，衡量波动范围。

```rust
use aurora_indicators::BollingerBands;

let mut bb = BollingerBands::new(20, 2.0);  // 20周期, 2倍标准差
// 或使用默认值
let mut bb = BollingerBands::default();

for price in prices {
    if let Some(bands) = bb.update(price) {
        println!("上轨: {:.2}", bands.upper);
        println!("中轨: {:.2}", bands.middle);
        println!("下轨: {:.2}", bands.lower);
    }
}
```

**输出**:
- `upper`: 上轨 (中轨 + K×标准差)
- `middle`: 中轨 (移动平均线)
- `lower`: 下轨 (中轨 - K×标准差)

**用途**: 支撑阻力、超买超卖、波动率分析

#### ATR - 平均真实波幅 (Average True Range)

衡量市场波动程度的指标。

```rust
use aurora_indicators::ATR;

let mut atr = ATR::new(14);

for kline in klines {
    if let Some(value) = atr.update(kline.high, kline.low, kline.close) {
        println!("ATR: {:.2}", value);
    }
}
```

**用途**: 止损位设置、波动率衡量、仓位管理

#### StdDev - 标准差 (Standard Deviation)

衡量价格相对于平均值的离散程度。

```rust
use aurora_indicators::StdDev;

let mut stddev = StdDev::new(20);

for price in prices {
    if let Some(value) = stddev.update(price) {
        println!("标准差: {:.2}", value);
    }
}
```

**用途**: 波动率分析、布林带计算

#### Keltner Channels - 肯特纳通道 (Keltner Channels)

基于ATR的价格通道。

```rust
use aurora_indicators::KeltnerChannels;

let mut keltner = KeltnerChannels::new(20, 2.0);

for kline in klines {
    if let Some(channels) = keltner.update(kline.high, kline.low, kline.close) {
        println!("上轨: {:.2}, 中轨: {:.2}, 下轨: {:.2}",
            channels.upper, channels.middle, channels.lower);
    }
}
```

**用途**: 趋势跟踪、支撑阻力

### 📦 成交量指标 (Volume Indicators)

结合成交量分析市场行为。

#### OBV - 能量潮 (On-Balance Volume)

通过成交量变化预测价格趋势。

```rust
use aurora_indicators::OBV;

let mut obv = OBV::new();

for kline in klines {
    let value = obv.update(kline.close, kline.volume);
    println!("OBV: {:.2}", value);
}
```

**用途**: 趋势确认、背离分析

#### MFI - 资金流量指数 (Money Flow Index)

成交量加权的RSI指标。

```rust
use aurora_indicators::MFI;

let mut mfi = MFI::new(14);

for kline in klines {
    if let Some(value) = mfi.update(
        kline.high, kline.low, kline.close, kline.volume
    ) {
        println!("MFI: {:.2}", value);
    }
}
```

**范围**: 0-100
- **超买**: > 80
- **超卖**: < 20

**用途**: 资金流向分析、超买超卖

#### VWAP - 成交量加权平均价 (Volume Weighted Average Price)

计算成交量加权的平均价格。

```rust
use aurora_indicators::VWAP;

let mut vwap = VWAP::new();

for kline in klines {
    let value = vwap.update(kline.close, kline.volume);
    println!("VWAP: {:.2}", value);
}
```

**用途**: 日内交易基准、机构成本分析

#### CMF - 佳庆资金流 (Chaikin Money Flow)

衡量特定时期内资金流入和流出情况。

```rust
use aurora_indicators::CMF;

let mut cmf = CMF::new(20);

for kline in klines {
    if let Some(value) = cmf.update(
        kline.high, kline.low, kline.close, kline.volume
    ) {
        println!("CMF: {:.2}", value);
    }
}
```

**范围**: -1 到 +1
- **正值**: 买压强
- **负值**: 卖压强

**用途**: 资金流向、趋势确认

#### ADLine - 累积/派发线 (Accumulation/Distribution Line)

通过价格和成交量关系识别供需变化。

```rust
use aurora_indicators::ADLine;

let mut adline = ADLine::new();

for kline in klines {
    let value = adline.update(
        kline.high, kline.low, kline.close, kline.volume
    );
    println!("A/D Line: {:.2}", value);
}
```

**用途**: 趋势确认、背离分析

## 使用示例

### 基本用法

```rust
use aurora_indicators::{MA, RSI, MACD};

fn main() {
    // 创建指标实例
    let mut ma20 = MA::new(20);
    let mut ma50 = MA::new(50);
    let mut rsi = RSI::new(14);
    let mut macd = MACD::default();
    
    // 模拟价格数据
    let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0];
    
    for price in prices {
        // 更新指标
        if let Some(ma20_val) = ma20.update(price) {
            println!("MA(20): {:.2}", ma20_val);
        }
        
        if let Some(ma50_val) = ma50.update(price) {
            println!("MA(50): {:.2}", ma50_val);
        }
        
        if let Some(rsi_val) = rsi.update(price) {
            println!("RSI: {:.2}", rsi_val);
        }
        
        let macd_val = macd.update(price);
        println!("MACD: {:.2}", macd_val.macd);
    }
}
```

### 策略中使用

```rust
use aurora_indicators::{MA, RSI};
use aurora_core::{Strategy, MarketEvent, SignalEvent, Signal};

pub struct MyStrategy {
    ma_short: MA,
    ma_long: MA,
    rsi: RSI,
}

impl MyStrategy {
    pub fn new() -> Self {
        Self {
            ma_short: MA::new(10),
            ma_long: MA::new(30),
            rsi: RSI::new(14),
        }
    }
}

impl Strategy for MyStrategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        if let MarketEvent::Kline(kline) = event {
            // 更新指标
            let ma_short = self.ma_short.update(kline.close)?;
            let ma_long = self.ma_long.update(kline.close)?;
            let rsi = self.rsi.update(kline.close)?;
            
            // 生成交易信号
            let signal = if ma_short > ma_long && rsi < 70.0 {
                Signal::Buy
            } else if ma_short < ma_long || rsi > 70.0 {
                Signal::Sell
            } else {
                Signal::Hold
            };
            
            Some(SignalEvent {
                signal,
                price: kline.close,
                timestamp: kline.timestamp,
            })
        } else {
            None
        }
    }
}
```

### 多指标组合

```rust
use aurora_indicators::{BollingerBands, RSI, MACD, ATR};

struct MultiIndicatorAnalyzer {
    bb: BollingerBands,
    rsi: RSI,
    macd: MACD,
    atr: ATR,
}

impl MultiIndicatorAnalyzer {
    fn new() -> Self {
        Self {
            bb: BollingerBands::default(),
            rsi: RSI::new(14),
            macd: MACD::default(),
            atr: ATR::new(14),
        }
    }
    
    fn analyze(&mut self, high: f64, low: f64, close: f64) -> Analysis {
        // 更新所有指标
        let bb = self.bb.update(close);
        let rsi = self.rsi.update(close);
        let macd = self.macd.update(close);
        let atr = self.atr.update(high, low, close);
        
        // 综合分析
        Analysis {
            trend: self.analyze_trend(&macd),
            momentum: self.analyze_momentum(&rsi),
            volatility: self.analyze_volatility(&bb, &atr),
        }
    }
    
    fn analyze_trend(&self, macd: &MACDOutput) -> Trend {
        if macd.histogram > 0.0 {
            Trend::Bullish
        } else {
            Trend::Bearish
        }
    }
    
    // ... 其他分析方法
}
```

### 批量数据处理

```rust
use aurora_indicators::MA;
use aurora_core::Kline;

fn process_historical_data(klines: &[Kline]) {
    let mut ma = MA::new(20);
    let mut results = Vec::new();
    
    for kline in klines {
        if let Some(ma_value) = ma.update(kline.close) {
            results.push((kline.timestamp, ma_value));
        }
    }
    
    // 输出结果
    for (timestamp, value) in results {
        println!("{}: {:.2}", timestamp, value);
    }
}
```

## 性能特点

### 时间复杂度

- **MA, EMA, RSI**: O(1) 每次更新
- **Bollinger Bands**: O(1) 每次更新（使用增量标准差）
- **MACD**: O(1) 每次更新
- **ATR**: O(1) 每次更新

### 空间复杂度

- **MA**: O(N) - N为周期长度
- **EMA**: O(1) - 只存储当前值
- **RSI**: O(1) - 使用EMA实现
- **Bollinger Bands**: O(N) - 存储N个价格用于标准差计算
- **ATR**: O(1) - 使用EMA实现

### 内存优化

所有指标都使用滑动窗口或增量计算，避免存储完整历史数据：

```rust
// MA 使用 VecDeque 实现固定大小窗口
pub struct MA {
    period: usize,
    values: VecDeque<f64>,  // 最多存储 period 个值
    sum: f64,                // 维护累计和，O(1)更新
}

// EMA 只需要存储当前值
pub struct EMA {
    period: usize,
    multiplier: f64,
    current_ema: Option<f64>,  // 只存储一个值
}
```

## 测试

每个指标都包含完整的单元测试：

```bash
# 运行所有测试
cargo test --package aurora-indicators

# 测试特定指标
cargo test --package aurora-indicators ma::tests
cargo test --package aurora-indicators rsi::tests
cargo test --package aurora-indicators macd::tests

# 运行集成测试
cargo test --package aurora-indicators --test integration_tests
```

## 依赖项

```toml
[dependencies]
aurora-core = { path = "../aurora-core" }

[dev-dependencies]
approx = "0.5"  # 浮点数比较
```

## 扩展指标

### 添加新指标

1. 在 `src/` 目录创建新文件（如 `my_indicator.rs`）
2. 实现指标结构和方法
3. 在 `src/lib.rs` 中导出

```rust
// src/my_indicator.rs
pub struct MyIndicator {
    period: usize,
    // ... 状态变量
}

impl MyIndicator {
    pub fn new(period: usize) -> Self {
        // 初始化
    }
    
    pub fn update(&mut self, price: f64) -> Option<f64> {
        // 更新逻辑
    }
    
    pub fn value(&self) -> Option<f64> {
        // 返回当前值
    }
    
    pub fn reset(&mut self) {
        // 重置状态
    }
}

#[cfg(test)]
mod tests;
```

## 设计原则

1. **简洁API**: 统一的创建、更新、获取接口
2. **状态封装**: 每个指标管理自己的状态，互不干扰
3. **类型安全**: 利用Rust类型系统防止错误
4. **内存高效**: 使用滑动窗口和增量计算
5. **文档完善**: 每个指标都有详细的注释和示例
6. **充分测试**: 单元测试覆盖所有功能

## 常见问题

### Q: 为什么有些指标返回 Option，有些不返回？

A: 返回 `Option` 的指标需要足够的数据点才能产生有效结果（如MA需要N个数据点）。而像EMA这样的指标可以从第一个数据点就开始计算，所以直接返回值。

### Q: 如何处理指标未准备好的情况？

A: 使用 `is_ready()` 方法检查或使用模式匹配：

```rust
if ma.is_ready() {
    let value = ma.value().unwrap();
}

// 或
if let Some(value) = ma.update(price) {
    // 使用value
}
```

### Q: 可以并发使用同一个指标吗？

A: 不可以。每个指标实例维护自己的状态。如果需要并发处理，为每个线程创建独立的指标实例。

### Q: 指标计算结果有误差正常吗？

A: 由于浮点数精度问题，可能存在微小误差。测试中使用 `approx` crate 进行近似比较。

## 相关 Crate

- **aurora-core**: 核心数据结构和接口
- **aurora-strategy**: 使用这些指标构建交易策略
- **aurora-backtester**: 在历史数据上测试指标效果
- **aurora-live**: 在实时数据中应用指标

## 版本

当前版本: **0.1.0**

## 许可证

本项目采用 Apache License 2.0 许可证。详见根目录的 LICENSE 文件。

## 参考资料

- [Technical Analysis Library](https://www.investopedia.com/terms/t/technicalanalysis.asp)
- [TA-Lib](https://www.ta-lib.org/)
- [TradingView Indicators](https://www.tradingview.com/scripts/)
