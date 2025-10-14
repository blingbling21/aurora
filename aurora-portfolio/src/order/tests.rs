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
fn test_order_creation_market() {
    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    assert_eq!(order.order_type, OrderType::Market);
    assert_eq!(order.side, OrderSide::Buy);
    assert_eq!(order.quantity, 10.0);
    assert_eq!(order.status, OrderStatus::Pending);
    assert_eq!(order.trigger_price, None);
    assert!(order.is_buy());
    assert!(!order.is_sell());
}

#[test]
fn test_order_creation_limit() {
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        5.0,
        1640995200000,
    );

    assert_eq!(order.trigger_price, Some(100.0));
    assert_eq!(order.status, OrderStatus::Pending);
}

#[test]
fn test_order_creation_stop_loss() {
    let order = Order::new(
        OrderType::StopLoss(95.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );

    assert_eq!(order.trigger_price, Some(95.0));
    assert!(order.is_sell());
}

#[test]
fn test_order_creation_take_profit() {
    let order = Order::new(
        OrderType::TakeProfit(110.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );

    assert_eq!(order.trigger_price, Some(110.0));
}

#[test]
fn test_market_order_should_trigger() {
    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    // 市价单应该立即触发
    assert!(order.should_trigger(100.0));
    assert!(order.should_trigger(50.0));
    assert!(order.should_trigger(200.0));
}

#[test]
fn test_limit_buy_order_trigger() {
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    // 买入限价单:当前价格<=限价时触发
    assert!(order.should_trigger(100.0)); // 等于
    assert!(order.should_trigger(95.0));  // 低于
    assert!(!order.should_trigger(105.0)); // 高于,不触发
}

#[test]
fn test_limit_sell_order_trigger() {
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );

    // 卖出限价单:当前价格>=限价时触发
    assert!(order.should_trigger(100.0)); // 等于
    assert!(order.should_trigger(105.0)); // 高于
    assert!(!order.should_trigger(95.0));  // 低于,不触发
}

#[test]
fn test_stop_loss_order_trigger() {
    let order = Order::new(
        OrderType::StopLoss(95.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );

    // 止损单:价格跌破止损价时触发
    assert!(order.should_trigger(95.0)); // 等于
    assert!(order.should_trigger(90.0)); // 低于
    assert!(!order.should_trigger(100.0)); // 高于,不触发
}

#[test]
fn test_take_profit_order_trigger() {
    let order = Order::new(
        OrderType::TakeProfit(110.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    );

    // 止盈单:价格涨至止盈价时触发
    assert!(order.should_trigger(110.0)); // 等于
    assert!(order.should_trigger(115.0)); // 高于
    assert!(!order.should_trigger(105.0)); // 低于,不触发
}

#[test]
fn test_order_trigger_state_change() {
    let mut order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    assert_eq!(order.status, OrderStatus::Pending);
    
    order.trigger();
    assert_eq!(order.status, OrderStatus::Triggered);
    
    // 再次触发不应改变状态
    order.trigger();
    assert_eq!(order.status, OrderStatus::Triggered);
}

#[test]
fn test_order_execute() {
    let mut order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    order.trigger();
    order.execute(100.5, 1640995260000);

    assert_eq!(order.status, OrderStatus::Executed);
    assert_eq!(order.executed_price, Some(100.5));
    assert_eq!(order.executed_at, Some(1640995260000));
    assert!(order.is_executed());
}

#[test]
fn test_order_cancel() {
    let mut order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    order.cancel();
    assert_eq!(order.status, OrderStatus::Cancelled);
    
    // 已取消的订单不应触发
    assert!(!order.should_trigger(100.0));
}

#[test]
fn test_order_cancel_after_execute() {
    let mut order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    order.execute(100.0, 1640995260000);
    order.cancel();
    
    // 已执行的订单不能被取消
    assert_eq!(order.status, OrderStatus::Executed);
}

#[test]
fn test_order_with_note() {
    let order = Order::new(
        OrderType::StopLoss(95.0),
        OrderSide::Sell,
        10.0,
        1640995200000,
    )
    .with_note("风控自动止损".to_string());

    assert_eq!(order.note, Some("风控自动止损".to_string()));
}

#[test]
fn test_order_status_checks() {
    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        10.0,
        1640995200000,
    );

    assert!(order.is_pending());
    assert!(!order.is_executed());
}

#[test]
fn test_order_type_equality() {
    assert_eq!(OrderType::Market, OrderType::Market);
    assert_eq!(OrderType::Limit(100.0), OrderType::Limit(100.0));
    assert_ne!(OrderType::Limit(100.0), OrderType::Limit(101.0));
    assert_ne!(OrderType::Market, OrderType::Limit(100.0));
}

#[test]
fn test_order_side_equality() {
    assert_eq!(OrderSide::Buy, OrderSide::Buy);
    assert_eq!(OrderSide::Sell, OrderSide::Sell);
    assert_ne!(OrderSide::Buy, OrderSide::Sell);
}

#[test]
fn test_order_status_equality() {
    assert_eq!(OrderStatus::Pending, OrderStatus::Pending);
    assert_eq!(OrderStatus::Executed, OrderStatus::Executed);
    assert_ne!(OrderStatus::Pending, OrderStatus::Executed);
}
