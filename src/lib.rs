use cxx::SharedPtr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct NTScalar {
    pub value: f64,
    pub alarm_severity: i32,
    pub alarm_status: i32,
    pub alarm_message: *const c_char,
    pub timestamp_seconds: i64,
    pub timestamp_nanoseconds: i32,
    pub timestamp_userTag: i32,
    pub display_limitLow: f64,
    pub display_limitHigh: f64,
    pub display_description: *const c_char,
    pub display_units: *const c_char,
    pub display_precision: i32,
    pub display_form_index: i32,
    pub display_form_choices: *const *const c_char,
    pub display_form_choices_count: usize,
    pub control_limitLow: f64,
    pub control_limitHigh: f64,
    pub control_minStep: f64,
    pub valueAlarm_active: bool,
    pub valueAlarm_lowAlarmLimit: f64,
    pub valueAlarm_lowWarningLimit: f64,
    pub valueAlarm_highWarningLimit: f64,
    pub valueAlarm_highAlarmLimit: f64,
    pub valueAlarm_lowAlarmSeverity: i32,
    pub valueAlarm_lowWarningSeverity: i32,
    pub valueAlarm_highWarningSeverity: i32,
    pub valueAlarm_highAlarmSeverity: i32,
    pub valueAlarm_hysteresis: i8,
}

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");

        // Declare opaque types
        type ClientProvider;
        type ClientChannel;
        type PVStructure;
        type NTScalar;

        // Declare functions that return shared pointers to C++ classes
        fn get_client_provider() -> SharedPtr<ClientProvider>;
        fn get_client_provider_with_config() -> SharedPtr<ClientProvider>;
        fn get_client_channel(name: &str) -> SharedPtr<ClientChannel>;
        fn get_pv_value(name: &str) -> String;
        fn extract_ntscalar(handle: SharedPtr<PVStructure>) -> SharedPtr<NTScalar>;
        
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

pub fn get_client_provider_with_config() -> SharedPtr<ffi::ClientProvider> {
    ffi::get_client_provider_with_config()
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
/// let value = epics_pvaclient_sys::get_pv(&pv_name);
/// assert_ne!(value, "Error: Timeout");
/// ```
pub fn get_pv(name: &str) -> String {
    ffi::get_pv_value(name)
}
