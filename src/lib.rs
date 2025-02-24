#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("pva_client_bridge.h");

        fn get_pv_value(name: &str) -> String;
    }
}

/// Get a PV value as a Rust string.
pub fn get_pv(name: &str) -> String {
    ffi::get_pv_value(name)
}
