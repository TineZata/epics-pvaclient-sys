#[test]
fn pvget_all_fields_as_enum_bool() {
    let pv_name = "TEST:PV_Bool";
    let pv_option= epics_pvaclient_sys::pvget_all_fields_as_nt_scalar(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_bool = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Boolean(val) => val,
            _ => panic!("Failed to convert NTScalarValue to bool: unexpected variant"),
        };
        assert_eq!(value_as_bool, true, "Unexpected value for PV {}", pv_name);
    }
}
