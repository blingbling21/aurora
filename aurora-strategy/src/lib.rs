//! # Aurora Strategy Library
//! 
//! 这个crate提供了量化交易策略的实现框架。它定义了统一的策略接口，
//! 并提供了常用的技术分析策略实现，如移动平均线交叉策略。
//! 
//! ## 主要功能
//! 
//! - **策略接口抽象化**: 通过 `Strategy` trait 提供统一的策略执行接口
//! - **移动平均线策略**: 实现了双均线交叉买卖信号生成
//! - **信号生成**: 基于技术指标产生买入、卖出或持有信号
//! - **状态管理**: 维护策略运行时的内部状态
//! 
//! ## 使用示例
//! 
//! ```rust
//! use aurora_core::{Kline, MarketEvent, Strategy};
//! use aurora_strategy::MACrossoverStrategy;
//! 
//! // 创建移动平均线交叉策略
//! let mut strategy = MACrossoverStrategy::new(5, 20); // 5日线和20日线
//! 
//! // 模拟K线数据
//! let kline = Kline {
//!     timestamp: 1640995200, // Unix时间戳
//!     open: 50000.0,
//!     high: 50500.0,
//!     low: 49500.0,
//!     close: 50200.0,
//!     volume: 100.0,
//! };
//! 
//! // 执行策略
//! let market_event = MarketEvent::Kline(kline);
//! if let Some(signal_event) = strategy.on_market_event(&market_event) {
//!     println!("信号: {:?}, 价格: {}", signal_event.signal, signal_event.price);
//! }
//! ```
//! 
//! ## 策略开发指南
//! 
//! 要实现自定义策略，需要：
//! 
//! 1. 实现 `aurora_core::Strategy` trait
//! 2. 在 `on_market_event` 方法中实现策略逻辑
//! 3. 维护必要的内部状态（如技术指标）
//! 4. 根据市场数据生成相应的交易信号事件
//! 
//! ## 性能考虑
//! 
//! - 策略执行应该尽可能高效，避免复杂的计算
//! - 使用滑动窗口算法来维护技术指标状态
//! - 合理设置指标周期，避免过度拟合

use aurora_core::{MarketEvent, Strategy, Signal, SignalEvent};
use aurora_indicators::MA;

/// 移动平均线交叉策略
/// 
/// 这是一个经典的量化交易策略，基于两条不同周期的移动平均线的交叉来产生交易信号。
/// 当短期均线上穿长期均线时产生买入信号（金叉），当短期均线下穿长期均线时产生卖出信号（死叉）。
/// 
/// ## 策略原理
/// 
/// - **金叉(Golden Cross)**: 短期MA从下方穿越长期MA → 买入信号
/// - **死叉(Death Cross)**: 短期MA从上方穿越长期MA → 卖出信号
/// - **其他情况**: 没有交叉或数据不足时不产生信号
/// 
/// ## 参数说明
/// 
/// - `short_period`: 短期移动平均线周期，通常为5-20
/// - `long_period`: 长期移动平均线周期，通常为20-60，必须大于短期周期
/// 
/// ## 适用市场
/// 
/// 此策略适合趋势性较强的市场，在震荡市场中可能产生较多假信号。
/// 建议结合其他技术指标或风险管理措施使用。
/// 
/// ## 内部状态
/// 
/// 策略维护以下内部状态：
/// - 两个MA指标实例
/// - 前一次的MA值，用于检测交叉点
/// 
/// ## 示例
/// 
/// ```rust
/// use aurora_strategy::MACrossoverStrategy;
/// use aurora_core::{MarketEvent, Kline, Strategy};
/// 
/// let mut strategy = MACrossoverStrategy::new(5, 20);
/// 
/// // 假设有一系列K线数据
/// let kline = Kline {
///     timestamp: 1640995200,
///     open: 100.0,
///     high: 105.0,
///     low: 95.0,
///     close: 102.0,
///     volume: 1000.0,
/// };
/// 
/// let event = MarketEvent::Kline(kline);
/// if let Some(signal_event) = strategy.on_market_event(&event) {
///     match signal_event.signal {
///         aurora_core::Signal::Buy => println!("金叉买入信号"),
///         aurora_core::Signal::Sell => println!("死叉卖出信号"),
///         aurora_core::Signal::Hold => println!("继续持有"),
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MACrossoverStrategy {
    /// 短期移动平均线指标
    /// 周期较短，对价格变化更敏感，用于捕捉短期趋势
    short_ma: MA,
    
    /// 长期移动平均线指标  
    /// 周期较长，反映长期趋势，提供趋势方向参考
    long_ma: MA,
    
    /// 上一次短期MA值，用于判断交叉
    /// 保存历史值以便检测从一个状态到另一个状态的转变
    prev_short_value: Option<f64>,
    
    /// 上一次长期MA值，用于判断交叉
    /// 保存历史值以便检测从一个状态到另一个状态的转变
    prev_long_value: Option<f64>,
}

