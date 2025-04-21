use cxx::SharedPtr;
use std::os::raw::c_char;

#[repr(C)]
pub struct NTScalar {
    pub value: f64,
    pub alarm_severity: i32,
    pub alarm_status: i32,
    pub alarm_message: *const c_char,
    pub timestamp_seconds: i64,
    pub timestamp_nanoseconds: i32,
    pub timestamp_user_tag: i32,
    pub display_limit_low: f64,
    pub display_limit_high: f64,
    pub display_description: *const c_char,
    pub display_units: *const c_char,
    pub display_precision: i32,
    pub display_form_index: i32,
    pub display_form_choices: *const *const c_char,
    pub display_form_choices_count: usize,
    pub control_limit_low: f64,
    pub control_limit_high: f64,
    pub control_min_step: f64,
    pub value_alarm_active: bool,
    pub value_alarm_low_alarm_limit: f64,
    pub value_alarm_low_warning_limit: f64,
    pub value_alarm_high_warning_limit: f64,
    pub value_alarm_high_alarm_limit: f64,
    pub value_alarm_low_alarm_severity: i32,
    pub value_alarm_low_warning_severity: i32,
    pub value_alarm_high_warning_severity: i32,
    pub value_alarm_high_alarm_severity: i32,
    pub value_alarm_hysteresis: i8,
}

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");

        // Declare opaque types
        type ClientProvider;
        type ClientChannel;
        type NTScalar;

        // Declare functions that return shared pointers to C++ classes
        fn get_client_provider() -> SharedPtr<ClientProvider>;
        fn get_client_channel(name: &str) -> SharedPtr<ClientChannel>;
        fn get_pv_value_fields_as_string(name: &str) -> String;
        fn get_pv_value_fields_as_struct(name: &str) -> SharedPtr<NTScalar>;
    }
}

/// # Returns a shared pointer with a valid client provider or null.
/// 
/// ## Example:
/// ```rust
/// let provider = epics_pvaclient_sys::get_client_provider();
/// assert!(!provider.is_null(), "Failed to create a valid channel provider");
/// ```
pub fn get_client_provider() -> SharedPtr<ffi::ClientProvider> {
    ffi::get_client_provider()
}

/// # Returns a shared pointer with a valid client channel or null.
/// 
/// ## Example:
/// ```rust
/// let channel_name = "TEST:PV1";
/// let channel = epics_pvaclient_sys::get_client_channel(&channel_name);
/// assert!(!channel.is_null(), "Failed to create a valid channel for {}", channel_name);
/// ```
pub fn get_client_channel(name: &str) -> SharedPtr<ffi::ClientChannel> {
    ffi::get_client_channel(name)
}

/// # Returns a string with the value of the specified PV or an error message for timeout.
/// 
/// ## Example:
/// ```rust
/// let pv_name = "TEST:PV1";
/// let value = epics_pvaclient_sys::get_pv_fields_as_string(&pv_name);
/// assert_ne!(value, "Error: Timeout");
/// ```
pub fn get_pv_fields_as_string(name: &str) -> String {
    ffi::get_pv_value_fields_as_string(name)
}

pub fn get_pv_fields_as_struct(name: &str) -> SharedPtr<ffi::NTScalar> {
    ffi::get_pv_value_fields_as_struct(name)
}
