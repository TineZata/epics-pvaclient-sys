#[test]
fn pvget_all_fields_as_nt_scalar_byte() {
    let pv_name = "TEST:PV_Byte";
    let pv_option= epics_pvaclient_sys::pvget_all_fields_as_nt_scalar(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_byte = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Byte(val) => val,
            _ => panic!("Failed to convert NTScalarValue to u8: unexpected variant"),
        };
        assert_eq!(value_as_byte, 0xA, "Unexpected value for PV {}", pv_name);
    }
}