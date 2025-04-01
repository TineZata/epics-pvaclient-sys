#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>

// Bring pvac types into the global namespace for compatibility with cxx
using ClientProvider = pvac::ClientProvider;
using ClientChannel = pvac::ClientChannel;

// Global map to store ClientProvider instances
extern std::unique_ptr<ClientProvider> rust_client_provider; 
// Global map to store ClientChannel instances
extern std::unique_ptr<ClientChannel> rust_client_channel; 

// Function to create a ClientProvider
std::unique_ptr<ClientProvider> get_client_provider();
// Function to create a ClientProvider
std::unique_ptr<ClientChannel> get_client_channel(rust::Str name);
// Function to get a PV value
rust::String get_pv_value(rust::Str name);
