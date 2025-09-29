use clap::Parser;
use tracing::{info, error};
use anyhow::Result;

mod engine;
mod portfolio;

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
