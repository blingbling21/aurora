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

//! 模拟交易经纪商(Paper Broker)
//!
//! 提供完整的模拟交易环境,包括订单簿、撮合引擎、手续费和滑点模拟。
//! 用于回测和策略验证,无需连接真实交易所。

use std::collections::HashMap;
use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::broker::Broker;
use crate::order::{Order, OrderStatus};
use crate::order_book::MatchingEngine;
use crate::fees::{TradeCostCalculator, FeeModel, SlippageModel};
use crate::trade::Trade;

/// 模拟交易经纪商
///
/// 提供与真实经纪商相同的接口,但所有交易都在内存中模拟执行。
/// 支持完整的订单簿模拟、手续费和滑点计算。
///
/// # 示例
///
/// ```rust
/// use aurora_portfolio::{PaperBroker, Broker, Order, OrderType, OrderSide};
/// use aurora_portfolio::{FeeModel, SlippageModel};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 创建模拟经纪商,初始资金10000 USDT
///     let mut broker = PaperBroker::new()
///         .with_balance("USDT", 10000.0)
///         .with_fee_model(FeeModel::Percentage(0.1))
///         .with_slippage_model(SlippageModel::Percentage(0.05));
///     
///     // 设置市场价格
///     broker.update_market_price("BTC/USDT", 50000.0, 1640995200000).await?;
///     
///     // 提交限价买单
///     let order = Order::new(
///         OrderType::Limit(49000.0),
///         OrderSide::Buy,
///         0.1,
///         1640995200000,
///     );
///     let order_id = broker.submit_order("BTC/USDT", order).await?;
///     
///     Ok(())
/// }
/// ```
pub struct PaperBroker {
    /// 撮合引擎
    matching_engine: MatchingEngine,
    /// 交易成本计算器
    cost_calculator: TradeCostCalculator,
    /// 账户余额: 资产符号 -> 余额
    balances: HashMap<String, f64>,
    /// 持仓信息: 交易对符号 -> 持仓数量
    positions: HashMap<String, f64>,
    /// 交易历史
    trade_history: Vec<Trade>,
    /// 是否启用手续费和滑点
    enable_costs: bool,
}

impl PaperBroker {
    /// 创建新的模拟经纪商
    ///
    /// 默认启用手续费和滑点模拟,使用默认的费率设置。
    pub fn new() -> Self {
        Self {
            matching_engine: MatchingEngine::new(),
            cost_calculator: TradeCostCalculator::default(),
            balances: HashMap::new(),
            positions: HashMap::new(),
            trade_history: Vec::new(),
            enable_costs: true,
        }
    }

    /// 设置账户余额
    ///
    /// # 参数
    ///
    /// * `asset` - 资产符号,如 "USDT", "BTC"
    /// * `amount` - 余额数量
    pub fn with_balance(mut self, asset: &str, amount: f64) -> Self {
        self.balances.insert(asset.to_string(), amount);
        self
    }

    /// 设置手续费模型
    pub fn with_fee_model(mut self, fee_model: FeeModel) -> Self {
        let slippage_model = self.cost_calculator.slippage_model().clone();
        self.cost_calculator = TradeCostCalculator::new(fee_model, slippage_model);
        self
    }

    /// 设置滑点模型
    pub fn with_slippage_model(mut self, slippage_model: SlippageModel) -> Self {
        let fee_model = self.cost_calculator.fee_model().clone();
        self.cost_calculator = TradeCostCalculator::new(fee_model, slippage_model);
        self
    }

    /// 启用或禁用手续费和滑点
    pub fn set_enable_costs(mut self, enable: bool) -> Self {
        self.enable_costs = enable;
        self
    }

    /// 检查并更新余额(买入)
    fn check_and_update_balance_buy(
        &mut self,
        quote_asset: &str,
        cost: f64,
    ) -> Result<()> {
        let balance = self.balances.get(quote_asset).copied().unwrap_or(0.0);
        if balance < cost {
            return Err(anyhow!(
                "余额不足: 需要 {} {},当前余额 {}",
                cost,
                quote_asset,
                balance
            ));
        }
        self.balances.insert(quote_asset.to_string(), balance - cost);
        Ok(())
    }

    /// 更新余额和持仓(卖出)
    fn update_balance_and_position_sell(
        &mut self,
        symbol: &str,
        quote_asset: &str,
        quantity: f64,
        proceeds: f64,
    ) -> Result<()> {
        // 检查持仓
        let position = self.positions.get(symbol).copied().unwrap_or(0.0);
        if position < quantity {
            return Err(anyhow!(
                "持仓不足: 需要 {} {},当前持仓 {}",
                quantity,
                symbol,
                position
            ));
        }

        // 更新持仓
        self.positions.insert(symbol.to_string(), position - quantity);

        // 更新余额
        let balance = self.balances.get(quote_asset).copied().unwrap_or(0.0);
        self.balances.insert(quote_asset.to_string(), balance + proceeds);

        Ok(())
    }

    /// 更新持仓(买入)
    fn update_position_buy(&mut self, symbol: &str, quantity: f64) {
        let position = self.positions.get(symbol).copied().unwrap_or(0.0);
        self.positions.insert(symbol.to_string(), position + quantity);
    }

