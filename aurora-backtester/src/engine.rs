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

use anyhow::{Result, anyhow};
use aurora_config::PortfolioConfig;
use aurora_core::{Kline, MarketEvent, Signal, Strategy};
use aurora_portfolio::{BasePortfolio, Portfolio, PortfolioAnalytics};
use aurora_strategy::{BuyAndHoldStrategy, MACrossoverStrategy};
use std::path::Path;
use tracing::{debug, error, info};

// 在库内部使用相对路径
use crate::pricing_mode::PricingMode;
use crate::result::BacktestResult;
use crate::time_utils::{parse_date_to_timestamp, validate_time_range, format_timestamp, TimeRangeValidation};

/// 运行回测
pub async fn run_backtest(
    data_path: &str,
    strategy_name: &str,
    short_period: usize,
    long_period: usize,
    portfolio_config: &PortfolioConfig,
    pricing_mode_config: Option<&aurora_config::PricingModeConfig>,
) -> Result<BacktestResult> {
    run_backtest_with_progress::<fn(u8)>(
        data_path,
        strategy_name,
        short_period,
        long_period,
        portfolio_config,
        pricing_mode_config,
        None,
        None,
        None,
        false, // 默认禁用基准回测
    )
    .await
}

/// 运行回测（支持进度回调和时间范围）
pub async fn run_backtest_with_progress<F>(
    data_path: &str,
    strategy_name: &str,
    short_period: usize,
    long_period: usize,
    portfolio_config: &PortfolioConfig,
    pricing_mode_config: Option<&aurora_config::PricingModeConfig>,
    progress_callback: Option<F>,
    start_time: Option<&str>,
    end_time: Option<&str>,
    enable_benchmark: bool,
) -> Result<BacktestResult>
where
    F: Fn(u8) + Send + Sync,
{
    // 验证数据文件是否存在
    if !Path::new(data_path).exists() {
        return Err(anyhow!("数据文件不存在: {}", data_path));
    }

    info!("加载数据文件: {}", data_path);
    
    // 解析时间范围
    let start_timestamp = if let Some(start_str) = start_time {
        Some(parse_date_to_timestamp(start_str)?)
    } else {
        None
    };
    
    let end_timestamp = if let Some(end_str) = end_time {
        Some(parse_date_to_timestamp(end_str)?)
    } else {
        None
    };
    
    // 加载并过滤数据
    let klines = load_klines_from_csv_with_filter(data_path, start_timestamp, end_timestamp)?;
    
    if let (Some(start), Some(end)) = (start_timestamp, end_timestamp) {
        info!(
            "应用时间过滤: {} 到 {}, 过滤后数据: {} 条",
            format_timestamp(start),
            format_timestamp(end),
            klines.len()
        );
    }
    
    info!("成功加载 {} 条K线数据", klines.len());

    if klines.is_empty() {
        return Err(anyhow!("没有有效的K线数据"));
    }

    // 创建策略
    let strategy = match strategy_name {
        "ma-crossover" => MACrossoverStrategy::new(short_period, long_period),
        _ => return Err(anyhow!("不支持的策略: {}", strategy_name)),
    };

    // 从配置创建定价模式
    let pricing_mode = PricingMode::from_config(pricing_mode_config);

    info!(
        "初始化回测引擎，策略: {}, 参数: {}:{}, 初始资金: {:.2}, 定价模式: {:?}",
        strategy_name, short_period, long_period, portfolio_config.initial_cash, pricing_mode
    );

    // 创建回测引擎并运行
    let mut engine = BacktestEngine::with_pricing_mode(strategy, portfolio_config, pricing_mode)?;
    let result = engine
        .run_with_progress(&klines, Some(data_path.to_string()), progress_callback, enable_benchmark)
        .await?;

    Ok(result)
}

/// 从CSV文件加载K线数据
fn load_klines_from_csv(file_path: &str) -> Result<Vec<Kline>> {
    load_klines_from_csv_with_filter(file_path, None, None)
}

