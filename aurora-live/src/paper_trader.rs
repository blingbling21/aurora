//! 模拟交易器模块
//!
//! 为实时环境提供模拟交易功能，使用统一的投资组合管理接口

use anyhow::Result;
use aurora_portfolio::{BasePortfolio, Portfolio};
use tracing::{debug, info};

/// 模拟交易者
///
/// 封装投资组合管理功能，为实时交易环境提供模拟交易能力
#[derive(Debug)]
pub struct PaperTrader {
    portfolio: BasePortfolio,
}

impl PaperTrader {
    /// 创建新的模拟交易者
    pub fn new(initial_cash: f64) -> Self {
        info!("💰 初始化模拟账户，初始资金: {:.2}", initial_cash);

        Self {
            portfolio: BasePortfolio::new(initial_cash),
        }
    }

    /// 执行模拟买入
    pub async fn execute_paper_buy(&mut self, price: f64, timestamp: i64) -> Result<()> {
        match self.portfolio.execute_buy(price, timestamp).await {
            Ok(trade) => {
                info!(
                    "📈 模拟买入成功: 价格={:.2}, 数量={:.6}, 总价值={:.2}",
                    trade.price, trade.quantity, trade.value
                );

                // 发送通知
                self.send_notification(&format!(
                    "模拟买入 {:.6} @ {:.2}",
                    trade.quantity, trade.price
                ));
                Ok(())
            }
            Err(e) => {
                debug!("模拟买入失败: {}", e);
                Err(e)
            }
        }
    }

    /// 执行模拟卖出
    pub async fn execute_paper_sell(&mut self, price: f64, timestamp: i64) -> Result<()> {
        match self.portfolio.execute_sell(price, timestamp).await {
            Ok(trade) => {
                info!(
                    "📉 模拟卖出成功: 价格={:.2}, 数量={:.6}, 总价值={:.2}",
                    trade.price, trade.quantity, trade.value
                );

                // 发送通知
                self.send_notification(&format!(
                    "模拟卖出 {:.6} @ {:.2}",
                    trade.quantity, trade.price
                ));
                Ok(())
            }
            Err(e) => {
                debug!("模拟卖出失败: {}", e);
                Err(e)
            }
        }
    }

    /// 更新权益记录
    pub fn update_equity(&mut self, timestamp: i64, current_price: f64) {
        self.portfolio.update_equity(timestamp, current_price);
    }

    /// 获取当前总权益
    pub fn get_total_equity(&self, current_price: f64) -> f64 {
        self.portfolio.get_total_equity(current_price)
    }

    /// 获取现金余额
    pub fn get_cash(&self) -> f64 {
        self.portfolio.get_cash()
    }

    /// 获取持仓数量
    pub fn get_position(&self) -> f64 {
        self.portfolio.get_position()
    }

    /// 打印当前账户状态
    pub fn print_status(&self, current_price: f64) {
        let total_equity = self.get_total_equity(current_price);
        let cash = self.get_cash();
        let position = self.get_position();
        let position_value = position * current_price;

        info!("📊 账户状态:");
        info!("  现金: {:.2}", cash);
        info!("  持仓: {:.6} (价值: {:.2})", position, position_value);
        info!("  总权益: {:.2}", total_equity);
        info!("  交易次数: {}", self.portfolio.get_trades().len());
    }

    /// 发送通知（模拟实现）
    fn send_notification(&self, message: &str) {
        debug!("📢 通知: {}", message);
    }

    /// 获取投资组合引用（用于分析）
    pub fn portfolio(&self) -> &BasePortfolio {
        &self.portfolio
    }

