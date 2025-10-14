//! Aurora Portfolio Management Library
//!
//! 提供投资组合管理的核心功能,包括交易执行、权益计算、风险管理等。
//! 适用于回测和实时交易环境。
//!
//! # 主要功能
//!
//! - **交易执行**: 买入/卖出操作的统一接口,支持多种订单类型
//! - **权益管理**: 实时计算总权益、现金余额、持仓价值
//! - **风险控制**: 最大回撤、止损止盈、连续亏损限制等多层风险管理
//! - **仓位管理**: 固定金额、固定比例、Kelly准则、金字塔加仓等多种策略
//! - **业绩分析**: 收益率、夏普比率等业绩指标计算
//!
//! # 使用示例
//!
//! ## 基础交易
//!
//! ```rust
//! use aurora_portfolio::{Portfolio, BasePortfolio};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建投资组合,初始资金10000
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
//!
//! ## 风险管理
//!
//! ```rust
//! use aurora_portfolio::{RiskManager, RiskRules};
//!
//! // 创建风险规则
//! let rules = RiskRules::new()
//!     .with_max_drawdown(15.0)           // 最大回撤15%
//!     .with_max_consecutive_losses(3)    // 最多连续亏损3次
//!     .with_min_equity(5000.0);          // 最低权益5000
//!
//! let mut risk_manager = RiskManager::new(rules, 10000.0);
//!
//! // 执行风险检查
//! let result = risk_manager.check_risk(9500.0, 5.0, 100.0);
//! if result.is_pass() {
//!     println!("风险检查通过,可以继续交易");
//! }
//! ```
//!
//! ## 仓位管理
//!
//! ```rust
//! use aurora_portfolio::{PositionManager, PositionSizingStrategy};
//!
//! // 使用Kelly准则
//! let strategy = PositionSizingStrategy::KellyCriterion {
//!     win_rate: 0.6,
//!     profit_loss_ratio: 2.0,
//!     kelly_fraction: 0.5,
//! };
//!
//! let manager = PositionManager::new(strategy);
//! let position_size = manager.calculate_position_size(10000.0, 0.0)?;
//! println!("建议仓位: {:.2}", position_size);
//! # Ok::<(), anyhow::Error>(())
//! ```

// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod analytics;
mod order;
mod portfolio;
mod position_manager;
mod risk_manager;
mod trade;

pub use analytics::{EquityPoint, PerformanceMetrics, PortfolioAnalytics};
pub use order::{Order, OrderSide, OrderStatus, OrderType};
pub use portfolio::{BasePortfolio, Portfolio};
pub use position_manager::{PositionManager, PositionSizingStrategy};
pub use risk_manager::{RiskCheckResult, RiskManager, RiskRules};
pub use trade::{Trade, TradeBuilder, TradeSide};
