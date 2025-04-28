

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
    let pv_option= epics_pvaclient_sys::get_pv_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_double = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Double(val) => val,
            _ => panic!("Failed to convert NTScalarValue to f64: unexpected variant"),
        };
        assert_eq!(value_as_double, 87654.321, "Unexpected value for PV {}", pv_name);
        assert_eq!(pv.display_precision, 3, "Unexpected display precision for PV {}", pv_name);
        assert_eq!(pv.display_units, "um", "Unexpected units for PV {}", pv_name);
    }
}