    /// 计算并打印业绩报告
    pub fn print_performance_report(&self, time_period_days: f64) {
        let metrics = self.portfolio.calculate_performance(time_period_days);
        metrics.print_report();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_paper_trader_creation() {
        let trader = PaperTrader::new(10000.0);

        // 验证初始状态
        assert_eq!(trader.get_cash(), 10000.0);
        assert_eq!(trader.get_position(), 0.0);
        assert_eq!(trader.get_total_equity(50000.0), 10000.0);
    }

    #[tokio::test]
    async fn test_paper_buy_execution() {
        let mut trader = PaperTrader::new(10000.0);
        let price = 50000.0;
        let timestamp = 1640995200000;

        // 执行买入
        let result = trader.execute_paper_buy(price, timestamp).await;
        assert!(result.is_ok());

        // 验证状态变化
        assert!(trader.get_position() > 0.0);
        assert!(trader.get_cash() < 10000.0);

        // 验证权益计算
        let expected_equity = trader.get_cash() + trader.get_position() * price;
        assert!((trader.get_total_equity(price) - expected_equity).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_paper_sell_execution() {
        let mut trader = PaperTrader::new(10000.0);
        let buy_price = 50000.0;
        let sell_price = 52000.0;
        let timestamp = 1640995200000;

        // 先买入
        let _ = trader.execute_paper_buy(buy_price, timestamp).await;
        let position_after_buy = trader.get_position();

        // 再卖出
        let result = trader
            .execute_paper_sell(sell_price, timestamp + 60000)
            .await;
        assert!(result.is_ok());

        // 验证状态变化
        assert_eq!(trader.get_position(), 0.0);

        // 验证盈亏 (卖价高于买价，应该有盈利)
        let final_cash = trader.get_cash();
        let profit = final_cash - 10000.0;
        assert!(profit > 0.0);
    }

    #[tokio::test]
    async fn test_paper_sell_without_position() {
        let mut trader = PaperTrader::new(10000.0);
        let price = 50000.0;
        let timestamp = 1640995200000;

        // 在没有持仓的情况下尝试卖出
        let result = trader.execute_paper_sell(price, timestamp).await;

        // 应该失败或没有效果
        if result.is_err() {
            // 如果返回错误，这是正确的行为
            assert!(true);
        } else {
            // 如果没有返回错误，持仓应该仍然为0
            assert_eq!(trader.get_position(), 0.0);
        }
    }

    #[test]
    fn test_equity_update() {
        let mut trader = PaperTrader::new(10000.0);
        let timestamp = 1640995200000;
        let price = 50000.0;

        // 更新权益
        trader.update_equity(timestamp, price);

        // 验证权益曲线有数据
        assert!(!trader.portfolio().get_equity_curve().is_empty());
    }

    #[test]
    fn test_status_methods() {
        let trader = PaperTrader::new(10000.0);

        // 测试各种状态获取方法
        assert_eq!(trader.get_cash(), 10000.0);
        assert_eq!(trader.get_position(), 0.0);
        assert_eq!(trader.get_total_equity(50000.0), 10000.0);

        // 测试投资组合引用
        let portfolio_ref = trader.portfolio();
        assert_eq!(portfolio_ref.get_cash(), 10000.0);
    }

    #[test]
    fn test_send_notification() {
        let trader = PaperTrader::new(10000.0);

        // 测试通知发送 (内部私有方法，通过其他方法间接测试)
        // 这里主要验证不会panic
        trader.send_notification("测试通知");
    }

    #[test]
    fn test_print_status() {
        let trader = PaperTrader::new(10000.0);
        let price = 50000.0;

        // 测试状态打印 (主要验证不会panic)
        trader.print_status(price);
    }

    #[test]
    fn test_print_performance_report() {
        let trader = PaperTrader::new(10000.0);
        let time_period_days = 30.0;

        // 测试业绩报告打印 (主要验证不会panic)
        trader.print_performance_report(time_period_days);
    }

    #[tokio::test]
    async fn test_multiple_trades_scenario() {
        let mut trader = PaperTrader::new(10000.0);

        // 执行多次交易
        let _ = trader.execute_paper_buy(50000.0, 1640995200000).await;
        let _ = trader.execute_paper_sell(52000.0, 1640995300000).await;
        let _ = trader.execute_paper_buy(51000.0, 1640995400000).await;

        // 验证交易历史
        assert!(!trader.portfolio().get_trades().is_empty());

        // 验证状态一致性
        assert!(trader.get_position() > 0.0); // 应该有持仓
        assert!(trader.get_cash() >= 0.0); // 现金余额应该非负（可能为0）

        // 验证总权益保持合理
        let total_equity = trader.get_total_equity(51000.0);
        assert!(total_equity > 0.0);
    }
}
