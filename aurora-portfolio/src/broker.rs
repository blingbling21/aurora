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

//! 经纪商抽象层
//!
//! 提供统一的经纪商接口,用于隔离模拟交易和实盘交易的实现细节。
//! 投资组合模块通过此接口与底层交易系统交互,无需关心具体实现。

use anyhow::Result;
use async_trait::async_trait;

use crate::order::{Order, OrderStatus};
use crate::trade::Trade;

/// 经纪商统一接口
///
/// 定义了与交易经纪商交互的标准方法,包括订单提交、取消、查询等操作。
/// 所有经纪商实现(模拟交易、实盘交易)都必须实现此trait。
///
/// # 设计理念
///
/// - **解耦**: 投资组合与具体交易实现解耦,便于切换和测试
/// - **异步**: 支持异步操作,适应实盘交易的网络延迟
/// - **统一**: 模拟和实盘使用相同接口,代码可复用
///
/// # 示例
///
/// ```rust,ignore
/// use aurora_portfolio::{Broker, Order, OrderType, OrderSide};
///
/// async fn execute_trade<B: Broker>(broker: &mut B) -> Result<()> {
///     // 创建限价买入订单
///     let order = Order::new(
///         OrderType::Limit(100.0),
///         OrderSide::Buy,
///         10.0,
///         1640995200000,
///     );
///     
///     // 提交订单
///     let order_id = broker.submit_order("BTC/USDT", order).await?;
///     
///     // 查询订单状态
///     let status = broker.get_order_status("BTC/USDT", &order_id).await?;
///     
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait Broker: Send + Sync {
    /// 提交订单
    ///
    /// 将订单提交给经纪商执行。订单可能立即执行(市价单)或等待触发(限价单)。
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号,如 "BTC/USDT"
    /// * `order` - 待提交的订单
    ///
    /// # 返回值
    ///
    /// 成功时返回订单ID,失败时返回错误信息
    ///
    /// # 错误
    ///
    /// - 订单参数无效
    /// - 余额不足
    /// - 网络连接失败(实盘)
    async fn submit_order(&mut self, symbol: &str, order: Order) -> Result<String>;

    /// 取消订单
    ///
    /// 取消一个待执行或已触发的订单。已执行的订单无法取消。
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `order_id` - 订单唯一标识符
    ///
    /// # 返回值
    ///
    /// 成功返回 `Ok(())`,失败返回错误信息
    ///
    /// # 错误
    ///
    /// - 订单不存在
    /// - 订单已执行,无法取消
    /// - 网络连接失败(实盘)
    async fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<()>;

    /// 查询订单状态
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `order_id` - 订单唯一标识符
    ///
    /// # 返回值
    ///
    /// 返回订单的当前状态
    ///
    /// # 错误
    ///
    /// - 订单不存在
    /// - 网络连接失败(实盘)
    async fn get_order_status(&self, symbol: &str, order_id: &str) -> Result<OrderStatus>;

    /// 获取订单详情
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `order_id` - 订单唯一标识符
    ///
    /// # 返回值
    ///
    /// 返回完整的订单信息
    ///
    /// # 错误
    ///
    /// - 订单不存在
    /// - 网络连接失败(实盘)
    async fn get_order(&self, symbol: &str, order_id: &str) -> Result<Order>;

    /// 获取所有待执行订单
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号,如果为 None 则返回所有交易对的订单
    ///
    /// # 返回值
    ///
    /// 返回所有状态为 Pending 或 Triggered 的订单列表
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>>;

    /// 获取交易历史
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号,如果为 None 则返回所有交易对的交易历史
    /// * `limit` - 返回记录数限制
    ///
    /// # 返回值
    ///
    /// 返回已执行的交易记录列表
    async fn get_trade_history(&self, symbol: Option<&str>, limit: Option<usize>) -> Result<Vec<Trade>>;

    /// 更新市场价格
    ///
    /// 模拟经纪商需要此方法来更新市场价格,触发限价单等。
    /// 实盘经纪商可能忽略此方法,因为价格由交易所实时提供。
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    /// * `price` - 当前市场价格
    /// * `timestamp` - 价格更新时间戳
    ///
    /// # 返回值
    ///
    /// 返回因价格更新而被触发和执行的订单列表
    async fn update_market_price(
        &mut self,
        symbol: &str,
        price: f64,
        timestamp: i64,
    ) -> Result<Vec<Trade>>;

    /// 获取当前市场价格
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    ///
    /// # 返回值
    ///
    /// 返回当前市场价格,如果交易对不存在则返回错误
    async fn get_current_price(&self, symbol: &str) -> Result<f64>;

    /// 获取账户余额
    ///
    /// # 参数
    ///
    /// * `asset` - 资产符号,如 "USDT", "BTC"
    ///
    /// # 返回值
    ///
    /// 返回指定资产的可用余额
    async fn get_balance(&self, asset: &str) -> Result<f64>;

    /// 获取持仓信息
    ///
    /// # 参数
    ///
    /// * `symbol` - 交易对符号
    ///
    /// # 返回值
    ///
    /// 返回指定交易对的持仓数量
    async fn get_position(&self, symbol: &str) -> Result<f64>;
}

#[cfg(test)]
mod tests;
