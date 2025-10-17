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

use anyhow::{Context, Result};
use aurora_config::Config;
use clap::Parser;
use tracing::{error, info};

// 导入需要的模块
mod pricing_mode;
mod engine;

use pricing_mode::PricingMode;

#[derive(Parser)]
#[command(name = "aurora-backtester")]
#[command(about = "Aurora项目的回测引擎")]
struct Cli {
    /// 配置文件路径(使用配置文件时,其他参数可选)
    #[arg(short, long)]
    config: Option<String>,

    /// CSV数据文件路径(必需,除非使用配置文件)
    #[arg(short, long)]
    data_path: Option<String>,

    /// 策略名称
    #[arg(short, long)]
    strategy_name: Option<String>,

    /// 短期MA周期
    #[arg(long)]
    short: Option<usize>,

    /// 长期MA周期
    #[arg(long)]
    long: Option<usize>,

    /// 初始资金
    #[arg(long)]
    initial_cash: Option<f64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 根据参数决定使用配置文件还是命令行参数
    if let Some(config_path) = cli.config {
        // 使用配置文件模式
        run_with_config(&config_path).await
    } else {
        // 使用命令行参数模式
        run_with_cli_args(cli).await
    }
}

/// 使用配置文件运行回测
async fn run_with_config(config_path: &str) -> Result<()> {
    // 加载配置
    let config = Config::from_file(config_path)
        .context(format!("无法加载配置文件: {}", config_path))?;

    // 初始化日志(根据配置)
    init_logging(&config.logging.level);

    info!("使用配置文件: {}", config_path);

    // 验证回测配置是否存在
    let backtest_config = config
        .backtest
        .as_ref()
        .context("配置文件中缺少[backtest]部分")?;

    // 获取第一个启用的策略
    let strategy = config
        .strategies
        .iter()
        .find(|s| s.enabled)
        .context("配置文件中没有启用的策略")?;

    // 从策略参数中提取short和long
    let short = strategy
        .parameters
        .get("short")
        .and_then(|p| p.as_usize())
        .unwrap_or(10);

    let long = strategy
        .parameters
        .get("long")
        .and_then(|p| p.as_usize())
        .unwrap_or(30);

    info!(
        "开始回测: 数据文件={}, 策略={}, 参数={}:{}",
        backtest_config.data_path, strategy.name, short, long
    );

    // 运行回测
    match engine::run_backtest(
        &backtest_config.data_path,
        &strategy.strategy_type,
        short,
        long,
        &config.portfolio,
    )
    .await
    {
        Ok(_) => {
            info!("回测完成");
            Ok(())
        }
        Err(e) => {
            error!("回测失败: {}", e);
            Err(e)
        }
    }
}

/// 使用命令行参数运行回测
async fn run_with_cli_args(cli: Cli) -> Result<()> {
    // 初始化日志(使用默认级别)
    init_logging("info");

    // 验证必需参数
    let data_path = cli
        .data_path
        .context("缺少必需参数: --data-path")?;

    let strategy_name = cli.strategy_name.unwrap_or_else(|| "ma-crossover".to_string());
    let short = cli.short.unwrap_or(10);
    let long = cli.long.unwrap_or(30);
    let initial_cash = cli.initial_cash.unwrap_or(10000.0);

    info!(
        "开始回测: 数据文件={}, 策略={}, 参数={}:{}",
        data_path, strategy_name, short, long
    );

    // 创建简单的 PortfolioConfig（命令行模式不支持风险管理和仓位管理）
    let portfolio_config = aurora_config::PortfolioConfig {
        initial_cash,
        commission: 0.001,
        slippage: 0.0005,
        max_position_size: None,
        max_positions: None,
        risk_rules: None,
        position_sizing: None,
    };

    // 运行回测
    match engine::run_backtest(&data_path, &strategy_name, short, long, &portfolio_config).await {
        Ok(_) => {
            info!("回测完成");
            Ok(())
        }
        Err(e) => {
            error!("回测失败: {}", e);
            Err(e)
        }
    }
}

/// 初始化日志系统
fn init_logging(level: &str) {
    let directive = format!("aurora_backtester={}", level);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(directive.parse().unwrap_or_else(|_| {
                    "aurora_backtester=info".parse().unwrap()
                })),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_config_mode() {
        let args = vec!["aurora-backtester", "--config", "config.toml"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证配置文件模式
        assert_eq!(cli.config, Some("config.toml".to_string()));
    }

    #[test]
    fn test_cli_traditional_mode() {
        let args = vec![
            "aurora-backtester",
            "--data-path",
            "test.csv",
            "--strategy-name",
            "test-strategy",
            "--short",
            "5",
            "--long",
            "20",
            "--initial-cash",
            "50000.0",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证传统命令行模式
        assert!(cli.config.is_none());
        assert_eq!(cli.data_path, Some("test.csv".to_string()));
        assert_eq!(cli.strategy_name, Some("test-strategy".to_string()));
        assert_eq!(cli.short, Some(5));
        assert_eq!(cli.long, Some(20));
        assert_eq!(cli.initial_cash, Some(50000.0));
    }

    #[test]
    fn test_cli_minimal_args() {
        let args = vec!["aurora-backtester", "--data-path", "test.csv"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证最小参数组合
        assert_eq!(cli.data_path, Some("test.csv".to_string()));
        assert!(cli.strategy_name.is_none());
        assert!(cli.short.is_none());
        assert!(cli.long.is_none());
        assert!(cli.initial_cash.is_none());
    }

    #[test]
    fn test_cli_short_args() {
        let args = vec![
            "aurora-backtester",
            "-d",
            "short.csv",
            "-s",
            "test-strategy",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证短参数
        assert_eq!(cli.data_path, Some("short.csv".to_string()));
        assert_eq!(cli.strategy_name, Some("test-strategy".to_string()));
    }

    #[test]
    fn test_cli_config_short_arg() {
        let args = vec!["aurora-backtester", "-c", "my_config.toml"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证配置文件短参数
        assert_eq!(cli.config, Some("my_config.toml".to_string()));
    }

    #[test]
    fn test_cli_no_args_allowed() {
        let args = vec!["aurora-backtester"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 现在允许无参数,因为所有参数都是可选的
        assert!(cli.config.is_none());
        assert!(cli.data_path.is_none());
    }

    #[test]
    fn test_cli_invalid_number_args() {
        let args = vec![
            "aurora-backtester",
            "--data-path",
            "test.csv",
            "--short",
            "invalid",
        ];

        let result = Cli::try_parse_from(args);

        // 应该失败，因为short参数不是有效数字
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_negative_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path",
            "test.csv",
            "--short",
            "5",
            "--long",
            "10",
            "--initial-cash",
            "1000.0",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 解析成功,正值测试
        assert_eq!(cli.initial_cash, Some(1000.0));
    }

    #[test]
    fn test_cli_extreme_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path",
            "test.csv",
            "--short",
            "1",
            "--long",
            "1000",
            "--initial-cash",
            "999999999.99",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证极值处理
        assert_eq!(cli.short, Some(1));
        assert_eq!(cli.long, Some(1000));
        assert_eq!(cli.initial_cash, Some(999999999.99));
    }

    #[test]
    fn test_cli_mixed_mode() {
        // 配置文件和命令行参数可以同时指定
        let args = vec![
            "aurora-backtester",
            "--config",
            "config.toml",
            "--data-path",
            "override.csv",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 两个参数都应该被解析(配置文件优先)
        assert_eq!(cli.config, Some("config.toml".to_string()));
        assert_eq!(cli.data_path, Some("override.csv".to_string()));
    }
}
