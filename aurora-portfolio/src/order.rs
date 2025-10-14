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

//! 订单类型和订单管理模块
//!
//! 提供多种订单类型支持,包括市价单、限价单、止损单和止盈单。
//! 用于实现订单层面的风险控制。

use serde::{Deserialize, Serialize};

/// 订单类型
///
/// 定义交易系统支持的各种订单类型,每种类型有不同的执行逻辑和风控规则。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    /// 市价单 - 以当前市场价格立即执行
    Market,
    
    /// 限价单 - 当价格达到指定价格时执行
    ///
    /// 参数: 限价价格
    Limit(f64),
    
    /// 止损单 - 当价格跌破指定价格时触发卖出,用于限制损失
    ///
    /// 参数: 止损价格
    StopLoss(f64),
    
    /// 止盈单 - 当价格涨至指定价格时触发卖出,用于锁定利润
    ///
    /// 参数: 止盈价格
    TakeProfit(f64),
}

/// 订单状态
///
/// 描述订单的当前生命周期状态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 待执行 - 订单已创建但尚未触发
    Pending,
    
    /// 已触发 - 订单条件满足,等待执行
    Triggered,
    
    /// 已执行 - 订单成功完成
    Executed,
    
    /// 已取消 - 订单被手动或系统取消
    Cancelled,
    
    /// 已过期 - 订单超过有效期未执行
    Expired,
}

/// 订单方向
///
/// 指示订单是买入还是卖出。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 买入订单
    Buy,
    
    /// 卖出订单
    Sell,
}

/// 订单结构体
///
/// 包含订单的完整信息,包括类型、方向、数量、状态等。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单唯一标识符
    pub id: String,
    
    /// 订单类型
    pub order_type: OrderType,
    
    /// 订单方向
    pub side: OrderSide,
    
    /// 订单数量
    pub quantity: f64,
    
    /// 订单状态
    pub status: OrderStatus,
    
    /// 创建时间戳(Unix毫秒)
    pub created_at: i64,
    
    /// 触发价格(用于限价单、止损单等)
    pub trigger_price: Option<f64>,
    
    /// 执行价格(订单实际成交价格)
    pub executed_price: Option<f64>,
    
    /// 执行时间戳
    pub executed_at: Option<i64>,
    
    /// 备注信息
    pub note: Option<String>,
}

impl Order {
    /// 创建新的订单
    ///
    /// # 参数
    ///
    /// * `order_type` - 订单类型
    /// * `side` - 订单方向
    /// * `quantity` - 订单数量
    /// * `created_at` - 创建时间戳
    ///
    /// # 返回值
    ///
    /// 返回新创建的订单,状态为Pending
    pub fn new(
        order_type: OrderType,
        side: OrderSide,
        quantity: f64,
        created_at: i64,
    ) -> Self {
        // 根据订单类型提取触发价格
        let trigger_price = match &order_type {
            OrderType::Limit(price) => Some(*price),
            OrderType::StopLoss(price) => Some(*price),
            OrderType::TakeProfit(price) => Some(*price),
            OrderType::Market => None,
        };

        // 生成订单ID
        let id = format!("{}-{:?}-{}", created_at, side, uuid::Uuid::new_v4());

        Self {
            id,
            order_type,
            side,
            quantity,
            status: OrderStatus::Pending,
            created_at,
            trigger_price,
            executed_price: None,
            executed_at: None,
            note: None,
        }
    }

    /// 检查订单是否应该被触发
    ///
    /// # 参数
    ///
    /// * `current_price` - 当前市场价格
    ///
    /// # 返回值
    ///
    /// 如果订单应该被触发返回true
    pub fn should_trigger(&self, current_price: f64) -> bool {
        if self.status != OrderStatus::Pending {
            return false;
        }

        match &self.order_type {
            OrderType::Market => true,
            OrderType::Limit(limit_price) => match self.side {
                OrderSide::Buy => current_price <= *limit_price,
                OrderSide::Sell => current_price >= *limit_price,
            },
            OrderType::StopLoss(stop_price) => {
                // 止损单通常是卖出方向,价格跌破止损价时触发
                current_price <= *stop_price
            }
            OrderType::TakeProfit(take_profit_price) => {
                // 止盈单通常是卖出方向,价格涨至止盈价时触发
                current_price >= *take_profit_price
            }
        }
    }

    /// 触发订单
    ///
    /// 将订单状态从Pending变为Triggered
    pub fn trigger(&mut self) {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Triggered;
        }
    }

    /// 执行订单
    ///
    /// # 参数
    ///
    /// * `executed_price` - 实际成交价格
    /// * `executed_at` - 成交时间戳
    pub fn execute(&mut self, executed_price: f64, executed_at: i64) {
        self.executed_price = Some(executed_price);
        self.executed_at = Some(executed_at);
        self.status = OrderStatus::Executed;
    }

    /// 取消订单
    pub fn cancel(&mut self) {
        if self.status == OrderStatus::Pending || self.status == OrderStatus::Triggered {
            self.status = OrderStatus::Cancelled;
        }
    }

    /// 检查订单是否为买入订单
    pub fn is_buy(&self) -> bool {
        self.side == OrderSide::Buy
    }

    /// 检查订单是否为卖出订单
    pub fn is_sell(&self) -> bool {
        self.side == OrderSide::Sell
    }

    /// 检查订单是否已执行
    pub fn is_executed(&self) -> bool {
        self.status == OrderStatus::Executed
    }

    /// 检查订单是否待执行
    pub fn is_pending(&self) -> bool {
        self.status == OrderStatus::Pending
    }

    /// 设置订单备注
    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }
}

#[cfg(test)]
mod tests;
