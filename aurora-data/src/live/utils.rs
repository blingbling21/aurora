//! # 实时数据模块工具函数
//!
//! 包含了实时数据处理相关的工具函数。

use super::stream::BinanceLiveStream;
use aurora_core::Kline;
use std::time::Duration;
use tracing::{error, info, warn};

/// 验证K线数据的有效性
///
/// 检查K线数据是否符合基本的有效性要求。
///
/// # 参数
///
/// * `kline` - 待验证的K线数据
///
/// # 返回值
///
/// 如果数据有效返回true，否则返回false
pub fn validate_kline(kline: &Kline) -> bool {
    // 基本数据验证
    if kline.high < kline.low {
        warn!("无效K线: 最高价 {} 小于最低价 {}", kline.high, kline.low);
        return false;
    }

    if kline.open < 0.0 || kline.high < 0.0 || kline.low < 0.0 || kline.close < 0.0 {
        warn!("无效K线: 包含负价格");
        return false;
    }

    if kline.volume < 0.0 {
        warn!("无效K线: 成交量为负数");
        return false;
    }

    if kline.timestamp <= 0 {
        warn!("无效K线: 时间戳无效");
        return false;
    }

    true
}

/// 实时数据流的高级便利函数
/// 
/// 这是一个高级封装函数，为命令行工具和简单用例提供开箱即用的数据流功能。
/// 它内部使用 BinanceLiveStream，但提供了更简单的接口，自动处理连接和数据循环。
/// 
/// 对于需要更精细控制的场景（如自定义错误处理、数据处理逻辑等），
/// 建议直接使用 BinanceLiveStream 结构体。
/// 
/// # 参数
///
/// * `symbol` - 交易对符号
/// * `stream_type` - 流类型（"kline" 或 "trade"）
/// * `interval` - K线时间间隔（仅对kline类型有效）
///
/// # 返回值
///
/// 成功时返回()，失败时返回anyhow::Result错误
///
/// # 示例
///
/// ```rust,no_run
/// use aurora_data::live::stream_data;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 简单的命令行工具用法
/// stream_data("BTCUSDT", "kline", "1m").await?;
/// # Ok(())
/// # }
/// ```
/// 
/// # 与 BinanceLiveStream 的对比
/// 
/// ```rust,no_run
/// use aurora_data::BinanceLiveStream;
/// use aurora_core::DataSource;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 更精细的控制
/// let mut stream = BinanceLiveStream::new();
/// stream.connect(&["BTCUSDT"]).await?;
/// 
/// while let Some(kline) = stream.next_kline().await? {
///     // 自定义处理逻辑
///     println!("收到数据: {}", kline.close);
///     
///     // 可以根据条件退出
///     if kline.close > 100000.0 {
///         break;
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub async fn stream_data(symbol: &str, stream_type: &str, interval: &str) -> anyhow::Result<()> {
    match stream_type {
        "kline" => {
            let mut stream = BinanceLiveStream::new();
            stream.set_interval(interval);
            stream
                .connect(&[symbol])
                .await
                .map_err(|e| anyhow::anyhow!("连接失败: {}", e))?;

            loop {
                match stream.next_kline().await {
                    Ok(Some(_kline)) => {
                        // K线数据已在stream层记录，这里不再重复记录
                    }
                    Ok(None) => {
                        info!("连接已关闭");
                        break;
                    }
                    Err(e) => {
                        error!("获取数据错误: {}", e);
                        break;
                    }
                }
            }
        }
        "trade" => {
            return Err(anyhow::anyhow!("trade流类型暂未实现"));
        }
        _ => {
            return Err(anyhow::anyhow!("不支持的流类型: {}", stream_type));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::Kline;

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

    /// 测试K线数据验证
    #[test]
    fn test_validate_kline() {
        // 有效的K线数据
        let valid_kline = create_test_kline();
        assert!(validate_kline(&valid_kline));

        // 无效的K线数据：最高价小于最低价
        let invalid_kline1 = Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 49000.0, // 最高价小于最低价
            low: 51000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!validate_kline(&invalid_kline1));

        // 无效的K线数据：负价格
        let invalid_kline2 = Kline {
            timestamp: 1640995200000,
            open: -50000.0, // 负价格
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!validate_kline(&invalid_kline2));

        // 无效的K线数据：负成交量
        let invalid_kline3 = Kline {
            timestamp: 1640995200000,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: -100.0, // 负成交量
        };
        assert!(!validate_kline(&invalid_kline3));

        // 无效的K线数据：无效时间戳
        let invalid_kline4 = Kline {
            timestamp: -1, // 无效时间戳
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
        };
        assert!(!validate_kline(&invalid_kline4));
    }

    #[test]
    fn test_process_kline_message() {
        let test_message = r#"{
            "e": "kline",
            "E": 123456789,
            "s": "BTCUSDT",
            "k": {
                "t": 1640995200000,
                "T": 1640995259999,
                "s": "BTCUSDT",
                "i": "1m",
                "f": 100,
                "L": 200,
                "o": "50000.00",
                "c": "50500.00",
                "h": "51000.00",
                "l": "49000.00",
                "v": "100.0",
                "n": 100,
                "x": true,
                "q": "5050000.0",
                "V": "50.0",
                "Q": "2525000.0"
            }
        }"#;

        // 这里只测试JSON解析
        let _: serde_json::Value = serde_json::from_str(test_message).unwrap();
    }

    #[test]
    fn test_process_trade_message() {
        let test_message = r#"{
            "e": "trade",
            "E": 123456789,
            "s": "BTCUSDT",
            "t": 12345,
            "p": "50000.00",
            "q": "0.001",
            "b": 88,
            "a": 50,
            "T": 1640995200000,
            "m": false,
            "M": true
        }"#;

        // 这里只测试JSON解析
        let _: serde_json::Value = serde_json::from_str(test_message).unwrap();
    }
}
