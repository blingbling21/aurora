use tracing::{info, debug};

/// 投资组合管理
#[derive(Debug, Clone)]
pub struct Portfolio {
    pub cash: f64,           // 现金余额
    pub position: f64,       // 持仓数量
    pub initial_equity: f64, // 初始资金
    trades: Vec<Trade>,      // 交易记录
    equity_curve: Vec<EquityPoint>, // 权益曲线
}

/// 交易记录
#[derive(Debug, Clone)]
pub struct Trade {
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

/// 权益曲线点
#[derive(Debug, Clone)]
pub struct EquityPoint {
    pub timestamp: i64,
    pub equity: f64,
    pub drawdown: f64,
}

/// 回测报告
#[derive(Debug)]
pub struct BacktestReport {
    pub total_return: f64,
    pub annualized_return: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub win_rate: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
}

impl Portfolio {
    /// 创建新的投资组合
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash: initial_cash,
            position: 0.0,
            initial_equity: initial_cash,
            trades: Vec::new(),
            equity_curve: Vec::new(),
        }
    }

    /// 执行买入操作
    pub fn execute_buy(&mut self, price: f64, timestamp: i64) {
        // 简化处理：用所有现金买入
        if self.cash > 0.0 {
            let quantity = self.cash / price;
            let value = quantity * price;
            
            self.position += quantity;
            self.cash = 0.0;
            
            let trade = Trade {
                timestamp,
                side: TradeSide::Buy,
                price,
                quantity,
                value,
            };
            
            self.trades.push(trade);
            
            info!("📈 买入: 价格={:.2}, 数量={:.6}, 总价值={:.2}", price, quantity, value);
            debug!("当前持仓: {:.6}, 现金: {:.2}", self.position, self.cash);
        }
    }

    /// 执行卖出操作
    pub fn execute_sell(&mut self, price: f64, timestamp: i64) {
        // 简化处理：卖出所有持仓
        if self.position > 0.0 {
            let quantity = self.position;
            let value = quantity * price;
            
            self.cash += value;
            self.position = 0.0;
            
            let trade = Trade {
                timestamp,
                side: TradeSide::Sell,
                price,
                quantity,
                value,
            };
            
            self.trades.push(trade);
            
            info!("📉 卖出: 价格={:.2}, 数量={:.6}, 总价值={:.2}", price, quantity, value);
            debug!("当前持仓: {:.6}, 现金: {:.2}", self.position, self.cash);
        }
    }

    /// 计算总权益
    pub fn get_total_equity(&self, current_price: f64) -> f64 {
        self.cash + self.position * current_price
    }

    /// 更新权益曲线
    pub fn update_equity_curve(&mut self, timestamp: i64, current_price: f64) {
        let current_equity = self.get_total_equity(current_price);
        let max_equity = self.equity_curve
            .iter()
            .map(|point| point.equity)
            .fold(self.initial_equity, f64::max);
        
        let drawdown = if max_equity > 0.0 {
            (max_equity - current_equity) / max_equity
        } else {
            0.0
        };

        self.equity_curve.push(EquityPoint {
            timestamp,
            equity: current_equity,
            drawdown,
        });
    }

    /// 生成回测报告
    pub fn generate_report(&self) -> BacktestReport {
        let final_equity = self.equity_curve.last().map(|p| p.equity).unwrap_or(self.initial_equity);
        let total_return = (final_equity - self.initial_equity) / self.initial_equity;
        
        // 简化的年化收益率计算（假设数据跨度为1年）
        let annualized_return = total_return;
        
        // 计算最大回撤
        let max_drawdown = self.equity_curve
            .iter()
            .map(|point| point.drawdown)
            .fold(0.0, f64::max);
        
        // 简化的夏普比率计算（假设无风险利率为0）
        let returns: Vec<f64> = self.equity_curve
            .windows(2)
            .map(|window| (window[1].equity - window[0].equity) / window[0].equity)
            .collect();
        
        let mean_return = if returns.is_empty() { 0.0 } else {
            returns.iter().sum::<f64>() / returns.len() as f64
        };
        
        let return_std = if returns.len() < 2 { 0.0 } else {
            let variance = returns.iter()
                .map(|r| (r - mean_return).powi(2))
                .sum::<f64>() / (returns.len() - 1) as f64;
            variance.sqrt()
        };
        
        let sharpe_ratio = if return_std > 0.0 {
            mean_return / return_std * (252.0_f64).sqrt() // 假设日数据
        } else {
            0.0
        };
        
        // 计算胜率
        let winning_trades = self.trades
            .windows(2)
            .filter(|trades| {
                if trades.len() == 2 && trades[0].side == TradeSide::Buy && trades[1].side == TradeSide::Sell {
                    trades[1].price > trades[0].price
                } else {
                    false
                }
            })
            .count();
        
        let total_trade_pairs = self.trades.len() / 2;
        let losing_trades = total_trade_pairs.saturating_sub(winning_trades);
        let win_rate = if total_trade_pairs > 0 {
            winning_trades as f64 / total_trade_pairs as f64
        } else {
            0.0
        };

        BacktestReport {
            total_return,
            annualized_return,
            max_drawdown,
            sharpe_ratio,
            win_rate,
            total_trades: self.trades.len(),
            winning_trades,
            losing_trades,
        }
    }

    /// 打印回测报告
    pub fn print_report(&self) {
        let report = self.generate_report();
        
        println!("\n=== 回测报告 ===");
        println!("初始资金: {:.2}", self.initial_equity);
        println!("最终权益: {:.2}", self.equity_curve.last().map(|p| p.equity).unwrap_or(self.initial_equity));
        println!("总收益率: {:.2}%", report.total_return * 100.0);
        println!("年化收益率: {:.2}%", report.annualized_return * 100.0);
        println!("最大回撤: {:.2}%", report.max_drawdown * 100.0);
        println!("夏普比率: {:.2}", report.sharpe_ratio);
        println!("总交易次数: {}", report.total_trades);
        println!("盈利交易: {}", report.winning_trades);
        println!("亏损交易: {}", report.losing_trades);
        println!("胜率: {:.2}%", report.win_rate * 100.0);
        println!("===============\n");
    }

    /// 获取交易记录
    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// 获取权益曲线
    pub fn equity_curve(&self) -> &[EquityPoint] {
        &self.equity_curve
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_creation() {
        let portfolio = Portfolio::new(10000.0);
        assert_eq!(portfolio.cash, 10000.0);
        assert_eq!(portfolio.position, 0.0);
        assert_eq!(portfolio.initial_equity, 10000.0);
        assert!(portfolio.trades.is_empty());
    }

    #[test]
    fn test_buy_execution() {
        let mut portfolio = Portfolio::new(10000.0);
        portfolio.execute_buy(50000.0, 1640995200000);
        
        assert_eq!(portfolio.cash, 0.0);
        assert_eq!(portfolio.position, 0.2); // 10000 / 50000 = 0.2
        assert_eq!(portfolio.trades.len(), 1);
        assert_eq!(portfolio.trades[0].side, TradeSide::Buy);
    }

    #[test]
    fn test_sell_execution() {
        let mut portfolio = Portfolio::new(10000.0);
        portfolio.execute_buy(50000.0, 1640995200000);
        portfolio.execute_sell(55000.0, 1640998800000);
        
        assert_eq!(portfolio.position, 0.0);
        assert_eq!(portfolio.cash, 11000.0); // 0.2 * 55000 = 11000
        assert_eq!(portfolio.trades.len(), 2);
        assert_eq!(portfolio.trades[1].side, TradeSide::Sell);
    }

    #[test]
    fn test_total_equity_calculation() {
        let mut portfolio = Portfolio::new(10000.0);
        portfolio.execute_buy(50000.0, 1640995200000);
        
        let equity = portfolio.get_total_equity(52000.0);
        assert_eq!(equity, 10400.0); // 0.2 * 52000 = 10400
    }
}