pub struct ProgrammaticCompensationRegistry {
    pub stablecoin_gas_denom: String,
}

impl ProgrammaticCompensationRegistry {
    pub fn compute_node_operator_payout(&self, active_hours: u32, base_hourly_rate: u64) -> u64 {
        // Calculates standard billing calculations for on-demand distributed cloud systems
        (active_hours as u64) * base_hourly_rate
    }

    pub fn calculate_withholding_buffer(&self, gross_amount: u64) -> u64 {
        ((gross_amount as f64) * 0.15) as u64
    }
}
