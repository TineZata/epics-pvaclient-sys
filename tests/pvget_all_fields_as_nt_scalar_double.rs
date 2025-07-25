#[test]
fn pvget_all_fields_as_nt_scalar_double() {
    let pv_name = "TEST:PV_Double";
    let pv_option= epics_pvaclient_sys::pvget_all_fields_as_nt_scalar(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_double = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Double(val) => val,
            _ => panic!("Failed to convert NTScalarValue to f64: unexpected variant"),
        };
        assert_eq!(value_as_double, 87654.321012300003, "Unexpected value for PV {}", pv_name);
        assert_eq!(pv.display_precision, 5, "Unexpected display precision for PV {}", pv_name);
        assert_eq!(pv.display_units, "um", "Unexpected units for PV {}", pv_name);
    }
}
