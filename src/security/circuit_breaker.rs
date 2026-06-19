pub struct ExecutionCircuitBreaker {
    pub total_value_locked: f64,
    pub max_slippage_allowed: f64,
    pub is_tripped: bool,
}

impl ExecutionCircuitBreaker {
    pub fn evaluate_market_anomaly(&mut self, instantaneous_drawdown: f64) -> bool {
        if instantaneous_drawdown > self.max_slippage_allowed {
            self.is_tripped = true; // Triggers immediate failover protocols
            return true;
        }
        self.is_tripped
    }
}