    /// 解析交易对,获取基础资产和计价资产
    ///
    /// 例: "BTC/USDT" -> ("BTC", "USDT")
    fn parse_symbol(&self, symbol: &str) -> Result<(String, String)> {
        let parts: Vec<&str> = symbol.split('/').collect();
        if parts.len() != 2 {
            return Err(anyhow!("无效的交易对格式: {}", symbol));
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    /// 执行交易并应用成本
    fn execute_trade_with_costs(
        &mut self,
        symbol: &str,
        mut trade: Trade,
        is_buy: bool,
    ) -> Result<Trade> {
        let (_base_asset, quote_asset) = self.parse_symbol(symbol)?;

        if self.enable_costs {
            // 计算交易成本
            let cost = if is_buy {
                self.cost_calculator.calculate_buy_cost(
                    trade.price,
                    trade.quantity,
                    None, // TODO: 添加成交量数据
                    None, // TODO: 添加波动率数据
                    false, // TODO: 区分 Maker/Taker
                )
            } else {
                self.cost_calculator.calculate_sell_cost(
                    trade.price,
                    trade.quantity,
                    None,
                    None,
                    false,
                )
            };

            // 更新交易记录中的价格和手续费
            trade.price = cost.executed_price;
            trade.fee = Some(cost.fee);

            // 更新余额和持仓
            if is_buy {
                self.check_and_update_balance_buy(&quote_asset, cost.total_cost.abs())?;
                self.update_position_buy(symbol, trade.quantity);
            } else {
                let proceeds = cost.total_cost.abs();
                self.update_balance_and_position_sell(
                    symbol,
                    &quote_asset,
                    trade.quantity,
                    proceeds,
                )?;
            }
        } else {
            // 不启用成本时,直接更新余额和持仓
            let trade_value = trade.price * trade.quantity;
            if is_buy {
                self.check_and_update_balance_buy(&quote_asset, trade_value)?;
                self.update_position_buy(symbol, trade.quantity);
            } else {
                self.update_balance_and_position_sell(
                    symbol,
                    &quote_asset,
                    trade.quantity,
                    trade_value,
                )?;
            }
        }

        Ok(trade)
    }
}

#[async_trait]
impl Broker for PaperBroker {
    async fn submit_order(&mut self, symbol: &str, order: Order) -> Result<String> {
        let order_id = order.id.clone();
        let is_buy = order.is_buy();

        // 提交订单到撮合引擎
        let trade_opt = self.matching_engine.submit_order(symbol, order)?;

        // 如果是市价单立即执行,应用成本
        if let Some(trade) = trade_opt {
            let trade = self.execute_trade_with_costs(symbol, trade, is_buy)?;
            self.trade_history.push(trade);
        }

        Ok(order_id)
    }

    async fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<()> {
        self.matching_engine.cancel_order(symbol, order_id)?;
        Ok(())
    }

    async fn get_order_status(&self, symbol: &str, order_id: &str) -> Result<OrderStatus> {
        let order = self.matching_engine.get_order(symbol, order_id)
            .ok_or_else(|| anyhow!("订单不存在: {}", order_id))?;
        Ok(order.status.clone())
    }

    async fn get_order(&self, symbol: &str, order_id: &str) -> Result<Order> {
        self.matching_engine.get_order(symbol, order_id)
            .cloned()
            .ok_or_else(|| anyhow!("订单不存在: {}", order_id))
    }

    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        Ok(self.matching_engine.get_open_orders(symbol))
    }

    async fn get_trade_history(
        &self,
        _symbol: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<Trade>> {
        let trades = if let Some(lim) = limit {
            self.trade_history.iter().rev().take(lim).cloned().collect()
        } else {
            self.trade_history.clone()
        };
        Ok(trades)
    }

    async fn update_market_price(
        &mut self,
        symbol: &str,
        price: f64,
        timestamp: i64,
    ) -> Result<Vec<Trade>> {
        // 更新价格并触发订单
        let mut trades = self.matching_engine.update_price(symbol, price, timestamp)?;

        // 对所有触发的交易应用成本
        let mut processed_trades = Vec::new();
        for trade in trades.drain(..) {
            let is_buy = trade.side == crate::trade::TradeSide::Buy;
            let processed_trade = self.execute_trade_with_costs(symbol, trade, is_buy)?;
            self.trade_history.push(processed_trade.clone());
            processed_trades.push(processed_trade);
        }

        Ok(processed_trades)
    }

    async fn get_current_price(&self, symbol: &str) -> Result<f64> {
        self.matching_engine.get_current_price(symbol)
            .ok_or_else(|| anyhow!("交易对 {} 的价格未设置", symbol))
    }

    async fn get_balance(&self, asset: &str) -> Result<f64> {
        Ok(self.balances.get(asset).copied().unwrap_or(0.0))
    }

    async fn get_position(&self, symbol: &str) -> Result<f64> {
        Ok(self.positions.get(symbol).copied().unwrap_or(0.0))
    }
}

impl Default for PaperBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
