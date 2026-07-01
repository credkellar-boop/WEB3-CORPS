use web3_corps::localization::translator::MultilingualTranslator;

#[test]
fn test_technical_crypto_glossary_translation() {
    let translator = MultilingualTranslator;

    let mandarin_mint = translator.translate_term("minting", "zh");
    let spanish_stake = translator.translate_term("staking", "es");

    assert_eq!(mandarin_mint, "铸造");
    assert_eq!(spanish_stake, "Participación (Staking)");
}

#[test]
fn test_fallback_unsupported_iso_codes() {
    let translator = MultilingualTranslator;
    let fallback = translator.translate_term("liquidity_pool", "xyz");
    
    assert_eq!(fallback, "liquidity_pool");
    // Ensures unmatched codes safely default to baseline text
}
