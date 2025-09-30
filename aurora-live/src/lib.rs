pub mod engine;
pub mod paper_trader;

pub use engine::*;
pub use paper_trader::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // 验证可以创建LiveEngine
        let strategy = aurora_strategy::MACrossoverStrategy::new(5, 10);
        let _engine = LiveEngine::new(strategy, 10000.0);

        // 验证可以创建PaperTrader
        let _trader = PaperTrader::new(10000.0);
    }

    #[test]
    fn test_public_interface() {
        // 验证导出的类型可以被访问
        let strategy = aurora_strategy::MACrossoverStrategy::new(5, 10);
        let engine = LiveEngine::new(strategy, 10000.0);
        let trader = PaperTrader::new(10000.0);

        // 验证基本方法可以调用
        assert!(trader.get_cash() > 0.0);
        assert_eq!(trader.get_position(), 0.0);
        
        // 验证引擎方法存在（不实际调用run，因为需要网络连接）
        let _paper_trader_ref = engine.paper_trader();
    }
}