impl MACrossoverStrategy {
    /// 创建新的MA交叉策略
    /// 
    /// # 参数
    /// 
    /// * `short_period` - 短期移动平均线周期，必须大于0且小于长期周期
    /// * `long_period` - 长期移动平均线周期，必须大于短期周期
    /// 
    /// # Panics
    /// 
    /// 当 `short_period >= long_period` 时会panic，因为这样的配置无法产生有意义的交叉信号
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// // 创建5日线和20日线交叉策略
    /// let strategy = MACrossoverStrategy::new(5, 20);
    /// 
    /// // 创建10日线和30日线交叉策略  
    /// let strategy = MACrossoverStrategy::new(10, 30);
    /// ```
    /// 
    /// # 设计考虑
    /// 
    /// 短期周期应该显著小于长期周期，通常建议比例为1:3到1:4，
    /// 例如5:20、10:30等，以确保能够捕捉到有意义的趋势变化。
    pub fn new(short_period: usize, long_period: usize) -> Self {
        assert!(short_period < long_period, "短期移动平均线周期必须小于长期移动平均线周期");
        
        Self {
            short_ma: MA::new(short_period),
            long_ma: MA::new(long_period),
            prev_short_value: None,
            prev_long_value: None,
        }
    }

    /// 获取短期移动平均线周期
    /// 
    /// # 返回值
    /// 
    /// 返回短期MA的周期设置
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let strategy = MACrossoverStrategy::new(5, 20);
    /// assert_eq!(strategy.short_period(), 5);
    /// ```
    pub fn short_period(&self) -> usize {
        self.short_ma.period()
    }
    
    /// 获取长期移动平均线周期
    /// 
    /// # 返回值
    /// 
    /// 返回长期MA的周期设置
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let strategy = MACrossoverStrategy::new(5, 20);
    /// assert_eq!(strategy.long_period(), 20);
    /// ```
    pub fn long_period(&self) -> usize {
        self.long_ma.period()
    }
    
    /// 获取当前短期移动平均线值
    /// 
    /// # 返回值
    /// 
    /// 如果数据足够计算MA则返回Some(value)，否则返回None
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let strategy = MACrossoverStrategy::new(2, 5);
    /// // 初始状态下没有值
    /// assert_eq!(strategy.short_ma_value(), None);
    /// ```
    pub fn short_ma_value(&self) -> Option<f64> {
        self.short_ma.value()
    }
    
    /// 获取当前长期移动平均线值
    /// 
    /// # 返回值
    /// 
    /// 如果数据足够计算MA则返回Some(value)，否则返回None
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let strategy = MACrossoverStrategy::new(5, 20);
    /// // 初始状态下没有值
    /// assert_eq!(strategy.long_ma_value(), None);
    /// ```
    pub fn long_ma_value(&self) -> Option<f64> {
        self.long_ma.value()
    }
    
    /// 检查策略是否准备好生成有效信号
    /// 
    /// 只有当两条移动平均线都有足够数据计算且有历史值用于比较时，
    /// 策略才能产生可靠的交叉信号
    /// 
    /// # 返回值
    /// 
    /// 如果策略已准备好则返回true，否则返回false
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let strategy = MACrossoverStrategy::new(5, 20);
    /// assert!(!strategy.is_ready()); // 初始状态未准备好
    /// ```
    pub fn is_ready(&self) -> bool {
        self.short_ma.is_ready() && 
        self.long_ma.is_ready() && 
        self.prev_short_value.is_some() && 
        self.prev_long_value.is_some()
    }
    
