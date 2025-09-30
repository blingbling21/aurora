use clap::Parser;
use tracing::{info, error};
use anyhow::Result;

mod engine;

#[derive(Parser)]
#[command(name = "aurora-backtester")]
#[command(about = "Aurora项目的回测引擎")]
struct Cli {
    /// CSV数据文件路径
    #[arg(short, long)]
    data_path: String,
    
    /// 策略名称
    #[arg(short, long, default_value = "ma-crossover")]
    strategy_name: String,
    
    /// 短期MA周期
    #[arg(long, default_value = "10")]
    short: usize,
    
    /// 长期MA周期
    #[arg(long, default_value = "30")]
    long: usize,
    
    /// 初始资金
    #[arg(long, default_value = "10000.0")]
    initial_cash: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aurora_backtester=info".parse()?)
        )
        .init();

    let cli = Cli::parse();
    
    info!("开始回测: 数据文件={}, 策略={}, 参数={}:{}", 
          cli.data_path, cli.strategy_name, cli.short, cli.long);
    
    match engine::run_backtest(
        &cli.data_path,
        &cli.strategy_name,
        cli.short,
        cli.long,
        cli.initial_cash,
    ).await {
        Ok(_) => info!("回测完成"),
        Err(e) => error!("回测失败: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_default_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path", "test.csv"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        // 验证默认值
        assert_eq!(cli.data_path, "test.csv");
        assert_eq!(cli.strategy_name, "ma-crossover");
        assert_eq!(cli.short, 10);
        assert_eq!(cli.long, 30);
        assert_eq!(cli.initial_cash, 10000.0);
    }

    #[test]
    fn test_cli_custom_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path", "custom.csv",
            "--strategy-name", "custom-strategy",
            "--short", "5",
            "--long", "20",
            "--initial-cash", "50000.0"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        // 验证自定义值
        assert_eq!(cli.data_path, "custom.csv");
        assert_eq!(cli.strategy_name, "custom-strategy");
        assert_eq!(cli.short, 5);
        assert_eq!(cli.long, 20);
        assert_eq!(cli.initial_cash, 50000.0);
    }

    #[test]
    fn test_cli_short_args() {
        let args = vec![
            "aurora-backtester",
            "-d", "short.csv",
            "-s", "test-strategy"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        // 验证短参数
        assert_eq!(cli.data_path, "short.csv");
        assert_eq!(cli.strategy_name, "test-strategy");
    }

    #[test]
    fn test_cli_missing_required_args() {
        let args = vec!["aurora-backtester"];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为缺少必需的data-path参数
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_invalid_number_args() {
        let args = vec![
            "aurora-backtester",
            "--data-path", "test.csv",
            "--short", "invalid"
        ];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为short参数不是有效数字
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_negative_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path", "test.csv",
            "--short", "5",
            "--long", "10",
            "--initial-cash", "-1000.0"
        ];
        
        let cli = Cli::try_parse_from(args);
        
        // 虽然解析可能成功，但负值应该在业务逻辑中处理
        if let Ok(parsed_cli) = cli {
            assert_eq!(parsed_cli.initial_cash, -1000.0);
        }
    }

    #[test]
    fn test_cli_help_message() {
        let args = vec!["aurora-backtester", "--help"];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为--help会导致程序退出
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_version_message() {
        let args = vec!["aurora-backtester", "--version"];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为--version会导致程序退出
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_extreme_values() {
        let args = vec![
            "aurora-backtester",
            "--data-path", "test.csv",
            "--short", "1",
            "--long", "1000",
            "--initial-cash", "999999999.99"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        // 验证极值处理
        assert_eq!(cli.short, 1);
        assert_eq!(cli.long, 1000);
        assert_eq!(cli.initial_cash, 999999999.99);
    }
}
