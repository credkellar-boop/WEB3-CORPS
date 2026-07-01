use web3_corps::hr::payroll::ProgrammaticCompensationRegistry;
use web3_corps::oracle::chainlink::RwaPriceOracleClient;

#[test]
fn test_oracle_appraisal_zip_routing() {
    let client = RwaPriceOracleClient {
        feedstock_endpoint: "https://oracle.feed/v1".to_string(),
    };

    let expensive_zone = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.fetch_latest_appraisal_usd("90210"))
        .unwrap();

    assert_eq!(expensive_zone, 12_500_000);
}

#[test]
fn test_payroll_compensation_math_boundaries() {
    let system = ProgrammaticCompensationRegistry {
        stablecoin_gas_denom: "USDC".to_string(),
    };
    let gross = system.compute_node_operator_payout(100, 25);
    let buffer = system.calculate_withholding_buffer(gross);

    assert_eq!(gross, 2500);
    assert_eq!(buffer, 375);
    // Asserts precision matching 15% threshold buffer holds true
}