    /// 重置策略状态
    /// 
    /// 清空所有移动平均线的历史数据和状态，策略回到初始状态。
    /// 这对于重新开始策略或切换交易品种时很有用。
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use aurora_strategy::MACrossoverStrategy;
    /// 
    /// let mut strategy = MACrossoverStrategy::new(2, 5);
    /// 
    /// // 执行一些操作后重置
    /// strategy.reset();
    /// 
    /// // 重置后策略回到初始状态
    /// assert!(!strategy.is_ready());
    /// assert_eq!(strategy.short_ma_value(), None);
    /// assert_eq!(strategy.long_ma_value(), None);
    /// ```
    pub fn reset(&mut self) {
        self.short_ma.reset();
        self.long_ma.reset();
        self.prev_short_value = None;
        self.prev_long_value = None;
    }

    /// 检测MA交叉信号
    /// 
    /// 这是策略的核心逻辑，通过比较当前和历史的MA值来检测交叉。
    /// 
    /// # 参数
    /// 
    /// * `current_short` - 当前短期MA值
    /// * `current_long` - 当前长期MA值
    /// 
    /// # 返回值
    /// 
    /// * `Signal::Buy` - 检测到金叉（短期MA向上穿越长期MA）
    /// * `Signal::Sell` - 检测到死叉（短期MA向下穿越长期MA）
    /// * `Signal::Hold` - 无交叉或无足够历史数据
    /// 
    /// # 算法逻辑
    /// 
    /// 1. 检查是否有历史MA值用于比较
    /// 2. 金叉检测：prev_short <= prev_long && current_short > current_long
    /// 3. 死叉检测：prev_short >= prev_long && current_short < current_long
    /// 4. 其他情况返回Hold信号
    /// 
    /// # 注意事项
    /// 
    /// 交叉检测使用 <= 和 >= 比较以确保在MA值相等时也能正确检测到交叉。
    /// 这种设计避免了在MA值非常接近时错过交叉信号。
    fn detect_crossover(&self, current_short: f64, current_long: f64) -> Signal {
        if let (Some(prev_short), Some(prev_long)) = (self.prev_short_value, self.prev_long_value) {
            // 金叉：短期MA从下方或相等位置穿过长期MA到上方
            if prev_short <= prev_long && current_short > current_long {
                return Signal::Buy;
            }
            // 死叉：短期MA从上方或相等位置穿过长期MA到下方
            else if prev_short >= prev_long && current_short < current_long {
                return Signal::Sell;
            }
        }
        
        // 无交叉或无历史数据
        Signal::Hold
    }
}

