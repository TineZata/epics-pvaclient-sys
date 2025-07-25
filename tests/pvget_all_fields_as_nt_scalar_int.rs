#[test]
fn pvget_all_fields_as_nt_scalar_int() {
    let pv_name = "TEST:PV_Int";
    let pv_option= epics_pvaclient_sys::pvget_all_fields_as_nt_scalar(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_int = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Int(val) => val,
            _ => panic!("Failed to convert NTScalarValue to i32: unexpected variant"),
        };
        assert_eq!(value_as_int, 1234, "Unexpected value for PV {}", pv_name);

    }
}