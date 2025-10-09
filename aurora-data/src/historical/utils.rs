//! # 历史数据模块工具函数
//!
//! 包含了历史数据处理相关的工具函数。

use crate::{DataError, DataResult};
use aurora_core::Kline;
use tracing::info;

/// 解析时间间隔字符串为毫秒数
///
/// 将Binance的时间间隔格式（如"1m", "5m", "1h", "1d"）转换为毫秒数。
///
/// # 参数
///
/// * `interval` - 时间间隔字符串
///
/// # 返回值
///
/// 成功时返回毫秒数，失败时返回DataError
pub fn parse_interval_to_ms(interval: &str) -> DataResult<i64> {
    match interval {
        "1m" => Ok(60 * 1000),
        "3m" => Ok(3 * 60 * 1000),
        "5m" => Ok(5 * 60 * 1000),
        "15m" => Ok(15 * 60 * 1000),
        "30m" => Ok(30 * 60 * 1000),
        "1h" => Ok(60 * 60 * 1000),
        "2h" => Ok(2 * 60 * 60 * 1000),
        "4h" => Ok(4 * 60 * 60 * 1000),
        "6h" => Ok(6 * 60 * 60 * 1000),
        "8h" => Ok(8 * 60 * 60 * 1000),
        "12h" => Ok(12 * 60 * 60 * 1000),
        "1d" => Ok(24 * 60 * 60 * 1000),
        "3d" => Ok(3 * 24 * 60 * 60 * 1000),
        "1w" => Ok(7 * 24 * 60 * 60 * 1000),
        "1M" => Ok(30 * 24 * 60 * 60 * 1000), // 近似值
        _ => Err(DataError::ConfigError(format!(
            "不支持的时间间隔: {}",
            interval
        ))),
    }
}

/// 将K线数据保存为CSV文件
///
/// 这个方法是异步的，可以处理大量数据而不阻塞。
///
/// # 参数
///
/// * `klines` - K线数据切片
/// * `file_path` - 输出文件路径
///
/// # 返回值
///
/// 成功时返回()，失败时返回DataError
pub async fn save_to_csv(klines: &[Kline], file_path: &str) -> DataResult<()> {
    let mut writer = csv::Writer::from_path(file_path)
        .map_err(|e| DataError::IoError(format!("创建CSV文件失败: {}", e)))?;

    // 写入表头
    writer
        .write_record(&["timestamp", "open", "high", "low", "close", "volume"])
        .map_err(|e| DataError::IoError(format!("写入CSV表头失败: {}", e)))?;

    // 写入数据行
    for kline in klines {
        writer
            .write_record(&[
                kline.timestamp.to_string(),
                kline.open.to_string(),
                kline.high.to_string(),
                kline.low.to_string(),
                kline.close.to_string(),
                kline.volume.to_string(),
            ])
            .map_err(|e| DataError::IoError(format!("写入CSV数据失败: {}", e)))?;
    }

    writer
        .flush()
        .map_err(|e| DataError::IoError(format!("刷新CSV文件失败: {}", e)))?;

    Ok(())
}

