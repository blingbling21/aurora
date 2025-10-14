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

//! Portfolio 模块的单元测试 - 测试风险管理和仓位管理集成

use super::*;
use crate::{PositionManager, PositionSizingStrategy, RiskManager, RiskRules, TradeSide};

// === 基础功能测试 ===

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
    assert!(portfolio.get_cash() < 10.0);  // 现金基本用完
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

// === 风险管理集成测试 ===

#[tokio::test]
async fn test_portfolio_with_risk_manager() {
        // 创建带风险管理的投资组合
        let rules = RiskRules::new()
            .with_max_drawdown(10.0)
            .with_max_consecutive_losses(2);
        let risk_manager = RiskManager::new(rules, 10000.0);
        
        let mut portfolio = BasePortfolio::new(10000.0).with_risk_manager(risk_manager);

        // 第一次买入应该成功
        let result = portfolio.execute_buy(100.0, 1000).await;
        assert!(result.is_ok());

        // 第一次亏损卖出
        let result = portfolio.execute_sell(95.0, 2000).await;
        assert!(result.is_ok());

        // 第二次买入
        let result = portfolio.execute_buy(95.0, 3000).await;
        assert!(result.is_ok());

        // 第二次亏损卖出
        let result = portfolio.execute_sell(90.0, 4000).await;
        assert!(result.is_ok());

        // 第三次买入应该被风控拒绝（连续亏损2次）
        let result = portfolio.execute_buy(90.0, 5000).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("风控拒绝"));
}

#[tokio::test]
async fn test_portfolio_with_position_manager() {
        // 创建带仓位管理的投资组合（固定20%仓位）
        let strategy = PositionSizingStrategy::FixedPercentage(0.2);
        let position_manager = PositionManager::new(strategy);
        
        let mut portfolio = BasePortfolio::new(10000.0).with_position_manager(position_manager);

        // 买入时应该只使用20%的资金（2000）
        let result = portfolio.execute_buy(100.0, 1000).await;
        assert!(result.is_ok());
        
        let trade = result.unwrap();
        // 应该买入约20个单位（2000 / 100）
        assert!((trade.quantity - 20.0).abs() < 1.0);
        
        // 现金应该还剩约8000
        assert!((portfolio.get_cash() - 8000.0).abs() < 100.0);
}

#[tokio::test]
async fn test_portfolio_with_both_managers() {
        // 同时使用风险管理和仓位管理
        let rules = RiskRules::new().with_min_equity(9000.0);
        let risk_manager = RiskManager::new(rules, 10000.0);
        
        let strategy = PositionSizingStrategy::FixedPercentage(0.5);
        let position_manager = PositionManager::new(strategy);
        
        let mut portfolio = BasePortfolio::new(10000.0)
            .with_risk_manager(risk_manager)
            .with_position_manager(position_manager);

        // 第一次买入（50%仓位）
        let result = portfolio.execute_buy(100.0, 1000).await;
        assert!(result.is_ok());
        
        // 亏损卖出，使权益低于9000
        let result = portfolio.execute_sell(70.0, 2000).await;
        assert!(result.is_ok());
        
        // 再次买入应该被风控拒绝（权益过低）
        let result = portfolio.execute_buy(70.0, 3000).await;
        assert!(result.is_err());
}

#[tokio::test]
async fn test_portfolio_without_managers() {
        // 不使用任何管理器（默认行为：全仓）
        let mut portfolio = BasePortfolio::new(10000.0);

        let result = portfolio.execute_buy(100.0, 1000).await;
        assert!(result.is_ok());
        
        let trade = result.unwrap();
        // 全仓买入应该使用所有资金
        assert!((trade.quantity - 100.0).abs() < 0.1);
        assert!(portfolio.get_cash() < 10.0); // 现金基本用完
}

#[tokio::test]
async fn test_kelly_criterion_strategy() {
    // 测试Kelly准则策略
    let strategy = PositionSizingStrategy::KellyCriterion {
        win_rate: 0.6,
        profit_loss_ratio: 2.0,
        kelly_fraction: 0.5,
    };
    let position_manager = PositionManager::new(strategy);
    
    let mut portfolio = BasePortfolio::new(10000.0).with_position_manager(position_manager);

    let result = portfolio.execute_buy(100.0, 1000).await;
    assert!(result.is_ok());
    
    // Kelly建议的仓位应该是合理的（不会是全仓）
    assert!(portfolio.get_cash() > 1000.0);
}
