use cxx::SharedPtr;

#[repr(i8)]
#[derive(Debug, Clone)]
pub enum EpicsDataType {
    Scalar = 0,
    ScalarArray = 1,
    Structure = 2,
    StructureArray = 3,
    Union = 4,
    UnionArray = 5,
}

impl TryFrom<i8> for EpicsDataType {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EpicsDataType::Scalar),
            1 => Ok(EpicsDataType::ScalarArray),
            2 => Ok(EpicsDataType::Structure),
            3 => Ok(EpicsDataType::StructureArray),
            4 => Ok(EpicsDataType::Union),
            5 => Ok(EpicsDataType::UnionArray),
            _ => Err(()),
        }
    }
}

#[repr(i8)]
#[derive(Debug, Clone)]
pub enum EpicsScalarType {
    PvBoolean = 0,
    PvByte = 1,
    PvShort = 2,
    PvInt = 3,
    PvLong = 4,
    PvUbyte = 5,
    PvUshort = 6,
    PvUint = 7,
    PvUlong = 8,
    PvFloat = 9,
    PvDouble = 10,
    PvString = 11,
}

impl TryFrom<i8> for EpicsScalarType {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EpicsScalarType::PvBoolean),
            1 => Ok(EpicsScalarType::PvByte),
            2 => Ok(EpicsScalarType::PvShort),
            3 => Ok(EpicsScalarType::PvInt),
            4 => Ok(EpicsScalarType::PvLong),
            5 => Ok(EpicsScalarType::PvUbyte),
            6 => Ok(EpicsScalarType::PvUshort),
            7 => Ok(EpicsScalarType::PvUint),
            8 => Ok(EpicsScalarType::PvUlong),
            9 => Ok(EpicsScalarType::PvFloat),
            10 => Ok(EpicsScalarType::PvDouble),
            11 => Ok(EpicsScalarType::PvString),
            _ => Err(()),
        }
    }
    
}
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct NTScalar {
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
/* enum_t value
            int index
            string[] choices
        alarm_t alarm
            int severity
            int status
            string message
        structure timeStamp
            long secondsPastEpoch
            int nanoseconds
            int userTag */
#[derive(Debug, Clone)]
pub struct NTEnum {
    pub value_index: i32,
    pub value_choices: Vec<String>,
    pub alarm_severity: i32,
    pub alarm_status: i32,
    pub alarm_message: String,
    pub timestamp_secondspastepoch: i64,
    pub timestamp_nanoseconds: i32,
    pub timestamp_user_tag: i32,
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
        fn get_pv_field_data_type(pvdata: SharedPtr<PVStructure>) -> i8;

        // Declare accessor methods for NTScalar
        fn nt_scalar_get_value_type(pvdata: SharedPtr<PVStructure>) -> i8;
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

        // Declare accessor methods for NTEnum
        fn nt_enum_get_value_index(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_enum_get_value_choices(pvdata: SharedPtr<PVStructure>) -> *const *const c_char;
        fn nt_enum_get_value_choices_count(pvdata: SharedPtr<PVStructure>) -> usize;
        fn nt_enum_get_alarm_severity(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_enum_get_alarm_status(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_enum_get_alarm_message(pvdata: SharedPtr<PVStructure>) -> *const c_char;
        fn nt_enum_get_timestamp_seconds(pvdata: SharedPtr<PVStructure>) -> i64;
        fn nt_enum_get_timestamp_nanoseconds(pvdata: SharedPtr<PVStructure>) -> i32;
        fn nt_enum_get_timestamp_user_tag(pvdata: SharedPtr<PVStructure>) -> i32;
    }
}


/// # Returns a shared pointer to the ClientProvider.
/// 
/// ## Example
/// ```
/// use epics_pvaclient_sys::get_client_provider;
/// let provider = get_client_provider();
/// ```
pub fn get_client_provider() -> SharedPtr<ffi::ClientProvider> {
    ffi::get_client_provider()
}

/// # Returns a shared pointer to the ClientChannel for the given name.
/// 
/// ## Example
/// ```
/// use epics_pvaclient_sys::get_client_channel;
/// let channel_name = "TEST:PV1";
/// let channel = get_client_channel(channel_name);
/// ```
pub fn get_client_channel(name: &str) -> SharedPtr<ffi::ClientChannel> {
    ffi::get_client_channel(name)
}

/// # Returns a string representation of all fields for the given PV name.
/// 
/// ## Example
/// ```
/// use epics_pvaclient_sys::pvget_all_fields_as_string;
/// let pv_name = "TEST:PV1";
/// let fields_string = pvget_all_fields_as_string(pv_name);
/// println!("Fields for {}: {}", pv_name, fields_string);
/// ```
pub fn pvget_all_fields_as_string(name: &str) -> String {
    ffi::get_pv_value_fields_as_string(name)
}

/// # Returns an NTScalar struct with all fields for the given PV name.
///
/// ## Example
/// ```
/// use epics_pvaclient_sys::pvget_all_fields_as_nt_scalar;
/// let pv_name = "TEST:PV_Double";
/// if let Some(nt_scalar) = pvget_all_fields_as_nt_scalar(pv_name) {
///    println!("NTScalar for {}: {:?}", pv_name, nt_scalar);
/// } else {
///   println!("No NTScalar found for {}", pv_name);
/// }
/// ```
pub fn pvget_all_fields_as_nt_scalar(name: &str) -> Option<NTScalar> {
    // Convert the shared pointer to NTScalar to a raw pointer
    let pv_struct_ptr = ffi::get_pv_value_fields_as_struct(name);
    
    // Check if the pointer is null
    if pv_struct_ptr.is_null() {
        return None; // Return None if the pointer is null
    }
    
    Some(NTScalar {
        value: {
            let pv_data_type = EpicsDataType::try_from(ffi::get_pv_field_data_type(pv_struct_ptr.clone()));
            if pv_data_type.is_err() {
                panic!("Failed to convert data type");
            }
            match pv_data_type.unwrap() {
                EpicsDataType::Scalar => {
                    // Data type is a normal scalar
                    let scalar_type = EpicsScalarType::try_from(ffi::nt_scalar_get_value_type(pv_struct_ptr.clone()));
                    if scalar_type.is_err() {
                        panic!("Failed to convert data type");
                    }
                    match scalar_type.unwrap() {
                        EpicsScalarType::PvBoolean => {
                            NTScalarValue::Boolean(ffi::nt_scalar_get_value_boolean(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvByte => {
                            NTScalarValue::Byte(ffi::nt_scalar_get_value_byte(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvShort => {
                            NTScalarValue::Short(ffi::nt_scalar_get_value_short(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvInt => {
                            NTScalarValue::Int(ffi::nt_scalar_get_value_int(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvLong => {
                            NTScalarValue::Long(ffi::nt_scalar_get_value_long(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvUbyte => {
                            NTScalarValue::UByte(ffi::nt_scalar_get_value_unsigned_byte(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvUshort => {
                            NTScalarValue::UShort(ffi::nt_scalar_get_value_unsigned_short(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvUint => {
                            NTScalarValue::UInt(ffi::nt_scalar_get_value_unsigned_int(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvUlong => {
                            NTScalarValue::ULong(ffi::nt_scalar_get_value_unsigned_long(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvFloat => {
                            NTScalarValue::Float(ffi::nt_scalar_get_value_float(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvDouble => {
                            NTScalarValue::Double(ffi::nt_scalar_get_value_double(pv_struct_ptr.clone()))
                        },
                        EpicsScalarType::PvString => {
                            let ptr = ffi::nt_scalar_get_value_string(pv_struct_ptr.clone());
                            if ptr.is_null() {
                                NTScalarValue::String(String::new())
                            } else {
                                NTScalarValue::String(unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() })
                            }
                        }
                    }
                },
                _ => panic!("Unsupported field data type"),
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

/// # Returns an NTEnum struct with all fields for the given PV name.
/// 
/// ## Example
/// ```
/// use epics_pvaclient_sys::pvget_all_fields_as_nt_enum;
/// let pv_name = "TEST:PV_Enum";
/// if let Some(nt_enum) = pvget_all_fields_as_nt_enum(pv_name) {
///   println!("NTEnum for {}: {:?}", pv_name, nt_enum);
/// } else {
///   println!("No NTEnum found for {}", pv_name);
/// }
/// ```
pub fn pvget_all_fields_as_nt_enum(name: &str) -> Option<NTEnum> {
    // Convert the shared pointer to NTEnum to a raw pointer
    let pv_struct_ptr = ffi::get_pv_value_fields_as_struct(name);
    
    // Check if the pointer is null
    if pv_struct_ptr.is_null() {
        return None; // Return None if the pointer is null
    }
    
    Some(NTEnum {
        value_index: ffi::nt_enum_get_value_index(pv_struct_ptr.clone()),
        value_choices: {
            let ptr = ffi::nt_enum_get_value_choices(pv_struct_ptr.clone());
            
            let count = ffi::nt_enum_get_value_choices_count(pv_struct_ptr.clone());
            if ptr.is_null() || count == 0 {
                Vec::new()
            } else {
                let choices = unsafe { std::slice::from_raw_parts(ptr, count) };
                choices.iter().map(|&choice| {
                    if choice.is_null() {
                        String::new()
                    } else {
                        unsafe { std::ffi::CStr::from_ptr(choice).to_string_lossy().into_owned() }
                    }
                }).collect::<Vec<_>>()
            }
        },
        alarm_severity: ffi::nt_enum_get_alarm_severity(pv_struct_ptr.clone()),
        alarm_status: ffi::nt_enum_get_alarm_status(pv_struct_ptr.clone()),
        alarm_message: {
            let ptr = ffi::nt_enum_get_alarm_message(pv_struct_ptr.clone());
            if ptr.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            }
        },
        timestamp_secondspastepoch: ffi::nt_enum_get_timestamp_seconds(pv_struct_ptr.clone()),
        timestamp_nanoseconds: ffi::nt_enum_get_timestamp_nanoseconds(pv_struct_ptr.clone()),
        timestamp_user_tag: ffi::nt_enum_get_timestamp_user_tag(pv_struct_ptr.clone()),
    })
}
