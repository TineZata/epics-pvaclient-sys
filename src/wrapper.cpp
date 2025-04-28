#include "wrapper.h"
#include <sstream>


std::shared_ptr<ClientProvider> rust_client_provider = nullptr;  // Initialize the global variable
std::shared_ptr<ClientChannel> rust_client_channel = nullptr;  // Initialize the global variable

std::shared_ptr<ClientProvider> get_client_provider() {
    try {
        
        if (!rust_client_provider) {
            rust_client_provider = std::make_shared<ClientProvider>("pva");
        }
        return rust_client_provider;
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientConfig: " << e.what() << std::endl;
        return nullptr;
    }
}


std::shared_ptr<ClientChannel> get_client_channel(rust::Str name) {
    try {
        if (!rust_client_provider) {
            rust_client_provider = get_client_provider();
        }
        std::string name_str(name);  // Convert Rust `&str` to C++ `std::string`
        auto channel = std::make_shared<ClientChannel>(rust_client_provider->connect(name_str));
        rust_client_channel = channel;
        return rust_client_channel;  
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientChannel: " << e.what() << std::endl;
        return nullptr;
    }
}

rust::String get_pv_value_fields_as_string(rust::Str name) {
    try {
        std::string name_str(name);  // Convert Rust string to std::string

        /*pvac::ClientProvider provider("pva");
        pvac::ClientChannel channel(provider.connect(name_str));*/
        if (!rust_client_provider) {
            rust_client_provider = get_client_provider();
        }
        if (!rust_client_channel) {
            rust_client_channel = get_client_channel(name_str);  // Use the global variable
        }
        
        std::ostringstream result;
        result << rust_client_channel->name() << " : " << rust_client_channel->get();

        return rust::String(result.str());  // Convert std::string to rust::String
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());  // Concatenate using std::string
        return rust::String(error_msg);  // Convert back to rust::String
    }
}
