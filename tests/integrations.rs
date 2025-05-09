

#[test]
fn valid_client_provider() {
    let provider = epics_pvaclient_sys::get_client_provider();
    assert!(!provider.is_null(), "Failed to create a valid channel provider");
}

#[test]
fn valid_client_channel() {
    let channel_name = "TEST:PV1";
    let channel = epics_pvaclient_sys::get_client_channel(&channel_name);
    assert!(!channel.is_null(), "Failed to create a valid channel for {}", channel_name);
}

#[test]
fn get_pv_as_string() {
    let pv_name = "TEST:PV1";
    let value = epics_pvaclient_sys::get_pv_all_fields_as_string(pv_name);
    // assert does not contain "Error"
    assert!(!value.contains("Error"), "Unexpected error message: {}", value);
}

