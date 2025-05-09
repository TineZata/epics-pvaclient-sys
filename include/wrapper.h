#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>
#include <pv/pvData.h>
#include <vector>
#include <memory>

using ClientProvider = pvac::ClientProvider;
using ClientChannel = pvac::ClientChannel;
using PVStructure = epics::pvData::PVStructure;

// Global map to store ClientProvider instances
extern std::shared_ptr<ClientProvider> client_provider; 
// List of all ClientChannel instances
extern std::vector<std::shared_ptr<ClientChannel>> client_channels;

// Function to create a ClientProvider
std::shared_ptr<ClientProvider> get_client_provider();
// Function to create a ClientConfig
//std::shared_ptr<EpicsPvaConfig> get_client_config();
// Function to create a ClientProvider
std::shared_ptr<ClientChannel> get_client_channel(rust::Str name);
// Function to get all PV value fields as a string
rust::String get_pv_value_fields_as_string(rust::Str name);

// Enum for scalar types
enum ScalarType {
    pvBoolean,
    pvByte,
    pvShort,
    pvInt,
    pvLong,
    pvUByte,
    pvUShort,
    pvUInt,
    pvULong,
    pvFloat,
    pvDouble,
    pvString,
};


std::shared_ptr<PVStructure> get_pv_value_fields_as_struct(rust::Str name);

extern "C" {
    bool nt_scalar_get_value_boolean(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int8_t nt_scalar_get_value_byte(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int16_t nt_scalar_get_value_short(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_value_int(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int64_t nt_scalar_get_value_long(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    uint8_t nt_scalar_get_value_unsigned_byte(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    uint16_t nt_scalar_get_value_unsigned_short(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    uint32_t nt_scalar_get_value_unsigned_int(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    uint64_t nt_scalar_get_value_unsigned_long(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    float nt_scalar_get_value_float(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_value_double(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    const char* nt_scalar_get_value_string(std::shared_ptr<PVStructure> pvStructureSharedPtr);

    int nt_scalar_get_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int nt_scalar_get_alarm_status(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    const char* nt_scalar_get_alarm_message(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int64_t nt_scalar_get_timestamp_seconds(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_timestamp_nanoseconds(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_timestamp_user_tag(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_display_limit_low(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_display_limit_high(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    const char* nt_scalar_get_display_description(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    const char* nt_scalar_get_display_units(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_display_precision(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_display_form_index(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    const char* const* nt_scalar_get_display_form_choices(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    size_t nt_scalar_get_display_form_choices_count(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_control_limit_low(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_control_limit_high(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_control_min_step(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    bool nt_scalar_get_value_alarm_active(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_value_alarm_low_alarm_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_value_alarm_low_warning_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_value_alarm_high_warning_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    double nt_scalar_get_value_alarm_high_alarm_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_value_alarm_low_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_value_alarm_low_warning_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_value_alarm_high_warning_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int32_t nt_scalar_get_value_alarm_high_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr);
    int8_t nt_scalar_get_value_alarm_hysteresis(std::shared_ptr<PVStructure> pvStructureSharedPtr);

    int8_t nt_scalar_get_value_type(std::shared_ptr<PVStructure> pvStructureSharedPtr);
}

