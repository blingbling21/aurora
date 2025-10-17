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
use crate::order::{OrderSide, OrderType};

#[test]
fn test_order_book_add_limit_order() {
    let mut order_book = OrderBook::new("BTC/USDT".to_string());
    
    let buy_order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    );
    
    assert!(order_book.add_order(buy_order).is_ok());
    assert_eq!(order_book.get_open_orders().len(), 1);
}

#[test]
fn test_order_book_cancel_order() {
    let mut order_book = OrderBook::new("BTC/USDT".to_string());
    
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    );
    
    let order_id = order.id.clone();
    order_book.add_order(order).unwrap();
    
    assert!(order_book.cancel_order(&order_id).is_ok());
    assert_eq!(order_book.get_open_orders().len(), 0);
}

#[test]
fn test_order_book_best_bid_ask() {
    let mut order_book = OrderBook::new("BTC/USDT".to_string());
    
    // 添加买单
    order_book.add_order(Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    )).unwrap();
    
    order_book.add_order(Order::new(
        OrderType::Limit(99.0),
        OrderSide::Buy,
        1.0,
        1001,
    )).unwrap();
    
    // 添加卖单
    order_book.add_order(Order::new(
        OrderType::Limit(101.0),
        OrderSide::Sell,
        1.0,
        1002,
    )).unwrap();
    
    order_book.add_order(Order::new(
        OrderType::Limit(102.0),
        OrderSide::Sell,
        1.0,
        1003,
    )).unwrap();
    
    assert_eq!(order_book.best_bid(), Some(100.0));
    assert_eq!(order_book.best_ask(), Some(101.0));
}

#[test]
fn test_order_book_depth() {
    let mut order_book = OrderBook::new("BTC/USDT".to_string());
    
    // 添加多个买单
    order_book.add_order(Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        2.0,
        1000,
    )).unwrap();
    
    order_book.add_order(Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        3.0,
        1001,
    )).unwrap();
    
    order_book.add_order(Order::new(
        OrderType::Limit(99.0),
        OrderSide::Buy,
        1.0,
        1002,
    )).unwrap();
    
    let bid_depth = order_book.get_bid_depth(2);
    assert_eq!(bid_depth.len(), 2);
    assert_eq!(bid_depth[0], (100.0, 5.0)); // 价格100的总量
    assert_eq!(bid_depth[1], (99.0, 1.0));
}

#[test]
fn test_matching_engine_market_order() {
    let mut engine = MatchingEngine::new();
    
    // 设置市场价格
    engine.update_price("BTC/USDT", 100.0, 1000).unwrap();
    
    // 提交市价买单
    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        1.0,
        1001,
    );
    
    let result = engine.submit_order("BTC/USDT", order);
    assert!(result.is_ok());
    
    let trade = result.unwrap();
    assert!(trade.is_some());
    
    let trade = trade.unwrap();
    assert_eq!(trade.price, 100.0);
    assert_eq!(trade.quantity, 1.0);
}

#[test]
fn test_matching_engine_limit_order_trigger() {
    let mut engine = MatchingEngine::new();
    
    // 提交限价买单,价格为100
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    );
    
    engine.submit_order("BTC/USDT", order).unwrap();
    
    // 市场价格更新为100,应触发订单
    let trades = engine.update_price("BTC/USDT", 100.0, 1001).unwrap();
    
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 100.0);
    assert_eq!(trades[0].quantity, 1.0);
}

#[test]
fn test_matching_engine_stop_loss_trigger() {
    let mut engine = MatchingEngine::new();
    
    // 提交止损单,止损价为95
    let order = Order::new(
        OrderType::StopLoss(95.0),
        OrderSide::Sell,
        1.0,
        1000,
    );
    
    engine.submit_order("BTC/USDT", order).unwrap();
    
    // 市场价格跌破95,应触发止损
    let trades = engine.update_price("BTC/USDT", 94.0, 1001).unwrap();
    
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].price, 94.0);
    assert_eq!(trades[0].side, TradeSide::Sell);
}

#[test]
fn test_matching_engine_multiple_orders() {
    let mut engine = MatchingEngine::new();
    
    // 提交多个限价买单
    engine.submit_order("BTC/USDT", Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    )).unwrap();
    
    engine.submit_order("BTC/USDT", Order::new(
        OrderType::Limit(99.0),
        OrderSide::Buy,
        1.0,
        1001,
    )).unwrap();
    
    // 价格更新为99,应触发两个订单
    let trades = engine.update_price("BTC/USDT", 99.0, 1002).unwrap();
    
    assert_eq!(trades.len(), 2);
}

#[test]
fn test_matching_engine_get_open_orders() {
    let mut engine = MatchingEngine::new();
    
    engine.submit_order("BTC/USDT", Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    )).unwrap();
    
    engine.submit_order("ETH/USDT", Order::new(
        OrderType::Limit(200.0),
        OrderSide::Buy,
        1.0,
        1001,
    )).unwrap();
    
    // 获取所有订单
    let all_orders = engine.get_open_orders(None);
    assert_eq!(all_orders.len(), 2);
    
    // 获取指定交易对的订单
    let btc_orders = engine.get_open_orders(Some("BTC/USDT"));
    assert_eq!(btc_orders.len(), 1);
}

#[test]
fn test_matching_engine_cancel_order() {
    let mut engine = MatchingEngine::new();
    
    let order = Order::new(
        OrderType::Limit(100.0),
        OrderSide::Buy,
        1.0,
        1000,
    );
    
    let order_id = order.id.clone();
    engine.submit_order("BTC/USDT", order).unwrap();
    
    assert!(engine.cancel_order("BTC/USDT", &order_id).is_ok());
    assert_eq!(engine.get_open_orders(Some("BTC/USDT")).len(), 0);
}
