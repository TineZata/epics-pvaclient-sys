#[test]
fn pvget_all_fields_as_nt_enum() {
    let pv_name = "TEST:PV_Enum";
    let pv_option = epics_pvaclient_sys::pvget_all_fields_as_nt_enum(pv_name);
    assert!(pv_option.is_some(), "Failed to get PV fields as struct for {}", pv_name);
    if let Some(pv) = pv_option {
        assert_eq!(pv.value_index, 3, "Unexpected value index for PV {}: expected 1, got {}", pv_name, pv.value_index);
        // assert value choices is index 0 = "False", index 1 = "True"
        assert_eq!(pv.value_choices.len(), 9, "Unexpected number of choices for PV {}: expected 9, got {}", pv_name, pv.value_choices.len());
        //  string[] choices [, " 1  :  10 mA", " 2  :   1 mA", " 3  : 100 uA", " 4  :  10 uA", " 5  :   1 uA", " 6  : 100 nA", " 7  :  10 nA", " 8  :   1 nA"]
        assert_eq!(pv.value_choices[0], " 0  : 100 mA", "Unexpected choice at index 0 for PV {}: expected ' 1  : 100 mA', got '{}'", pv_name, pv.value_choices[0]);
        assert_eq!(pv.value_choices[1], " 1  :  10 mA", "Unexpected choice at index 0 for PV {}: expected ' 1  :  10 mA', got '{}'", pv_name, pv.value_choices[1]);
        assert_eq!(pv.value_choices[2], " 2  :   1 mA", "Unexpected choice at index 1 for PV {}: expected ' 2  :   1 mA', got '{}'", pv_name, pv.value_choices[2]);
        assert_eq!(pv.value_choices[3], " 3  : 100 uA", "Unexpected choice at index 2 for PV {}: expected ' 3  : 100 uA', got '{}'", pv_name, pv.value_choices[3]);
        assert_eq!(pv.value_choices[4], " 4  :  10 uA", "Unexpected choice at index 3 for PV {}: expected ' 4  :  10 uA', got '{}'", pv_name, pv.value_choices[4]);
        assert_eq!(pv.value_choices[5], " 5  :   1 uA", "Unexpected choice at index 4 for PV {}: expected ' 5  :   1 uA', got '{}'", pv_name, pv.value_choices[5]);
        assert_eq!(pv.value_choices[6], " 6  : 100 nA", "Unexpected choice at index 5 for PV {}: expected ' 6  : 100 nA', got '{}'", pv_name, pv.value_choices[6]);
        assert_eq!(pv.value_choices[7], " 7  :  10 nA", "Unexpected choice at index 6 for PV {}: expected ' 7  :  10 nA', got '{}'", pv_name, pv.value_choices[7]);
        assert_eq!(pv.value_choices[8], " 8  :   1 nA", "Unexpected choice at index 7 for PV {}: expected ' 8  :   1 nA', got '{}'", pv_name, pv.value_choices[8]);
        
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
