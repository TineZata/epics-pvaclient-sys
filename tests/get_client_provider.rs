#[test]
fn valid_client_provider() {
    let provider = epics_pvaclient_sys::get_client_provider();
    assert!(!provider.is_null(), "Failed to create a valid channel provider");
}
