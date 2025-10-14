pub mod engine;
pub mod portfolio;

pub use engine::*;
pub use portfolio::*;

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_portfolio::Portfolio;
    use aurora_config::PortfolioConfig;

    /// 创建默认的测试用 PortfolioConfig
    fn create_test_portfolio_config(initial_cash: f64) -> PortfolioConfig {
        PortfolioConfig {
            initial_cash,
            commission: 0.001,
            slippage: 0.0005,
            max_position_size: None,
            max_positions: None,
            risk_rules: None,
            position_sizing: None,
        }
    }

    #[test]
    fn test_module_exports() {
        // 验证可以创建BacktestEngine
        let strategy = aurora_strategy::MACrossoverStrategy::new(5, 10);
        let portfolio_config = create_test_portfolio_config(10000.0);
        let _engine = BacktestEngine::new(strategy, &portfolio_config);
    }

    #[test]
    fn test_public_interface() {
        // 验证导出的类型可以被访问
        let strategy = aurora_strategy::MACrossoverStrategy::new(5, 10);
        let portfolio_config = create_test_portfolio_config(10000.0);
        let engine = BacktestEngine::new(strategy, &portfolio_config);

        // 验证基本方法可以调用
        assert!(engine.is_ok());
        let engine = engine.unwrap();
        let portfolio_ref = engine.portfolio();
        assert_eq!(portfolio_ref.get_cash(), 10000.0);
        assert_eq!(portfolio_ref.get_position(), 0.0);
    }

    #[tokio::test]
    async fn test_run_backtest_function_exists() {
        // 验证run_backtest函数存在（但不实际运行，因为需要有效的数据文件）
        // 这里只是验证函数签名正确
        let result = std::panic::catch_unwind(|| {
            // 创建一个异步运行时来测试函数签名
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // 使用不存在的文件来快速失败，验证函数可以被调用
                let portfolio_config = create_test_portfolio_config(10000.0);
                let _ = run_backtest("nonexistent.csv", "ma-crossover", 5, 10, &portfolio_config).await;
            });
        });

        // 不管成功还是失败，只要不是编译错误就说明函数存在且签名正确
        let _ = result;
    }
}
