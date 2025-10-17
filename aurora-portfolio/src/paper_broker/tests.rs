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
use crate::order::{OrderType, OrderSide};

#[tokio::test]
async fn test_paper_broker_market_order_buy() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        0.1,
        1001,
    );

    let result = broker.submit_order("BTC/USDT", order).await;
    assert!(result.is_ok());

    // 检查余额和持仓
    let usdt_balance = broker.get_balance("USDT").await.unwrap();
    assert_eq!(usdt_balance, 5000.0); // 10000 - 50000 * 0.1

    let btc_position = broker.get_position("BTC/USDT").await.unwrap();
    assert_eq!(btc_position, 0.1);
}

#[tokio::test]
async fn test_paper_broker_market_order_sell() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    // 先买入
    broker.submit_order("BTC/USDT", Order::new(
        OrderType::Market,
        OrderSide::Buy,
        0.1,
        1001,
    )).await.unwrap();

    // 再卖出
    let result = broker.submit_order("BTC/USDT", Order::new(
        OrderType::Market,
        OrderSide::Sell,
        0.1,
        1002,
    )).await;

    assert!(result.is_ok());

    // 余额应该回到初始值
    let usdt_balance = broker.get_balance("USDT").await.unwrap();
    assert_eq!(usdt_balance, 10000.0);

    let btc_position = broker.get_position("BTC/USDT").await.unwrap();
    assert_eq!(btc_position, 0.0);
}

#[tokio::test]
async fn test_paper_broker_limit_order() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    // 提交限价买单
    let order = Order::new(
        OrderType::Limit(49000.0),
        OrderSide::Buy,
        0.1,
        1001,
    );

    broker.submit_order("BTC/USDT", order).await.unwrap();

    // 价格未触发,持仓应为0
    let position = broker.get_position("BTC/USDT").await.unwrap();
    assert_eq!(position, 0.0);

    // 价格下降,触发订单
    let trades = broker.update_market_price("BTC/USDT", 49000.0, 1002).await.unwrap();
    assert_eq!(trades.len(), 1);

    // 检查持仓
    let position = broker.get_position("BTC/USDT").await.unwrap();
    assert_eq!(position, 0.1);
}

#[tokio::test]
async fn test_paper_broker_with_fees() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .with_fee_model(FeeModel::Percentage(0.1))
        .with_slippage_model(SlippageModel::Percentage(0.05));

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        0.1,
        1001,
    );

    broker.submit_order("BTC/USDT", order).await.unwrap();

    // 检查交易历史中的手续费
    let history = broker.get_trade_history(None, None).await.unwrap();
    assert_eq!(history.len(), 1);
    assert!(history[0].fee.is_some());

    // 由于有手续费和滑点,余额应该少于无成本情况
    let balance = broker.get_balance("USDT").await.unwrap();
    assert!(balance < 5000.0);
}

#[tokio::test]
async fn test_paper_broker_insufficient_balance() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 1000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    let order = Order::new(
        OrderType::Market,
        OrderSide::Buy,
        1.0, // 需要50000 USDT,但只有1000
        1001,
    );

    let result = broker.submit_order("BTC/USDT", order).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_paper_broker_insufficient_position() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    // 尝试卖出但没有持仓
    let order = Order::new(
        OrderType::Market,
        OrderSide::Sell,
        0.1,
        1001,
    );

    let result = broker.submit_order("BTC/USDT", order).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_paper_broker_cancel_order() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    let order = Order::new(
        OrderType::Limit(49000.0),
        OrderSide::Buy,
        0.1,
        1001,
    );

    let order_id = broker.submit_order("BTC/USDT", order).await.unwrap();

    // 取消订单
    let result = broker.cancel_order("BTC/USDT", &order_id).await;
    assert!(result.is_ok());

    // 检查订单状态
    let open_orders = broker.get_open_orders(Some("BTC/USDT")).await.unwrap();
    assert_eq!(open_orders.len(), 0);
}

#[tokio::test]
async fn test_paper_broker_get_order_status() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 10000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();

    let order = Order::new(
        OrderType::Limit(49000.0),
        OrderSide::Buy,
        0.1,
        1001,
    );

    let order_id = broker.submit_order("BTC/USDT", order).await.unwrap();

    let status = broker.get_order_status("BTC/USDT", &order_id).await.unwrap();
    assert_eq!(status, OrderStatus::Pending);
}

#[tokio::test]
async fn test_paper_broker_multiple_symbols() {
    let mut broker = PaperBroker::new()
        .with_balance("USDT", 20000.0)
        .set_enable_costs(false);

    broker.update_market_price("BTC/USDT", 50000.0, 1000).await.unwrap();
    broker.update_market_price("ETH/USDT", 3000.0, 1000).await.unwrap();

    // 买入BTC
    broker.submit_order("BTC/USDT", Order::new(
        OrderType::Market,
        OrderSide::Buy,
        0.1,
        1001,
    )).await.unwrap();

    // 买入ETH
    broker.submit_order("ETH/USDT", Order::new(
        OrderType::Market,
        OrderSide::Buy,
        1.0,
        1002,
    )).await.unwrap();

    // 检查持仓
    let btc_position = broker.get_position("BTC/USDT").await.unwrap();
    assert_eq!(btc_position, 0.1);

    let eth_position = broker.get_position("ETH/USDT").await.unwrap();
    assert_eq!(eth_position, 1.0);

    // 检查余额
    let balance = broker.get_balance("USDT").await.unwrap();
    assert_eq!(balance, 12000.0); // 20000 - 5000 - 3000
}
