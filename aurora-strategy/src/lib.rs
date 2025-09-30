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


