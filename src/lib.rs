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

pub fn get_client_provider() -> SharedPtr<ffi::ClientProvider> {
    ffi::get_client_provider()
}

pub fn get_client_channel(name: &str) -> SharedPtr<ffi::ClientChannel> {
    ffi::get_client_channel(name)
}

/// Get a PV value as a Rust string.
pub fn get_pv(name: &str) -> String {
    ffi::get_pv_value(name)
}
