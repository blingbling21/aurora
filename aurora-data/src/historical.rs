use aurora_core::Kline;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{info, debug, warn};

/// Binance API返回的K线数据格式
#[derive(Debug, Deserialize)]
struct BinanceKline(
    i64,    // 开盘时间
    String, // 开盘价
    String, // 最高价
    String, // 最低价
    String, // 收盘价
    String, // 成交量
    i64,    // 收盘时间
    String, // 成交额
    i64,    // 成交笔数
    String, // 主动买入成交量
    String, // 主动买入成交额
    String, // 忽略字段
);

impl From<BinanceKline> for Kline {
    fn from(binance_kline: BinanceKline) -> Self {
        Kline {
            timestamp: binance_kline.0,
            open: binance_kline.1.parse().unwrap_or(0.0),
            high: binance_kline.2.parse().unwrap_or(0.0),
            low: binance_kline.3.parse().unwrap_or(0.0),
            close: binance_kline.4.parse().unwrap_or(0.0),
            volume: binance_kline.5.parse().unwrap_or(0.0),
        }
    }
}

/// 从Binance下载历史K线数据
pub async fn download_data(
    symbol: &str,
    interval: &str,
    start_time: Option<&str>,
    end_time: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    let client = Client::new();
    let url = "https://api.binance.com/api/v3/klines";
    
    // 构建请求参数
    let mut params = vec![
        ("symbol", symbol.to_uppercase()),
        ("interval", interval.to_string()),
        ("limit", "1000".to_string()),
    ];
    
    if let Some(start) = start_time {
        // 将日期字符串转换为时间戳（这里简化处理）
        warn!("时间参数解析功能尚未完全实现，使用默认时间范围");
    }
    
    if let Some(end) = end_time {
        warn!("时间参数解析功能尚未完全实现，使用默认时间范围");
    }

    info!("正在请求数据，URL: {}", url);
    debug!("请求参数: {:?}", params);

    // 发送HTTP请求
    let response = client
        .get(url)
        .query(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("API请求失败，状态码: {}", response.status()));
    }

    let binance_klines: Vec<BinanceKline> = response.json().await?;
    info!("成功获取到 {} 条K线数据", binance_klines.len());

    // 转换为我们的Kline格式
    let klines: Vec<Kline> = binance_klines
        .into_iter()
        .map(Kline::from)
        .collect();

    // 保存到CSV文件
    let default_path = format!("{}_{}.csv", symbol.to_lowercase(), interval);
    let output_path = output.unwrap_or(&default_path);
    save_to_csv(&klines, output_path)?;
    
    info!("数据已保存到: {}", output_path);
    Ok(())
}

/// 将K线数据保存为CSV文件
fn save_to_csv(klines: &[Kline], file_path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(file_path)?;
    
    // 写入表头
    writer.write_record(&["timestamp", "open", "high", "low", "close", "volume"])?;
    
    // 写入数据
    for kline in klines {
        writer.write_record(&[
            kline.timestamp.to_string(),
            kline.open.to_string(),
            kline.high.to_string(),
            kline.low.to_string(),
            kline.close.to_string(),
            kline.volume.to_string(),
        ])?;
    }
    
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_binance_kline_conversion() {
        let binance_kline = BinanceKline(
            1640995200000,
            "50000.00".to_string(),
            "51000.00".to_string(),
            "49000.00".to_string(),
            "50500.00".to_string(),
            "100.0".to_string(),
            1640998800000,
            "5050000.0".to_string(),
            1000,
            "50.0".to_string(),
            "2525000.0".to_string(),
            "0".to_string(),
        );
        
        let kline: Kline = binance_kline.into();
        
        assert_eq!(kline.timestamp, 1640995200000);
        assert_eq!(kline.open, 50000.0);
        assert_eq!(kline.high, 51000.0);
        assert_eq!(kline.low, 49000.0);
        assert_eq!(kline.close, 50500.0);
        assert_eq!(kline.volume, 100.0);
    }
    
    #[test]
    fn test_save_to_csv() {
        let klines = vec![
            Kline {
                timestamp: 1640995200000,
                open: 50000.0,
                high: 51000.0,
                low: 49000.0,
                close: 50500.0,
                volume: 100.0,
            }
        ];
        
        let temp_file = "test_klines.csv";
        let result = save_to_csv(&klines, temp_file);
        assert!(result.is_ok());
        
        // 清理测试文件
        if Path::new(temp_file).exists() {
            std::fs::remove_file(temp_file).unwrap();
        }
    }
}