/// 从Binance下载历史K线数据的便利函数
///
/// 这是一个向后兼容的函数，提供了简单的接口来下载历史数据。
/// 建议在新代码中使用BinanceHistoricalDownloader结构体。
///
/// # 参数
///
/// * `symbol` - 交易对符号
/// * `interval` - 时间间隔
/// * `start_time` - 开始时间字符串（暂未实现）
/// * `end_time` - 结束时间字符串（暂未实现）
/// * `output` - 输出文件路径
///
/// # 返回值
///
/// 成功时返回()，失败时返回anyhow::Result错误
///
/// # 示例
///
/// ```rust,no_run
/// use aurora_data::historical::download_data;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// download_data("BTCUSDT", "1h", None, None, Some("btc.csv")).await?;
/// # Ok(())
/// # }
/// ```
///
/// # 注意
///
/// 这个函数目前还不支持时间范围参数，将在后续版本中实现。
pub async fn download_data(
    symbol: &str,
    interval: &str,
    start_time: Option<&str>,
    end_time: Option<&str>,
    output: Option<&str>,
) -> anyhow::Result<()> {
    use super::downloader::BinanceHistoricalDownloader;
    use tracing::warn;

    // 创建下载器实例
    let downloader = BinanceHistoricalDownloader::new();

    // 处理时间参数（目前暂不支持）
    if start_time.is_some() || end_time.is_some() {
        warn!("时间参数功能正在开发中，当前版本将获取最新的1000条数据");
    }

    // 获取数据
    let klines = downloader
        .fetch_klines(
            symbol,
            interval,
            None, // 暂时不支持时间范围
            None,
            Some(1000),
        )
        .await
        .map_err(|e| anyhow::anyhow!("获取数据失败: {}", e))?;

    // 保存到文件
    let default_path = format!("{}_{}.csv", symbol.to_lowercase(), interval);
    let output_path = output.unwrap_or(&default_path);

    save_to_csv(&klines, output_path)
        .await
        .map_err(|e| anyhow::anyhow!("保存文件失败: {}", e))?;

    info!("数据已保存到: {}", output_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::Kline;
    use std::fs;
    use std::path::Path;

    /// 创建测试用的Kline数据
    fn create_test_kline() -> Kline {
        Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        }
    }

    /// 测试时间间隔解析
    #[test]
    fn test_parse_interval_to_ms() {
        // 测试各种有效间隔
        assert_eq!(parse_interval_to_ms("1m").unwrap(), 60 * 1000);
        assert_eq!(parse_interval_to_ms("5m").unwrap(), 5 * 60 * 1000);
        assert_eq!(parse_interval_to_ms("1h").unwrap(), 60 * 60 * 1000);
        assert_eq!(parse_interval_to_ms("1d").unwrap(), 24 * 60 * 60 * 1000);
        assert_eq!(parse_interval_to_ms("1w").unwrap(), 7 * 24 * 60 * 60 * 1000);

        // 测试无效间隔
        let result = parse_interval_to_ms("invalid");
        assert!(result.is_err());

        if let Err(DataError::ConfigError(msg)) = result {
            assert!(msg.contains("不支持的时间间隔"));
        } else {
            panic!("期望ConfigError");
        }
    }

    /// 测试CSV保存功能
    #[tokio::test]
    async fn test_save_to_csv() {
        let klines = vec![
            create_test_kline(),
            Kline {
                timestamp: 1640998800000,
                open: 50500.0,
                high: 52000.0,
                low: 50000.0,
                close: 51500.0,
                volume: 200.0,
            },
        ];

        let temp_file = "test_klines_utils.csv";

        // 测试保存功能
        let result = save_to_csv(&klines, temp_file).await;
        assert!(result.is_ok(), "保存CSV失败: {:?}", result);

        // 验证文件是否创建
        assert!(Path::new(temp_file).exists());

        // 读取并验证文件内容
        let content = fs::read_to_string(temp_file).expect("读取文件失败");
        assert!(content.contains("timestamp,open,high,low,close,volume")); // 表头
        assert!(content.contains("1640995200000")); // 第一行数据
        assert!(content.contains("50000")); // 价格数据

        // 清理测试文件
        if Path::new(temp_file).exists() {
            fs::remove_file(temp_file).expect("删除测试文件失败");
        }
    }

    /// 测试空数据保存
    #[tokio::test]
    async fn test_save_empty_csv() {
        let klines: Vec<Kline> = vec![];
        let temp_file = "test_empty_utils.csv";

        let result = save_to_csv(&klines, temp_file).await;
        assert!(result.is_ok());

        // 验证只有表头
        if Path::new(temp_file).exists() {
            let content = fs::read_to_string(temp_file).expect("读取文件失败");
            let lines: Vec<&str> = content.lines().collect();
            assert_eq!(lines.len(), 1); // 只有表头行
            assert_eq!(lines[0], "timestamp,open,high,low,close,volume");

            fs::remove_file(temp_file).expect("删除测试文件失败");
        }
    }
}
