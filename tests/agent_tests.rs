use web3_corps::agent::manager::AgentManager;
use web3_corps::agent::cfo::CfoAgent;

#[test]
fn test_agent_registration_and_lookup() {
    let mut manager = AgentManager::new();
    let cfo = Box::new(CfoAgent { multi_sig_threshold: 3 });
    
    manager.register_agent("CFO_UNIT_01".to_string(), cfo);
    assert!(manager.active_matrix.contains_key("CFO_UNIT_01"));
}
