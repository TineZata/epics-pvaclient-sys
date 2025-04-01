#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>


// Global map to store ClientProvider instances
extern std::unique_ptr<pvac::ClientProvider> rust_client_provider; 
// Global map to store ClientChannel instances
extern std::unique_ptr<pvac::ClientChannel> rust_client_channel; 

// Function to create a ClientProvider
std::unique_ptr<pvac::ClientProvider> get_client_provider();
// Function to create a ClientProvider
std::unique_ptr<pvac::ClientChannel> get_client_channel(rust::Str name);
// Function to get a PV value
rust::String get_pv_value(rust::Str name);
