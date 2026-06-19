pub struct MultilingualTranslator;

impl MultilingualTranslator {
    pub fn translate_term(&self, term: &str, target_iso: &str) -> String {
        match (term, target_iso) {
            ("minting", "zh") => "铸造".to_string(),
            ("staking", "es") => "Participación (Staking)".to_string(),
            _ => term.to_string(),
        }
    }
}
