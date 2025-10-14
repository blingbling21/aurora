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

use super::*;

#[test]
fn test_risk_rules_creation() {
    let rules = RiskRules::new();

    assert_eq!(rules.max_drawdown_pct, None);
    assert_eq!(rules.max_daily_loss_pct, None);
    assert_eq!(rules.max_consecutive_losses, None);
}

#[test]
fn test_risk_rules_builder() {
    let rules = RiskRules::new()
        .with_max_drawdown(15.0)
        .with_max_daily_loss(5.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(1000.0);

    assert_eq!(rules.max_drawdown_pct, Some(15.0));
    assert_eq!(rules.max_daily_loss_pct, Some(5.0));
    assert_eq!(rules.max_consecutive_losses, Some(3));
    assert_eq!(rules.min_equity, Some(1000.0));
}

#[test]
fn test_risk_manager_pass() {
    let rules = RiskRules::new().with_max_drawdown(20.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    let result = manager.check_risk(10000.0, 5.0, 100.0);
    assert!(result.is_pass());
    assert!(!result.should_stop_trading());
}

#[test]
fn test_max_drawdown_trigger() {
    let rules = RiskRules::new().with_max_drawdown(15.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 回撤达到15%,应该触发
    let result = manager.check_risk(8500.0, 15.0, 100.0);
    assert!(!result.is_pass());
    assert!(result.should_stop_trading());
    assert!(matches!(result, RiskCheckResult::MaxDrawdownReached(_)));

    // 确认交易已停止
    assert!(manager.should_stop_trading());
}

#[test]
fn test_max_daily_loss_trigger() {
    let rules = RiskRules::new().with_max_daily_loss(5.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 单日亏损6%,应该触发
    let result = manager.check_risk(9400.0, 10.0, 100.0);
    assert!(!result.is_pass());
    assert!(matches!(result, RiskCheckResult::MaxDailyLossReached(_)));
}

#[test]
fn test_consecutive_losses_trigger() {
    let rules = RiskRules::new().with_max_consecutive_losses(3);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 记录3次连续亏损
    manager.record_trade_result(false);
    manager.record_trade_result(false);
    manager.record_trade_result(false);

    assert_eq!(manager.get_consecutive_losses(), 3);

    // 应该触发限制
    let result = manager.check_risk(9500.0, 5.0, 100.0);
    assert!(!result.is_pass());
    assert!(matches!(
        result,
        RiskCheckResult::MaxConsecutiveLossesReached(_)
    ));
}

#[test]
fn test_consecutive_losses_reset() {
    let rules = RiskRules::new().with_max_consecutive_losses(3);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 两次亏损
    manager.record_trade_result(false);
    manager.record_trade_result(false);
    assert_eq!(manager.get_consecutive_losses(), 2);

    // 一次盈利,计数器重置
    manager.record_trade_result(true);
    assert_eq!(manager.get_consecutive_losses(), 0);
    assert_eq!(manager.get_consecutive_wins(), 1);
}

#[test]
fn test_min_equity_trigger() {
    let rules = RiskRules::new().with_min_equity(5000.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 权益低于最低要求
    let result = manager.check_risk(4500.0, 10.0, 100.0);
    assert!(!result.is_pass());
    assert!(matches!(result, RiskCheckResult::MinEquityBreached(_)));
}

#[test]
fn test_stop_loss_trigger() {
    let rules = RiskRules::new().with_stop_loss_price(95.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 价格跌破止损价
    let result = manager.check_risk(10000.0, 0.0, 94.0);
    assert!(!result.is_pass());
    assert!(matches!(result, RiskCheckResult::StopLoss(_)));
}

#[test]
fn test_take_profit_trigger() {
    let rules = RiskRules::new().with_take_profit_price(110.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 价格涨至止盈价
    let result = manager.check_risk(11000.0, 0.0, 112.0);
    assert!(!result.is_pass());
    assert!(matches!(result, RiskCheckResult::TakeProfit(_)));
}

#[test]
fn test_calculate_stop_loss() {
    let rules = RiskRules::new();
    let manager = RiskManager::new(rules, 10000.0);

    // 入场价100,止损2%
    let stop_price = manager.calculate_stop_loss(100.0, 2.0);
    assert_eq!(stop_price, 98.0);

    // 入场价200,止损5%
    let stop_price2 = manager.calculate_stop_loss(200.0, 5.0);
    assert_eq!(stop_price2, 190.0);
}

#[test]
fn test_calculate_take_profit() {
    let rules = RiskRules::new();
    let manager = RiskManager::new(rules, 10000.0);

    // 入场价100,止盈5%
    let tp_price = manager.calculate_take_profit(100.0, 5.0);
    assert_eq!(tp_price, 105.0);

    // 入场价200,止盈10%
    let tp_price2 = manager.calculate_take_profit(200.0, 10.0);
    assert!((tp_price2 - 220.0).abs() < 0.01);
}

#[test]
fn test_set_stop_loss_take_profit() {
    let rules = RiskRules::new();
    let mut manager = RiskManager::new(rules, 10000.0);

    manager.set_stop_loss_take_profit(100.0, 2.0, 5.0);

    assert_eq!(manager.entry_price, Some(100.0));
    assert_eq!(manager.get_rules().stop_loss_price, Some(98.0));
    assert_eq!(manager.get_rules().take_profit_price, Some(105.0));
}

#[test]
fn test_clear_stop_loss_take_profit() {
    let rules = RiskRules::new();
    let mut manager = RiskManager::new(rules, 10000.0);

    manager.set_stop_loss_take_profit(100.0, 2.0, 5.0);
    manager.clear_stop_loss_take_profit();

    assert_eq!(manager.entry_price, None);
    assert_eq!(manager.get_rules().stop_loss_price, None);
    assert_eq!(manager.get_rules().take_profit_price, None);
}

#[test]
fn test_reset_daily_stats() {
    let rules = RiskRules::new().with_max_daily_loss(5.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 模拟一天结束,重置统计
    manager.reset_daily_stats(11000.0);

    // 新的单日亏损计算基于新的起始权益
    let result = manager.check_risk(10450.0, 0.0, 100.0);
    // 亏损 (11000-10450)/11000 = 5%, 刚好触发
    assert!(!result.is_pass());
}

#[test]
fn test_resume_trading() {
    let rules = RiskRules::new().with_max_drawdown(15.0);
    let mut manager = RiskManager::new(rules, 10000.0);

    // 触发风控
    manager.check_risk(8500.0, 15.0, 100.0);
    assert!(manager.should_stop_trading());

    // 恢复交易
    manager.resume_trading();
    assert!(!manager.should_stop_trading());
    assert_eq!(manager.get_consecutive_losses(), 0);
}

#[test]
fn test_risk_check_result_methods() {
    let pass = RiskCheckResult::Pass;
    assert!(pass.is_pass());
    assert!(!pass.should_stop_trading());
    assert_eq!(pass.get_reason(), None);

    let stop_loss = RiskCheckResult::StopLoss("触发止损".to_string());
    assert!(!stop_loss.is_pass());
    assert!(stop_loss.should_stop_trading());
    assert_eq!(stop_loss.get_reason(), Some("触发止损"));
}

#[test]
fn test_multiple_rules() {
    let rules = RiskRules::new()
        .with_max_drawdown(20.0)
        .with_max_daily_loss(5.0)
        .with_max_consecutive_losses(3)
        .with_min_equity(5000.0);

    let mut manager = RiskManager::new(rules, 10000.0);

    // 所有条件都通过
    let result = manager.check_risk(9600.0, 4.0, 100.0);
    assert!(result.is_pass());

    // 触发单日亏损限制
    let result2 = manager.check_risk(9400.0, 6.0, 100.0);
    assert!(!result2.is_pass());
}

#[test]
fn test_update_rules() {
    let initial_rules = RiskRules::new().with_max_drawdown(10.0);
    let mut manager = RiskManager::new(initial_rules, 10000.0);

    let new_rules = RiskRules::new().with_max_drawdown(20.0);
    manager.update_rules(new_rules);

    assert_eq!(manager.get_rules().max_drawdown_pct, Some(20.0));
}
