#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>

rust::String get_pv_value(rust::Str name);

class PvaClientChannel {
    public:
        // Constructor
        PvaClientChannel(std::string name);
    
        // Connect method
        //bool connect();
    
        // Factory method to create and return a connected channel
        static std::unique_ptr<PvaClientChannel> create_connected(rust::Str name);
    
    private:
        std::unique_ptr<pvac::ClientProvider> provider;  // Store the provider
        std::unique_ptr<pvac::ClientChannel> channel;    // Store the channel
    };
