        multi_sig_threshold: 3,
    });

    manager.register_agent("CFO_UNIT_01".to_string(), cfo);
    assert!(manager.active_matrix.contains_key("CFO_UNIT_01"));

#[test]
fn test_legal_clause_verification_parameters() {
    let validator = SmartLegalContractValidator;
    let invalid_agreement = "This contract lacks protective provisions.";
    
    let valid_agreement =
        "Parties agree to standard arbitration rules and will indemnify network operators.";

    assert_eq!(validator.verify_liability_clauses(invalid_agreement), false);
    assert_eq!(validator.verify_liability_clauses(valid_agreement), true);
}
