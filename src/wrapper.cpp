#include "wrapper.h"
#include <sstream>

std::shared_ptr<ClientProvider> get_client_provider() {
    try { 
        return std::make_shared<ClientProvider>("pva");
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientConfig: " << e.what() << std::endl;
        return nullptr;
    }
}


std::shared_ptr<ClientChannel> get_client_channel(rust::Str name) {
    try {
        std::shared_ptr<ClientProvider> cp = get_client_provider();
        std::string name_str(name);  // Convert Rust `&str` to C++ `std::string`
        auto channel = std::make_shared<ClientChannel>(cp->connect(name_str));
        return channel;  
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientChannel: " << e.what() << std::endl;
        return nullptr;
    }
}

rust::String get_pv_value_fields_as_string(rust::Str name) {
    try {
        std::string name_str(name);  // Convert Rust string to std::string
        std::shared_ptr<ClientChannel> channel = get_client_channel(name_str); 
        if (!channel) {
            std::cerr << "ClientChannel is not initialized " << name << std::endl;
            return rust::String("Error: ClientChannel is not initialized.");
        }
        std::ostringstream result;
        result << channel->name() << " : " << channel->get(3.0, nullptr);
        return rust::String(result.str());  // Convert std::string to rust::String
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());  // Concatenate using std::string
        return rust::String(error_msg);  // Convert back to rust::String
    }
}
