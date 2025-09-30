//! Aurora Portfolio Management Library
//!
//! 提供投资组合管理的核心功能，包括交易执行、权益计算、风险管理等。
//! 适用于回测和实时交易环境。
//!
//! # 主要功能
//!
//! - **交易执行**: 买入/卖出操作的统一接口
//! - **权益管理**: 实时计算总权益、现金余额、持仓价值
//! - **风险控制**: 最大回撤、止损等风险管理功能
//! - **业绩分析**: 收益率、夏普比率等业绩指标计算
//!
//! # 使用示例
//!
//! ```rust
//! use aurora_portfolio::{Portfolio, BasePortfolio};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建投资组合，初始资金10000
//!     let mut portfolio = BasePortfolio::new(10000.0);
//!     
//!     // 执行买入操作
//!     let trade = portfolio.execute_buy(100.0, 1640995200000).await?;
//!     println!("买入: {:?}", trade);
//!     
//!     // 更新权益曲线
//!     portfolio.update_equity(1640995260000, 105.0);
//!     
//!     // 获取当前权益
//!     let equity = portfolio.get_total_equity(105.0);
//!     println!("当前权益: {:.2}", equity);
//!     
//!     Ok(())
//! }
//! ```

mod portfolio;
mod trade;
mod analytics;

pub use portfolio::{Portfolio, BasePortfolio};
pub use trade::{Trade, TradeSide, TradeBuilder};
pub use analytics::{EquityPoint, PerformanceMetrics, PortfolioAnalytics};