#include "wrapper.h"
#include <sstream>

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

PvaClientChannel::PvaClientChannel(std::string name)
    : provider(std::make_unique<pvac::ClientProvider>("pva")),
      channel(std::make_unique<pvac::ClientChannel>(provider->connect(name))) {}

/*bool PvaClientChannel::connect() {
    try {
        // ✅ Ensure the provider remains valid while the channel is in use
        channel = std::make_unique<pvac::ClientChannel>(provider->connect(channel->name()));

        return true;  // ✅ Connection successful
    } catch (const std::exception& e) {
        std::cerr << "Error connecting to PV: " << e.what() << std::endl;
        return false;
    }
}*/

std::unique_ptr<PvaClientChannel> PvaClientChannel::create_connected(rust::Str name) {
    try {
        std::string name_str(name);  // Convert Rust `&str` to C++ `std::string`

        // ✅ Create the provider (so it stays alive with the channel)
        auto provider = std::make_unique<pvac::ClientProvider>("pva");

        // ✅ Create and connect the channel using `provider.connect()`
        auto channel = std::make_unique<pvac::ClientChannel>(provider->connect(name_str));

        // ✅ Return a new `PvaClientChannel` holding both provider and channel
        auto pva_channel = std::make_unique<PvaClientChannel>(name_str);
        pva_channel->provider = std::move(provider);
        pva_channel->channel = std::move(channel);

        return pva_channel;
    } catch (const std::exception& e) {
        std::cerr << "Error creating connected PvaClientChannel: " << e.what() << std::endl;
        return nullptr;
    }
}