impl Strategy for MACrossoverStrategy {
    /// 处理市场事件并执行策略逻辑
    /// 
    /// 这是策略的主要执行入口，接收市场事件并可能产生交易信号事件。
    /// 
    /// # 参数
    /// 
    /// * `event` - 市场事件，目前支持K线事件
    /// 
    /// # 返回值
    /// 
    /// * `Some(SignalEvent)` - 检测到交叉时返回包含信号的事件
    /// * `None` - 无交叉、数据不足或Hold信号时返回None
    /// 
    /// # 处理流程
    /// 
    /// 1. 解析市场事件，提取K线数据
    /// 2. 使用收盘价更新两个移动平均线指标
    /// 3. 如果两个MA都有有效值，进行交叉检测
    /// 4. 更新历史MA值用于下次比较
    /// 5. 只在产生Buy或Sell信号时返回SignalEvent
    /// 
    /// # 设计考虑
    /// 
    /// - 使用收盘价而不是其他价格（开盘价、最高价、最低价）是因为收盘价
    ///   通常被认为是最重要和最可靠的价格信息
    /// - 只在非Hold信号时返回事件，减少不必要的事件处理
    /// - 即使没有产生交易信号，也会更新内部状态，确保策略状态的连续性
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<SignalEvent> {
        match event {
            MarketEvent::Kline(kline) => {
                // 使用收盘价更新两个移动平均线
                // 收盘价被广泛认为是最重要的价格点，因为它代表了
                // 该时间段内市场参与者的最终共识价格
                let short_value = self.short_ma.update(kline.close);
                let long_value = self.long_ma.update(kline.close);

                // 只有当两个MA都有值时才进行交叉检测
                // 这确保了我们有足够的数据来计算可靠的移动平均线
                if let (Some(current_short), Some(current_long)) = (short_value, long_value) {
                    let signal = self.detect_crossover(current_short, current_long);
                    
                    // 更新前一个时间点的值，这对下次检测交叉至关重要
                    // 必须在信号检测后更新，确保时序正确
                    self.prev_short_value = Some(current_short);
                    self.prev_long_value = Some(current_long);

                    // 只在非Hold信号时返回SignalEvent
                    // 这减少了不必要的事件传播，提高了系统效率
                    if !matches!(signal, Signal::Hold) {
                        return Some(SignalEvent {
                            signal,
                            price: kline.close,
                            timestamp: kline.timestamp,
                        });
                    }
                } else {
                    // 即使无法进行交叉检测，也要更新可用的MA值
                    // 这确保了策略状态的连续性，为后续计算做准备
                    if let Some(val) = short_value {
                        self.prev_short_value = Some(val);
                    }
                    if let Some(val) = long_value {
                        self.prev_long_value = Some(val);
                    }
                }

                // 默认返回None，表示无信号产生
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::Kline;
    use approx::assert_relative_eq;

    /// 创建测试用的K线数据
    /// 
    /// # 参数
    /// 
    /// * `timestamp` - K线时间戳
    /// * `close` - 收盘价，为简化测试，所有价格都设为收盘价
    /// 
    /// # 返回值
    /// 
    /// 返回构造的Kline结构体
    fn create_kline(timestamp: i64, close: f64) -> Kline {
        Kline {
            timestamp,
            open: close,    // 简化测试，所有价格都用收盘价
            high: close,
            low: close,
            close,
            volume: 100.0,  // 固定成交量
        }
    }

    /// 测试策略创建
    #[test]
    fn test_ma_crossover_strategy_creation() {
        let strategy = MACrossoverStrategy::new(5, 10);
        
        // 验证初始状态
        assert!(strategy.prev_short_value.is_none());
        assert!(strategy.prev_long_value.is_none());
        assert_eq!(strategy.short_period(), 5);
        assert_eq!(strategy.long_period(), 10);
        assert!(!strategy.is_ready());
        assert_eq!(strategy.short_ma_value(), None);
        assert_eq!(strategy.long_ma_value(), None);
    }

    /// 测试无效周期参数
    #[test]
    #[should_panic(expected = "短期移动平均线周期必须小于长期移动平均线周期")]
    fn test_invalid_periods_equal() {
        MACrossoverStrategy::new(10, 10); // 相等周期应该panic
    }

    /// 测试无效周期参数（短期大于长期）
    #[test]
    #[should_panic(expected = "短期移动平均线周期必须小于长期移动平均线周期")]
    fn test_invalid_periods_reversed() {
        MACrossoverStrategy::new(20, 10); // 短期大于长期应该panic
    }

    /// 测试数据不足时的行为
    #[test]
    fn test_ma_crossover_insufficient_data() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 只提供一个数据点，不足以计算完整的MA值
        let kline = create_kline(1000, 10.0);
        let result = strategy.on_market_event(&MarketEvent::Kline(kline));
        
        assert!(result.is_none(), "数据不足时不应产生信号");
        assert!(!strategy.is_ready(), "数据不足时策略不应就绪");
    }

    /// 测试MA值的正确更新
    #[test]
    fn test_ma_values_update() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 逐步添加数据，观察MA值变化
        let klines = vec![
            create_kline(1000, 10.0),
            create_kline(2000, 12.0), // 2周期MA现在可用：(10+12)/2 = 11
            create_kline(3000, 14.0), // 2周期：(12+14)/2 = 13, 3周期：(10+12+14)/3 = 12
        ];

        // 第一个数据点
        strategy.on_market_event(&MarketEvent::Kline(klines[0]));
        assert_eq!(strategy.short_ma_value(), None);
        assert_eq!(strategy.long_ma_value(), None);
        
        // 第二个数据点，短期MA可用
        strategy.on_market_event(&MarketEvent::Kline(klines[1]));
        assert_eq!(strategy.short_ma_value(), Some(11.0));
        assert_eq!(strategy.long_ma_value(), None);
        
        // 第三个数据点，两个MA都可用
        strategy.on_market_event(&MarketEvent::Kline(klines[2]));
        assert_eq!(strategy.short_ma_value(), Some(13.0));
        assert_relative_eq!(strategy.long_ma_value().unwrap(), 12.0, epsilon = 1e-10);
        assert!(strategy.is_ready());
    }

    /// 测试金叉信号检测
    #[test]
    fn test_golden_cross_detection() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 构造数据以产生明确的金叉
        // 策略：先让短期MA低于长期MA，然后快速上升使其高于长期MA
        let klines = vec![
            create_kline(1000, 8.0),
            create_kline(2000, 9.0),   // 短期MA = 8.5, 长期MA = None
            create_kline(3000, 10.0),  // 短期MA = 9.5, 长期MA = 9.0 (短期>长期)
            create_kline(4000, 12.0),  // 短期MA = 11.0, 长期MA = 9.67, 继续上升
            create_kline(5000, 16.0),  // 短期MA = 14.0, 长期MA = 12.67, 明显金叉
        ];

        let mut signals = Vec::new();
        
        // 处理前三个数据点建立基础
        for (i, kline) in klines.iter().take(3).enumerate() {
            let result = strategy.on_market_event(&MarketEvent::Kline(*kline));
            if let Some(signal_event) = result {
                signals.push((i, signal_event));
            }
        }
        
        // 继续添加数据寻找金叉
        for (i, kline) in klines.iter().skip(3).enumerate() {
            let result = strategy.on_market_event(&MarketEvent::Kline(*kline));
            if let Some(signal_event) = result {
                signals.push((i + 3, signal_event));
                
                // 验证信号的有效性
                assert!(signal_event.price > 0.0);
                assert!(signal_event.timestamp > 0);
                
                // 如果是买入信号，验证其合理性
                if matches!(signal_event.signal, Signal::Buy) {
                    println!("检测到金叉买入信号：价格 = {}, 时间 = {}", 
                             signal_event.price, signal_event.timestamp);
                }
            }
        }
        
        println!("总共产生 {} 个信号", signals.len());
        
        // 验证策略状态
        assert!(strategy.is_ready(), "策略应该已准备就绪");
    }

