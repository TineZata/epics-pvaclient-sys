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

        // Declare accessor methods for NTScalar
        fn nt_scalar_get_value(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_alarm_severity(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_alarm_status(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_alarm_message(scalar: &NTScalar) -> *const c_char;
        fn nt_scalar_get_timestamp_seconds(scalar: &NTScalar) -> i64;
        fn nt_scalar_get_timestamp_nanoseconds(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_timestamp_user_tag(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_display_limit_low(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_display_limit_high(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_display_description(scalar: &NTScalar) -> *const c_char;
        fn nt_scalar_get_display_units(scalar: &NTScalar) -> *const c_char;
        fn nt_scalar_get_display_precision(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_display_form_index(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_display_form_choices(scalar: &NTScalar) -> *const *const c_char;
        fn nt_scalar_get_display_form_choices_count(scalar: &NTScalar) -> usize;
        fn nt_scalar_get_control_limit_low(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_control_limit_high(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_control_min_step(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_value_alarm_active(scalar: &NTScalar) -> bool;
        fn nt_scalar_get_value_alarm_low_alarm_limit(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_value_alarm_low_warning_limit(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_value_alarm_high_warning_limit(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_value_alarm_high_alarm_limit(scalar: &NTScalar) -> f64;
        fn nt_scalar_get_value_alarm_low_alarm_severity(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_value_alarm_low_warning_severity(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_value_alarm_high_warning_severity(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_value_alarm_high_alarm_severity(scalar: &NTScalar) -> i32;
        fn nt_scalar_get_value_alarm_hysteresis(scalar: &NTScalar) -> i8;

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

pub fn get_pv_fields_as_struct(name: &str) -> Option<NTScalar> {
    // Convert the shared pointer to NTScalar to a raw pointer
    let nt_scalar_ptr = ffi::get_pv_value_fields_as_struct(name);
    
    // Check if the pointer is null
    if nt_scalar_ptr.is_null() {
        return None; // Return None if the pointer is null
    }

    // Convert the shared pointer to a reference to NTScalar
    let nt_scalar = nt_scalar_ptr.as_ref()?;

    Some(NTScalar {
        value: ffi::nt_scalar_get_value(nt_scalar),
        alarm_severity: ffi::nt_scalar_get_alarm_severity(nt_scalar),
        alarm_status: ffi::nt_scalar_get_alarm_status(nt_scalar),
        alarm_message: {
            let ptr = ffi::nt_scalar_get_alarm_message(nt_scalar);
            if ptr.is_null() {
                std::ptr::null()
            } else {
                ptr
            }
        },
        timestamp_seconds: ffi::nt_scalar_get_timestamp_seconds(nt_scalar),
        timestamp_nanoseconds: ffi::nt_scalar_get_timestamp_nanoseconds(nt_scalar),
        timestamp_user_tag: ffi::nt_scalar_get_timestamp_user_tag(nt_scalar),
        display_limit_low: ffi::nt_scalar_get_display_limit_low(nt_scalar),
        display_limit_high: ffi::nt_scalar_get_display_limit_high(nt_scalar),
        display_description:  {
            let ptr = ffi::nt_scalar_get_display_description(nt_scalar);
            if ptr.is_null() {
                std::ptr::null()
            } else {
                ptr
            }
        },
        display_units: {
            let ptr = ffi::nt_scalar_get_display_units(nt_scalar);
            if ptr.is_null() {
                std::ptr::null()
            } else {
                ptr
            }
        },
        display_precision: ffi::nt_scalar_get_display_precision(nt_scalar),
        display_form_index: ffi::nt_scalar_get_display_form_index(nt_scalar),
        display_form_choices: {
            let ptr = ffi::nt_scalar_get_display_form_choices(nt_scalar);
            if ptr.is_null() {
                std::ptr::null()
            } else {
                ptr
            }
        },
        display_form_choices_count: ffi::nt_scalar_get_display_form_choices_count(nt_scalar),
        control_limit_low: ffi::nt_scalar_get_control_limit_low(nt_scalar),
        control_limit_high: ffi::nt_scalar_get_control_limit_high(nt_scalar),
        control_min_step: ffi::nt_scalar_get_control_min_step(nt_scalar),
        value_alarm_active: ffi::nt_scalar_get_value_alarm_active(nt_scalar),
        value_alarm_low_alarm_limit: ffi::nt_scalar_get_value_alarm_low_alarm_limit(nt_scalar),
        value_alarm_low_warning_limit: ffi::nt_scalar_get_value_alarm_low_warning_limit(nt_scalar),
        value_alarm_high_warning_limit: ffi::nt_scalar_get_value_alarm_high_warning_limit(nt_scalar),
        value_alarm_high_alarm_limit: ffi::nt_scalar_get_value_alarm_high_alarm_limit(nt_scalar),
        value_alarm_low_alarm_severity: ffi::nt_scalar_get_value_alarm_low_alarm_severity(nt_scalar),
        value_alarm_low_warning_severity: ffi::nt_scalar_get_value_alarm_low_warning_severity(nt_scalar),
        value_alarm_high_warning_severity: ffi::nt_scalar_get_value_alarm_high_warning_severity(nt_scalar),
        value_alarm_high_alarm_severity: ffi::nt_scalar_get_value_alarm_high_alarm_severity(nt_scalar),
        value_alarm_hysteresis: ffi::nt_scalar_get_value_alarm_hysteresis(nt_scalar),
    })
}
