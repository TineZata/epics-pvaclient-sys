#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>
#include <pv/pvData.h>
#include <string>
#include <vector>
#include <memory>

// Bring pvac types into the global namespace for compatibility with cxx
using ClientProvider = pvac::ClientProvider;
using ClientChannel = pvac::ClientChannel;

// Opaque handle for PVStructure
using PVStructure = epics::pvData::PVStructure;

// Struct to hold the packed NTScalar fields
struct NTScalar {
    double value;
    int32_t alarm_severity;
    int32_t alarm_status;
    const char* alarm_message = nullptr;
    int64_t timestamp_seconds;
    int32_t timestamp_nanoseconds;
    int32_t timestamp_userTag;
    double display_limitLow;
    double display_limitHigh;
    const char* display_description = nullptr;
    const char* display_units = nullptr;
    int32_t display_precision;
    int32_t display_form_index;
    const char** display_form_choices = nullptr;
    size_t display_form_choices_count = 0;
    double control_limitLow;
    double control_limitHigh;
    double control_minStep;
    bool valueAlarm_active;
    double valueAlarm_lowAlarmLimit;
    double valueAlarm_lowWarningLimit;
    double valueAlarm_highWarningLimit;
    double value_alarm_high_alarm_limit;
    int32_t value_alarm_low_alarm_severity;
    int32_t value_alarm_low_warning_severity;
    int32_t value_alarm_high_warning_severity;
    int32_t value_alarm_high_alarm_severity;
    int8_t value_alarm_hysteresis;

    // Destructor to clean up dynamically allocated memory
    ~NTScalar() {
        delete alarm_message;
        delete display_description;
        delete display_units;
        if (display_form_choices) {
            for (size_t i = 0; i < display_form_choices_count; ++i) {
                delete display_form_choices[i];
            }
            delete[] display_form_choices;
        }
    }
};

// Bring epics types into the global namespace for compatibility with cxx
//using EpicsPvaConfig = epics::pvAccess::Config;

// Global map to store ClientProvider instances
extern std::shared_ptr<ClientProvider> rust_client_provider; 
// Global map to store ClientChannel instances
extern std::shared_ptr<ClientChannel> rust_client_channel; 

// Function to create a ClientProvider
std::shared_ptr<ClientProvider> get_client_provider();
// Function to create a ClientConfig
//std::shared_ptr<EpicsPvaConfig> get_client_config();
// Function to create a ClientProvider
std::shared_ptr<ClientChannel> get_client_channel(rust::Str name);
// Function to get all PV value fields as a string
rust::String get_pv_value_fields_as_string(rust::Str name);
// Function for extract_ntscalar
std::shared_ptr<NTScalar> get_pv_value_fields_as_struct(rust::Str name);
