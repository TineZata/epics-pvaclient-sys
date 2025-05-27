#include "wrapper.h"
#include "helpers.h"
#include <sstream>

// initialize the global variables
std::shared_ptr<ClientProvider> client_provider = nullptr; // Global variable for ClientProvider
std::vector<std::shared_ptr<ClientChannel>> client_channels; // Global vector for ClientChannel instances

std::shared_ptr<ClientProvider> get_client_provider() {
    try { 
        if (client_provider) {
            return client_provider;  // Return existing client provider if it exists
        }
        // Create a new ClientProvider instance
        client_provider = std::make_shared<ClientProvider>("pva");
        // Check if the client provider is valid
        if (!client_provider) {
            std::cerr << "Failed to create ClientProvider." << std::endl;
            return nullptr;
        }
        else {
            return client_provider;  // Return the existing client provider
        }
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientConfig: " << e.what() << std::endl;
        return nullptr;
    }
}


std::shared_ptr<ClientChannel> get_client_channel(rust::Str name) {
    try {
        // Convert Rust `&str` (rust::Str) to C++ `std::string`
        std::string name_str(name);

        // Check is channel is in the list of channels
        for (const std::shared_ptr<ClientChannel>& channel : client_channels) {
            if (channel->name() == name_str) {
                return channel;  // Return existing channel if found
            }
        }
        // If not found, create a new channel and add it to the list
        client_provider = get_client_provider(); // Ensure client provider is initialized
        std::shared_ptr<ClientChannel> channel = std::make_shared<ClientChannel>(client_provider->connect(name_str));
        // If channel is valid, add it to the list
        if (channel) {
            client_channels.push_back(channel);  // Store the channel in the global list
        } else {
            std::cerr << "Failed to create ClientChannel." << std::endl;
        }
        return channel;  
    } catch (const std::exception& e) {
        std::cerr << "Error creating ClientChannel: " << e.what() << std::endl;
        return nullptr;
    }
}

rust::String get_pv_value_fields_as_string(rust::Str name) {
    try {
        // Convert Rust `&str` (rust::Str) to C++ `std::string`
        std::string name_str(name);
        // Get the channel using the provided name
        auto channel = get_client_channel(name);
        if (!channel) {
            std::cerr << "ClientChannel is not initialized." << std::endl;
            return rust::String("Error: ClientChannel is not valid.");
        }else {
            std::ostringstream result;
            result << name_str << " : " << channel->get(3.0, nullptr);
            return rust::String(result.str());  // Convert std::string to rust::String
        }
    } catch (const std::exception& e) {
        std::string error_msg = "Error: " + std::string(e.what());  // Concatenate using std::string
        return rust::String(error_msg);  // Convert back to rust::String
    }
}

std::shared_ptr<PVStructure> get_pv_value_fields_as_struct(rust::Str name) {
    // Get the channel using the provided name
    auto channel = get_client_channel(name);
    // Check if the channel is initialized
    if (!channel) {
        std::cerr << "ClientChannel is not valid." << std::endl;
        return nullptr;
    }

    // Retrieve the shared pointer from rust_client_channel
    std::shared_ptr<const PVStructure> pvStructureSharedPtr;
    try {
        pvStructureSharedPtr = channel->get(3.0, nullptr);
    } catch (const std::exception &e) {
        std::cerr << "Error getting : channel->get() " << channel->name() << " " << e.what() << std::endl;
        return nullptr;
    }
    return std::const_pointer_cast<PVStructure>(pvStructureSharedPtr);
}

int8_t get_pv_field_data_type(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return -1;
    }
    try {
        auto pvField = pvStructure->getSubField<epics::pvData::PVField>("value");
        return (int8_t)(pvField->getField()->getType());
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return -1;
    }
    return -1;
}
