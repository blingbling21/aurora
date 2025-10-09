//! # 历史数据类型定义
//!
//! 定义了与历史数据获取相关的数据类型和转换实现。

use aurora_core::Kline;
use serde::{Deserialize, Serialize};

/// Binance API返回的原始K线数据格式
///
/// 这个结构体对应Binance API返回的数组格式数据。
/// 每个字段都是字符串格式，需要转换为相应的数值类型。
///
/// ## 字段说明
///
/// 0. 开盘时间 (timestamp)
/// 1. 开盘价 (string)  
/// 2. 最高价 (string)
/// 3. 最低价 (string)
/// 4. 收盘价 (string)
/// 5. 成交量 (string)
/// 6. 收盘时间 (timestamp)
/// 7. 成交额 (string)
/// 8. 成交笔数 (integer)
/// 9. 主动买入成交量 (string)
/// 10. 主动买入成交额 (string)
/// 11. 忽略字段
///
/// ## 注意事项
///
/// Binance返回的所有价格和数量字段都是字符串格式，
/// 这是为了保持精度，避免浮点数精度问题。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct BinanceKline(
    /// 开盘时间（毫秒时间戳）
    pub i64,
    /// 开盘价（字符串格式，保持精度）
    pub String,
    /// 最高价（字符串格式，保持精度）
    pub String,
    /// 最低价（字符串格式，保持精度）
    pub String,
    /// 收盘价（字符串格式，保持精度）
    pub String,
    /// 成交量（字符串格式，保持精度）
    pub String,
    /// 收盘时间（毫秒时间戳）
    pub i64,
    /// 成交额（字符串格式）
    pub String,
    /// 成交笔数
    pub i64,
    /// 主动买入成交量（字符串格式）
    pub String,
    /// 主动买入成交额（字符串格式）
    pub String,
    /// 忽略字段（通常为"0"）
    pub String,
);

impl From<BinanceKline> for Kline {
    /// 将Binance原始数据转换为标准Kline格式
    ///
    /// 这个转换过程包括：
    /// 1. 字符串价格转换为f64浮点数
    /// 2. 提取必要的字段
    /// 3. 处理解析错误（使用默认值0.0）
    ///
    /// # 错误处理
    ///
    /// 如果字符串解析失败，将使用0.0作为默认值。
    /// 在生产环境中，应该考虑更严格的错误处理。
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binance_kline_creation() {
        let binance_kline = BinanceKline(
            1640995200000,
            "50000.0".to_string(),
            "51000.0".to_string(),
            "49000.0".to_string(),
            "50500.0".to_string(),
            "100.5".to_string(),
            1640995259999,
            "5025000.0".to_string(),
            150,
            "75.25".to_string(),
            "3768750.0".to_string(),
            "0".to_string(),
        );

        // 验证字段访问
        assert_eq!(binance_kline.0, 1640995200000);
        assert_eq!(binance_kline.1, "50000.0");
        assert_eq!(binance_kline.2, "51000.0");
        assert_eq!(binance_kline.3, "49000.0");
        assert_eq!(binance_kline.4, "50500.0");
        assert_eq!(binance_kline.5, "100.5");
    }

    #[test]
    fn test_binance_kline_to_kline_conversion() {
        let binance_kline = BinanceKline(
            1640995200000,
            "50000.0".to_string(),
            "51000.0".to_string(),
            "49000.0".to_string(),
            "50500.0".to_string(),
            "100.5".to_string(),
            1640995259999,
            "5025000.0".to_string(),
            150,
            "75.25".to_string(),
            "3768750.0".to_string(),
            "0".to_string(),
        );

        let kline: Kline = binance_kline.into();

        // 验证转换结果
        assert_eq!(kline.timestamp, 1640995200000);
        assert_eq!(kline.open, 50000.0);
        assert_eq!(kline.high, 51000.0);
        assert_eq!(kline.low, 49000.0);
        assert_eq!(kline.close, 50500.0);
        assert_eq!(kline.volume, 100.5);
    }

