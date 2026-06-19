use crate::agent::{AutonomousAgent, Message};
use async_trait::async_trait;

pub struct RiskAnalystAgent;

#[async_trait]
impl AutonomousAgent for RiskAnalystAgent {
    fn name(&self) -> &str { "Risk Analyst Agent" }
    fn role_permissions(&self) -> Vec<String> { vec!["compute_credit_score".to_string(), "evaluate_collateral_ratio".to_string()] }

    async fn process_message(&self, msg: Message) -> anyhow::Result<Option<Message>> {
        let loan_amount = msg.payload.get("requested_loan_usd").and_then(|v| v.as_u64()).unwrap_or(0);
        let collateral_valuation = msg.payload.get("collateral_valuation_usd").and_then(|v| v.as_u64()).unwrap_or(1);

        let loan_to_value = (loan_amount as f64) / (collateral_valuation as f64);
        let risk_approved = loan_to_value <= 0.75; // Enforces a strict 75% LTV limit

        Ok(Some(Message {
            id: format!("{}-risk-appraisal", msg.id),
            sender: self.name().to_string(),
            recipient: msg.sender,
            payload: serde_json::json!({ "ltv_ratio": loan_to_value, "risk_clearance_passed": risk_approved }),
        }))
    }
}
