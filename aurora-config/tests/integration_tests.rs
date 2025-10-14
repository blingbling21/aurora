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

//! Aurora配置管理集成测试

use aurora_config::{Config, StrategyParameter};
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_complete_backtest_config() {
    let config_content = r#"
        [data_source]
        provider = "binance"
        timeout = 60
        max_retries = 5

        [[strategies]]
        name = "MA交叉策略"
        strategy_type = "ma-crossover"
        enabled = true

        [strategies.parameters]
        short = 10
        long = 30

        [portfolio]
        initial_cash = 50000.0
        commission = 0.001
        slippage = 0.0005
        max_position_size = 10000.0

        [logging]
        level = "debug"
        format = "json"
        output = "backtest.log"

        [backtest]
        data_path = "btc_1h.csv"
        symbol = "BTCUSDT"
        interval = "1h"
        start_time = "2024-01-01"
        end_time = "2024-12-31"
    "#;

    let config = Config::from_str(config_content).unwrap();

    // 验证数据源配置
    assert_eq!(config.data_source.provider, "binance");
    assert_eq!(config.data_source.timeout, 60);
    assert_eq!(config.data_source.max_retries, 5);

    // 验证策略配置
    assert_eq!(config.strategies.len(), 1);
    let strategy = &config.strategies[0];
    assert_eq!(strategy.name, "MA交叉策略");
    assert_eq!(strategy.strategy_type, "ma-crossover");
    assert!(strategy.enabled);
    assert_eq!(
        strategy.parameters.get("short").unwrap().as_i64(),
        Some(10)
    );
    assert_eq!(
        strategy.parameters.get("long").unwrap().as_i64(),
        Some(30)
    );

    // 验证投资组合配置
    assert_eq!(config.portfolio.initial_cash, 50000.0);
    assert_eq!(config.portfolio.commission, 0.001);
    assert_eq!(config.portfolio.slippage, 0.0005);
    assert_eq!(config.portfolio.max_position_size, Some(10000.0));

    // 验证日志配置
    assert_eq!(config.logging.level, "debug");
    assert_eq!(config.logging.format, "json");
    assert_eq!(config.logging.output, Some("backtest.log".to_string()));

    // 验证回测配置
    assert!(config.backtest.is_some());
    let backtest = config.backtest.as_ref().unwrap();
    assert_eq!(backtest.data_path, "btc_1h.csv");
    assert_eq!(backtest.symbol, Some("BTCUSDT".to_string()));
    assert_eq!(backtest.interval, Some("1h".to_string()));
    assert_eq!(backtest.start_time, Some("2024-01-01".to_string()));
    assert_eq!(backtest.end_time, Some("2024-12-31".to_string()));
}

#[test]
fn test_complete_live_config() {
    let config_content = r#"
        [data_source]
        provider = "binance"
        api_key = "test_api_key"
        api_secret = "test_api_secret"
        ws_url = "wss://stream.binance.com:9443"

        [[strategies]]
        name = "RSI超卖策略"
        strategy_type = "rsi-oversold"
        enabled = true

        [strategies.parameters]
        period = 14
        oversold = 30.0
        overbought = 70.0

        [portfolio]
        initial_cash = 10000.0
        commission = 0.001
        max_positions = 3

        [logging]
        level = "info"
        format = "pretty"

        [live]
        symbol = "ETHUSDT"
        interval = "5m"
        paper_trading = true
    "#;

    let config = Config::from_str(config_content).unwrap();

    // 验证数据源配置
    assert_eq!(config.data_source.api_key, Some("test_api_key".to_string()));
    assert_eq!(
        config.data_source.api_secret,
        Some("test_api_secret".to_string())
    );
    assert_eq!(
        config.data_source.ws_url,
        Some("wss://stream.binance.com:9443".to_string())
    );

    // 验证策略配置
    let strategy = &config.strategies[0];
    assert_eq!(strategy.name, "RSI超卖策略");
    assert_eq!(
        strategy.parameters.get("period").unwrap().as_i64(),
        Some(14)
    );
    assert_eq!(
        strategy.parameters.get("oversold").unwrap().as_f64(),
        Some(30.0)
    );

    // 验证实时配置
    assert!(config.live.is_some());
    let live = config.live.as_ref().unwrap();
    assert_eq!(live.symbol, "ETHUSDT");
    assert_eq!(live.interval, "5m");
    assert!(live.paper_trading);
}

#[test]
fn test_multi_strategy_config() {
    let config_content = r#"
        [[strategies]]
        name = "MA快速"
        strategy_type = "ma-crossover"
        enabled = true

        [strategies.parameters]
        short = 5
        long = 15

        [[strategies]]
        name = "MA中速"
        strategy_type = "ma-crossover"
        enabled = true

        [strategies.parameters]
        short = 10
        long = 30

        [[strategies]]
        name = "MA慢速"
        strategy_type = "ma-crossover"
        enabled = false

        [strategies.parameters]
        short = 20
        long = 60

        [portfolio]
        initial_cash = 30000.0

        [logging]
        level = "info"
    "#;

    let config = Config::from_str(config_content).unwrap();

    assert_eq!(config.strategies.len(), 3);
    assert!(config.strategies[0].enabled);
    assert!(config.strategies[1].enabled);
    assert!(!config.strategies[2].enabled);
}

