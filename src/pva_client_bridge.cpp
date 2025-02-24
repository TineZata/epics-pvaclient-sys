#include "pva_client_bridge.h"
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
