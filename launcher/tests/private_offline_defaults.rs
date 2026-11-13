use horizon_launcher::backend::Settings;

#[test]
fn offline_names_are_enabled_for_new_and_existing_settings() {
    for text in ["", "{}", r#"{"language":"en"}"#] {
        assert!(Settings::parse(text).dev_offline);
    }
}

#[test]
fn explicit_opt_out_survives_serialization() {
    let settings = Settings::parse(r#"{"dev_offline":false}"#);
    assert!(!settings.dev_offline);
    let saved = serde_json::to_string(&settings).unwrap();
    assert!(!Settings::parse(&saved).dev_offline);
}
