#[test]
fn test_valid_client_provider() {
    let provider = epics_pvaclient_sys::get_client_provider();
    assert!(!provider.is_null(), "Failed to create a valid channel provider");
}

#[test]
fn test_valid_client_channel() {
    let channel_name = "TEST:PV1";
    let channel = epics_pvaclient_sys::get_client_channel(&channel_name);
    assert!(!channel.is_null(), "Failed to create a valid channel for {}", channel_name);
}

#[test]
fn test_get_pv_does_not_timeout() {
    let pv_name = "TEST:PV1";
    let value = epics_pvaclient_sys::get_pv(&pv_name);
    assert_ne!(value, "Error: Timeout");
}