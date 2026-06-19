use web3_corps::identity::did::DecentralizedIdentifierResolver;
use web3_corps::insurance::pool::CryptographicRiskPoolEngine;

#[test]
fn test_sovereign_did_signature_matching() {
    let resolver = DecentralizedIdentifierResolver;
    let valid_did = "did:web3corps:node_operator_01";
    
    let is_valid = resolver.verify_sovereign_signature(valid_did, "auth_challenge_payload", &[2, 4, 6, 8]);
    assert_eq!(is_valid, true);
}

#[test]
fn test_insurance_premium_risk_scaling() {
    let mut pool = CryptographicRiskPoolEngine { total_reserve_liquidity_usdc: 1_000_000, active_underwritten_value: 500_000 };
    let premium = pool.calculate_property_premium(100_000, 0.5); // 100k valuation, 0.5 hazard multiplier
    
    assert_eq!(premium, 750); // Asserts mathematical risk indexing matches expected baseline profiles
    assert!(pool.execute_automated_claim_disbursement(50_000).is_ok());
    assert_eq!(pool.total_reserve_liquidity_usdc, 950_000);
}
