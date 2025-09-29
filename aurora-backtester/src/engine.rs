use aurora_core::{Kline, MarketEvent, Signal, Strategy};
use aurora_strategy::MACrossoverStrategy;
use crate::portfolio::Portfolio;
use anyhow::{Result, anyhow};
use std::path::Path;
use tracing::{info, debug, error};

/// 运行回测
pub async fn run_backtest(
    data_path: &str,
    strategy_name: &str,
    short_period: usize,
    long_period: usize,
    initial_cash: f64,
) -> Result<()> {
    // 验证数据文件是否存在
    if !Path::new(data_path).exists() {
        return Err(anyhow!("数据文件不存在: {}", data_path));
    }

    info!("加载数据文件: {}", data_path);
    let klines = load_klines_from_csv(data_path)?;
    info!("成功加载 {} 条K线数据", klines.len());

    if klines.is_empty() {
        return Err(anyhow!("没有有效的K线数据"));
    }

    // 创建策略
    let strategy = match strategy_name {
        "ma-crossover" => MACrossoverStrategy::new(short_period, long_period),
        _ => return Err(anyhow!("不支持的策略: {}", strategy_name)),
    };

    info!("初始化回测引擎，策略: {}, 参数: {}:{}, 初始资金: {:.2}", 
          strategy_name, short_period, long_period, initial_cash);

    // 创建回测引擎并运行
    let mut engine = BacktestEngine::new(strategy, initial_cash);
    engine.run(&klines).await?;

    Ok(())
}

/// 从CSV文件加载K线数据
fn load_klines_from_csv(file_path: &str) -> Result<Vec<Kline>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut klines = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(kline) => klines.push(kline),
            Err(e) => {
                error!("解析CSV行失败: {}", e);
                continue;
            }
        }
    }

    // 按时间戳排序
    klines.sort_by_key(|k: &Kline| k.timestamp);

    Ok(klines)
}

/// 回测引擎
pub struct BacktestEngine {
    strategy: MACrossoverStrategy,
    portfolio: Portfolio,
}

impl BacktestEngine {
    /// 创建新的回测引擎
    pub fn new(strategy: MACrossoverStrategy, initial_cash: f64) -> Self {
        Self {
            strategy,
            portfolio: Portfolio::new(initial_cash),
        }
    }

    /// 运行回测
    pub async fn run(&mut self, klines: &[Kline]) -> Result<()> {
        info!("开始回测，数据时间范围: {} - {}", 
              klines.first().map(|k| k.timestamp).unwrap_or(0),
              klines.last().map(|k| k.timestamp).unwrap_or(0));

        let mut processed_count = 0;
        let total_count = klines.len();

        for kline in klines {
            // 创建市场事件
            let market_event = MarketEvent::Kline(kline.clone());

            // 让策略处理事件
            if let Some(signal_event) = self.strategy.on_market_event(&market_event) {
                debug!("收到交易信号: {:?} at price {:.2}", signal_event.signal, signal_event.price);

                // 执行交易信号
                match signal_event.signal {
                    Signal::Buy => {
                        self.portfolio.execute_buy(signal_event.price, signal_event.timestamp);
                    }
                    Signal::Sell => {
                        self.portfolio.execute_sell(signal_event.price, signal_event.timestamp);
                    }
                    Signal::Hold => {
                        // 不做任何操作
                    }
                }
            }

            // 更新权益曲线
            self.portfolio.update_equity_curve(kline.timestamp, kline.close);

            processed_count += 1;
            
            // 每处理10%的数据输出一次进度
            if processed_count % (total_count / 10).max(1) == 0 {
                let progress = (processed_count as f64 / total_count as f64) * 100.0;
                let current_equity = self.portfolio.get_total_equity(kline.close);
                info!("回测进度: {:.1}%, 当前权益: {:.2}", progress, current_equity);
            }
        }

        info!("回测完成，处理了 {} 条K线数据", processed_count);
        
        // 打印回测报告
        self.portfolio.print_report();

        Ok(())
    }

    /// 获取投资组合的引用
    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_csv() -> Result<String> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_data.csv");
        let mut file = File::create(&file_path)?;

        writeln!(file, "timestamp,open,high,low,close,volume")?;
        writeln!(file, "1640995200000,50000.0,51000.0,49000.0,50500.0,100.0")?;
        writeln!(file, "1640995260000,50500.0,51500.0,50000.0,51000.0,120.0")?;
        writeln!(file, "1640995320000,51000.0,52000.0,50500.0,51500.0,110.0")?;
        writeln!(file, "1640995380000,51500.0,52500.0,51000.0,52000.0,130.0")?;
        writeln!(file, "1640995440000,52000.0,53000.0,51500.0,52500.0,125.0")?;

        Ok(file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_load_klines_from_csv() {
        let csv_path = create_test_csv().unwrap();
        let klines = load_klines_from_csv(&csv_path).unwrap();
        
        assert_eq!(klines.len(), 5);
        assert_eq!(klines[0].timestamp, 1640995200000);
        assert_eq!(klines[0].close, 50500.0);
        assert_eq!(klines[4].close, 52500.0);
        
        // 清理临时文件
        std::fs::remove_file(&csv_path).ok();
    }

    #[tokio::test]
    async fn test_backtest_engine() {
        let csv_path = create_test_csv().unwrap();
        let klines = load_klines_from_csv(&csv_path).unwrap();
        
        let strategy = MACrossoverStrategy::new(2, 3);
        let mut engine = BacktestEngine::new(strategy, 10000.0);
        
        let result = engine.run(&klines).await;
        assert!(result.is_ok());
        
        // 验证投资组合状态
        let portfolio = engine.portfolio();
        assert!(!portfolio.equity_curve().is_empty());
        
        // 清理临时文件
        std::fs::remove_file(&csv_path).ok();
    }

    #[test]
    fn test_nonexistent_file() {
        let result = load_klines_from_csv("nonexistent.csv");
        assert!(result.is_err());
    }
}