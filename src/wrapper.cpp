#include "wrapper.h"
#include <pva/client.h>
#include <sstream>


std::unique_ptr<ClientProvider> rust_client_provider = nullptr;  // Initialize the global variable
std::unique_ptr<ClientChannel> rust_client_channel = nullptr;  // Initialize the global variable

rust::String get_pv_value(rust::Str name) {
    try {
        std::string name_str(name);  // Convert Rust string to std::string

        pvac::ClientProvider provider("pva");
        pvac::ClientChannel channel(provider.connect(name_str));
        
        std::ostringstream result;
        result << channel.name() << " : " << channel.get();

        return rust::String(result.str());  // Convert std::string to rust::String
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());  // Concatenate using std::string
        return rust::String(error_msg);  // Convert back to rust::String
    }
}

std::unique_ptr<ClientProvider> get_client_provider() {
    try {
        if (!rust_client_provider) {
            rust_client_provider = std::make_unique<ClientProvider>("pva");
        }
        return std::move(rust_client_provider);
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientProvider: " << e.what() << std::endl;
        return nullptr;
    }
}

std::unique_ptr<ClientChannel> get_client_channel(rust::Str name) {
    try {
        if (!rust_client_provider) {
            rust_client_provider = get_client_provider();
        }
        std::string name_str(name);  // Convert Rust `&str` to C++ `std::string`
        auto channel = std::make_unique<ClientChannel>(rust_client_provider->connect(name_str));
        rust_client_channel = std::move(channel);
        return std::move(rust_client_channel);  
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientChannel: " << e.what() << std::endl;
        return nullptr;
    }
}

