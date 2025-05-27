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
    }
    else {
        assert!(false, "Failed to get PV fields as struct for {}", pv_name);
    }
}
