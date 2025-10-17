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

//! 订单簿和撮合引擎
//!
//! 提供订单簿管理和订单撮合功能,用于模拟真实市场的订单执行机制。
//! 支持限价单、止损单等多种订单类型的触发和撮合。

use std::collections::{BTreeMap, HashMap};
use anyhow::{Result, anyhow};

use crate::order::{Order, OrderType};
use crate::trade::{Trade, TradeSide, TradeBuilder};

/// 订单簿
///
/// 管理某个交易对的所有待执行订单,按价格和时间排序。
/// 买单按价格降序排列(高价优先),卖单按价格升序排列(低价优先)。
#[derive(Debug, Clone)]
pub struct OrderBook {
    /// 交易对符号
    symbol: String,
    /// 买单簿: 价格 -> 订单列表(价格从高到低)
    bids: BTreeMap<OrderedFloat, Vec<Order>>,
    /// 卖单簿: 价格 -> 订单列表(价格从低到高)
    asks: BTreeMap<OrderedFloat, Vec<Order>>,
    /// 止损单列表(价格触发后转为市价单)
    stop_orders: Vec<Order>,
    /// 订单ID索引: ID -> 订单
    order_index: HashMap<String, Order>,
}

/// 用于 BTreeMap 的可排序浮点数包装
///
/// 由于 f64 不实现 Ord,需要包装一层以支持 BTreeMap
#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl OrderBook {
    /// 创建新的订单簿
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            stop_orders: Vec::new(),
            order_index: HashMap::new(),
        }
    }

    /// 添加订单到订单簿
    ///
    /// # 参数
    ///
    /// * `order` - 待添加的订单
    ///
    /// # 返回值
    ///
    /// 成功返回 `Ok(())`,失败返回错误信息
    pub fn add_order(&mut self, order: Order) -> Result<()> {
        // 检查订单是否已存在
        if self.order_index.contains_key(&order.id) {
            return Err(anyhow!("订单ID已存在: {}", order.id));
        }

        // 根据订单类型添加到相应的列表
        match &order.order_type {
            OrderType::Market => {
                // 市价单应立即执行,不应添加到订单簿
                return Err(anyhow!("市价单不应添加到订单簿"));
            }
            OrderType::Limit(price) => {
                let price_key = OrderedFloat(*price);
                if order.is_buy() {
                    self.bids.entry(price_key).or_insert_with(Vec::new).push(order.clone());
                } else {
                    self.asks.entry(price_key).or_insert_with(Vec::new).push(order.clone());
                }
            }
            OrderType::StopLoss(_) | OrderType::TakeProfit(_) => {
                // 止损和止盈单添加到独立列表
                self.stop_orders.push(order.clone());
            }
        }

        // 添加到索引
        self.order_index.insert(order.id.clone(), order);
        Ok(())
    }

    /// 取消订单
    ///
    /// # 参数
    ///
    /// * `order_id` - 订单ID
    ///
    /// # 返回值
    ///
    /// 成功返回被取消的订单,失败返回错误信息
    pub fn cancel_order(&mut self, order_id: &str) -> Result<Order> {
        // 从索引中查找订单
        let order = self.order_index.remove(order_id)
            .ok_or_else(|| anyhow!("订单不存在: {}", order_id))?;

        // 从相应的订单簿中删除
        match &order.order_type {
            OrderType::Limit(price) => {
                let price_key = OrderedFloat(*price);
                if order.is_buy() {
                    if let Some(orders) = self.bids.get_mut(&price_key) {
                        orders.retain(|o| o.id != order_id);
                        if orders.is_empty() {
                            self.bids.remove(&price_key);
                        }
                    }
                } else {
                    if let Some(orders) = self.asks.get_mut(&price_key) {
                        orders.retain(|o| o.id != order_id);
                        if orders.is_empty() {
                            self.asks.remove(&price_key);
                        }
                    }
                }
            }
            OrderType::StopLoss(_) | OrderType::TakeProfit(_) => {
                self.stop_orders.retain(|o| o.id != order_id);
            }
            _ => {}
        }

        Ok(order)
    }

    /// 获取订单
    ///
    /// # 参数
    ///
    /// * `order_id` - 订单ID
    pub fn get_order(&self, order_id: &str) -> Option<&Order> {
        self.order_index.get(order_id)
    }

    /// 获取所有待执行订单
    pub fn get_open_orders(&self) -> Vec<Order> {
        self.order_index.values().cloned().collect()
    }

    /// 获取买单簿深度
    ///
    /// # 参数
    ///
    /// * `depth` - 返回的价格档位数量
    ///
    /// # 返回值
    ///
    /// 返回 (价格, 数量) 的列表,按价格从高到低排列
    pub fn get_bid_depth(&self, depth: usize) -> Vec<(f64, f64)> {
        self.bids
            .iter()
            .rev() // 买单按价格从高到低
            .take(depth)
            .map(|(price, orders)| {
                let total_quantity: f64 = orders.iter().map(|o| o.quantity).sum();
                (price.0, total_quantity)
            })
            .collect()
    }

    /// 获取卖单簿深度
    ///
    /// # 参数
    ///
    /// * `depth` - 返回的价格档位数量
    ///
    /// # 返回值
    ///
    /// 返回 (价格, 数量) 的列表,按价格从低到高排列
    pub fn get_ask_depth(&self, depth: usize) -> Vec<(f64, f64)> {
        self.asks
            .iter()
            .take(depth)
            .map(|(price, orders)| {
                let total_quantity: f64 = orders.iter().map(|o| o.quantity).sum();
                (price.0, total_quantity)
            })
            .collect()
    }

    /// 获取最优买价(Bid)
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.iter().rev().next().map(|(price, _)| price.0)
    }

    /// 获取最优卖价(Ask)
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.iter().next().map(|(price, _)| price.0)
    }

    /// 获取交易对符号
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

