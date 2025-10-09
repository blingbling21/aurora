//! # 数据加载模块
//!
//! 提供从各种格式（CSV、数据库等）加载历史数据的功能。
//! 这个模块专门负责数据的读取和格式转换，实现数据源的抽象。

use crate::{DataError, DataResult};
use aurora_core::Kline;
use std::path::Path;
use tracing::{debug, error, info};

/// CSV 数据加载器
///
/// 专门负责从 CSV 文件加载 K线数据
#[derive(Debug)]
pub struct CsvDataLoader;

impl CsvDataLoader {
    /// 创建新的 CSV 数据加载器
    pub fn new() -> Self {
        Self
    }

    /// 从 CSV 文件加载 K线数据
    ///
    /// # 参数
    ///
    /// * `file_path` - CSV 文件路径
    ///
    /// # 返回值
    ///
    /// 返回按时间戳排序的 K线数据向量
    ///
    /// # 错误
    ///
    /// 如果文件不存在、格式错误或数据无效，返回相应错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use aurora_data::CsvDataLoader;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let loader = CsvDataLoader::new();
    /// let klines = loader.load_from_csv("data.csv")?;
    /// println!("加载了 {} 条K线数据", klines.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_from_csv(&self, file_path: &str) -> DataResult<Vec<Kline>> {
        // 验证文件是否存在
        if !Path::new(file_path).exists() {
            return Err(DataError::FileNotFound(file_path.to_string()));
        }

        info!("开始从CSV文件加载数据: {}", file_path);

        let mut reader =
            csv::Reader::from_path(file_path).map_err(|e| DataError::IoError(e.to_string()))?;

        let mut klines = Vec::new();
        let mut processed_count = 0;
        let mut error_count = 0;

        for result in reader.deserialize() {
            match result {
                Ok(kline) => {
                    // 基础数据验证
                    if self.validate_kline(&kline) {
                        klines.push(kline);
                        processed_count += 1;
                    } else {
                        error_count += 1;
                        debug!("跳过无效K线数据");
                    }
                }
                Err(e) => {
                    error_count += 1;
                    error!("解析CSV行失败: {}", e);
                    continue;
                }
            }
        }

        if klines.is_empty() {
            return Err(DataError::InvalidData("没有有效的K线数据".to_string()));
        }

        // 按时间戳排序
        klines.sort_by_key(|k: &Kline| k.timestamp);

        info!(
            "CSV数据加载完成: 成功处理 {} 条记录，跳过 {} 条无效记录",
            processed_count, error_count
        );

        Ok(klines)
    }

    /// 验证 K线数据的有效性
    ///
    /// # 验证规则
    ///
    /// - 时间戳必须大于0
    /// - 价格必须大于0
    /// - 最高价 >= 最低价
    /// - 最高价 >= 开盘价和收盘价
    /// - 最低价 <= 开盘价和收盘价
    /// - 成交量必须 >= 0
    fn validate_kline(&self, kline: &Kline) -> bool {
        // 基础有效性检查
        if kline.timestamp <= 0 {
            debug!("无效时间戳: {}", kline.timestamp);
            return false;
        }

        if kline.open <= 0.0 || kline.high <= 0.0 || kline.low <= 0.0 || kline.close <= 0.0 {
            debug!("价格不能为负或零");
            return false;
        }

        if kline.volume < 0.0 {
            debug!("成交量不能为负");
            return false;
        }

        // 价格逻辑关系检查
        if kline.high < kline.low {
            debug!("最高价不能小于最低价");
            return false;
        }

        if kline.high < kline.open || kline.high < kline.close {
            debug!("最高价必须大于等于开盘价和收盘价");
            return false;
        }

        if kline.low > kline.open || kline.low > kline.close {
            debug!("最低价必须小于等于开盘价和收盘价");
            return false;
        }

        true
    }
}

impl Default for CsvDataLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_data_loader_creation() {
        let loader = CsvDataLoader::new();
        // 只是确保能创建实例
        assert!(true);
    }

    #[test]
    fn test_validate_kline_valid_data() {
        let loader = CsvDataLoader::new();
        let kline = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };

        assert!(loader.validate_kline(&kline));
    }

    #[test]
    fn test_validate_kline_invalid_timestamp() {
        let loader = CsvDataLoader::new();
        let kline = Kline {
            timestamp: 0, // 无效时间戳
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };

        assert!(!loader.validate_kline(&kline));
    }

    #[test]
    fn test_validate_kline_invalid_prices() {
        let loader = CsvDataLoader::new();
        let kline = Kline {
            timestamp: 1640995200000,
            open: 100.0,
            high: 90.0, // 最高价小于开盘价
            low: 95.0,
            close: 102.0,
            volume: 1000.0,
        };

        assert!(!loader.validate_kline(&kline));
    }

    #[test]
    fn test_load_from_nonexistent_file() {
        let loader = CsvDataLoader::new();
        let result = loader.load_from_csv("nonexistent.csv");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), DataError::FileNotFound(_)));
    }

    #[test]
    fn test_load_from_csv_with_valid_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "timestamp,open,high,low,close,volume").unwrap();
        writeln!(temp_file, "1640995200000,100.0,105.0,95.0,102.0,1000.0").unwrap();
        writeln!(temp_file, "1640995260000,102.0,107.0,100.0,105.0,1500.0").unwrap();

        let loader = CsvDataLoader::new();
        let result = loader.load_from_csv(temp_file.path().to_str().unwrap());

        assert!(result.is_ok());
        let klines = result.unwrap();
        assert_eq!(klines.len(), 2);
        assert_eq!(klines[0].timestamp, 1640995200000);
        assert_eq!(klines[1].timestamp, 1640995260000);
    }
}
