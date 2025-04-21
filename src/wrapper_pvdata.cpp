#include "wrapper.h"

// Helper function to extract a string array
std::vector<const char *> extract_string_array(const std::shared_ptr<const epics::pvData::PVStringArray> &array) {
    std::vector<const char *> result;
    if (array) {
        epics::pvData::shared_vector<const std::string> data;
        array->getAs(data);
        for (const auto &str : data) {
            result.push_back(strdup(str.c_str()));
        }
    }
    return result;
}

// Extract NTScalar fields from a PVStructure
std::shared_ptr<NTScalar> get_pv_value_fields_as_struct(rust::Str name)
{
    std::string name_str(name);  // Convert Rust `&str` to C++ `std::string`
    if (!rust_client_provider)
    {
        rust_client_provider = get_client_provider(); // Assuming this function initializes the provider
    }
    if (!rust_client_channel)
    {
        rust_client_channel = get_client_channel(name_str); // Assuming this function initializes the channel
    }

    // Check if the global channel is initialized
    if (!rust_client_channel)
    {
        std::cerr << "ClientChannel is not initialized." << std::endl;
        return nullptr;
    }
    // Retrieve the shared pointer from rust_client_channel
    auto pvStructureSharedPtr = rust_client_channel->get();
    if (!pvStructureSharedPtr) {
        std::cerr << "Error: rust_client_channel->get() returned nullptr." << std::endl;
        return nullptr;
    }

    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        std::cerr << "Error: rust_client_channel->get().get() returned nullptr." << std::endl;
        return nullptr;
    }
    auto ntScalar = std::make_shared<NTScalar>();

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVDouble>("value");
        ntScalar->value = valueField ? valueField->get() : 0.0;

        // Extract alarm
        auto alarmField = pvStructure->getSubField<epics::pvData::PVStructure>("alarm");
        if (alarmField)
        {
            ntScalar->alarm_severity = alarmField->getSubField<epics::pvData::PVInt>("severity")->get();
            ntScalar->alarm_status = alarmField->getSubField<epics::pvData::PVInt>("status")->get();
            ntScalar->alarm_message = strdup(alarmField->getSubField<epics::pvData::PVString>("message")->get().c_str());
        }

        // Extract timestamp
        auto timestampField = pvStructure->getSubField<epics::pvData::PVStructure>("timeStamp");
        if (timestampField)
        {
            ntScalar->timestamp_seconds = timestampField->getSubField<epics::pvData::PVLong>("secondsPastEpoch")->get();
            ntScalar->timestamp_nanoseconds = timestampField->getSubField<epics::pvData::PVInt>("nanoseconds")->get();
            ntScalar->timestamp_userTag = timestampField->getSubField<epics::pvData::PVInt>("userTag")->get();
        }

        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField)
        {
            ntScalar->display_limitLow = displayField->getSubField<epics::pvData::PVDouble>("limitLow")->get();
            ntScalar->display_limitHigh = displayField->getSubField<epics::pvData::PVDouble>("limitHigh")->get();
            ntScalar->display_description = strdup(displayField->getSubField<epics::pvData::PVString>("description")->get().c_str());
            ntScalar->display_units = strdup(displayField->getSubField<epics::pvData::PVString>("units")->get().c_str());
            ntScalar->display_precision = displayField->getSubField<epics::pvData::PVInt>("precision")->get();

            auto formField = displayField->getSubField<epics::pvData::PVStructure>("form");
            if (formField) {
                ntScalar->display_form_index = formField->getSubField<epics::pvData::PVInt>("index")->get();
                auto choicesField = formField->getSubField<epics::pvData::PVValueArray<std::string>>("choices");
                auto castedChoicesField = std::dynamic_pointer_cast<const epics::pvData::PVStringArray>(choicesField);
                if (castedChoicesField) {
                    auto choices = extract_string_array(castedChoicesField);
                    ntScalar->display_form_choices_count = choices.size();
                    ntScalar->display_form_choices = new const char *[choices.size()];
                    std::copy(choices.begin(), choices.end(), ntScalar->display_form_choices);
                }
            }
        }

        // Extract control
        auto controlField = pvStructure->getSubField<epics::pvData::PVStructure>("control");
        if (controlField)
        {
            ntScalar->control_limitLow = controlField->getSubField<epics::pvData::PVDouble>("limitLow")->get();
            ntScalar->control_limitHigh = controlField->getSubField<epics::pvData::PVDouble>("limitHigh")->get();
            ntScalar->control_minStep = controlField->getSubField<epics::pvData::PVDouble>("minStep")->get();
        }

        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField)
        {
            ntScalar->valueAlarm_active = valueAlarmField->getSubField<epics::pvData::PVBoolean>("active")->get();
            ntScalar->valueAlarm_lowAlarmLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("lowAlarmLimit")->get();
            ntScalar->valueAlarm_lowWarningLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("lowWarningLimit")->get();
            ntScalar->valueAlarm_highWarningLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("highWarningLimit")->get();
            ntScalar->value_alarm_high_alarm_limit = valueAlarmField->getSubField<epics::pvData::PVDouble>("highAlarmLimit")->get();
            ntScalar->value_alarm_low_alarm_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("lowAlarmSeverity")->get();
            ntScalar->value_alarm_low_warning_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("lowWarningSeverity")->get();
            ntScalar->value_alarm_high_warning_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("highWarningSeverity")->get();
            ntScalar->value_alarm_high_alarm_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("highAlarmSeverity")->get();
            ntScalar->value_alarm_hysteresis = valueAlarmField->getSubField<epics::pvData::PVByte>("hysteresis")->get();
        }
    }
    catch (std::exception &e)
    {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return nullptr;
    }

    return ntScalar;
}
