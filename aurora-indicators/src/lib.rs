use std::collections::VecDeque;

/// 移动平均线 (Moving Average) 指标
#[derive(Debug, Clone)]
pub struct MA {
    period: usize,
    values: VecDeque<f64>,
    sum: f64,
}

impl MA {
    /// 创建新的移动平均线指标
    /// 
    /// # 参数
    /// * `period` - 移动平均的周期
    pub fn new(period: usize) -> Self {
        assert!(period > 0, "Period must be greater than 0");
        
        Self {
            period,
            values: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }

    /// 更新指标并返回最新的移动平均值
    /// 
    /// # 参数
    /// * `price` - 新的价格数据
    /// 
    /// # 返回值
    /// * `Some(f64)` - 如果有足够的数据点，返回移动平均值
    /// * `None` - 如果数据点不足
    pub fn update(&mut self, price: f64) -> Option<f64> {
        self.values.push_back(price);
        self.sum += price;

        // 如果超出了期间长度，移除最旧的值
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
    pub fn value(&self) -> Option<f64> {
        if self.values.len() == self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }

    /// 重置指标状态
    pub fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_ma_creation() {
        let ma = MA::new(5);
        assert_eq!(ma.period, 5);
        assert_eq!(ma.values.len(), 0);
        assert_eq!(ma.sum, 0.0);
    }

    #[test]
    #[should_panic(expected = "Period must be greater than 0")]
    fn test_ma_zero_period_panic() {
        MA::new(0);
    }

    #[test]
    fn test_ma_insufficient_data() {
        let mut ma = MA::new(3);
        
        assert_eq!(ma.update(10.0), None);
        assert_eq!(ma.update(20.0), None);
        assert_eq!(ma.value(), None);
    }

    #[test]
    fn test_ma_sufficient_data() {
        let mut ma = MA::new(3);
        
        ma.update(10.0);
        ma.update(20.0);
        let result = ma.update(30.0);
        
        assert!(result.is_some());
        assert_relative_eq!(result.unwrap(), 20.0, epsilon = 1e-10);
        assert_relative_eq!(ma.value().unwrap(), 20.0, epsilon = 1e-10);
    }

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
    }

    #[test]
    fn test_ma_reset() {
        let mut ma = MA::new(3);
        
        ma.update(10.0);
        ma.update(20.0);
        ma.update(30.0);
        
        assert!(ma.value().is_some());
        
        ma.reset();
        
        assert_eq!(ma.values.len(), 0);
        assert_eq!(ma.sum, 0.0);
        assert!(ma.value().is_none());
    }

    #[test]
    fn test_ma_single_period() {
        let mut ma = MA::new(1);
        
        let result1 = ma.update(42.0);
        assert_relative_eq!(result1.unwrap(), 42.0, epsilon = 1e-10);
        
        let result2 = ma.update(84.0);
        assert_relative_eq!(result2.unwrap(), 84.0, epsilon = 1e-10);
    }
}