/// 从CSV文件加载K线数据（支持时间范围过滤）
///
/// # 参数
///
/// * `file_path` - CSV文件路径
/// * `start_time` - 开始时间戳（毫秒，可选）
/// * `end_time` - 结束时间戳（毫秒，可选）
///
/// # 返回值
///
/// 返回过滤后的K线数据，并进行时间范围验证
fn load_klines_from_csv_with_filter(
    file_path: &str,
    start_time: Option<i64>,
    end_time: Option<i64>,
) -> Result<Vec<Kline>> {
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

    // 如果没有数据，直接返回
    if klines.is_empty() {
        return Ok(klines);
    }

    // 获取原始数据的时间范围
    let data_start = klines.first().unwrap().timestamp;
    let data_end = klines.last().unwrap().timestamp;

    // 验证时间范围
    if start_time.is_some() || end_time.is_some() {
        let validation = validate_time_range(start_time, end_time, data_start, data_end);
        
        match validation {
            TimeRangeValidation::Valid => {
                // 有效，继续执行
            }
            TimeRangeValidation::NoOverlap { config_start, config_end, data_start, data_end } => {
                return Err(anyhow!(
                    "配置的时间范围与数据完全不重叠！\n\
                     配置范围: {} 到 {}\n\
                     数据范围: {} 到 {}",
                    format_timestamp(config_start),
                    format_timestamp(config_end),
                    format_timestamp(data_start),
                    format_timestamp(data_end)
                ));
            }
            TimeRangeValidation::StartBeforeData { config_start, data_start } => {
                info!(
                    "警告: 配置的开始时间 {} 早于数据开始时间 {}，将使用数据开始时间",
                    format_timestamp(config_start),
                    format_timestamp(data_start)
                );
            }
            TimeRangeValidation::EndAfterData { config_end, data_end } => {
                info!(
                    "警告: 配置的结束时间 {} 晚于数据结束时间 {}，将使用数据结束时间",
                    format_timestamp(config_end),
                    format_timestamp(data_end)
                );
            }
            TimeRangeValidation::InvalidRange { start, end } => {
                return Err(anyhow!(
                    "无效的时间范围: 开始时间 {} 晚于结束时间 {}",
                    format_timestamp(start),
                    format_timestamp(end)
                ));
            }
        }
    }

    // 应用时间过滤
    if start_time.is_some() || end_time.is_some() {
        let filter_start = start_time.unwrap_or(i64::MIN);
        let filter_end = end_time.unwrap_or(i64::MAX);
        
        klines.retain(|k| k.timestamp >= filter_start && k.timestamp <= filter_end);
        
        info!(
            "时间范围过滤: {} 到 {}, 保留 {} 条数据",
            format_timestamp(filter_start.max(data_start)),
            format_timestamp(filter_end.min(data_end)),
            klines.len()
        );
    }

    Ok(klines)
}

/// 回测引擎
pub struct BacktestEngine {
    strategy: MACrossoverStrategy,
    portfolio: BasePortfolio,
    pricing_mode: PricingMode,
    stop_loss_pct: Option<f64>,
    take_profit_pct: Option<f64>,
}

impl BacktestEngine {
    /// 创建新的回测引擎
    ///
    /// # 参数
    ///
    /// * `strategy` - 交易策略
    /// * `portfolio_config` - 投资组合配置（包含风险管理和仓位管理规则）
    pub fn new(strategy: MACrossoverStrategy, portfolio_config: &PortfolioConfig) -> Result<Self> {
        Self::with_pricing_mode(strategy, portfolio_config, PricingMode::default())
    }

