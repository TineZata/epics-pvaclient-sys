#[test]
fn test_get_pv_as_double_does_not_timeout() {
    let pv_name = "TEST:PV_Double";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_double = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Double(val) => val,
            _ => panic!("Failed to convert NTScalarValue to f64: unexpected variant"),
        };
        assert_eq!(value_as_double, 87654.321012, "Unexpected value for PV {}", pv_name);
        assert_eq!(pv.display_precision, 5, "Unexpected display precision for PV {}", pv_name);
        assert_eq!(pv.display_units, "um", "Unexpected units for PV {}", pv_name);
    }
}

#[test]
fn test_get_pv_as_int_does_not_timeout() {
    let pv_name = "TEST:PV_Int";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_int = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Int(val) => val,
            _ => panic!("Failed to convert NTScalarValue to i32: unexpected variant"),
        };
        assert_eq!(value_as_int, 123456, "Unexpected value for PV {}", pv_name);

    }
}

#[test]
fn test_get_pv_as_string_does_not_timeout() {
    let pv_name = "TEST:PV_String";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_string = match pv.value {
            epics_pvaclient_sys::NTScalarValue::String(ref val) => val,
            _ => panic!("Failed to convert NTScalarValue to String: unexpected variant"),
        };
        assert_eq!(value_as_string, "Hello World", "Unexpected value for PV {}", pv_name);
    }
}

#[test]
fn test_get_pv_as_bool_does_not_timeout() {
    let pv_name = "TEST:PV_Bool";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_bool = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Boolean(val) => val,
            _ => panic!("Failed to convert NTScalarValue to bool: unexpected variant"),
        };
        assert_eq!(value_as_bool, true, "Unexpected value for PV {}", pv_name);
    }
}

#[test]
fn test_get_pv_as_short_does_not_timeout() {
    let pv_name = "TEST:PV_Short";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_short = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Short(val) => val,
            _ => panic!("Failed to convert NTScalarValue to i16: unexpected variant"),
        };
        assert_eq!(value_as_short, 8, "Unexpected value for PV {}", pv_name);
    }
}

#[test]
fn test_get_pv_as_enum_does_not_timeout() {
    let pv_name = "TEST:PV_Enum";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_enum = match pv.value {
            epics_pvaclient_sys::NTScalarValue::UByte(val) => val,
            _ => panic!("Failed to convert NTScalarValue to i32: unexpected variant"),
        };
        assert_eq!(value_as_enum, 1, "Unexpected value for PV {}", pv_name);
    }
}

#[test]
fn test_get_pv_as_byte_does_not_timeout() {
    let pv_name = "TEST:PV_Byte";
    let pv_option= epics_pvaclient_sys::get_pv_all_fields_as_struct(&pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {        
        let value_as_byte = match pv.value {
            epics_pvaclient_sys::NTScalarValue::Byte(val) => val,
            _ => panic!("Failed to convert NTScalarValue to u8: unexpected variant"),
        };
        assert_eq!(value_as_byte, 0xA, "Unexpected value for PV {}", pv_name);
    }
}
