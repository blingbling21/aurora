//! 交易记录相关数据结构和功能

use serde::{Deserialize, Serialize};

/// 交易记录
///
/// 记录单次交易的完整信息，包括时间、方向、价格、数量等。
/// 用于后续的业绩分析和风险控制。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trade {
    /// 交易时间戳（Unix毫秒）
    pub timestamp: i64,
    /// 交易方向（买入/卖出）
    pub side: TradeSide,
    /// 成交价格
    pub price: f64,
    /// 交易数量
    pub quantity: f64,
    /// 交易总价值（价格 × 数量）
    pub value: f64,
    /// 交易手续费（可选）
    pub fee: Option<f64>,
    /// 交易备注（可选）
    pub note: Option<String>,
}

/// 交易方向枚举
///
/// 标识交易的买卖方向，用于计算持仓变化和损益。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

/// 交易构建器
///
/// 提供便捷的交易记录创建方式，支持可选字段的设置。
///
/// # 示例
///
/// ```rust
/// use aurora_portfolio::{TradeBuilder, TradeSide};
///
/// let trade = TradeBuilder::new(TradeSide::Buy, 100.0, 10.0, 1640995200000)
///     .with_fee(5.0)
///     .with_note("开仓买入".to_string())
///     .build();
/// ```
pub struct TradeBuilder {
    timestamp: i64,
    side: TradeSide,
    price: f64,
    quantity: f64,
    fee: Option<f64>,
    note: Option<String>,
}

impl TradeBuilder {
    /// 创建新的交易构建器
    ///
    /// # 参数
    ///
    /// * `side` - 交易方向
    /// * `price` - 成交价格
    /// * `quantity` - 交易数量
    /// * `timestamp` - 交易时间戳
    pub fn new(side: TradeSide, price: f64, quantity: f64, timestamp: i64) -> Self {
        Self {
            timestamp,
            side,
            price,
            quantity,
            fee: None,
            note: None,
        }
    }

    /// 设置交易手续费
    pub fn with_fee(mut self, fee: f64) -> Self {
        self.fee = Some(fee);
        self
    }

    /// 设置交易备注
    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }

    /// 构建交易记录
    pub fn build(self) -> Trade {
        let value = self.price * self.quantity;
        Trade {
            timestamp: self.timestamp,
            side: self.side,
            price: self.price,
            quantity: self.quantity,
            value,
            fee: self.fee,
            note: self.note,
        }
    }
}

impl Trade {
    /// 创建买入交易记录
    ///
    /// # 参数
    ///
    /// * `price` - 买入价格
    /// * `quantity` - 买入数量
    /// * `timestamp` - 交易时间戳
    pub fn new_buy(price: f64, quantity: f64, timestamp: i64) -> Self {
        TradeBuilder::new(TradeSide::Buy, price, quantity, timestamp).build()
    }

    /// 创建卖出交易记录
    ///
    /// # 参数
    ///
    /// * `price` - 卖出价格
    /// * `quantity` - 卖出数量
    /// * `timestamp` - 交易时间戳
    pub fn new_sell(price: f64, quantity: f64, timestamp: i64) -> Self {
        TradeBuilder::new(TradeSide::Sell, price, quantity, timestamp).build()
    }

    /// 获取净交易价值（扣除手续费后）
    pub fn net_value(&self) -> f64 {
        match self.fee {
            Some(fee) => self.value - fee,
            None => self.value,
        }
    }

    /// 判断是否为买入交易
    pub fn is_buy(&self) -> bool {
        self.side == TradeSide::Buy
    }

    /// 判断是否为卖出交易
    pub fn is_sell(&self) -> bool {
        self.side == TradeSide::Sell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let trade = Trade::new_buy(100.0, 10.0, 1640995200000);

        assert_eq!(trade.side, TradeSide::Buy);
        assert_eq!(trade.price, 100.0);
        assert_eq!(trade.quantity, 10.0);
        assert_eq!(trade.value, 1000.0);
        assert_eq!(trade.timestamp, 1640995200000);
        assert!(trade.is_buy());
        assert!(!trade.is_sell());
    }

    #[test]
    fn test_trade_builder() {
        let trade = TradeBuilder::new(TradeSide::Sell, 105.0, 5.0, 1640995260000)
            .with_fee(2.5)
            .with_note("止盈卖出".to_string())
            .build();

        assert_eq!(trade.side, TradeSide::Sell);
        assert_eq!(trade.fee, Some(2.5));
        assert_eq!(trade.note, Some("止盈卖出".to_string()));
        assert_eq!(trade.net_value(), 522.5); // 525.0 - 2.5
    }

    #[test]
    fn test_trade_side_equality() {
        assert_eq!(TradeSide::Buy, TradeSide::Buy);
        assert_eq!(TradeSide::Sell, TradeSide::Sell);
        assert_ne!(TradeSide::Buy, TradeSide::Sell);
    }
}