    #[test]
    fn test_binance_kline_with_invalid_numbers() {
        let binance_kline = BinanceKline(
            1640995200000,
            "invalid".to_string(),
            "51000.0".to_string(),
            "not_a_number".to_string(),
            "50500.0".to_string(),
            "abc".to_string(),
            1640995259999,
            "5025000.0".to_string(),
            150,
            "75.25".to_string(),
            "3768750.0".to_string(),
            "0".to_string(),
        );

        let kline: Kline = binance_kline.into();

        // 验证无效数字被转换为0.0
        assert_eq!(kline.timestamp, 1640995200000);
        assert_eq!(kline.open, 0.0); // invalid
        assert_eq!(kline.high, 51000.0);
        assert_eq!(kline.low, 0.0); // not_a_number
        assert_eq!(kline.close, 50500.0);
        assert_eq!(kline.volume, 0.0); // abc
    }

    #[test]
    fn test_binance_kline_clone() {
        let binance_kline = BinanceKline(
            1640995200000,
            "50000.0".to_string(),
            "51000.0".to_string(),
            "49000.0".to_string(),
            "50500.0".to_string(),
            "100.5".to_string(),
            1640995259999,
            "5025000.0".to_string(),
            150,
            "75.25".to_string(),
            "3768750.0".to_string(),
            "0".to_string(),
        );

        let cloned = binance_kline.clone();

        // 验证克隆后的数据一致
        assert_eq!(cloned.0, binance_kline.0);
        assert_eq!(cloned.1, binance_kline.1);
        assert_eq!(cloned.4, binance_kline.4);
    }

    #[test]
    fn test_binance_kline_debug_format() {
        let binance_kline = BinanceKline(
            1640995200000,
            "50000.0".to_string(),
            "51000.0".to_string(),
            "49000.0".to_string(),
            "50500.0".to_string(),
            "100.5".to_string(),
            1640995259999,
            "5025000.0".to_string(),
            150,
            "75.25".to_string(),
            "3768750.0".to_string(),
            "0".to_string(),
        );

        // 验证Debug格式化不会panic
        let debug_string = format!("{:?}", binance_kline);
        assert!(debug_string.contains("BinanceKline"));
        assert!(debug_string.contains("50000.0"));
    }

    #[test]
    fn test_binance_kline_with_empty_strings() {
        let binance_kline = BinanceKline(
            1640995200000,
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            1640995259999,
            "".to_string(),
            0,
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );

        let kline: Kline = binance_kline.into();

        // 验证空字符串被转换为0.0
        assert_eq!(kline.timestamp, 1640995200000);
        assert_eq!(kline.open, 0.0);
        assert_eq!(kline.high, 0.0);
        assert_eq!(kline.low, 0.0);
        assert_eq!(kline.close, 0.0);
        assert_eq!(kline.volume, 0.0);
    }

    #[test]
    fn test_binance_kline_with_scientific_notation() {
        let binance_kline = BinanceKline(
            1640995200000,
            "5e4".to_string(),     // 50000.0
            "5.1e4".to_string(),   // 51000.0
            "4.9e4".to_string(),   // 49000.0
            "5.05e4".to_string(),  // 50500.0
            "1.005e2".to_string(), // 100.5
            1640995259999,
            "5.025e6".to_string(),
            150,
            "7.525e1".to_string(),
            "3.76875e6".to_string(),
            "0".to_string(),
        );

        let kline: Kline = binance_kline.into();

        // 验证科学计数法被正确解析
        assert_eq!(kline.timestamp, 1640995200000);
        assert_eq!(kline.open, 50000.0);
        assert_eq!(kline.high, 51000.0);
        assert_eq!(kline.low, 49000.0);
        assert_eq!(kline.close, 50500.0);
        assert_eq!(kline.volume, 100.5);
    }
}
