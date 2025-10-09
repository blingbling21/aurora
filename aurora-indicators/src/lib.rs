//! Aurora 技术指标库
//!
//! 提供各种技术分析指标的计算功能，这些指标是构建量化交易策略的基础组件。
//!
//! # 支持的指标
//!
//! - **移动平均线 (MA)**: 简单移动平均线，用于识别趋势方向
//!
//! # 设计原则
//!
//! - **状态管理**: 每个指标维护自己的内部状态，支持流式数据处理
//! - **内存效率**: 使用滑动窗口避免存储过多历史数据
//! - **类型安全**: 利用Rust类型系统确保计算正确性
//!
//! # 示例
//!
//! ```rust
//! use aurora_indicators::MA;
//!
//! let mut ma = MA::new(5); // 5周期移动平均线
//!
//! // 前4个数据点不会产生结果
//! assert_eq!(ma.update(100.0), None);
//! assert_eq!(ma.update(102.0), None);
//! assert_eq!(ma.update(98.0), None);
//! assert_eq!(ma.update(105.0), None);
//!
//! // 第5个数据点开始产生移动平均值
//! let avg = ma.update(95.0).unwrap();
//! assert_eq!(avg, 100.0); // (100+102+98+105+95)/5 = 100
//! ```

use std::collections::VecDeque;

/// 移动平均线 (Moving Average) 指标
///
/// 计算指定周期内价格的算术平均值，是最基础的趋势跟踪指标。
/// 移动平均线能够平滑价格波动，帮助识别趋势方向。
///
/// # 算法原理
///
/// 对于周期为N的移动平均线，在时刻t的值为：
/// MA(t) = (P(t) + P(t-1) + ... + P(t-N+1)) / N
///
/// 其中P(t)表示时刻t的价格。
///
/// # 内存复杂度
///
/// 空间复杂度为O(N)，其中N是周期长度。使用双端队列(VecDeque)
/// 实现滑动窗口，确保内存使用效率。
///
/// # 时间复杂度
///
/// 每次更新的时间复杂度为O(1)，通过维护累计和避免重复计算。
///
/// # 示例
///
/// ```rust
/// use aurora_indicators::MA;
///
/// // 创建10周期移动平均线
/// let mut ma10 = MA::new(10);
///
/// // 逐步添加数据
/// for i in 1..=10 {
///     let price = 100.0 + i as f64;
///     if let Some(avg) = ma10.update(price) {
///         println!("MA(10) = {:.2}", avg);
///     }
/// }
///
/// // 继续添加数据，观察滑动效果
/// let new_avg = ma10.update(120.0).unwrap();
/// println!("新的MA(10) = {:.2}", new_avg);
/// ```
#[derive(Debug, Clone)]
pub struct MA {
    /// 移动平均的周期长度
    period: usize,
    /// 存储最近N个价格的滑动窗口
    values: VecDeque<f64>,
    /// 当前窗口内所有价格的累计和
    sum: f64,
}

