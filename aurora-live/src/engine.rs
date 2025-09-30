use aurora_core::{MarketEvent, Signal, Strategy};
use aurora_strategy::MACrossoverStrategy;
use aurora_portfolio::Portfolio;
use crate::paper_trader::PaperTrader;
use anyhow::{Result, anyhow};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{info, error, debug};

/// 运行实时模拟交易
pub async fn run_live_trading(
    symbol: &str,
    interval: &str,
    strategy_name: &str,
    short_period: usize,
    long_period: usize,
    initial_cash: f64,
) -> Result<()> {
    // 创建策略
    let strategy = match strategy_name {
        "ma-crossover" => MACrossoverStrategy::new(short_period, long_period),
        _ => return Err(anyhow!("不支持的策略: {}", strategy_name)),
    };

    info!("初始化实时交易引擎，策略: {}, 参数: {}:{}, 交易对: {}", 
          strategy_name, short_period, long_period, symbol);

    // 创建实时引擎并运行
    let mut engine = LiveEngine::new(strategy, initial_cash);
    engine.run(symbol, interval).await?;

    Ok(())
}

/// 实时交易引擎
pub struct LiveEngine {
    strategy: MACrossoverStrategy,
    paper_trader: PaperTrader,
    last_status_time: std::time::Instant,
}

impl LiveEngine {
    /// 创建新的实时引擎
    pub fn new(strategy: MACrossoverStrategy, initial_cash: f64) -> Self {
        Self {
            strategy,
            paper_trader: PaperTrader::new(initial_cash),
            last_status_time: std::time::Instant::now(),
        }
    }