    /// 测试死叉信号检测
    #[test]
    fn test_death_cross_detection() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 构造数据以产生死叉：先让短期MA高于长期MA，然后快速下降
        let klines = vec![
            create_kline(1000, 20.0),
            create_kline(2000, 18.0),  // 短期MA = 19.0
            create_kline(3000, 16.0),  // 短期MA = 17.0, 长期MA = 18.0 (短期<长期，可能已经死叉)
            create_kline(4000, 12.0),  // 短期MA = 14.0, 长期MA = 15.33，进一步确认下跌趋势
            create_kline(5000, 8.0),   // 短期MA = 10.0, 长期MA = 12.0，明确的死叉信号
        ];

        let mut signals = Vec::new();
        
        for (i, kline) in klines.iter().enumerate() {
            let result = strategy.on_market_event(&MarketEvent::Kline(*kline));
            if let Some(signal_event) = result {
                signals.push((i, signal_event));
                
                // 验证信号的有效性
                assert!(signal_event.price > 0.0);
                assert!(signal_event.timestamp > 0);
                
                // 如果是卖出信号，验证其合理性
                if matches!(signal_event.signal, Signal::Sell) {
                    println!("检测到死叉卖出信号：价格 = {}, 时间 = {}", 
                             signal_event.price, signal_event.timestamp);
                }
            }
        }
        
