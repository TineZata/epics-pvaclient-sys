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
    const char* alarm_message;
    int64_t timestamp_seconds;
    int32_t timestamp_nanoseconds;
    int32_t timestamp_userTag;
    double display_limitLow;
    double display_limitHigh;
    const char* display_description;
    const char* display_units;
    int32_t display_precision;
    int32_t display_form_index;
    const char** display_form_choices;
    size_t display_form_choices_count;
    double control_limitLow;
    double control_limitHigh;
    double control_minStep;
    bool valueAlarm_active;
    double valueAlarm_lowAlarmLimit;
    double valueAlarm_lowWarningLimit;
    double valueAlarm_highWarningLimit;
    double valueAlarm_highAlarmLimit;
    int32_t valueAlarm_lowAlarmSeverity;
    int32_t valueAlarm_lowWarningSeverity;
    int32_t valueAlarm_highWarningSeverity;
    int32_t valueAlarm_highAlarmSeverity;
    int8_t valueAlarm_hysteresis;
};

// Bring epics types into the global namespace for compatibility with cxx
//using EpicsPvaConfig = epics::pvAccess::Config;

// Global map to store ClientProvider instances
extern std::shared_ptr<ClientProvider> rust_client_provider; 
// Global map to store ClientChannel instances
extern std::shared_ptr<ClientChannel> rust_client_channel; 

// Function to create a ClientProvider
std::shared_ptr<ClientProvider> get_client_provider();
// Function to create a ClientProvider with configuration
std::shared_ptr<ClientProvider> get_client_provider_with_config();
// Function to create a ClientConfig
//std::shared_ptr<EpicsPvaConfig> get_client_config();
// Function to create a ClientProvider
std::shared_ptr<ClientChannel> get_client_channel(rust::Str name);
// Function to get a PV value
rust::String get_pv_value(rust::Str name);
// Function for extract_ntscalar
std::shared_ptr<NTScalar> extract_ntscalar(std::shared_ptr<PVStructure> pvStructure);
