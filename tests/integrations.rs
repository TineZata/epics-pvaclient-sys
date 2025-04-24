use epics_pvaclient_sys::NTScalar;

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
    let value = epics_pvaclient_sys::get_pv_fields_as_string(&pv_name);
    assert_ne!(value, "Error: Timeout");
}

#[test]
fn test_get_pv_as_struct_does_not_timeout() {
    let pv_name = "TEST:PV1";
    let value: Option<PVStructure> = epics_pvaclient_sys::get_pv_fields_as_struct(&pv_name);
    
    assert!(value.is_some(), "Failed to get PV fields as struct for {}", pv_name);
}