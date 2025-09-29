use clap::Parser;
use tracing::{info, error};
use anyhow::Result;

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
                .add_directive("aurora_live=info".parse()?)
        )
        .init();

    let cli = Cli::parse();
    
    info!("启动实时模拟交易: 交易对={}, 策略={}, 参数={}:{}", 
          cli.symbol, cli.strategy_name, cli.short, cli.long);
    
    match engine::run_live_trading(
        &cli.symbol,
        &cli.interval,
        &cli.strategy_name,
        cli.short,
        cli.long,
        cli.initial_cash,
    ).await {
        Ok(_) => info!("实时交易结束"),
        Err(e) => error!("实时交易失败: {}", e),
    }

    Ok(())
}