impl MA {
    /// 创建新的移动平均线指标
    ///
    /// # 参数
    ///
    /// * `period` - 移动平均的周期，必须大于0
    ///
    /// # Panics
    ///
    /// 如果周期为0，函数会panic
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let ma5 = MA::new(5);   // 5周期移动平均线
    /// let ma20 = MA::new(20); // 20周期移动平均线
    /// ```
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "移动平均周期必须大于0");

        Self {
            period,
            values: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    /// 更新指标并返回最新的移动平均值
    ///
    /// # 参数
    ///
    /// * `price` - 新的价格数据
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有足够的数据点（>= period），返回移动平均值
    /// * `None` - 如果数据点不足，返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(3);
    ///
    /// assert_eq!(ma.update(10.0), None);  // 数据不足
    /// assert_eq!(ma.update(20.0), None);  // 数据不足
    ///
    /// let avg = ma.update(30.0).unwrap(); // 有足够数据了
    /// assert_eq!(avg, 20.0); // (10+20+30)/3 = 20
    /// ```
    pub fn update(&mut self, price: f64) -> Option<f64> {
        // 添加新的价格到队列尾部
        self.values.push_back(price);
        self.sum += price;

        // 如果超出了周期长度，移除最旧的值
        if self.values.len() > self.period {
            if let Some(old_value) = self.values.pop_front() {
                self.sum -= old_value;
            }
        }

        // 只有当有足够的数据点时才返回平均值
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// 获取当前的移动平均值（如果可用）
    ///
    /// # 返回值
    ///
    /// * `Some(f64)` - 如果已有足够的数据点，返回当前移动平均值
    /// * `None` - 如果数据点不足，返回None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(3);
    ///
    /// assert_eq!(ma.value(), None); // 还没有数据
    ///
    /// ma.update(10.0);
    /// ma.update(20.0);
    /// assert_eq!(ma.value(), None); // 数据仍然不足
    ///
    /// ma.update(30.0);
    /// assert_eq!(ma.value(), Some(20.0)); // 现在有值了
    /// ```
    pub fn value(&self) -> Option<f64> {
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// 重置指标状态
    ///
    /// 清空所有历史数据，将指标恢复到初始状态。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(3);
    ///
    /// ma.update(10.0);
    /// ma.update(20.0);
    /// ma.update(30.0);
    /// assert_eq!(ma.value(), Some(20.0));
    ///
    /// ma.reset();
    /// assert_eq!(ma.value(), None); // 重置后没有值
    /// ```
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }

    /// 获取当前的周期设置
    ///
    /// # 返回值
    ///
    /// 返回创建指标时设置的周期长度
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let ma = MA::new(10);
    /// assert_eq!(ma.period(), 10);
    /// ```
    pub fn period(&self) -> usize {
        self.period
    }

    /// 获取当前已接收的数据点数量
    ///
    /// # 返回值
    ///
    /// 返回当前存储在滑动窗口中的数据点数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(5);
    ///
    /// assert_eq!(ma.len(), 0);
    ///
    /// ma.update(100.0);
    /// assert_eq!(ma.len(), 1);
    ///
    /// ma.update(102.0);
    /// assert_eq!(ma.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// 检查指标是否为空（没有数据）
    ///
    /// # 返回值
    ///
    /// 如果指标中没有任何数据，返回true；否则返回false
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(3);
    ///
    /// assert!(ma.is_empty());
    ///
    /// ma.update(100.0);
    /// assert!(!ma.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// 检查指标是否已准备好（有足够的数据）
    ///
    /// # 返回值
    ///
    /// 如果指标已有足够的数据点来计算移动平均值，返回true
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_indicators::MA;
    ///
    /// let mut ma = MA::new(3);
    ///
    /// assert!(!ma.is_ready());
    ///
    /// ma.update(10.0);
    /// ma.update(20.0);
    /// assert!(!ma.is_ready()); // 还需要一个数据点
    ///
    /// ma.update(30.0);
    /// assert!(ma.is_ready()); // 现在准备好了
    /// ```
    pub fn is_ready(&self) -> bool {
        self.values.len() == self.period
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    /// 测试MA指标创建
    #[test]
    fn test_ma_creation() {
        let ma = MA::new(5);
        assert_eq!(ma.period(), 5);
        assert_eq!(ma.len(), 0);
        assert_eq!(ma.sum, 0.0);
        assert!(ma.is_empty());
        assert!(!ma.is_ready());
    }

    /// 测试周期为0时的panic行为
    #[test]
    #[should_panic(expected = "移动平均周期必须大于0")]
    fn test_ma_zero_period_panic() {
        MA::new(0);
    }

    /// 测试数据不足时的行为
    #[test]
    fn test_ma_insufficient_data() {
        let mut ma = MA::new(3);

        assert_eq!(ma.update(10.0), None);
        assert_eq!(ma.len(), 1);
        assert!(!ma.is_empty());
        assert!(!ma.is_ready());

        assert_eq!(ma.update(20.0), None);
        assert_eq!(ma.len(), 2);
        assert!(!ma.is_ready());

        assert_eq!(ma.value(), None);
    }

    /// 测试有足够数据时的计算
    #[test]
    fn test_ma_sufficient_data() {
        let mut ma = MA::new(3);

        ma.update(10.0);
        ma.update(20.0);
        let result = ma.update(30.0);

        assert!(result.is_some());
        assert_relative_eq!(result.unwrap(), 20.0, epsilon = 1e-10);
        assert_relative_eq!(ma.value().unwrap(), 20.0, epsilon = 1e-10);
        assert!(ma.is_ready());
        assert_eq!(ma.len(), 3);
    }

    /// 测试滑动窗口功能
    #[test]
    fn test_ma_sliding_window() {
        let mut ma = MA::new(3);

        // 填充初始数据
        ma.update(10.0);
        ma.update(20.0);
        let result1 = ma.update(30.0);
        assert_relative_eq!(result1.unwrap(), 20.0, epsilon = 1e-10); // (10+20+30)/3 = 20

        // 添加新数据，应该滑动窗口
        let result2 = ma.update(40.0);
        assert_relative_eq!(result2.unwrap(), 30.0, epsilon = 1e-10); // (20+30+40)/3 = 30

        let result3 = ma.update(50.0);
        assert_relative_eq!(result3.unwrap(), 40.0, epsilon = 1e-10); // (30+40+50)/3 = 40

        // 验证窗口大小始终保持为3
        assert_eq!(ma.len(), 3);
    }

    /// 测试重置功能
    #[test]
    fn test_ma_reset() {
        let mut ma = MA::new(3);

        ma.update(10.0);
        ma.update(20.0);
        ma.update(30.0);

        assert!(ma.value().is_some());
        assert!(ma.is_ready());
        assert!(!ma.is_empty());

        ma.reset();

        assert_eq!(ma.len(), 0);
        assert_eq!(ma.sum, 0.0);
        assert!(ma.value().is_none());
        assert!(!ma.is_ready());
        assert!(ma.is_empty());
    }

    /// 测试单周期MA
    #[test]
    fn test_ma_single_period() {
        let mut ma = MA::new(1);

        let result1 = ma.update(42.0);
        assert_relative_eq!(result1.unwrap(), 42.0, epsilon = 1e-10);

        let result2 = ma.update(84.0);
        assert_relative_eq!(result2.unwrap(), 84.0, epsilon = 1e-10);

        assert_eq!(ma.len(), 1); // 单周期只保存一个值
    }

    /// 测试大数据量的处理
    #[test]
    fn test_ma_large_dataset() {
        let mut ma = MA::new(100);

        // 添加前99个数据点，不应该有返回值
        for i in 1..100 {
            assert_eq!(ma.update(i as f64), None);
        }

        assert_eq!(ma.len(), 99);
        assert!(!ma.is_ready());

        // 第100个数据点应该产生结果
        let result = ma.update(100.0);
        assert!(result.is_some());
        assert!(ma.is_ready());
        assert_eq!(ma.len(), 100);

        // 验证计算结果: (1+2+...+100)/100 = 50.5
        assert_relative_eq!(result.unwrap(), 50.5, epsilon = 1e-10);
    }

    /// 测试极端值处理
    #[test]
    fn test_ma_extreme_values() {
        let mut ma = MA::new(3);

        // 测试0值
        ma.update(0.0);
        ma.update(0.0);
        let result1 = ma.update(0.0);
        assert_relative_eq!(result1.unwrap(), 0.0, epsilon = 1e-10);

        // 测试负值
        ma.reset();
        ma.update(-10.0);
        ma.update(-20.0);
        let result2 = ma.update(-30.0);
        assert_relative_eq!(result2.unwrap(), -20.0, epsilon = 1e-10);

        // 测试很大的值
        ma.reset();
        ma.update(1e10);
        ma.update(1e10);
        let result3 = ma.update(1e10);
        assert_relative_eq!(result3.unwrap(), 1e10, epsilon = 1e-5);
    }

    /// 测试精度保持
    #[test]
    fn test_ma_precision() {
        let mut ma = MA::new(3);

        // 使用需要高精度的小数
        ma.update(1.0 / 3.0); // 0.333...
        ma.update(2.0 / 3.0); // 0.666...
        let result = ma.update(1.0);

        let expected = (1.0 / 3.0 + 2.0 / 3.0 + 1.0) / 3.0; // 2/3
        assert_relative_eq!(result.unwrap(), expected, epsilon = 1e-15);
    }

    /// 测试克隆功能
    #[test]
    fn test_ma_clone() {
        let mut ma1 = MA::new(3);
        ma1.update(10.0);
        ma1.update(20.0);

        let ma2 = ma1.clone();

        // 克隆的对象应该有相同的状态
        assert_eq!(ma1.period(), ma2.period());
        assert_eq!(ma1.len(), ma2.len());
        assert_eq!(ma1.sum, ma2.sum);
        assert_eq!(ma1.value(), ma2.value());
    }

    /// 测试Debug格式化
    #[test]
    fn test_ma_debug_format() {
        let mut ma = MA::new(5);
        ma.update(100.0);

        let debug_str = format!("{:?}", ma);
        assert!(debug_str.contains("MA"));
        assert!(debug_str.contains("period: 5"));
    }

    /// 测试边界条件：接近溢出的情况
    #[test]
    fn test_ma_overflow_resistance() {
        let mut ma = MA::new(2);

        // 使用接近f64最大值的数字
        let large_val = f64::MAX / 10.0;
        ma.update(large_val);
        let result = ma.update(large_val);

        assert!(result.is_some());
        assert_relative_eq!(result.unwrap(), large_val, epsilon = large_val * 1e-15);
    }
}
