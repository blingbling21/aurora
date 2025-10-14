// Copyright 2025 blingbling21
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 配置加载器单元测试

#[cfg(test)]
mod tests {
    use crate::Config;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_load_valid_config() {
        let config_content = r#"
            [data_source]
            provider = "binance"

            [[strategies]]
            name = "Test Strategy"
            strategy_type = "ma-crossover"
            enabled = true

            [strategies.parameters]
            short = 10
            long = 30

            [portfolio]
            initial_cash = 10000.0
            commission = 0.001

            [logging]
            level = "info"
            format = "pretty"
        "#;

        let config = Config::from_str(config_content).unwrap();
        assert_eq!(config.strategies.len(), 1);
        assert_eq!(config.strategies[0].name, "Test Strategy");
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let config_content = r#"
            [[strategies]]
            name = "File Strategy"
            strategy_type = "test"

            [portfolio]
            initial_cash = 20000.0

            [logging]
            level = "debug"
        "#;

        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        let config = Config::from_file(&config_path).unwrap();
        assert_eq!(config.portfolio.initial_cash, 20000.0);
        assert_eq!(config.logging.level, "debug");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = Config::from_file("nonexistent_file.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_toml_syntax() {
        let config_content = r#"
            [data_source
            provider = "binance"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_negative_initial_cash() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = -1000.0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_commission() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0
            commission = 1.5

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_slippage() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0
            slippage = -0.01

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_no_strategies() {
        let config_content = r#"
            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_empty_strategy_name() {
        let config_content = r#"
            [[strategies]]
            name = ""
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_empty_strategy_type() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = ""

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_log_level() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "invalid"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_log_format() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"
            format = "xml"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_backtest_empty_data_path() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"

            [backtest]
            data_path = ""
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_live_empty_symbol() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"

            [live]
            symbol = ""
            interval = "1m"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_valid_backtest_config() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"

            [backtest]
            data_path = "data.csv"
            symbol = "BTCUSDT"
            interval = "1h"
        "#;

        let config = Config::from_str(config_content).unwrap();
        assert!(config.backtest.is_some());
        assert_eq!(
            config.backtest.as_ref().unwrap().data_path,
            "data.csv"
        );
    }

    #[test]
    fn test_validation_valid_live_config() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"

            [live]
            symbol = "ETHUSDT"
            interval = "5m"
            paper_trading = true
        "#;

        let config = Config::from_str(config_content).unwrap();
        assert!(config.live.is_some());
        assert_eq!(config.live.as_ref().unwrap().symbol, "ETHUSDT");
    }

    #[test]
    fn test_validation_zero_max_position_size() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0
            max_position_size = 0.0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_zero_max_positions() {
        let config_content = r#"
            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]
            initial_cash = 10000.0
            max_positions = 0

            [logging]
            level = "info"
        "#;

        let result = Config::from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_strategies() {
        let config_content = r#"
            [[strategies]]
            name = "Strategy 1"
            strategy_type = "ma-crossover"
            enabled = true

            [strategies.parameters]
            short = 10
            long = 30

            [[strategies]]
            name = "Strategy 2"
            strategy_type = "rsi-oversold"
            enabled = false

            [strategies.parameters]
            period = 14
            oversold = 30

            [portfolio]
            initial_cash = 10000.0

            [logging]
            level = "info"
        "#;

        let config = Config::from_str(config_content).unwrap();
        assert_eq!(config.strategies.len(), 2);
        assert_eq!(config.strategies[0].name, "Strategy 1");
        assert!(config.strategies[0].enabled);
        assert_eq!(config.strategies[1].name, "Strategy 2");
        assert!(!config.strategies[1].enabled);
    }

    #[test]
    fn test_all_log_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error"];

        for level in levels {
            let config_content = format!(
                r#"
                [[strategies]]
                name = "Test"
                strategy_type = "test"

                [portfolio]
                initial_cash = 10000.0

                [logging]
                level = "{}"
            "#,
                level
            );

            let config = Config::from_str(&config_content).unwrap();
            assert_eq!(config.logging.level, level);
        }
    }

    #[test]
    fn test_all_log_formats() {
        let formats = vec!["json", "pretty"];

        for format in formats {
            let config_content = format!(
                r#"
                [[strategies]]
                name = "Test"
                strategy_type = "test"

                [portfolio]
                initial_cash = 10000.0

                [logging]
                level = "info"
                format = "{}"
            "#,
                format
            );

            let config = Config::from_str(&config_content).unwrap();
            assert_eq!(config.logging.format, format);
        }
    }
}
