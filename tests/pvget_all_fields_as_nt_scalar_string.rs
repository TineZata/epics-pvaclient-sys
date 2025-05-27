#[test]
fn pvget_all_fields_as_nt_scalar_string() {
    let pv_name = "TEST:PV_String";
    let pv_option= epics_pvaclient_sys::pvget_all_fields_as_nt_scalar(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_string = match pv.value {
            epics_pvaclient_sys::NTScalarValue::String(ref val) => val,
            _ => panic!("Failed to convert NTScalarValue to String: unexpected variant"),
        };
        assert_eq!(value_as_string, "Hello World", "Unexpected value for PV {}", pv_name);
    }
}