use web3_corps::compliance::regulatory::RegulatorySanctionEngine;
use web3_corps::legal::contract_validator::SmartLegalContractValidator;

#[test]
fn test_sanctioned_wallet_blocks_execution() {
    let engine = RegulatorySanctionEngine;
    let blocklist_triggered = engine.verify_ofac_compliance("0xBadActorAddress1");
    let clean_wallet_passed = engine.verify_ofac_compliance("0xGoodLiquidityProvider");

    assert_eq!(blocklist_triggered, false); // Blocklist target must be flagged false (non-compliant)
    assert_eq!(clean_wallet_passed, true);
}

#[test]
fn test_legal_clause_verification_parameters() {
    let validator = SmartLegalContractValidator;
    let invalid_agreement = "This contract lacks protective provisions.";
    let valid_agreement = "Parties agree to standard arbitration rules and will indemnify network operators.";

    assert_eq!(validator.verify_liability_clauses(invalid_agreement), false);
    assert_eq!(validator.verify_liability_clauses(valid_agreement), true);
}