/// 撮合引擎
///
/// 负责根据市场价格触发和撮合订单,生成交易记录。
pub struct MatchingEngine {
    /// 各交易对的订单簿
    order_books: HashMap<String, OrderBook>,
    /// 各交易对的当前价格
    current_prices: HashMap<String, f64>,
}

impl MatchingEngine {
    /// 创建新的撮合引擎
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
            current_prices: HashMap::new(),
        }
    }

    /// 获取或创建订单簿
    fn get_or_create_order_book(&mut self, symbol: &str) -> &mut OrderBook {
        self.order_books
            .entry(symbol.to_string())
            .or_insert_with(|| OrderBook::new(symbol.to_string()))
    }

    /// 提交订单
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `order` - 待提交的订单
    ///
    /// # 返回值
    ///
    /// 对于市价单,立即返回交易记录;对于其他类型订单,返回 None
    pub fn submit_order(&mut self, symbol: &str, order: Order) -> Result<Option<Trade>> {
        match order.order_type {
            OrderType::Market => {
                // 市价单立即执行
                let current_price = self.current_prices.get(symbol)
                    .ok_or_else(|| anyhow!("交易对 {} 的市场价格未设置", symbol))?;
                
                let trade = self.execute_market_order(symbol, order, *current_price)?;
                Ok(Some(trade))
            }
            _ => {
                // 其他类型订单添加到订单簿
                let order_book = self.get_or_create_order_book(symbol);
                order_book.add_order(order)?;
                Ok(None)
            }
        }
    }

    /// 执行市价单
    fn execute_market_order(&self, _symbol: &str, mut order: Order, price: f64) -> Result<Trade> {
        let timestamp = order.created_at;
        order.execute(price, timestamp);

        let side = if order.is_buy() {
            TradeSide::Buy
        } else {
            TradeSide::Sell
        };

        let trade = TradeBuilder::new(side, price, order.quantity, timestamp)
            .build();

        Ok(trade)
    }

    /// 更新市场价格并触发订单
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `price` - 新的市场价格
    /// * `timestamp` - 价格更新时间戳
    ///
    /// # 返回值
    ///
    /// 返回被触发并执行的交易列表
    pub fn update_price(&mut self, symbol: &str, price: f64, timestamp: i64) -> Result<Vec<Trade>> {
        // 更新当前价格
        self.current_prices.insert(symbol.to_string(), price);

        // 获取或创建订单簿
        let order_book = self.order_books
            .entry(symbol.to_string())
            .or_insert_with(|| OrderBook::new(symbol.to_string()));

        let mut trades = Vec::new();

        // 检查并触发限价单
        trades.extend(Self::match_limit_orders_static(order_book, price, timestamp)?);

        // 检查并触发止损/止盈单
        trades.extend(Self::match_stop_orders_static(order_book, price, timestamp)?);

        Ok(trades)
    }

    /// 撮合限价单(静态方法)
    fn match_limit_orders_static(
        order_book: &mut OrderBook,
        price: f64,
        timestamp: i64,
    ) -> Result<Vec<Trade>> {
        let mut trades = Vec::new();
        let mut orders_to_remove = Vec::new();

        // 检查买单簿:如果市场价格低于限价买单的价格,则触发
        for (order_price, orders) in order_book.bids.iter_mut() {
            if price <= order_price.0 {
                for order in orders.iter_mut() {
                    if order.should_trigger(price) {
                        order.execute(price, timestamp);
                        let trade = TradeBuilder::new(
                            TradeSide::Buy,
                            price,
                            order.quantity,
                            timestamp,
                        ).build();
                        trades.push(trade);
                        orders_to_remove.push(order.id.clone());
                    }
                }
            }
        }

        // 检查卖单簿:如果市场价格高于限价卖单的价格,则触发
        for (order_price, orders) in order_book.asks.iter_mut() {
            if price >= order_price.0 {
                for order in orders.iter_mut() {
                    if order.should_trigger(price) {
                        order.execute(price, timestamp);
                        let trade = TradeBuilder::new(
                            TradeSide::Sell,
                            price,
                            order.quantity,
                            timestamp,
                        ).build();
                        trades.push(trade);
                        orders_to_remove.push(order.id.clone());
                    }
                }
            }
        }

        // 从订单簿中移除已执行的订单
        for order_id in orders_to_remove {
            order_book.cancel_order(&order_id)?;
        }

        Ok(trades)
    }

    /// 撮合止损/止盈单(静态方法)
    fn match_stop_orders_static(
        order_book: &mut OrderBook,
        price: f64,
        timestamp: i64,
    ) -> Result<Vec<Trade>> {
        let mut trades = Vec::new();
        let mut orders_to_remove = Vec::new();

        for order in order_book.stop_orders.iter_mut() {
            if order.should_trigger(price) {
                order.execute(price, timestamp);
                let side = if order.is_buy() {
                    TradeSide::Buy
                } else {
                    TradeSide::Sell
                };
                let trade = TradeBuilder::new(side, price, order.quantity, timestamp)
                    .build();
                trades.push(trade);
                orders_to_remove.push(order.id.clone());
            }
        }

        // 从止损单列表中移除已执行的订单
        for order_id in &orders_to_remove {
            order_book.cancel_order(order_id)?;
        }

        Ok(trades)
    }

    /// 取消订单
    pub fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<Order> {
        let order_book = self.order_books.get_mut(symbol)
            .ok_or_else(|| anyhow!("交易对 {} 不存在", symbol))?;
        order_book.cancel_order(order_id)
    }

    /// 获取订单
    pub fn get_order(&self, symbol: &str, order_id: &str) -> Option<&Order> {
        self.order_books.get(symbol)?.get_order(order_id)
    }

    /// 获取所有待执行订单
    pub fn get_open_orders(&self, symbol: Option<&str>) -> Vec<Order> {
        if let Some(sym) = symbol {
            self.order_books.get(sym)
                .map(|ob| ob.get_open_orders())
                .unwrap_or_default()
        } else {
            self.order_books.values()
                .flat_map(|ob| ob.get_open_orders())
                .collect()
        }
    }

    /// 获取当前价格
    pub fn get_current_price(&self, symbol: &str) -> Option<f64> {
        self.current_prices.get(symbol).copied()
    }

    /// 获取订单簿
    pub fn get_order_book(&self, symbol: &str) -> Option<&OrderBook> {
        self.order_books.get(symbol)
    }
}

impl Default for MatchingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
