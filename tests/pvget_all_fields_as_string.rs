#[test]
fn pvget_all_fields_as_string() {
    let pv_name = "TEST:PV1";
    let value = epics_pvaclient_sys::pvget_all_fields_as_string(pv_name);
    // assert does not contain "Error"
    assert!(!value.contains("Error"), "Unexpected error message: {}", value);
}
