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

//! 配置类型单元测试

#[cfg(test)]
mod tests {
    use crate::types::*;
    use std::collections::HashMap;

    #[test]
    fn test_strategy_parameter_integer() {
        let param = StrategyParameter::Integer(42);
        assert_eq!(param.as_i64(), Some(42));
        assert_eq!(param.as_usize(), Some(42));
        assert_eq!(param.as_f64(), Some(42.0));
        assert_eq!(param.as_str(), None);
        assert_eq!(param.as_bool(), None);
    }

    #[test]
    fn test_strategy_parameter_float() {
        let param = StrategyParameter::Float(3.14);
        assert_eq!(param.as_i64(), None);
        assert_eq!(param.as_usize(), None);
        assert_eq!(param.as_f64(), Some(3.14));
        assert_eq!(param.as_str(), None);
        assert_eq!(param.as_bool(), None);
    }

    #[test]
    fn test_strategy_parameter_string() {
        let param = StrategyParameter::String("test".to_string());
        assert_eq!(param.as_i64(), None);
        assert_eq!(param.as_usize(), None);
        assert_eq!(param.as_f64(), None);
        assert_eq!(param.as_str(), Some("test"));
        assert_eq!(param.as_bool(), None);
    }

    #[test]
    fn test_strategy_parameter_bool() {
        let param = StrategyParameter::Bool(true);
        assert_eq!(param.as_i64(), None);
        assert_eq!(param.as_usize(), None);
        assert_eq!(param.as_f64(), None);
        assert_eq!(param.as_str(), None);
        assert_eq!(param.as_bool(), Some(true));
    }

    #[test]
    fn test_strategy_parameter_negative_to_usize() {
        let param = StrategyParameter::Integer(-5);
        assert_eq!(param.as_usize(), None);
    }

