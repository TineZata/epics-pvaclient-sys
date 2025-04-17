use cxx::SharedPtr;

#[cxx::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("wrapper.h");

        // Declare opaque types for C++ classes
        type ClientProvider;
        type ClientChannel;

        fn get_client_provider() -> SharedPtr<ClientProvider>;
        fn get_client_channel(name: &str) -> SharedPtr<ClientChannel>;
        fn get_pv_value(name: &str) -> String;
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
/// let value = epics_pvaclient_sys::get_pv(&pv_name);
/// assert_ne!(value, "Error: Timeout");
/// ```
pub fn get_pv(name: &str) -> String {
    ffi::get_pv_value(name)
}
