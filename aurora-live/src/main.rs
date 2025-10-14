use anyhow::{Context, Result};
use aurora_config::Config;
use clap::Parser;
use tracing::{error, info};

mod engine;
mod paper_trader;

#[derive(Parser)]
#[command(name = "aurora-live")]
#[command(about = "Aurora项目的实时模拟交易引擎")]
struct Cli {
    /// 配置文件路径(使用配置文件时,其他参数可选)
    #[arg(short, long)]
    config: Option<String>,

    /// 交易对符号 (例如: BTCUSDT)
    #[arg(short, long)]
    symbol: Option<String>,

    /// 策略名称
    #[arg(long)]
    strategy_name: Option<String>,

    /// 短期MA周期
    #[arg(long)]
    short: Option<usize>,

    /// 长期MA周期
    #[arg(long)]
    long: Option<usize>,

    /// 初始模拟资金
    #[arg(long)]
    initial_cash: Option<f64>,

    /// K线时间间隔
    #[arg(short, long)]
    interval: Option<String>,
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

/// 使用配置文件运行实时交易
async fn run_with_config(config_path: &str) -> Result<()> {
    // 加载配置
    let config = Config::from_file(config_path)
        .context(format!("无法加载配置文件: {}", config_path))?;

    // 初始化日志(根据配置)
    init_logging(&config.logging.level);

    info!("使用配置文件: {}", config_path);

    // 验证实时交易配置是否存在
    let live_config = config
        .live
        .as_ref()
        .context("配置文件中缺少[live]部分")?;

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
        "启动实时模拟交易: 交易对={}, 策略={}, 参数={}:{}",
        live_config.symbol, strategy.name, short, long
    );

    // 运行实时交易
    match engine::run_live_trading(
        &live_config.symbol,
        &live_config.interval,
        &strategy.strategy_type,
        short,
        long,
        config.portfolio.initial_cash,
    )
    .await
    {
        Ok(_) => {
            info!("实时交易结束");
            Ok(())
        }
        Err(e) => {
            error!("实时交易失败: {}", e);
            Err(e)
        }
    }
}

/// 使用命令行参数运行实时交易
async fn run_with_cli_args(cli: Cli) -> Result<()> {
    // 初始化日志(使用默认级别)
    init_logging("info");

    // 验证必需参数
    let symbol = cli.symbol.context("缺少必需参数: --symbol")?;

    let strategy_name = cli
        .strategy_name
        .unwrap_or_else(|| "ma-crossover".to_string());
    let short = cli.short.unwrap_or(10);
    let long = cli.long.unwrap_or(30);
    let initial_cash = cli.initial_cash.unwrap_or(10000.0);
    let interval = cli.interval.unwrap_or_else(|| "1m".to_string());

    info!(
        "启动实时模拟交易: 交易对={}, 策略={}, 参数={}:{}",
        symbol, strategy_name, short, long
    );

    // 运行实时交易
    match engine::run_live_trading(&symbol, &interval, &strategy_name, short, long, initial_cash)
        .await
    {
        Ok(_) => {
            info!("实时交易结束");
            Ok(())
        }
        Err(e) => {
            error!("实时交易失败: {}", e);
            Err(e)
        }
    }
}

/// 初始化日志系统
fn init_logging(level: &str) {
    let directive = format!("aurora_live={}", level);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env().add_directive(
                directive
                    .parse()
                    .unwrap_or_else(|_| "aurora_live=info".parse().unwrap()),
            ),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_config_mode() {
        let args = vec!["aurora-live", "--config", "config.toml"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证配置文件模式
        assert_eq!(cli.config, Some("config.toml".to_string()));
    }

    #[test]
    fn test_cli_traditional_mode() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "ETHUSDT",
            "--strategy-name",
            "test-strategy",
            "--short",
            "5",
            "--long",
            "20",
            "--initial-cash",
            "50000.0",
            "--interval",
            "5m",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证传统命令行模式
        assert!(cli.config.is_none());
        assert_eq!(cli.symbol, Some("ETHUSDT".to_string()));
        assert_eq!(cli.strategy_name, Some("test-strategy".to_string()));
        assert_eq!(cli.short, Some(5));
        assert_eq!(cli.long, Some(20));
        assert_eq!(cli.initial_cash, Some(50000.0));
        assert_eq!(cli.interval, Some("5m".to_string()));
    }

    #[test]
    fn test_cli_minimal_args() {
        let args = vec!["aurora-live", "--symbol", "BTCUSDT"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证最小参数组合
        assert_eq!(cli.symbol, Some("BTCUSDT".to_string()));
        assert!(cli.strategy_name.is_none());
        assert!(cli.short.is_none());
        assert!(cli.long.is_none());
        assert!(cli.initial_cash.is_none());
        assert!(cli.interval.is_none());
    }

    #[test]
    fn test_cli_short_args() {
        let args = vec!["aurora-live", "-s", "ADAUSDT", "-i", "15m"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证短参数
        assert_eq!(cli.symbol, Some("ADAUSDT".to_string()));
        assert_eq!(cli.interval, Some("15m".to_string()));
    }

    #[test]
    fn test_cli_config_short_arg() {
        let args = vec!["aurora-live", "-c", "my_config.toml"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证配置文件短参数
        assert_eq!(cli.config, Some("my_config.toml".to_string()));
    }

    #[test]
    fn test_cli_no_args_allowed() {
        let args = vec!["aurora-live"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 现在允许无参数
        assert!(cli.config.is_none());
        assert!(cli.symbol.is_none());
    }

    #[test]
    fn test_cli_invalid_number_args() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "BTCUSDT",
            "--short",
            "not_a_number",
        ];

        let result = Cli::try_parse_from(args);

        // 应该失败，因为short参数不是有效数字
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_various_trading_pairs() {
        let test_pairs = vec![
            "BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT", "XRPUSDT", "DOTUSDT", "LINKUSDT",
            "LTCUSDT",
        ];

        for pair in test_pairs {
            let args = vec!["aurora-live", "--symbol", pair];

            let cli = Cli::try_parse_from(args).unwrap();
            assert_eq!(cli.symbol, Some(pair.to_string()));
        }
    }

    #[test]
    fn test_cli_various_intervals() {
        let test_intervals = vec![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ];

        for interval in test_intervals {
            let args = vec!["aurora-live", "--symbol", "BTCUSDT", "--interval", interval];

            let cli = Cli::try_parse_from(args).unwrap();
            assert_eq!(cli.interval, Some(interval.to_string()));
        }
    }

    #[test]
    fn test_cli_zero_values() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "BTCUSDT",
            "--short",
            "0",
            "--long",
            "0",
            "--initial-cash",
            "0.0",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 解析成功,零值由业务逻辑处理
        assert_eq!(cli.short, Some(0));
        assert_eq!(cli.long, Some(0));
        assert_eq!(cli.initial_cash, Some(0.0));
    }

    #[test]
    fn test_cli_extreme_values() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "BTCUSDT",
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
    fn test_cli_case_sensitivity() {
        let args = vec!["aurora-live", "--symbol", "btcusdt"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证参数值保持原样
        assert_eq!(cli.symbol, Some("btcusdt".to_string()));
    }

    #[test]
    fn test_cli_mixed_mode() {
        // 配置文件和命令行参数可以同时指定
        let args = vec![
            "aurora-live",
            "--config",
            "config.toml",
            "--symbol",
            "OVERRIDE",
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 两个参数都应该被解析
        assert_eq!(cli.config, Some("config.toml".to_string()));
        assert_eq!(cli.symbol, Some("OVERRIDE".to_string()));
    }
}
