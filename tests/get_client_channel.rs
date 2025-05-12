#[test]
fn valid_client_channel() {
    let channel_name = "TEST:PV1";
    let channel = epics_pvaclient_sys::get_client_channel(&channel_name);
    assert!(!channel.is_null(), "Failed to create a valid channel for {}", channel_name);
}
