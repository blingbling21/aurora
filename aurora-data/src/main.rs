use clap::{Parser, Subcommand};
use tracing::{info, error};
use anyhow::Result;

mod historical;
mod live;

#[derive(Parser)]
#[command(name = "aurora-data")]
#[command(about = "Aurora项目的数据采集工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 下载历史数据
    Download {
        /// 交易对符号 (例如: BTCUSDT)
        #[arg(short, long)]
        symbol: String,
        
        /// 时间间隔 (例如: 1m, 5m, 1h, 1d)
        #[arg(short, long, default_value = "1h")]
        interval: String,
        
        /// 开始时间 (例如: 2024-01-01)
        #[arg(long)]
        start_time: Option<String>,
        
        /// 结束时间 (例如: 2024-12-31)
        #[arg(long)]
        end_time: Option<String>,
        
        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// 接收实时数据流
    Stream {
        /// 交易对符号 (例如: BTCUSDT)
        #[arg(short, long)]
        symbol: String,
        
        /// 流类型 (kline, trade)
        #[arg(long, default_value = "kline")]
        stream_type: String,
        
        /// 时间间隔 (仅对kline有效)
        #[arg(short, long, default_value = "1m")]
        interval: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aurora_data=info".parse()?)
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Download {
            symbol,
            interval,
            start_time,
            end_time,
            output,
        } => {
            info!("开始下载历史数据: {} {}", symbol, interval);
            
            match historical::download_data(&symbol, &interval, start_time.as_deref(), end_time.as_deref(), output.as_deref()).await {
                Ok(_) => info!("历史数据下载完成"),
                Err(e) => error!("下载失败: {}", e),
            }
        }
        
        Commands::Stream {
            symbol,
            stream_type,
            interval,
        } => {
            info!("开始实时数据流: {} {} {}", symbol, stream_type, interval);
            
            match live::stream_data(&symbol, &stream_type, &interval).await {
                Ok(_) => info!("实时数据流结束"),
                Err(e) => error!("流接收失败: {}", e),
            }
        }
    }

    Ok(())
}
