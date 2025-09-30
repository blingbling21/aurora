use clap::{Parser, Subcommand};
use tracing::{info, error};
use anyhow::Result;
use aurora_data::{historical, live};

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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_download_command_minimal() {
        let args = vec![
            "aurora-data",
            "download",
            "--symbol", "BTCUSDT"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Download { symbol, interval, start_time, end_time, output } => {
                assert_eq!(symbol, "BTCUSDT");
                assert_eq!(interval, "1h"); // 默认值
                assert!(start_time.is_none());
                assert!(end_time.is_none());
                assert!(output.is_none());
            }
            _ => panic!("预期Download命令"),
        }
    }

    #[test]
    fn test_download_command_full() {
        let args = vec![
            "aurora-data",
            "download",
            "--symbol", "ETHUSDT",
            "--interval", "1d",
            "--start-time", "2024-01-01",
            "--end-time", "2024-12-31",
            "--output", "eth_data.csv"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Download { symbol, interval, start_time, end_time, output } => {
                assert_eq!(symbol, "ETHUSDT");
                assert_eq!(interval, "1d");
                assert_eq!(start_time, Some("2024-01-01".to_string()));
                assert_eq!(end_time, Some("2024-12-31".to_string()));
                assert_eq!(output, Some("eth_data.csv".to_string()));
            }
            _ => panic!("预期Download命令"),
        }
    }

    #[test]
    fn test_stream_command_minimal() {
        let args = vec![
            "aurora-data",
            "stream",
            "--symbol", "BTCUSDT"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Stream { symbol, stream_type, interval } => {
                assert_eq!(symbol, "BTCUSDT");
                assert_eq!(stream_type, "kline"); // 默认值
                assert_eq!(interval, "1m"); // 默认值
            }
            _ => panic!("预期Stream命令"),
        }
    }

    #[test]
    fn test_stream_command_full() {
        let args = vec![
            "aurora-data",
            "stream",
            "--symbol", "ADAUSDT",
            "--stream-type", "trade",
            "--interval", "5m"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Stream { symbol, stream_type, interval } => {
                assert_eq!(symbol, "ADAUSDT");
                assert_eq!(stream_type, "trade");
                assert_eq!(interval, "5m");
            }
            _ => panic!("预期Stream命令"),
        }
    }

    #[test]
    fn test_download_short_args() {
        let args = vec![
            "aurora-data",
            "download",
            "-s", "BNBUSDT",
            "-i", "30m",
            "-o", "output.csv"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Download { symbol, interval, output, .. } => {
                assert_eq!(symbol, "BNBUSDT");
                assert_eq!(interval, "30m");
                assert_eq!(output, Some("output.csv".to_string()));
            }
            _ => panic!("预期Download命令"),
        }
    }

    #[test]
    fn test_stream_short_args() {
        let args = vec![
            "aurora-data",
            "stream",
            "-s", "XRPUSDT",
            "-i", "15m"
        ];
        
        let cli = Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            Commands::Stream { symbol, interval, .. } => {
                assert_eq!(symbol, "XRPUSDT");
                assert_eq!(interval, "15m");
            }
            _ => panic!("预期Stream命令"),
        }
    }

    #[test]
    fn test_missing_subcommand() {
        let args = vec!["aurora-data"];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为缺少子命令
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_symbol_arg() {
        let args = vec![
            "aurora-data",
            "download"
        ];
        
        let result = Cli::try_parse_from(args);
        
        // 应该失败，因为缺少必需的symbol参数
        assert!(result.is_err());
    }

    #[test]
    fn test_various_intervals() {
        let test_intervals = vec![
            "1m", "3m", "5m", "15m", "30m",
            "1h", "2h", "4h", "6h", "8h", "12h",
            "1d", "3d", "1w", "1M"
        ];
        
        for interval in test_intervals {
            let args = vec![
                "aurora-data",
                "download",
                "--symbol", "BTCUSDT",
                "--interval", interval
            ];
            
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Download { interval: parsed_interval, .. } => {
                    assert_eq!(parsed_interval, interval);
                }
                _ => panic!("预期Download命令"),
            }
        }
    }

    #[test]
    fn test_various_symbols() {
        let test_symbols = vec![
            "BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT",
            "XRPUSDT", "DOTUSDT", "LINKUSDT", "LTCUSDT"
        ];
        
        for symbol in test_symbols {
            let args = vec![
                "aurora-data",
                "stream",
                "--symbol", symbol
            ];
            
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Stream { symbol: parsed_symbol, .. } => {
                    assert_eq!(parsed_symbol, symbol);
                }
                _ => panic!("预期Stream命令"),
            }
        }
    }

    #[test]
    fn test_stream_types() {
        let stream_types = vec!["kline", "trade", "ticker", "depth"];
        
        for stream_type in stream_types {
            let args = vec![
                "aurora-data",
                "stream",
                "--symbol", "BTCUSDT",
                "--stream-type", stream_type
            ];
            
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Stream { stream_type: parsed_type, .. } => {
                    assert_eq!(parsed_type, stream_type);
                }
                _ => panic!("预期Stream命令"),
            }
        }
    }

    #[test]
    fn test_time_formats() {
        let time_formats = vec![
            "2024-01-01",
            "2024-12-31",
            "2023-06-15",
            "2025-03-20"
        ];
        
        for time in time_formats {
            let args = vec![
                "aurora-data",
                "download",
                "--symbol", "BTCUSDT",
                "--start-time", time,
                "--end-time", time
            ];
            
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Download { start_time, end_time, .. } => {
                    assert_eq!(start_time, Some(time.to_string()));
                    assert_eq!(end_time, Some(time.to_string()));
                }
                _ => panic!("预期Download命令"),
            }
        }
    }

    #[test]
    fn test_output_file_paths() {
        let output_paths = vec![
            "data.csv",
            "./output/data.csv",
            "../data/btc.csv",
            "/tmp/crypto_data.csv",
            "C:\\data\\crypto.csv"
        ];
        
        for path in output_paths {
            let args = vec![
                "aurora-data",
                "download",
                "--symbol", "BTCUSDT",
                "--output", path
            ];
            
            let cli = Cli::try_parse_from(args).unwrap();
            
            match cli.command {
                Commands::Download { output, .. } => {
                    assert_eq!(output, Some(path.to_string()));
                }
                _ => panic!("预期Download命令"),
            }
        }
    }
}
