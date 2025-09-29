use tracing::{info, debug};

/// 模拟交易者 - 负责管理模拟投资组合和记录模拟交易
#[derive(Debug)]
pub struct PaperTrader {
    pub cash: f64,           // 现金余额
    pub position: f64,       // 持仓数量
    pub initial_cash: f64,   // 初始资金
    trades: Vec<PaperTrade>, // 模拟交易记录
}

/// 模拟交易记录
#[derive(Debug, Clone)]
pub struct PaperTrade {
    pub timestamp: i64,
    pub side: TradeSide,
    pub price: f64,
    pub quantity: f64,
    pub value: f64,
}

/// 交易方向
#[derive(Debug, Clone, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl PaperTrader {
    /// 创建新的模拟交易者
    pub fn new(initial_cash: f64) -> Self {
        info!("💰 初始化模拟账户，初始资金: {:.2}", initial_cash);
        
        Self {
            cash: initial_cash,
            position: 0.0,
            initial_cash,
            trades: Vec::new(),
        }
    }

    /// 执行模拟买入
    pub fn execute_paper_buy(&mut self, price: f64, timestamp: i64) {
        if self.cash > 0.0 {
            let quantity = self.cash / price;
            let value = quantity * price;
            
            self.position += quantity;
            self.cash = 0.0;
            
            let trade = PaperTrade {
                timestamp,
                side: TradeSide::Buy,
                price,
                quantity,
                value,
            };
            
            self.trades.push(trade);
            
            info!("🟢 模拟买入: 价格={:.2}, 数量={:.6}, 总价值={:.2}", price, quantity, value);
            debug!("账户状态 - 持仓: {:.6}, 现金: {:.2}", self.position, self.cash);
            
            // 发送通知（这里可以扩展为实际的通知系统）
            self.send_notification(&format!("模拟买入 {:.6} @ {:.2}", quantity, price));
        } else {
            debug!("现金不足，无法执行买入操作");
        }
    }

    /// 执行模拟卖出
    pub fn execute_paper_sell(&mut self, price: f64, timestamp: i64) {
        if self.position > 0.0 {
            let quantity = self.position;
            let value = quantity * price;
            
            self.cash += value;
            self.position = 0.0;
            
            let trade = PaperTrade {
                timestamp,
                side: TradeSide::Sell,
                price,
                quantity,
                value,
            };
            
            self.trades.push(trade);
            
            info!("🔴 模拟卖出: 价格={:.2}, 数量={:.6}, 总价值={:.2}", price, quantity, value);
            debug!("账户状态 - 持仓: {:.6}, 现金: {:.2}", self.position, self.cash);
            
            // 发送通知
            self.send_notification(&format!("模拟卖出 {:.6} @ {:.2}", quantity, price));
        } else {
            debug!("无持仓，无法执行卖出操作");
        }
    }

    /// 获取总权益
    pub fn get_total_equity(&self, current_price: f64) -> f64 {
        self.cash + self.position * current_price
    }

    /// 计算总收益率
    pub fn get_total_return(&self, current_price: f64) -> f64 {
        let current_equity = self.get_total_equity(current_price);
        (current_equity - self.initial_cash) / self.initial_cash
    }

    /// 打印账户状态
    pub fn print_status(&self, current_price: f64) {
        let equity = self.get_total_equity(current_price);
        let return_pct = self.get_total_return(current_price) * 100.0;
        
        info!("📊 账户状态:");
        info!("  现金: {:.2}", self.cash);
        info!("  持仓: {:.6} (价值: {:.2})", self.position, self.position * current_price);
        info!("  总权益: {:.2}", equity);
        info!("  收益率: {:.2}%", return_pct);
        info!("  交易次数: {}", self.trades.len());
    }

    /// 发送通知（模拟实现）
    fn send_notification(&self, message: &str) {
        // 这里可以扩展为实际的通知系统，比如：
        // - Webhook到外部系统
        // - Telegram Bot通知
        // - 邮件通知
        // - 数据库记录
        
        debug!("📢 通知: {}", message);
    }

    /// 获取交易记录
    pub fn trades(&self) -> &[PaperTrade] {
        &self.trades
    }

    /// 生成简单的统计报告
    pub fn generate_stats(&self, current_price: f64) -> PaperTradingStats {
        let total_trades = self.trades.len();
        let buy_trades = self.trades.iter().filter(|t| t.side == TradeSide::Buy).count();
        let sell_trades = self.trades.iter().filter(|t| t.side == TradeSide::Sell).count();
        
        // 计算盈利交易（简化计算）
        let profitable_pairs = self.trades
            .windows(2)
            .filter(|pair| {
                if pair.len() == 2 && pair[0].side == TradeSide::Buy && pair[1].side == TradeSide::Sell {
                    pair[1].price > pair[0].price
                } else {
                    false
                }
            })
            .count();
        
        let total_pairs = sell_trades.min(buy_trades);
        let win_rate = if total_pairs > 0 {
            profitable_pairs as f64 / total_pairs as f64
        } else {
            0.0
        };

        PaperTradingStats {
            initial_cash: self.initial_cash,
            current_equity: self.get_total_equity(current_price),
            total_return: self.get_total_return(current_price),
            total_trades,
            win_rate,
            current_price,
        }
    }
}

