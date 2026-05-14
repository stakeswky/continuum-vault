#[path = "../src/continuum/custom_item_validators.rs"]
mod custom_item_validators;

use custom_item_validators::validate_continuum_fields;

#[test]
fn mnemonic_requires_network_field() {
    let fields_ok = vec![
        ("continuum:item_type", "crypto_mnemonic"),
        ("continuum:network", "BTC"),
    ];
    assert!(validate_continuum_fields(&fields_ok).is_ok());

    let fields_missing = vec![("continuum:item_type", "crypto_mnemonic")];
    assert!(validate_continuum_fields(&fields_missing).is_err());
}