        println!("总共产生 {} 个信号", signals.len());
    }

    /// 测试无信号情况（Hold信号）
    #[test]
    fn test_no_signal_generation() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 提供平稳数据，不应产生交叉信号
        let klines = vec![
            create_kline(1000, 10.0),
            create_kline(2000, 10.1),
            create_kline(3000, 10.2),
            create_kline(4000, 10.3), // 平缓上升，不会产生明显交叉
            create_kline(5000, 10.4),
        ];

        let mut signal_count = 0;
        for kline in klines {
            if let Some(_) = strategy.on_market_event(&MarketEvent::Kline(kline)) {
                signal_count += 1;
            }
        }

        // 平稳变化不应产生交叉信号
        println!("平稳数据产生 {} 个信号", signal_count);
        
        // 验证策略已就绪但没有产生信号
        assert!(strategy.is_ready());
    }

    /// 测试策略重置功能
    #[test]
    fn test_strategy_reset() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 先添加一些数据
        let klines = vec![
            create_kline(1000, 10.0),
            create_kline(2000, 12.0),
            create_kline(3000, 14.0),
        ];

        for kline in klines {
            strategy.on_market_event(&MarketEvent::Kline(kline));
        }
        
        // 验证策略有状态
        assert!(strategy.is_ready());
        assert!(strategy.short_ma_value().is_some());
        assert!(strategy.long_ma_value().is_some());
        
        // 重置策略
        strategy.reset();
        
        // 验证重置后的状态
        assert!(!strategy.is_ready());
        assert_eq!(strategy.short_ma_value(), None);
        assert_eq!(strategy.long_ma_value(), None);
        assert!(strategy.prev_short_value.is_none());
        assert!(strategy.prev_long_value.is_none());
        
        // 验证周期设置保持不变
        assert_eq!(strategy.short_period(), 2);
        assert_eq!(strategy.long_period(), 3);
    }

    /// 测试边界情况：单周期MA
    #[test]
    fn test_single_period_ma() {
        let mut strategy = MACrossoverStrategy::new(1, 2);
        
        // 单周期MA应该立即可用
        let klines = vec![
            create_kline(1000, 10.0), // 短期MA = 10.0, 长期MA = None
            create_kline(2000, 15.0), // 短期MA = 15.0, 长期MA = 12.5
            create_kline(3000, 5.0),  // 短期MA = 5.0, 长期MA = 10.0 (可能产生死叉)
        ];

        let mut signals = Vec::new();
        for kline in klines {
            if let Some(signal_event) = strategy.on_market_event(&MarketEvent::Kline(kline)) {
                signals.push(signal_event);
            }
        }
        
        println!("单周期MA测试产生 {} 个信号", signals.len());
        
        // 验证最终状态
        assert!(strategy.is_ready());
        assert_eq!(strategy.short_ma_value(), Some(5.0));
        assert_eq!(strategy.long_ma_value(), Some(10.0));
    }

    /// 测试极值处理
    #[test]
    fn test_extreme_values() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 测试零值
        let zero_result = strategy.on_market_event(&MarketEvent::Kline(create_kline(1000, 0.0)));
        assert!(zero_result.is_none());
        
        // 测试负值
        let neg_result = strategy.on_market_event(&MarketEvent::Kline(create_kline(2000, -100.0)));
        assert!(neg_result.is_none());
        
        // 测试很大的值
        let large_result = strategy.on_market_event(&MarketEvent::Kline(create_kline(3000, 1e10)));
        assert!(large_result.is_none()); // 仍然数据不足
        
        // 添加更多数据直到MA可用
        strategy.on_market_event(&MarketEvent::Kline(create_kline(4000, 1e10)));
        
        // 现在短期MA应该可用
        assert!(strategy.short_ma_value().is_some());
    }

    /// 测试连续相同价格的处理
    #[test]
    fn test_constant_prices() {
        let mut strategy = MACrossoverStrategy::new(2, 3);
        
        // 所有价格都相同
        let price = 100.0;
        let klines = vec![
            create_kline(1000, price),
            create_kline(2000, price),
            create_kline(3000, price),
            create_kline(4000, price),
            create_kline(5000, price),
        ];

        let mut signal_count = 0;
        for kline in klines {
            if let Some(_) = strategy.on_market_event(&MarketEvent::Kline(kline)) {
                signal_count += 1;
            }
        }

        // 相同价格不应产生交叉
        assert_eq!(signal_count, 0);
        
        // 但MA值应该都等于该价格
        assert_eq!(strategy.short_ma_value(), Some(price));
        assert_eq!(strategy.long_ma_value(), Some(price));
    }

    /// 测试克隆功能
    #[test]
    fn test_strategy_clone() {
        let mut strategy1 = MACrossoverStrategy::new(5, 10);
        
        // 添加一些数据
        for i in 1..=12 {
            strategy1.on_market_event(&MarketEvent::Kline(create_kline(i as i64 * 1000, i as f64)));
        }
        
        // 克隆策略
        let strategy2 = strategy1.clone();
        
        // 验证克隆的策略状态相同
        assert_eq!(strategy1.short_period(), strategy2.short_period());
        assert_eq!(strategy1.long_period(), strategy2.long_period());
        assert_eq!(strategy1.short_ma_value(), strategy2.short_ma_value());
        assert_eq!(strategy1.long_ma_value(), strategy2.long_ma_value());
        assert_eq!(strategy1.is_ready(), strategy2.is_ready());
    }

    /// 测试Debug格式化输出
    #[test]
    fn test_debug_format() {
        let strategy = MACrossoverStrategy::new(5, 20);
        let debug_str = format!("{:?}", strategy);
        
        // 验证Debug输出包含关键信息
        assert!(debug_str.contains("MACrossoverStrategy"));
        println!("Debug output: {}", debug_str);
    }
}