    #[test]
    fn test_data_source_config_default() {
        let config = DataSourceConfig::default();
        assert_eq!(config.provider, "binance");
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_retries, 3);
        assert!(config.api_key.is_none());
        assert!(config.api_secret.is_none());
    }

    #[test]
    fn test_portfolio_config_default() {
        let config = PortfolioConfig::default();
        assert_eq!(config.initial_cash, 10000.0);
        assert_eq!(config.commission, 0.001);
        assert_eq!(config.slippage, 0.0);
        assert!(config.max_position_size.is_none());
        assert!(config.max_positions.is_none());
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, "pretty");
        assert!(config.output.is_none());
    }

    #[test]
    fn test_strategy_config_creation() {
        let mut params = HashMap::new();
        params.insert("short".to_string(), StrategyParameter::Integer(10));
        params.insert("long".to_string(), StrategyParameter::Integer(30));

        let strategy = StrategyConfig {
            name: "MA交叉".to_string(),
            strategy_type: "ma-crossover".to_string(),
            parameters: params,
            enabled: true,
        };

        assert_eq!(strategy.name, "MA交叉");
        assert_eq!(strategy.strategy_type, "ma-crossover");
        assert_eq!(strategy.parameters.len(), 2);
        assert!(strategy.enabled);
    }

    #[test]
    fn test_backtest_config_creation() {
        let config = BacktestConfig {
            data_path: "data.csv".to_string(),
            symbol: Some("BTCUSDT".to_string()),
            interval: Some("1h".to_string()),
            start_time: Some("2024-01-01".to_string()),
            end_time: Some("2024-12-31".to_string()),
            pricing_mode: None,
            benchmark: None,
        };

        assert_eq!(config.data_path, "data.csv");
        assert_eq!(config.symbol, Some("BTCUSDT".to_string()));
    }

    #[test]
    fn test_live_config_creation() {
        let config = LiveConfig {
            symbol: "ETHUSDT".to_string(),
            interval: "5m".to_string(),
            paper_trading: true,
        };

        assert_eq!(config.symbol, "ETHUSDT");
        assert_eq!(config.interval, "5m");
        assert!(config.paper_trading);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            data_source: DataSourceConfig::default(),
            strategies: vec![],
            portfolio: PortfolioConfig::default(),
            logging: LogConfig::default(),
            backtest: None,
            live: None,
        };

        // 测试序列化
        let toml_str = toml::to_string(&config).unwrap();
        assert!(!toml_str.is_empty());
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [data_source]
            provider = "binance"
            timeout = 30
            max_retries = 3

            [[strategies]]
            name = "Test Strategy"
            strategy_type = "test"
            enabled = true

            [portfolio]
            initial_cash = 10000.0
            commission = 0.001
            slippage = 0.0

            [logging]
            level = "info"
            format = "pretty"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.data_source.provider, "binance");
        assert_eq!(config.strategies.len(), 1);
        assert_eq!(config.portfolio.initial_cash, 10000.0);
    }

    #[test]
    fn test_strategy_parameters_in_toml() {
        let toml_str = r#"
            name = "MA Strategy"
            strategy_type = "ma-crossover"
            enabled = true

            [parameters]
            short = 10
            long = 30
            threshold = 0.02
            use_ema = true
            pair = "BTCUSDT"
        "#;

        let strategy: StrategyConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(strategy.parameters.len(), 5);

        // 验证各种类型的参数
        assert_eq!(
            strategy.parameters.get("short").unwrap().as_i64(),
            Some(10)
        );
        assert_eq!(
            strategy.parameters.get("long").unwrap().as_i64(),
            Some(30)
        );
        assert_eq!(
            strategy.parameters.get("threshold").unwrap().as_f64(),
            Some(0.02)
        );
        assert_eq!(
            strategy.parameters.get("use_ema").unwrap().as_bool(),
            Some(true)
        );
        assert_eq!(
            strategy.parameters.get("pair").unwrap().as_str(),
            Some("BTCUSDT")
        );
    }

    #[test]
    fn test_optional_fields() {
        let toml_str = r#"
            [data_source]
            provider = "binance"

            [[strategies]]
            name = "Test"
            strategy_type = "test"

            [portfolio]

            [logging]
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        // 验证默认值
        assert_eq!(config.data_source.timeout, 30);
        assert_eq!(config.portfolio.initial_cash, 10000.0);
        assert_eq!(config.logging.level, "info");
        assert!(config.backtest.is_none());
        assert!(config.live.is_none());
    }

    #[test]
    fn test_data_source_with_api_keys() {
        let toml_str = r#"
            provider = "binance"
            api_key = "test_key"
            api_secret = "test_secret"
            base_url = "https://api.example.com"
            ws_url = "wss://stream.example.com"
        "#;

        let config: DataSourceConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_key, Some("test_key".to_string()));
        assert_eq!(config.api_secret, Some("test_secret".to_string()));
        assert_eq!(config.base_url, Some("https://api.example.com".to_string()));
        assert_eq!(config.ws_url, Some("wss://stream.example.com".to_string()));
    }

    #[test]
    fn test_portfolio_with_limits() {
        let toml_str = r#"
            initial_cash = 50000.0
            commission = 0.002
            slippage = 0.0005
            max_position_size = 10000.0
            max_positions = 5
        "#;

        let config: PortfolioConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.initial_cash, 50000.0);
        assert_eq!(config.commission, 0.002);
        assert_eq!(config.slippage, 0.0005);
        assert_eq!(config.max_position_size, Some(10000.0));
        assert_eq!(config.max_positions, Some(5));
    }

    #[test]
    fn test_risk_rules_config() {
        let toml_str = r#"
            max_drawdown_pct = 15.0
            max_daily_loss_pct = 5.0
            max_consecutive_losses = 3
            min_equity = 5000.0
            stop_loss_pct = 2.0
            take_profit_pct = 5.0
        "#;

        let config: RiskRulesConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.max_drawdown_pct, Some(15.0));
        assert_eq!(config.max_daily_loss_pct, Some(5.0));
        assert_eq!(config.max_consecutive_losses, Some(3));
        assert_eq!(config.min_equity, Some(5000.0));
        assert_eq!(config.stop_loss_pct, Some(2.0));
        assert_eq!(config.take_profit_pct, Some(5.0));
    }

    #[test]
    fn test_risk_rules_validation_valid() {
        let config = RiskRulesConfig {
            max_drawdown_pct: Some(15.0),
            max_daily_loss_pct: Some(5.0),
            max_consecutive_losses: Some(3),
            max_single_trade_loss_pct: Some(3.0),
            min_equity: Some(5000.0),
            stop_loss_pct: Some(2.0),
            take_profit_pct: Some(5.0),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_risk_rules_validation_invalid_drawdown() {
        let config = RiskRulesConfig {
            max_drawdown_pct: Some(150.0), // 无效: 超过100
            max_daily_loss_pct: None,
            max_consecutive_losses: None,
            max_single_trade_loss_pct: None,
            min_equity: None,
            stop_loss_pct: None,
            take_profit_pct: None,
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_position_sizing_fixed_percentage() {
        let toml_str = r#"
            strategy_type = "fixed_percentage"
            percentage = 0.2
        "#;

        let config: PositionSizingConfig = toml::from_str(toml_str).unwrap();
        match config {
            PositionSizingConfig::FixedPercentage { percentage } => {
                assert_eq!(percentage, 0.2);
            }
            _ => panic!("Expected FixedPercentage"),
        }
    }

    #[test]
    fn test_position_sizing_kelly() {
        let toml_str = r#"
            strategy_type = "kelly_criterion"
            win_rate = 0.6
            profit_loss_ratio = 2.0
            kelly_fraction = 0.5
        "#;

        let config: PositionSizingConfig = toml::from_str(toml_str).unwrap();
        match config {
            PositionSizingConfig::KellyCriterion {
                win_rate,
                profit_loss_ratio,
                kelly_fraction,
            } => {
                assert_eq!(win_rate, 0.6);
                assert_eq!(profit_loss_ratio, 2.0);
                assert_eq!(kelly_fraction, 0.5);
            }
            _ => panic!("Expected KellyCriterion"),
        }
    }

    #[test]
    fn test_position_sizing_pyramid() {
        let toml_str = r#"
            strategy_type = "pyramid"
            initial_percentage = 0.1
            profit_threshold = 5.0
            max_percentage = 0.5
            increment = 0.1
        "#;

        let config: PositionSizingConfig = toml::from_str(toml_str).unwrap();
        match config {
            PositionSizingConfig::Pyramid {
                initial_percentage,
                profit_threshold,
                max_percentage,
                increment,
            } => {
                assert_eq!(initial_percentage, 0.1);
                assert_eq!(profit_threshold, 5.0);
                assert_eq!(max_percentage, 0.5);
                assert_eq!(increment, 0.1);
            }
            _ => panic!("Expected Pyramid"),
        }
    }

    #[test]
    fn test_position_sizing_validation_valid() {
        let config = PositionSizingConfig::FixedPercentage { percentage: 0.2 };
        assert!(config.validate().is_ok());

        let config2 = PositionSizingConfig::KellyCriterion {
            win_rate: 0.6,
            profit_loss_ratio: 2.0,
            kelly_fraction: 0.5,
        };
        assert!(config2.validate().is_ok());
    }

    #[test]
    fn test_position_sizing_validation_invalid() {
        let config = PositionSizingConfig::FixedPercentage { percentage: 1.5 };
        assert!(config.validate().is_err());

        let config2 = PositionSizingConfig::KellyCriterion {
            win_rate: 1.5, // 无效
            profit_loss_ratio: 2.0,
            kelly_fraction: 0.5,
        };
        assert!(config2.validate().is_err());
    }

    #[test]
    fn test_portfolio_with_risk_and_position() {
        let toml_str = r#"
            initial_cash = 10000.0
            commission = 0.001

            [risk_rules]
            max_drawdown_pct = 15.0
            max_consecutive_losses = 3

            [position_sizing]
            strategy_type = "fixed_percentage"
            percentage = 0.2
        "#;

        let config: PortfolioConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.initial_cash, 10000.0);
        assert!(config.risk_rules.is_some());
        assert!(config.position_sizing.is_some());

        let risk_rules = config.risk_rules.unwrap();
        assert_eq!(risk_rules.max_drawdown_pct, Some(15.0));

        let position_sizing = config.position_sizing.unwrap();
        match position_sizing {
            PositionSizingConfig::FixedPercentage { percentage } => {
                assert_eq!(percentage, 0.2);
            }
            _ => panic!("Expected FixedPercentage"),
        }
    }

    #[test]
    fn test_benchmark_config_disabled() {
        let benchmark = BenchmarkConfig {
            enabled: false,
            data_path: None,
        };
        assert!(benchmark.validate().is_ok());
    }

    #[test]
    fn test_benchmark_config_enabled_with_path() {
        let benchmark = BenchmarkConfig {
            enabled: true,
            data_path: Some("benchmark_btc_1h.csv".to_string()),
        };
        assert!(benchmark.validate().is_ok());
    }

    #[test]
    fn test_benchmark_config_enabled_without_path() {
        let benchmark = BenchmarkConfig {
            enabled: true,
            data_path: None,
        };
        let result = benchmark.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("启用基准时必须指定数据文件路径"));
    }

    #[test]
    fn test_backtest_config_with_benchmark() {
        let toml_str = r#"
            data_path = "btc_1h.csv"
            symbol = "BTCUSDT"
            interval = "1h"

            [benchmark]
            enabled = true
            data_path = "benchmark_btc_1h.csv"
        "#;

        let config: BacktestConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.data_path, "btc_1h.csv");
        assert!(config.benchmark.is_some());

        let benchmark = config.benchmark.unwrap();
        assert_eq!(benchmark.enabled, true);
        assert_eq!(benchmark.data_path, Some("benchmark_btc_1h.csv".to_string()));
        assert!(benchmark.validate().is_ok());
    }

    #[test]
    fn test_backtest_config_without_benchmark() {
        let toml_str = r#"
            data_path = "btc_1h.csv"
            symbol = "BTCUSDT"
            interval = "1h"
        "#;

        let config: BacktestConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.data_path, "btc_1h.csv");
        assert!(config.benchmark.is_none());
    }

    #[test]
    fn test_benchmark_config_serialization() {
        let benchmark = BenchmarkConfig {
            enabled: true,
            data_path: Some("benchmark.csv".to_string()),
        };

        let toml_str = toml::to_string(&benchmark).unwrap();
        let deserialized: BenchmarkConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.enabled, benchmark.enabled);
        assert_eq!(deserialized.data_path, benchmark.data_path);
    }
}