#[test]
fn test_config_file_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("roundtrip.toml");

    let original_content = r#"
        [data_source]
        provider = "binance"
        timeout = 30

        [[strategies]]
        name = "Test Strategy"
        strategy_type = "test"

        [strategies.parameters]
        param1 = 100
        param2 = 2.5

        [portfolio]
        initial_cash = 15000.0
        commission = 0.0015

        [logging]
        level = "warn"
        format = "json"
    "#;

    // 写入文件
    let mut file = fs::File::create(&config_path).unwrap();
    file.write_all(original_content.as_bytes()).unwrap();

    // 从文件加载
    let config = Config::from_file(&config_path).unwrap();

    // 验证加载的内容
    assert_eq!(config.data_source.provider, "binance");
    assert_eq!(config.strategies[0].name, "Test Strategy");
    assert_eq!(config.portfolio.initial_cash, 15000.0);
    assert_eq!(config.logging.level, "warn");
}

#[test]
fn test_minimal_valid_config() {
    let config_content = r#"
        [[strategies]]
        name = "Minimal"
        strategy_type = "test"

        [portfolio]

        [logging]
    "#;

    let config = Config::from_str(config_content).unwrap();

    // 验证默认值被正确应用
    assert_eq!(config.data_source.provider, "binance");
    assert_eq!(config.data_source.timeout, 30);
    assert_eq!(config.portfolio.initial_cash, 10000.0);
    assert_eq!(config.portfolio.commission, 0.001);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.format, "pretty");
}

#[test]
fn test_strategy_parameter_types() {
    let config_content = r#"
        [[strategies]]
        name = "Complex Strategy"
        strategy_type = "complex"

        [strategies.parameters]
        int_param = 42
        float_param = 3.14159
        string_param = "BTCUSDT"
        bool_param = true
        negative_int = -10

        [portfolio]

        [logging]
    "#;

    let config = Config::from_str(config_content).unwrap();
    let params = &config.strategies[0].parameters;

    // 验证整数参数
    match params.get("int_param").unwrap() {
        StrategyParameter::Integer(v) => assert_eq!(*v, 42),
        _ => panic!("Expected Integer"),
    }

    // 验证浮点数参数
    match params.get("float_param").unwrap() {
        StrategyParameter::Float(v) => assert!((v - 3.14159).abs() < 1e-5),
        _ => panic!("Expected Float"),
    }

    // 验证字符串参数
    match params.get("string_param").unwrap() {
        StrategyParameter::String(v) => assert_eq!(v, "BTCUSDT"),
        _ => panic!("Expected String"),
    }

    // 验证布尔参数
    match params.get("bool_param").unwrap() {
        StrategyParameter::Bool(v) => assert!(*v),
        _ => panic!("Expected Bool"),
    }

    // 验证负整数
    match params.get("negative_int").unwrap() {
        StrategyParameter::Integer(v) => assert_eq!(*v, -10),
        _ => panic!("Expected Integer"),
    }
}

#[test]
fn test_config_with_all_optional_fields() {
    let config_content = r#"
        [data_source]
        provider = "okx"
        api_key = "key123"
        api_secret = "secret456"
        base_url = "https://api.okx.com"
        ws_url = "wss://ws.okx.com"
        timeout = 45
        max_retries = 10

        [[strategies]]
        name = "Full Config Strategy"
        strategy_type = "advanced"
        enabled = true

        [strategies.parameters]
        param = 1

        [portfolio]
        initial_cash = 100000.0
        commission = 0.0008
        slippage = 0.0002
        max_position_size = 20000.0
        max_positions = 10

        [logging]
        level = "trace"
        format = "json"
        output = "/var/log/aurora.log"

        [backtest]
        data_path = "/data/historical.csv"
        symbol = "BTCUSDT"
        interval = "4h"
        start_time = "2023-01-01"
        end_time = "2023-12-31"

        [live]
        symbol = "ETHUSDT"
        interval = "15m"
        paper_trading = false
    "#;

    let config = Config::from_str(config_content).unwrap();

    // 验证所有字段都被正确解析
    assert_eq!(config.data_source.provider, "okx");
    assert!(config.data_source.api_key.is_some());
    assert!(config.data_source.api_secret.is_some());
    assert!(config.data_source.base_url.is_some());
    assert!(config.data_source.ws_url.is_some());
    assert_eq!(config.data_source.timeout, 45);
    assert_eq!(config.data_source.max_retries, 10);

    assert!(config.portfolio.max_position_size.is_some());
    assert!(config.portfolio.max_positions.is_some());

    assert!(config.logging.output.is_some());

    assert!(config.backtest.is_some());
    assert!(config.live.is_some());
}

#[test]
fn test_error_handling_chain() {
    // 测试文件不存在
    let result1 = Config::from_file("nonexistent.toml");
    assert!(result1.is_err());

    // 测试无效TOML
    let result2 = Config::from_str("[invalid toml");
    assert!(result2.is_err());

    // 测试验证失败
    let result3 = Config::from_str(
        r#"
        [[strategies]]
        name = "Test"
        strategy_type = "test"

        [portfolio]
        initial_cash = -100.0

        [logging]
    "#,
    );
    assert!(result3.is_err());
}
