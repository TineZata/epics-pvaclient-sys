use cxx::SharedPtr;

pub enum NTScalarValue {
    Boolean(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Float(f32),
    Double(f64),
    String(String),
}

pub struct PVStructure {
    pub value: NTScalarValue,
    pub alarm_severity: i32,
    pub alarm_status: i32,
    pub alarm_message: String,
    pub timestamp_seconds: i64,
    pub timestamp_nanoseconds: i32,
    pub timestamp_user_tag: i32,
    pub display_limit_low: f64,
    pub display_limit_high: f64,
    pub display_description: String,
    pub display_units: String,
    pub display_precision: i32,
    pub display_form_index: i32,
    pub display_form_choices: Vec<String>,
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
    pub value_alarm_hysteresis: i8
}

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");

        // Declare opaque types
        type ClientProvider;
        type ClientChannel;
        type PVStructure;

        // Declare functions that return shared pointers to C++ classes
        fn get_client_provider() -> SharedPtr<ClientProvider>;
        fn get_client_channel(name: &str) -> SharedPtr<ClientChannel>;
        fn get_pv_value_fields_as_string(name: &str) -> String;
        fn get_pv_value_fields_as_struct(name: &str) -> SharedPtr<PVStructure>;

        // Declare accessor methods for NTScalar
        fn nt_scalar_get_value_boolean(pvdata: SharedPtr<PVStructure>) -> bool;
        fn nt_scalar_get_value_byte(pvdata: SharedPtr<PVStructure>) -> i8;
        fn nt_scalar_get_value_short(pvdata: SharedPtr<PVStructure>) -> i16;
        fn nt_scalar_get_value_int(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_value_long(pvdata: SharedPtr<PVStructure>) -> i64;
        fn nt_scalar_get_value_unsigned_byte(pvdata: SharedPtr<PVStructure>) -> u8;
        fn nt_scalar_get_value_unsigned_short(pvdata: SharedPtr<PVStructure>) -> u16;
        fn nt_scalar_get_value_unsigned_int(pvdata: SharedPtr<PVStructure>) -> u32;
        fn nt_scalar_get_value_unsigned_long(pvdata: SharedPtr<PVStructure>) -> u64;
        fn nt_scalar_get_value_float(pvdata: SharedPtr<PVStructure>) -> f32;
        fn nt_scalar_get_value_double(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_string(pvdata: SharedPtr<PVStructure>) -> *const c_char;
        fn nt_scalar_get_value_type(pvdata: SharedPtr<PVStructure>) -> i8;

        fn nt_scalar_get_alarm_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_alarm_status(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_alarm_message(pvdata: SharedPtr<PVStructure>) -> *const c_char;
        fn nt_scalar_get_timestamp_seconds(pvdata: SharedPtr<PVStructure>) -> i64;
        fn nt_scalar_get_timestamp_nanoseconds(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_timestamp_user_tag(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_display_limit_low(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_display_limit_high(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_display_description(pvdata: SharedPtr<PVStructure>) -> *const c_char;
        fn nt_scalar_get_display_units(pvdata: SharedPtr<PVStructure>) -> *const c_char;
        fn nt_scalar_get_display_precision(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_display_form_index(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_display_form_choices(pvdata: SharedPtr<PVStructure>) -> *const *const c_char;
        fn nt_scalar_get_display_form_choices_count(pvdata: SharedPtr<PVStructure>) -> usize;
        fn nt_scalar_get_control_limit_low(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_control_limit_high(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_control_min_step(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_alarm_active(pvdata: SharedPtr<PVStructure>) -> bool;
        fn nt_scalar_get_value_alarm_low_alarm_limit(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_alarm_low_warning_limit(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_alarm_high_warning_limit(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_alarm_high_alarm_limit(pvdata: SharedPtr<PVStructure>) -> f64;
        fn nt_scalar_get_value_alarm_low_alarm_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_value_alarm_low_warning_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_value_alarm_high_warning_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_value_alarm_high_alarm_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_scalar_get_value_alarm_hysteresis(pvdata: SharedPtr<PVStructure>) -> i8;

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

pub fn get_pv_fields_as_struct(name: &str) -> Option<PVStructure> {
    // Convert the shared pointer to NTScalar to a raw pointer
    let pv_struct_ptr = ffi::get_pv_value_fields_as_struct(name);
    
    // Check if the pointer is null
    if pv_struct_ptr.is_null() {
        return None; // Return None if the pointer is null
    }
    
    Some(PVStructure {
        value: {
            let value_type = ffi::nt_scalar_get_value_type(pv_struct_ptr.clone());
            match value_type {
                0 => NTScalarValue::Boolean(ffi::nt_scalar_get_value_boolean(pv_struct_ptr.clone())),
                1 => NTScalarValue::Byte(ffi::nt_scalar_get_value_byte(pv_struct_ptr.clone())),
                2 => NTScalarValue::Short(ffi::nt_scalar_get_value_short(pv_struct_ptr.clone())),
                3 => NTScalarValue::Int(ffi::nt_scalar_get_value_int(pv_struct_ptr.clone())),
                4 => NTScalarValue::Long(ffi::nt_scalar_get_value_long(pv_struct_ptr.clone())),
                5 => NTScalarValue::UByte(ffi::nt_scalar_get_value_unsigned_byte(pv_struct_ptr.clone())),
                6 => NTScalarValue::UShort(ffi::nt_scalar_get_value_unsigned_short(pv_struct_ptr.clone())),
                7 => NTScalarValue::UInt(ffi::nt_scalar_get_value_unsigned_int(pv_struct_ptr.clone())),
                8 => NTScalarValue::ULong(ffi::nt_scalar_get_value_unsigned_long(pv_struct_ptr.clone())),
                9 => NTScalarValue::Float(ffi::nt_scalar_get_value_float(pv_struct_ptr.clone())),
                10 => NTScalarValue::Double(ffi::nt_scalar_get_value_double(pv_struct_ptr.clone())),
                11 => {
                    let ptr = ffi::nt_scalar_get_value_string(pv_struct_ptr.clone());
                    if ptr.is_null() {
                        NTScalarValue::String(String::new())
                    } else {
                        NTScalarValue::String(unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() })
                    }
                },
                _ => panic!("Unknown type"),
            }
        },
        alarm_severity: ffi::nt_scalar_get_alarm_severity(pv_struct_ptr.clone()),
        alarm_status: ffi::nt_scalar_get_alarm_status(pv_struct_ptr.clone()),
        alarm_message: {
            let ptr = ffi::nt_scalar_get_alarm_message(pv_struct_ptr.clone());
            if ptr.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            }
        },
        timestamp_seconds: ffi::nt_scalar_get_timestamp_seconds(pv_struct_ptr.clone()),
        timestamp_nanoseconds: ffi::nt_scalar_get_timestamp_nanoseconds(pv_struct_ptr.clone()),
        timestamp_user_tag: ffi::nt_scalar_get_timestamp_user_tag(pv_struct_ptr.clone()),
        display_limit_low: ffi::nt_scalar_get_display_limit_low(pv_struct_ptr.clone()),
        display_limit_high: ffi::nt_scalar_get_display_limit_high(pv_struct_ptr.clone()),
        display_description:  {
            let ptr = ffi::nt_scalar_get_display_description(pv_struct_ptr.clone());
            if ptr.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            }
        },
        display_units: {
            let ptr = ffi::nt_scalar_get_display_units(pv_struct_ptr.clone());
            if ptr.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            }
        },
        display_precision: ffi::nt_scalar_get_display_precision(pv_struct_ptr.clone()),
        display_form_index: ffi::nt_scalar_get_display_form_index(pv_struct_ptr.clone()),
        display_form_choices: {
            let ptr = ffi::nt_scalar_get_display_form_choices(pv_struct_ptr.clone());
            if ptr.is_null() {
                Vec::new()
            } else {
                let choices = unsafe { std::slice::from_raw_parts(ptr, ffi::nt_scalar_get_display_form_choices_count(pv_struct_ptr.clone())) };
                choices.iter().map(|&choice| {
                    if choice.is_null() {
                        String::new()
                    } else {
                        unsafe { std::ffi::CStr::from_ptr(choice).to_string_lossy().into_owned() }
                    }
                }).collect::<Vec<_>>()
            }
        },
        display_form_choices_count: ffi::nt_scalar_get_display_form_choices_count(pv_struct_ptr.clone()),
        control_limit_low: ffi::nt_scalar_get_control_limit_low(pv_struct_ptr.clone()),
        control_limit_high: ffi::nt_scalar_get_control_limit_high(pv_struct_ptr.clone()),
        control_min_step: ffi::nt_scalar_get_control_min_step(pv_struct_ptr.clone()),
        value_alarm_active: ffi::nt_scalar_get_value_alarm_active(pv_struct_ptr.clone()),
        value_alarm_low_alarm_limit: ffi::nt_scalar_get_value_alarm_low_alarm_limit(pv_struct_ptr.clone()),
        value_alarm_low_warning_limit: ffi::nt_scalar_get_value_alarm_low_warning_limit(pv_struct_ptr.clone()),
        value_alarm_high_warning_limit: ffi::nt_scalar_get_value_alarm_high_warning_limit(pv_struct_ptr.clone()),
        value_alarm_high_alarm_limit: ffi::nt_scalar_get_value_alarm_high_alarm_limit(pv_struct_ptr.clone()),
        value_alarm_low_alarm_severity: ffi::nt_scalar_get_value_alarm_low_alarm_severity(pv_struct_ptr.clone()),
        value_alarm_low_warning_severity: ffi::nt_scalar_get_value_alarm_low_warning_severity(pv_struct_ptr.clone()),
        value_alarm_high_warning_severity: ffi::nt_scalar_get_value_alarm_high_warning_severity(pv_struct_ptr.clone()),
        value_alarm_high_alarm_severity: ffi::nt_scalar_get_value_alarm_high_alarm_severity(pv_struct_ptr.clone()),
        value_alarm_hysteresis: ffi::nt_scalar_get_value_alarm_hysteresis(pv_struct_ptr.clone()),
    })
}
