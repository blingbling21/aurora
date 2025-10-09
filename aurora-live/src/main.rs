use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

mod engine;
mod paper_trader;

#[derive(Parser)]
#[command(name = "aurora-live")]
#[command(about = "Aurora项目的实时模拟交易引擎")]
struct Cli {
    /// 交易对符号 (例如: BTCUSDT)
    #[arg(short, long)]
    symbol: String,

    /// 策略名称
    #[arg(long, default_value = "ma-crossover")]
    strategy_name: String,

    /// 短期MA周期
    #[arg(long, default_value = "10")]
    short: usize,

    /// 长期MA周期
    #[arg(long, default_value = "30")]
    long: usize,

    /// 初始模拟资金
    #[arg(long, default_value = "10000.0")]
    initial_cash: f64,

    /// K线时间间隔
    #[arg(short, long, default_value = "1m")]
    interval: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aurora_live=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    info!(
        "启动实时模拟交易: 交易对={}, 策略={}, 参数={}:{}",
        cli.symbol, cli.strategy_name, cli.short, cli.long
    );

    match engine::run_live_trading(
        &cli.symbol,
        &cli.interval,
        &cli.strategy_name,
        cli.short,
        cli.long,
        cli.initial_cash,
    )
    .await
    {
        Ok(_) => info!("实时交易结束"),
        Err(e) => error!("实时交易失败: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_default_values() {
        let args = vec!["aurora-live", "--symbol", "BTCUSDT"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证默认值
        assert_eq!(cli.symbol, "BTCUSDT");
        assert_eq!(cli.strategy_name, "ma-crossover");
        assert_eq!(cli.short, 10);
        assert_eq!(cli.long, 30);
        assert_eq!(cli.initial_cash, 10000.0);
        assert_eq!(cli.interval, "1m");
    }

    #[test]
    fn test_cli_custom_values() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "ETHUSDT",
            "--strategy-name",
            "custom-strategy",
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

        // 验证自定义值
        assert_eq!(cli.symbol, "ETHUSDT");
        assert_eq!(cli.strategy_name, "custom-strategy");
        assert_eq!(cli.short, 5);
        assert_eq!(cli.long, 20);
        assert_eq!(cli.initial_cash, 50000.0);
        assert_eq!(cli.interval, "5m");
    }

    #[test]
    fn test_cli_short_args() {
        let args = vec!["aurora-live", "-s", "ADAUSDT", "-i", "15m"];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证短参数
        assert_eq!(cli.symbol, "ADAUSDT");
        assert_eq!(cli.interval, "15m");
    }

    #[test]
    fn test_cli_missing_required_args() {
        let args = vec!["aurora-live"];

        let result = Cli::try_parse_from(args);

        // 应该失败，因为缺少必需的symbol参数
        assert!(result.is_err());
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
            "BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT", "XRPUSDT", "DOTUSDT", "LINKUSDT", "LTCUSDT",
        ];

        for pair in test_pairs {
            let args = vec!["aurora-live", "--symbol", pair];

            let cli = Cli::try_parse_from(args).unwrap();
            assert_eq!(cli.symbol, pair);
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
            assert_eq!(cli.interval, interval);
        }
    }

    #[test]
    fn test_cli_zero_and_negative_values() {
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

        let cli = Cli::try_parse_from(args);

        // 解析应该成功，但这些值在业务逻辑中应该被验证
        if let Ok(parsed_cli) = cli {
            assert_eq!(parsed_cli.short, 0);
            assert_eq!(parsed_cli.long, 0);
            assert_eq!(parsed_cli.initial_cash, 0.0);
        }
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
        assert_eq!(cli.short, 1);
        assert_eq!(cli.long, 1000);
        assert_eq!(cli.initial_cash, 999999999.99);
    }

    #[test]
    fn test_cli_case_sensitivity() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "btcusdt", // 小写
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证参数值保持原样（区分大小写）
        assert_eq!(cli.symbol, "btcusdt");
    }

    #[test]
    fn test_cli_special_characters_in_symbol() {
        let args = vec![
            "aurora-live",
            "--symbol",
            "BTC-USDT", // 带连字符
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        // 验证特殊字符被保留
        assert_eq!(cli.symbol, "BTC-USDT");
    }
}
