pub struct L3Paymaster {
    pub gas_token_address: String,
    pub native_token_burn_percentage: f64,
}

impl L3Paymaster {
    pub async fn validate_and_sponsor_tx(&self, user_balance: u64, estimated_gas: u64) -> anyhow::Result<bool> {
        // Evaluates if the equivalent user asset holdings can successfully absorb the L3 gas weight
        let adjusted_cost = (estimated_gas as f64 * (1.0 - self.native_token_burn_percentage)) as u64;
        Ok(user_balance >= adjusted_cost)
    }
}
