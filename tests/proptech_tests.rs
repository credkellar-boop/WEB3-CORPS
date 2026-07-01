use web3_corps::proptech::leasing::CommercialLeaseManager;
use web3_corps::proptech::residential::SustainablePropertyProfile;

#[test]
fn test_solar_sustainability_underwriting() {
    let green_home = SustainablePropertyProfile {
        address: "100 Solar Way".to_string(),
        solar_output_kw: 15.5,
        storage_capacity_kwh: 24.0,
    };
    assert!(green_home.audit_energy_independence());
    assert!(green_home.calculate_carbon_offset_tokens() > 0);
}

#[test]
fn test_automated_commercial_rental_split() {
    let manager = CommercialLeaseManager {
        lease_id: "L-99".to_string(),
        monthly_rent_usdc: 5000,
    };
    let result = manager.process_monthly_rent(5000).unwrap();
    assert_eq!(result[0].0, "Treasury_Wallet");
    assert_eq!(result[0].1, 4750);
    // Asserts 95% baseline yield split calculation passes
}