    /// 使用指定的定价模式创建回测引擎
    ///
    /// # 参数
    ///
    /// * `strategy` - 交易策略
    /// * `portfolio_config` - 投资组合配置
    /// * `pricing_mode` - 定价模式（控制买卖价格计算方式）
    pub fn with_pricing_mode(
        strategy: MACrossoverStrategy,
        portfolio_config: &PortfolioConfig,
        pricing_mode: PricingMode,
    ) -> Result<Self> {
        let mut portfolio = BasePortfolio::new(portfolio_config.initial_cash);
        
        // 提取止损止盈百分比（如果配置了的话）
        let stop_loss_pct = portfolio_config
            .risk_rules
            .as_ref()
            .and_then(|r| r.stop_loss_pct);
        let take_profit_pct = portfolio_config
            .risk_rules
            .as_ref()
            .and_then(|r| r.take_profit_pct);
        
        // 配置风险管理器（如果提供）
        if let Some(ref risk_rules_config) = portfolio_config.risk_rules {
            let risk_rules = risk_rules_config.to_risk_rules();
            let risk_manager = aurora_portfolio::RiskManager::new(risk_rules, portfolio_config.initial_cash);
            portfolio = portfolio.with_risk_manager(risk_manager);
            info!("已启用风险管理");
            
            if stop_loss_pct.is_some() || take_profit_pct.is_some() {
                info!(
                    "已配置动态止损止盈: 止损={}%, 止盈={}%",
                    stop_loss_pct.map(|v| v.to_string()).unwrap_or("未设置".to_string()),
                    take_profit_pct.map(|v| v.to_string()).unwrap_or("未设置".to_string())
                );
            }
        }
        
        // 配置仓位管理器（如果提供）
        if let Some(ref position_sizing_config) = portfolio_config.position_sizing {
            let position_strategy = position_sizing_config.to_position_sizing_strategy();
            let position_manager = aurora_portfolio::PositionManager::new(position_strategy);
            portfolio = portfolio.with_position_manager(position_manager);
            info!("已启用仓位管理");
        }

        info!("定价模式: {:?}", pricing_mode);
        
        Ok(Self {
            strategy,
            portfolio,
            pricing_mode,
            stop_loss_pct,
            take_profit_pct,
        })
    }

    /// 运行回测
    ///
    /// # 参数
    ///
    /// * `klines` - K线数据
    /// * `data_path` - 数据文件路径（可选）
    /// 运行回测
    ///
    /// # 参数
    /// * `klines` - K线数据
    /// * `data_path` - 数据文件路径（可选）
    /// * `enable_benchmark` - 是否启用基准回测（Buy & Hold策略）
    ///
    /// # 返回
    /// * 回测结果
    pub async fn run(&mut self, klines: &[Kline], data_path: Option<String>, enable_benchmark: bool) -> Result<BacktestResult> {
        self.run_with_progress(klines, data_path, None::<fn(u8)>, enable_benchmark).await
    }

