//! 投资组合管理核心模块

use async_trait::async_trait;
use anyhow::Result;
use tracing::{info, debug, warn};

use crate::trade::Trade;
use crate::analytics::{EquityPoint, PerformanceMetrics, PortfolioAnalytics};

/// 投资组合管理统一接口
///
/// 定义了投资组合管理的标准行为，适用于回测和实时交易环境。
/// 支持异步操作以适应实时交易的需求。
#[async_trait]
pub trait Portfolio: Send + Sync {
    /// 执行买入操作
    ///
    /// # 参数
    ///
    /// * `price` - 买入价格
    /// * `timestamp` - 交易时间戳
    ///
    /// # 返回值
    ///
    /// 成功时返回交易记录，失败时返回错误信息
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade>;

    /// 执行卖出操作
    ///
    /// # 参数
    ///
    /// * `price` - 卖出价格
    /// * `timestamp` - 交易时间戳
    ///
    /// # 返回值
    ///
    /// 成功时返回交易记录，失败时返回错误信息
    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade>;

    /// 获取总权益
    ///
    /// # 参数
    ///
    /// * `current_price` - 当前市场价格
    ///
    /// # 返回值
    ///
    /// 返回当前总权益（现金 + 持仓价值）
    fn get_total_equity(&self, current_price: f64) -> f64;

    /// 获取现金余额
    fn get_cash(&self) -> f64;

    /// 获取持仓数量
    fn get_position(&self) -> f64;

    /// 获取交易记录
    fn get_trades(&self) -> &[Trade];

    /// 更新权益曲线
    ///
    /// # 参数
    ///
    /// * `timestamp` - 时间戳
    /// * `current_price` - 当前价格
    fn update_equity(&mut self, timestamp: i64, current_price: f64);

    /// 获取权益曲线
    fn get_equity_curve(&self) -> &[EquityPoint];

    /// 计算业绩指标
    fn calculate_performance(&self, time_period_days: f64) -> PerformanceMetrics;
}

/// 基础投资组合实现
///
/// 提供投资组合管理的标准实现，适用于大多数场景。
/// 支持简单的全仓买卖策略，可以被扩展以支持更复杂的仓位管理。
#[derive(Debug, Clone)]
pub struct BasePortfolio {
    /// 现金余额
    cash: f64,
    /// 持仓数量
    position: f64,
    /// 初始权益
    initial_equity: f64,
    /// 交易记录
    trades: Vec<Trade>,
    /// 权益曲线
    equity_curve: Vec<EquityPoint>,
    /// 历史最高权益（用于计算回撤）
    max_equity: f64,
}

impl BasePortfolio {
    /// 创建新的投资组合
    ///
    /// # 参数
    ///
    /// * `initial_cash` - 初始现金金额
    ///
    /// # 示例
    ///
    /// ```rust
    /// use aurora_portfolio::{BasePortfolio, Portfolio};
    ///
    /// let portfolio = BasePortfolio::new(10000.0);
    /// assert_eq!(portfolio.get_cash(), 10000.0);
    /// assert_eq!(portfolio.get_position(), 0.0);
    /// ```
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash: initial_cash,
            position: 0.0,
            initial_equity: initial_cash,
            trades: Vec::new(),
            equity_curve: Vec::new(),
            max_equity: initial_cash,
        }
    }

    /// 检查是否可以买入
    ///
    /// # 参数
    ///
    /// * `price` - 买入价格
    ///
    /// # 返回值
    ///
    /// 如果现金足够买入至少最小单位，返回true
    fn can_buy(&self, price: f64) -> bool {
        self.cash > price * 0.001 // 最小买入单位
    }

    /// 检查是否可以卖出
    fn can_sell(&self) -> bool {
        self.position > 0.0
    }

    /// 计算买入数量
    ///
    /// 使用全部现金买入（简化处理）
    fn calculate_buy_quantity(&self, price: f64) -> f64 {
        self.cash / price
    }

    /// 计算卖出数量
    ///
    /// 卖出全部持仓（简化处理）
    fn calculate_sell_quantity(&self) -> f64 {
        self.position
    }

    /// 验证交易参数
    fn validate_trade_params(&self, price: f64, timestamp: i64) -> Result<()> {
        if price <= 0.0 {
            return Err(anyhow::anyhow!("价格必须大于0"));
        }
        if timestamp < 0 {
            return Err(anyhow::anyhow!("时间戳不能为负数"));
        }
        Ok(())
    }
}

#[async_trait]
impl Portfolio for BasePortfolio {
    async fn execute_buy(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        self.validate_trade_params(price, timestamp)?;

        if !self.can_buy(price) {
            return Err(anyhow::anyhow!("现金不足，无法买入"));
        }

        let quantity = self.calculate_buy_quantity(price);
        let value = quantity * price;

        // 更新持仓和现金
        self.position += quantity;
        self.cash = 0.0; // 全仓买入

        // 创建交易记录
        let trade = Trade::new_buy(price, quantity, timestamp);
        self.trades.push(trade.clone());

        info!(
            "执行买入: 价格={:.2}, 数量={:.6}, 总价值={:.2}",
            price, quantity, value
        );
        debug!(
            "买入后状态: 持仓={:.6}, 现金={:.2}",
            self.position, self.cash
        );

        Ok(trade)
    }