/// 模拟交易统计
#[derive(Debug)]
pub struct PaperTradingStats {
    pub initial_cash: f64,
    pub current_equity: f64,
    pub total_return: f64,
    pub total_trades: usize,
    pub win_rate: f64,
    pub current_price: f64,
}

impl PaperTradingStats {
    /// 打印统计报告
    pub fn print_report(&self) {
        println!("\n=== 模拟交易统计 ===");
        println!("初始资金: {:.2}", self.initial_cash);
        println!("当前权益: {:.2}", self.current_equity);
        println!("总收益率: {:.2}%", self.total_return * 100.0);
        println!("当前价格: {:.2}", self.current_price);
        println!("交易次数: {}", self.total_trades);
        println!("胜率: {:.2}%", self.win_rate * 100.0);
        println!("==================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_trader_creation() {
        let trader = PaperTrader::new(10000.0);
        assert_eq!(trader.cash, 10000.0);
        assert_eq!(trader.position, 0.0);
        assert_eq!(trader.initial_cash, 10000.0);
        assert!(trader.trades.is_empty());
    }

    #[test]
    fn test_paper_buy() {
        let mut trader = PaperTrader::new(10000.0);
        trader.execute_paper_buy(50000.0, 1640995200000);
        
        assert_eq!(trader.cash, 0.0);
        assert_eq!(trader.position, 0.2); // 10000 / 50000 = 0.2
        assert_eq!(trader.trades.len(), 1);
        assert_eq!(trader.trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn test_paper_sell() {
        let mut trader = PaperTrader::new(10000.0);
        trader.execute_paper_buy(50000.0, 1640995200000);
        trader.execute_paper_sell(55000.0, 1640998800000);
        
        assert_eq!(trader.position, 0.0);
        assert_eq!(trader.cash, 11000.0); // 0.2 * 55000 = 11000
        assert_eq!(trader.trades.len(), 2);
        assert_eq!(trader.trades[1].side, TradeSide::Sell);
    }

    #[test]
    fn test_total_equity() {
        let mut trader = PaperTrader::new(10000.0);
        trader.execute_paper_buy(50000.0, 1640995200000);
        
        let equity = trader.get_total_equity(52000.0);
        assert_eq!(equity, 10400.0); // 0.2 * 52000 = 10400
        
        let return_pct = trader.get_total_return(52000.0);
        assert_eq!(return_pct, 0.04); // 4% return
    }

    #[test]
    fn test_generate_stats() {
        let mut trader = PaperTrader::new(10000.0);
        trader.execute_paper_buy(50000.0, 1640995200000);
        trader.execute_paper_sell(55000.0, 1640998800000);
        
        let stats = trader.generate_stats(55000.0);
        assert_eq!(stats.initial_cash, 10000.0);
        assert_eq!(stats.current_equity, 11000.0);
        assert_eq!(stats.total_trades, 2);
        assert_eq!(stats.win_rate, 1.0); // 100% win rate
    }
}