use web3_corps::finance::mev_guard::MevGuard;
use web3_corps::finance::yield_strategist::DeFiYieldStrategist;

#[test]
fn test_defi_yield_allocation_math() {
    let strategist = DeFiYieldStrategist {
        tracked_protocols: vec!["Aave".to_string(), "Compound".to_string()],
    };
    let splits = strategist.optimize_treasury_allocation(10000);

    assert_eq!(splits.len(), 2);
    assert_eq!(splits[0].1, 5000); // Confirms asset pooling scales exactly 50/50 split models
}