    /// 运行回测（支持进度回调）
    ///
    /// # 参数
    /// 运行回测（支持进度回调）
    ///
    /// # 参数
    ///
    /// * `klines` - K线数据
    /// * `data_path` - 数据文件路径（可选）
    /// * `progress_callback` - 进度回调函数，参数为进度百分比(0-100)
    /// * `enable_benchmark` - 是否启用基准回测（Buy & Hold策略）
    pub async fn run_with_progress<F>(
        &mut self,
        klines: &[Kline],
        data_path: Option<String>,
        progress_callback: Option<F>,
        enable_benchmark: bool,
    ) -> Result<BacktestResult>
    where
        F: Fn(u8) + Send + Sync,
    {
        info!(
            "开始回测，数据时间范围: {} - {}",
            klines.first().map(|k| k.timestamp).unwrap_or(0),
            klines.last().map(|k| k.timestamp).unwrap_or(0)
        );

        let mut processed_count = 0;
        let total_count = klines.len();
        let mut last_reported_progress: u8 = 0;

        for kline in klines {
            // 创建市场事件
            let market_event = MarketEvent::Kline(kline.clone());

            // 让策略处理事件
            if let Some(signal_event) = self.strategy.on_market_event(&market_event) {
                // 执行交易信号，使用定价模式确定实际交易价格
                match signal_event.signal {
                    Signal::Buy => {
                        let buy_price = self.pricing_mode.get_buy_price(kline);
                        debug!(
                            "收到买入信号，信号价格: {:.2}, 实际买入价格: {:.2}",
                            signal_event.price, buy_price
                        );
                        match self
                            .portfolio
                            .execute_buy(buy_price, signal_event.timestamp)
                            .await
                        {
                            Ok(_trade) => {
                                // 买入成功后，如果配置了止损止盈百分比，则设置止损止盈价格
                                if self.stop_loss_pct.is_some() || self.take_profit_pct.is_some() {
                                    if let Some(risk_manager) = self.portfolio.get_risk_manager_mut() {
                                        let stop_loss = self.stop_loss_pct.unwrap_or(0.0);
                                        let take_profit = self.take_profit_pct.unwrap_or(0.0);
                                        
                                        if stop_loss > 0.0 && take_profit > 0.0 {
                                            risk_manager.set_stop_loss_take_profit(buy_price, stop_loss, take_profit);
                                            debug!(
                                                "已设置止损止盈: 入场价={:.2}, 止损={}%, 止盈={}%",
                                                buy_price, stop_loss, take_profit
                                            );
                                        } else if stop_loss > 0.0 {
                                            let stop_price = risk_manager.calculate_stop_loss(buy_price, stop_loss);
                                            risk_manager.update_rules(
                                                risk_manager.get_rules().clone()
                                                    .with_stop_loss_price(stop_price)
                                            );
                                            debug!(
                                                "已设置止损: 入场价={:.2}, 止损价={:.2} ({}%)",
                                                buy_price, stop_price, stop_loss
                                            );
                                        } else if take_profit > 0.0 {
                                            let take_price = risk_manager.calculate_take_profit(buy_price, take_profit);
                                            risk_manager.update_rules(
                                                risk_manager.get_rules().clone()
                                                    .with_take_profit_price(take_price)
                                            );
                                            debug!(
                                                "已设置止盈: 入场价={:.2}, 止盈价={:.2} ({}%)",
                                                buy_price, take_price, take_profit
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                debug!("买入失败: {}", e);
                            }
                        }
                    }
                    Signal::Sell => {
                        let sell_price = self.pricing_mode.get_sell_price(kline);
                        debug!(
                            "收到卖出信号，信号价格: {:.2}, 实际卖出价格: {:.2}",
                            signal_event.price, sell_price
                        );
                        match self
                            .portfolio
                            .execute_sell(sell_price, signal_event.timestamp)
                            .await
                        {
                            Ok(_trade) => {
                                // 卖出成功后，清除止损止盈设置
                                if let Some(risk_manager) = self.portfolio.get_risk_manager_mut() {
                                    risk_manager.clear_stop_loss_take_profit();
                                    debug!("已清除止损止盈设置");
                                }
                            }
                            Err(e) => {
                                debug!("卖出失败: {}", e);
                            }
                        }
                    }
                    Signal::Hold => {
                        // 不做任何操作
                    }
                }
            }

            // 更新权益曲线，使用标记价格（中间价）
            let mark_price = self.pricing_mode.get_mark_price(kline);
            self.portfolio.update_equity(kline.timestamp, mark_price);

            processed_count += 1;

            // 计算当前进度(保持为浮点数以获得更高精度)
            let current_progress_f64 = (processed_count as f64 / total_count as f64) * 100.0;
            let current_progress = (current_progress_f64 as u8).min(100);
            
            // 定期回调进度,而不是只在进度变化时回调
            // 这样可以确保前端定期接收到进度更新,即使进度值没有变化
            // 每处理 1% 的数据或者至少每处理 100 条数据时回调一次
            let progress_interval = (total_count / 100).max(100);
            if processed_count % progress_interval == 0 || current_progress > last_reported_progress {
                last_reported_progress = current_progress;
                if let Some(ref callback) = progress_callback {
                    callback(current_progress);
                }
                
                // 每次回调进度时让出控制权，允许其他异步任务运行
                // 这确保 WebSocket 消息能及时发送，不会被阻塞
                tokio::task::yield_now().await;
            }

            // 每处理10%的数据输出一次进度日志
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

        // 计算回测报告
        let time_period_days = if !klines.is_empty() {
            let start_time = klines.first().unwrap().timestamp;
            let end_time = klines.last().unwrap().timestamp;
            (end_time - start_time) as f64 / (24.0 * 60.0 * 60.0 * 1000.0)
        } else {
            1.0
        };

        // 从权益曲线获取初始权益
        let initial_equity = self.portfolio.get_equity_curve()
            .first()
            .map(|p| p.equity)
            .unwrap_or(self.portfolio.get_cash());
        
        let final_equity = self.portfolio.get_total_equity(
            klines.last().map(|k| k.close).unwrap_or(0.0)
        );
        
        let metrics = self.portfolio.calculate_performance(time_period_days);
        
        // 打印报告（保留原有行为）
        metrics.print_report();

        // 收集交易记录和权益曲线
        let equity_curve = self.portfolio.get_equity_curve().to_vec();
        let trades = self.portfolio.get_trades().to_vec();

        // 根据配置决定是否运行基准策略回测（Buy & Hold）
        let result = if enable_benchmark {
            info!("开始运行基准策略（Buy & Hold）回测...");
            let benchmark_result = self.run_benchmark(klines, initial_equity).await?;
            
            // 计算 Alpha
            let benchmark_return = benchmark_result.metrics.total_return;
            let alpha = PortfolioAnalytics::calculate_alpha(metrics.total_return, benchmark_return);
            let annualized_alpha = PortfolioAnalytics::calculate_annualized_alpha(
                metrics.total_return,
                benchmark_return,
                time_period_days,
            );
            
            info!(
                "基准收益率: {:.2}%, 策略收益率: {:.2}%, Alpha: {:.2}%, 年化Alpha: {:.2}%",
                benchmark_return, metrics.total_return, alpha, annualized_alpha
            );

            // 创建并返回包含基准数据的结果
            BacktestResult::new_with_benchmark(
                metrics,
                equity_curve,
                trades,
                time_period_days,
                initial_equity,
                final_equity,
                data_path,
                benchmark_result.equity_curve,
                benchmark_return,
            )
        } else {
            info!("基准回测已禁用，跳过 Buy & Hold 策略");
            // 创建并返回不含基准数据的结果
            BacktestResult::new(
                metrics,
                equity_curve,
                trades,
                time_period_days,
                initial_equity,
                final_equity,
                data_path,
            )
        };

        Ok(result)
    }

    /// 运行基准策略（Buy & Hold）回测
    ///
    /// # 参数
    ///
    /// * `klines` - K线数据
    /// * `initial_cash` - 初始资金
    ///
    /// # 返回值
    ///
    /// 返回基准策略的回测结果
    async fn run_benchmark(&self, klines: &[Kline], initial_cash: f64) -> Result<BacktestResult> {
        // 创建基准策略
        let mut benchmark_strategy = BuyAndHoldStrategy::new();
        
        // 创建基准投资组合（只使用初始资金，不使用风险管理和仓位管理）
        let mut benchmark_portfolio = BasePortfolio::new(initial_cash);
        
        // 运行基准回测
        for kline in klines {
            let market_event = MarketEvent::Kline(kline.clone());
            
            // 让基准策略处理事件
            if let Some(signal_event) = benchmark_strategy.on_market_event(&market_event) {
                match signal_event.signal {
                    Signal::Buy => {
                        let buy_price = self.pricing_mode.get_buy_price(kline);
                        let _ = benchmark_portfolio.execute_buy(buy_price, signal_event.timestamp).await;
                    }
                    Signal::Sell => {
                        let sell_price = self.pricing_mode.get_sell_price(kline);
                        let _ = benchmark_portfolio.execute_sell(sell_price, signal_event.timestamp).await;
                    }
                    Signal::Hold => {}
                }
            }
            
            // 更新权益曲线
            let mark_price = self.pricing_mode.get_mark_price(kline);
            benchmark_portfolio.update_equity(kline.timestamp, mark_price);
        }
        
        // 计算基准回测报告
        let time_period_days = if !klines.is_empty() {
            let start_time = klines.first().unwrap().timestamp;
            let end_time = klines.last().unwrap().timestamp;
            (end_time - start_time) as f64 / (24.0 * 60.0 * 60.0 * 1000.0)
        } else {
            1.0
        };
        
        let final_equity = benchmark_portfolio.get_total_equity(
            klines.last().map(|k| k.close).unwrap_or(0.0)
        );
        
        let metrics = benchmark_portfolio.calculate_performance(time_period_days);
        let equity_curve = benchmark_portfolio.get_equity_curve().to_vec();
        let trades = benchmark_portfolio.get_trades().to_vec();
        
        Ok(BacktestResult::new(
            metrics,
            equity_curve,
            trades,
            time_period_days,
            initial_cash,
            final_equity,
            None,
        ))
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

        // 测试时禁用基准回测以提高测试速度
        let result = engine.run(&klines, None, false).await;
        assert!(result.is_ok());
        
        let backtest_result = result.unwrap();
        assert_eq!(backtest_result.trades.len(), backtest_result.metrics.total_trades * 2);
        assert!(!backtest_result.equity_curve.is_empty());

        // _temp_dir 在这里自动清理
    }

    #[test]
    fn test_nonexistent_file() {
        let result = load_klines_from_csv("nonexistent.csv");
        assert!(result.is_err());
    }
}
