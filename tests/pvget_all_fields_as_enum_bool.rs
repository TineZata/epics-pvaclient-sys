#[test]
fn pvget_all_fields_as_enum_bool() {
    let pv_name = "TEST:PV_Bool";
    let pv_option = epics_pvaclient_sys::pvget_all_fields_as_nt_enum(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {
        assert_eq!(pv.value_index, 1, "Unexpected value index for PV {}: expected 1, got {}", pv_name, pv.value_index);
        // assert value choices is index 0 = "False", index 1 = "True"
        assert_eq!(pv.value_choices.len(), 2, "Unexpected number of choices for PV {}: expected 2, got {}", pv_name, pv.value_choices.len());
        assert_eq!(pv.value_choices[0], "False", "Unexpected choice at index 0 for PV {}: expected 'False', got '{}'", pv_name, pv.value_choices[0]);
        assert_eq!(pv.value_choices[1], "True", "Unexpected choice at index 1 for PV {}: expected 'True', got '{}'", pv_name, pv.value_choices[1]);
        assert_eq!(pv.alarm_severity, 0, "Unexpected alarm severity for PV {}: expected 0, got {}", pv_name, pv.alarm_severity);
        assert_eq!(pv.alarm_status, 2, "Unexpected alarm status for PV {}: expected 2, got {}", pv_name, pv.alarm_status);
        assert_eq!(pv.alarm_message, "UDF", "Unexpected alarm message for PV {}: expected 'UDF', got '{}'", pv_name, pv.alarm_message);
        assert!(pv.timestamp_secondspastepoch > 600000000, "Unexpected timestamp for PV {}: expected > 6000000000, got {}", pv_name, pv.timestamp_secondspastepoch);
        assert!(pv.timestamp_nanoseconds == 0, "Unexpected nanoseconds for PV {}: expected == 0, got {}", pv_name, pv.timestamp_nanoseconds);
        assert!(pv.timestamp_user_tag == 0, "Unexpected user tag for PV {}: expected 0, got {}", pv_name, pv.timestamp_user_tag);
    }
    else {
        assert!(false, "Failed to get PV fields as struct for {}", pv_name);
    }
}