    async fn execute_sell(&mut self, price: f64, timestamp: i64) -> Result<Trade> {
        self.validate_trade_params(price, timestamp)?;

        if !self.can_sell() {
            return Err(anyhow::anyhow!("无持仓，无法卖出"));
        }

        let quantity = self.calculate_sell_quantity();
        let value = quantity * price;

        // 更新持仓和现金
        self.cash += value;
        self.position = 0.0; // 全部卖出

        // 创建交易记录
        let trade = Trade::new_sell(price, quantity, timestamp);
        self.trades.push(trade.clone());

        info!(
            "执行卖出: 价格={:.2}, 数量={:.6}, 总价值={:.2}",
            price, quantity, value
        );
        debug!(
            "卖出后状态: 持仓={:.6}, 现金={:.2}",
            self.position, self.cash
        );

        Ok(trade)
    }

    fn get_total_equity(&self, current_price: f64) -> f64 {
        self.cash + (self.position * current_price)
    }

    fn get_cash(&self) -> f64 {
        self.cash
    }

    fn get_position(&self) -> f64 {
        self.position
    }

    fn get_trades(&self) -> &[Trade] {
        &self.trades
    }

    fn update_equity(&mut self, timestamp: i64, current_price: f64) {
        let equity = self.get_total_equity(current_price);

        // 更新历史最高权益
        if equity > self.max_equity {
            self.max_equity = equity;
        }

        // 计算当前回撤
        let drawdown = if self.max_equity > 0.0 {
            ((self.max_equity - equity) / self.max_equity) * 100.0
        } else {
            0.0
        };

        // 创建权益点
        let equity_point = EquityPoint {
            timestamp,
            equity,
            drawdown,
        };

        self.equity_curve.push(equity_point);

        // 如果回撤超过警告阈值，记录日志
        if drawdown > 10.0 {
            warn!("当前回撤较大: {:.2}%", drawdown);
        }
    }

    fn get_equity_curve(&self) -> &[EquityPoint] {
        &self.equity_curve
    }

    fn calculate_performance(&self, time_period_days: f64) -> PerformanceMetrics {
        let final_equity = if let Some(last_point) = self.equity_curve.last() {
            last_point.equity
        } else {
            self.initial_equity
        };

        PortfolioAnalytics::calculate_metrics(
            self.initial_equity,
            final_equity,
            &self.equity_curve,
            &self.trades,
            time_period_days,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TradeSide;

    #[tokio::test]
    async fn test_portfolio_creation() {
        let portfolio = BasePortfolio::new(10000.0);
        
        assert_eq!(portfolio.get_cash(), 10000.0);
        assert_eq!(portfolio.get_position(), 0.0);
        assert_eq!(portfolio.get_total_equity(100.0), 10000.0);
        assert!(portfolio.get_trades().is_empty());
    }

    #[tokio::test]
    async fn test_buy_operation() {
        let mut portfolio = BasePortfolio::new(10000.0);
        
        let trade = portfolio.execute_buy(100.0, 1640995200000).await.unwrap();
        
        assert_eq!(trade.side, TradeSide::Buy);
        assert_eq!(trade.price, 100.0);
        assert_eq!(trade.quantity, 100.0); // 10000 / 100
        assert_eq!(portfolio.get_cash(), 0.0);
        assert_eq!(portfolio.get_position(), 100.0);
        assert_eq!(portfolio.get_trades().len(), 1);
    }

    #[tokio::test]
    async fn test_sell_operation() {
        let mut portfolio = BasePortfolio::new(10000.0);
        
        // 先买入
        portfolio.execute_buy(100.0, 1640995200000).await.unwrap();
        
        // 再卖出
        let trade = portfolio.execute_sell(105.0, 1640995260000).await.unwrap();
        
        assert_eq!(trade.side, TradeSide::Sell);
        assert_eq!(trade.price, 105.0);
        assert_eq!(trade.quantity, 100.0);
        assert_eq!(portfolio.get_cash(), 10500.0);
        assert_eq!(portfolio.get_position(), 0.0);
        assert_eq!(portfolio.get_trades().len(), 2);
    }

    #[tokio::test]
    async fn test_equity_update() {
        let mut portfolio = BasePortfolio::new(10000.0);
        
        portfolio.execute_buy(100.0, 1640995200000).await.unwrap();
        portfolio.update_equity(1640995260000, 105.0);
        
        let equity_curve = portfolio.get_equity_curve();
        assert_eq!(equity_curve.len(), 1);
        assert_eq!(equity_curve[0].equity, 10500.0); // 100 * 105
        assert_eq!(equity_curve[0].drawdown, 0.0);
    }

    #[tokio::test]
    async fn test_invalid_operations() {
        let mut portfolio = BasePortfolio::new(10000.0);
        
        // 测试无持仓时卖出
        let result = portfolio.execute_sell(100.0, 1640995200000).await;
        assert!(result.is_err());
        
        // 测试无效价格
        let result = portfolio.execute_buy(-100.0, 1640995200000).await;
        assert!(result.is_err());
        
        // 测试无效时间戳
        let result = portfolio.execute_buy(100.0, -1).await;
        assert!(result.is_err());
    }
}