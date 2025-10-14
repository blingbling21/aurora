use anyhow::{Result, anyhow};
use aurora_config::PortfolioConfig;
use aurora_core::{Kline, MarketEvent, Signal, Strategy};
use aurora_portfolio::{BasePortfolio, Portfolio};
use aurora_strategy::MACrossoverStrategy;
use std::path::Path;
use tracing::{debug, error, info};

/// 运行回测
pub async fn run_backtest(
    data_path: &str,
    strategy_name: &str,
    short_period: usize,
    long_period: usize,
    portfolio_config: &PortfolioConfig,
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

    info!(
        "初始化回测引擎，策略: {}, 参数: {}:{}, 初始资金: {:.2}",
        strategy_name, short_period, long_period, portfolio_config.initial_cash
    );

    // 创建回测引擎并运行
    let mut engine = BacktestEngine::new(strategy, portfolio_config)?;
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
    portfolio: BasePortfolio,
}

impl BacktestEngine {
    /// 创建新的回测引擎
    ///
    /// # 参数
    ///
    /// * `strategy` - 交易策略
    /// * `portfolio_config` - 投资组合配置（包含风险管理和仓位管理规则）
    pub fn new(strategy: MACrossoverStrategy, portfolio_config: &PortfolioConfig) -> Result<Self> {
        let mut portfolio = BasePortfolio::new(portfolio_config.initial_cash);
        
        // 配置风险管理器（如果提供）
        if let Some(ref risk_rules_config) = portfolio_config.risk_rules {
            let risk_rules = risk_rules_config.to_risk_rules();
            let risk_manager = aurora_portfolio::RiskManager::new(risk_rules, portfolio_config.initial_cash);
            portfolio = portfolio.with_risk_manager(risk_manager);
            info!("已启用风险管理");
        }
        
        // 配置仓位管理器（如果提供）
        if let Some(ref position_sizing_config) = portfolio_config.position_sizing {
            let position_strategy = position_sizing_config.to_position_sizing_strategy();
            let position_manager = aurora_portfolio::PositionManager::new(position_strategy);
            portfolio = portfolio.with_position_manager(position_manager);
            info!("已启用仓位管理");
        }
        
        Ok(Self {
            strategy,
            portfolio,
        })
    }

    /// 运行回测
    pub async fn run(&mut self, klines: &[Kline]) -> Result<()> {
        info!(
            "开始回测，数据时间范围: {} - {}",
            klines.first().map(|k| k.timestamp).unwrap_or(0),
            klines.last().map(|k| k.timestamp).unwrap_or(0)
        );

        let mut processed_count = 0;
        let total_count = klines.len();

        for kline in klines {
            // 创建市场事件
            let market_event = MarketEvent::Kline(kline.clone());

            // 让策略处理事件
            if let Some(signal_event) = self.strategy.on_market_event(&market_event) {
                debug!(
                    "收到交易信号: {:?} at price {:.2}",
                    signal_event.signal, signal_event.price
                );

                // 执行交易信号
                match signal_event.signal {
                    Signal::Buy => {
                        if let Err(e) = self
                            .portfolio
                            .execute_buy(signal_event.price, signal_event.timestamp)
                            .await
                        {
                            debug!("买入失败: {}", e);
                        }
                    }
                    Signal::Sell => {
                        if let Err(e) = self
                            .portfolio
                            .execute_sell(signal_event.price, signal_event.timestamp)
                            .await
                        {
                            debug!("卖出失败: {}", e);
                        }
                    }
                    Signal::Hold => {
                        // 不做任何操作
                    }
                }
            }

            // 更新权益曲线
            self.portfolio.update_equity(kline.timestamp, kline.close);

            processed_count += 1;

            // 每处理10%的数据输出一次进度
            if processed_count % (total_count / 10).max(1) == 0 {
                let progress = (processed_count as f64 / total_count as f64) * 100.0;
                let current_equity = self.portfolio.get_total_equity(kline.close);
                info!(
                    "回测进度: {:.1}%, 当前权益: {:.2}",
                    progress, current_equity
                );
            }
        }

        info!("回测完成，处理了 {} 条K线数据", processed_count);

        // 计算并打印回测报告
        let time_period_days = if !klines.is_empty() {
            let start_time = klines.first().unwrap().timestamp;
            let end_time = klines.last().unwrap().timestamp;
            (end_time - start_time) as f64 / (24.0 * 60.0 * 60.0 * 1000.0)
        } else {
            1.0
        };

        let metrics = self.portfolio.calculate_performance(time_period_days);
        metrics.print_report();

        Ok(())
    }

    /// 获取投资组合的引用
    pub fn portfolio(&self) -> &BasePortfolio {
        &self.portfolio
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{TempDir, tempdir};

    fn create_test_portfolio_config() -> PortfolioConfig {
        PortfolioConfig {
            initial_cash: 10000.0,
            commission: 0.001,
            slippage: 0.0005,
            max_position_size: None,
            max_positions: None,
            risk_rules: None,
            position_sizing: None,
        }
    }

    fn create_test_csv() -> Result<(String, TempDir)> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_data.csv");
        let mut file = File::create(&file_path)?;

        writeln!(file, "timestamp,open,high,low,close,volume")?;
        writeln!(file, "1640995200000,50000.0,51000.0,49000.0,50500.0,100.0")?;
        writeln!(file, "1640995260000,50500.0,51500.0,50000.0,51000.0,120.0")?;
        writeln!(file, "1640995320000,51000.0,52000.0,50500.0,51500.0,110.0")?;
        writeln!(file, "1640995380000,51500.0,52500.0,51000.0,52000.0,130.0")?;
        writeln!(file, "1640995440000,52000.0,53000.0,51500.0,52500.0,125.0")?;

        Ok((file_path.to_string_lossy().to_string(), dir))
    }

    #[test]
    fn test_load_klines_from_csv() {
        let (csv_path, _temp_dir) = create_test_csv().unwrap();
        let klines = load_klines_from_csv(&csv_path).unwrap();

        assert_eq!(klines.len(), 5);
        assert_eq!(klines[0].timestamp, 1640995200000);
        assert_eq!(klines[0].close, 50500.0);
        assert_eq!(klines[4].close, 52500.0);

        // _temp_dir 在这里自动清理
    }

    #[tokio::test]
    async fn test_backtest_engine() {
        let (csv_path, _temp_dir) = create_test_csv().unwrap();
        let klines = load_klines_from_csv(&csv_path).unwrap();

        let strategy = MACrossoverStrategy::new(2, 3);
        let portfolio_config = create_test_portfolio_config();
        let mut engine = BacktestEngine::new(strategy, &portfolio_config).unwrap();

        let result = engine.run(&klines).await;
        assert!(result.is_ok());

        // 验证投资组合状态
        let portfolio = engine.portfolio();
        assert!(!portfolio.get_equity_curve().is_empty());

        // _temp_dir 在这里自动清理
    }

    #[test]
    fn test_nonexistent_file() {
        let result = load_klines_from_csv("nonexistent.csv");
        assert!(result.is_err());
    }
}