    /// 运行实时引擎
    pub async fn run(&mut self, symbol: &str, interval: &str) -> Result<()> {
        let stream_name = format!("{}@kline_{}", symbol.to_lowercase(), interval);
        
        // 尝试多个 Binance WebSocket 端点
        let endpoints = [
            "wss://stream.binance.com:9443",
            "wss://stream.binance.com:443", 
        ];
        
        let mut current_endpoint = 0;
        let mut consecutive_failures = 0;
        
        loop {
            let url = format!("{}/ws/{}", endpoints[current_endpoint], stream_name);
            info!("尝试连接到WebSocket端点 {}: {}", current_endpoint + 1, url);
            
            match self.connect_and_trade(&url).await {
                Ok(_) => {
                    info!("WebSocket连接正常结束");
                    consecutive_failures = 0;
                    break;
                }
                Err(e) => {
                    error!("WebSocket连接错误: {}", e);
                    consecutive_failures += 1;
                    
                    // 如果当前端点连续失败3次，尝试下一个端点
                    if consecutive_failures >= 3 {
                        current_endpoint = (current_endpoint + 1) % endpoints.len();
                        consecutive_failures = 0;
                        info!("切换到下一个端点，5秒后重试...");
                    } else {
                        info!("5秒后重试当前端点...");
                    }
                    
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
        
        Ok(())
    }

    /// 连接WebSocket并进行交易
    async fn connect_and_trade(&mut self, url: &str) -> Result<()> {
        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket连接成功，开始接收实时数据");
        
        let (mut write, mut read) = ws_stream.split();
        
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    debug!("收到K线消息");
                    
                    if let Err(e) = self.process_kline_message(&text).await {
                        error!("处理K线消息失败: {}", e);
                    }
                    
                    // 定期打印账户状态（每5分钟）
                    if self.last_status_time.elapsed().as_secs() >= 300 {
                        self.print_periodic_status().await;
                        self.last_status_time = std::time::Instant::now();
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket连接被关闭");
                    break;
                }
                Ok(Message::Ping(ping)) => {
                    debug!("收到Ping，发送Pong");
                    if let Err(e) = write.send(Message::Pong(ping)).await {
                        error!("发送Pong失败: {}", e);
                    }
                }
                Ok(_) => {
                    // 忽略其他类型的消息
                }
                Err(e) => {
                    error!("WebSocket消息错误: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Ok(())
    }

    /// 处理K线消息
    async fn process_kline_message(&mut self, text: &str) -> Result<()> {
        let value: Value = serde_json::from_str(text)?;
        
        if let Some(kline_data) = value.get("k") {
            // 只处理完成的K线
            let is_closed = kline_data["x"].as_bool().unwrap_or(false);
            if !is_closed {
                return Ok(());
            }

            let kline = aurora_core::Kline {
                timestamp: kline_data["t"].as_i64().unwrap_or(0),
                open: kline_data["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                high: kline_data["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                low: kline_data["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                close: kline_data["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                volume: kline_data["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            };

            info!("📊 收到完成K线: 时间={}, 收盘价={:.2}, 成交量={:.2}", 
                  kline.timestamp, kline.close, kline.volume);

            // 让策略处理K线数据
            let market_event = MarketEvent::Kline(kline.clone());
            if let Some(signal_event) = self.strategy.on_market_event(&market_event) {
                info!("🚨 策略信号: {:?} at price {:.2}", signal_event.signal, signal_event.price);
                
                // 处理交易信号
                match signal_event.signal {
                    Signal::Buy => {
                        if let Err(e) = self.paper_trader.execute_paper_buy(signal_event.price, signal_event.timestamp).await {
                            error!("执行买入失败: {}", e);
                        }
                    }
                    Signal::Sell => {
                        if let Err(e) = self.paper_trader.execute_paper_sell(signal_event.price, signal_event.timestamp).await {
                            error!("执行卖出失败: {}", e);
                        }
                    }
                    Signal::Hold => {
                        // 不执行任何操作
                    }
                }

                // 在有交易信号时立即打印状态
                self.paper_trader.print_status(kline.close);
            }
        }
        
        Ok(())
    }

    /// 定期打印账户状态
    async fn print_periodic_status(&mut self) {
        info!("⏰ 定期状态报告:");
        
        // 简化状态报告，不需要当前价格参数
        info!("  交易次数: {}", self.paper_trader.portfolio().get_trades().len());
        info!("  现金: {:.2}", self.paper_trader.get_cash());
        info!("  持仓: {:.6}", self.paper_trader.get_position());
    }

    /// 获取模拟交易者的引用
    pub fn paper_trader(&self) -> &PaperTrader {
        &self.paper_trader
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_engine_creation() {
        let strategy = MACrossoverStrategy::new(10, 30);
        let engine = LiveEngine::new(strategy, 10000.0);
        
        assert_eq!(engine.paper_trader.get_cash(), 10000.0);
        assert_eq!(engine.paper_trader.get_position(), 0.0);
        assert_eq!(engine.paper_trader.get_total_equity(100.0), 10000.0);
    }

    #[tokio::test]
    async fn test_kline_message_processing() {
        let strategy = MACrossoverStrategy::new(2, 3);
        let mut engine = LiveEngine::new(strategy, 10000.0);
        
        // 模拟完成的K线消息
        let test_message = r#"{
            "e": "kline",
            "E": 123456789,
            "s": "BTCUSDT",
            "k": {
                "t": 1640995200000,
                "T": 1640995259999,
                "s": "BTCUSDT",
                "i": "1m",
                "f": 100,
                "L": 200,
                "o": "50000.00",
                "c": "50500.00",
                "h": "51000.00",
                "l": "49000.00",
                "v": "100.0",
                "n": 100,
                "x": true,
                "q": "5050000.0",
                "V": "50.0",
                "Q": "2525000.0"
            }
        }"#;
        
        let result = engine.process_kline_message(test_message).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_incomplete_kline_ignored() {
        let strategy = MACrossoverStrategy::new(2, 3);
        let mut engine = LiveEngine::new(strategy, 10000.0);
        
        // 模拟未完成的K线消息（x: false）
        let test_message = r#"{
            "e": "kline",
            "E": 123456789,
            "s": "BTCUSDT",
            "k": {
                "t": 1640995200000,
                "T": 1640995259999,
                "s": "BTCUSDT",
                "i": "1m",
                "o": "50000.00",
                "c": "50500.00",
                "h": "51000.00",
                "l": "49000.00",
                "v": "100.0",
                "x": false
            }
        }"#;
        
        let result = engine.process_kline_message(test_message).await;
        assert!(result.is_ok());
        // 未完成的K线不应该触发任何交易
        assert_eq!(engine.paper_trader.portfolio().get_trades().len(), 0);
    }
}