#include "wrapper.h"

// Helper function to extract a string array
std::vector<const char *> extract_string_array(const std::shared_ptr<const epics::pvData::PVStringArray> &array) {
    std::vector<const char *> result;
    if (array) {
        epics::pvData::shared_vector<const std::string> data;
        array->getAs(data);
        for (const auto &str : data) {
            result.push_back(_strdup(str.c_str()));
        }
    }
    return result;
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

bool nt_scalar_get_value_boolean(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    bool value = false;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return false;
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVBoolean>("value");
        value = valueField ? valueField->get() : false;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return false;
    }
    return value;
}

int8_t nt_scalar_get_value_byte(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int8_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int8_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVByte>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int8_t(0);
    }
    return value;
}

int16_t nt_scalar_get_value_short(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int16_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int16_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVShort>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int16_t(0);
    }
    return value;
}

int32_t nt_scalar_get_value_int(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVInt>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return value;
}

int64_t nt_scalar_get_value_long(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int64_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int64_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVLong>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int64_t(0);
    }
    return value;
}

uint8_t nt_scalar_get_value_unsigned_byte(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    uint8_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return uint8_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVUByte>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return uint8_t(0);
    }
    return value;
}

uint16_t nt_scalar_get_value_unsigned_short(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    uint16_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return uint16_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVUShort>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return uint16_t(0);
    }
    return value;
}

uint32_t nt_scalar_get_value_unsigned_int(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    uint32_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return uint32_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVUInt>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return uint32_t(0);
    }
    return value;
}

uint64_t nt_scalar_get_value_unsigned_long(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    uint64_t value = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return uint64_t(0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVULong>("value");
        value = valueField ? valueField->get() : 0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return uint64_t(0);
    }
    return value;
}

float nt_scalar_get_value_float(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    float value = 0.0f;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return float(0.0f);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVFloat>("value");
        value = valueField ? valueField->get() : 0.0f;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return float(0.0f);
    }
    return value;
}

double nt_scalar_get_value_double(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double value = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVDouble>("value");
        value = valueField ? valueField->get() : 0.0;
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
     }

    return value;
}

const char* nt_scalar_get_value_string(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* value = "";
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return "";
    }

    try {
        // Extract value
        auto valueField = pvStructure->getSubField<epics::pvData::PVString>("value");
        if (valueField) {
            // Use strdup to allocate memory for the C-style string
            value = _strdup(valueField->get().c_str());
        }
     } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return "";
    }
    return value;
}

int nt_scalar_get_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int alarm_severity = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int(0);
    }
    try {
        // Extract alarm
        auto alarmField = pvStructure->getSubField<epics::pvData::PVStructure>("alarm");
        if (alarmField) {
            alarm_severity = alarmField->getSubField<epics::pvData::PVInt>("severity")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int(0);
    }
    return alarm_severity;
}

int nt_scalar_get_alarm_status(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int alarm_status = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int(0);
    }
    try {
        // Extract alarm
        auto alarmField = pvStructure->getSubField<epics::pvData::PVStructure>("alarm");
        if (alarmField) {
            alarm_status = alarmField->getSubField<epics::pvData::PVInt>("status")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int(0);
    }
    return alarm_status;
}

const char* nt_scalar_get_alarm_message(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* alarm_message = nullptr;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return nullptr;
    }
    try {
        // Extract alarm
        auto alarmField = pvStructure->getSubField<epics::pvData::PVStructure>("alarm");
        if (alarmField) {
            alarm_message = _strdup(alarmField->getSubField<epics::pvData::PVString>("message")->get().c_str());
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return nullptr;
    }
    return alarm_message;
}

int64_t nt_scalar_get_timestamp_seconds(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int64_t timestamp_seconds = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int64_t(0);
    }
    try {
        // Extract timestamp
        auto timestampField = pvStructure->getSubField<epics::pvData::PVStructure>("timeStamp");
        if (timestampField) {
            timestamp_seconds = timestampField->getSubField<epics::pvData::PVLong>("secondsPastEpoch")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int64_t(0);
    }
    return timestamp_seconds;
}

int32_t nt_scalar_get_timestamp_nanoseconds(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t timestamp_nanoseconds = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract timestamp
        auto timestampField = pvStructure->getSubField<epics::pvData::PVStructure>("timeStamp");
        if (timestampField) {
            timestamp_nanoseconds = timestampField->getSubField<epics::pvData::PVInt>("nanoseconds")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return timestamp_nanoseconds;
}

int32_t nt_scalar_get_timestamp_user_tag(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t timestamp_user_tag = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract timestamp
        auto timestampField = pvStructure->getSubField<epics::pvData::PVStructure>("timeStamp");
        if (timestampField) {
            timestamp_user_tag = timestampField->getSubField<epics::pvData::PVInt>("userTag")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return timestamp_user_tag;
}

double nt_scalar_get_display_limit_low(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double display_limitLow = 0.0;
    // Retrieve the raw pointer from the shared pointer 
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            display_limitLow = displayField->getSubField<epics::pvData::PVDouble>("limitLow")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return display_limitLow;
}

double nt_scalar_get_display_limit_high(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double display_limitHigh = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            display_limitHigh = displayField->getSubField<epics::pvData::PVDouble>("limitHigh")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return display_limitHigh;
}

const char* nt_scalar_get_display_description(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* display_description = nullptr;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return nullptr;
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            display_description = _strdup(displayField->getSubField<epics::pvData::PVString>("description")->get().c_str());
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return nullptr;
    }
    return display_description;
}

const char* nt_scalar_get_display_units(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* display_units = nullptr;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return nullptr;
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            display_units = _strdup(displayField->getSubField<epics::pvData::PVString>("units")->get().c_str());
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return nullptr;
    }
    return display_units;
}

int32_t nt_scalar_get_display_precision(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t display_precision = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            display_precision = displayField->getSubField<epics::pvData::PVInt>("precision")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return display_precision;
}

int32_t nt_scalar_get_display_form_index(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t display_form_index = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            auto formField = displayField->getSubField<epics::pvData::PVStructure>("form");
            if (formField) {
                display_form_index = formField->getSubField<epics::pvData::PVInt>("index")->get();
            }
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return display_form_index;
}

const char* const* nt_scalar_get_display_form_choices(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* const* display_form_choices = nullptr;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return nullptr;
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            auto formField = displayField->getSubField<epics::pvData::PVStructure>("form");
            if (formField) {
                auto choicesField = formField->getSubField<epics::pvData::PVValueArray<std::string>>("choices");
                auto castedChoicesField = std::dynamic_pointer_cast<const epics::pvData::PVStringArray>(choicesField);
                if (castedChoicesField) {
                    display_form_choices = extract_string_array(castedChoicesField).data();
                }
            }
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return nullptr;
    }
    return display_form_choices;
}

size_t nt_scalar_get_display_form_choices_count(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    size_t display_form_choices_count = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return size_t(0);
    }
    try {
        // Extract display
        auto displayField = pvStructure->getSubField<epics::pvData::PVStructure>("display");
        if (displayField) {
            auto formField = displayField->getSubField<epics::pvData::PVStructure>("form");
            if (formField) {
                auto choicesField = formField->getSubField<epics::pvData::PVValueArray<std::string>>("choices");
                auto castedChoicesField = std::dynamic_pointer_cast<const epics::pvData::PVStringArray>(choicesField);
                if (castedChoicesField) {
                    display_form_choices_count = castedChoicesField->getLength();
                }
            }
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return size_t(0);
    }
    return display_form_choices_count;
}

double nt_scalar_get_control_limit_low(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double control_limitLow = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract control
        auto controlField = pvStructure->getSubField<epics::pvData::PVStructure>("control");
        if (controlField) {
            control_limitLow = controlField->getSubField<epics::pvData::PVDouble>("limitLow")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return control_limitLow;
}

double nt_scalar_get_control_limit_high(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double control_limitHigh = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract control
        auto controlField = pvStructure->getSubField<epics::pvData::PVStructure>("control");
        if (controlField) {
            control_limitHigh = controlField->getSubField<epics::pvData::PVDouble>("limitHigh")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return control_limitHigh;
}

double nt_scalar_get_control_min_step(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double control_minStep = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract control
        auto controlField = pvStructure->getSubField<epics::pvData::PVStructure>("control");
        if (controlField) {
            control_minStep = controlField->getSubField<epics::pvData::PVDouble>("minStep")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return control_minStep;
}

bool nt_scalar_get_value_alarm_active(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    bool valueAlarm_active = false;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return false;
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_active = valueAlarmField->getSubField<epics::pvData::PVBoolean>("active")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return false;
    }
    return valueAlarm_active;
}

double nt_scalar_get_value_alarm_low_alarm_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double valueAlarm_lowAlarmLimit = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_lowAlarmLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("lowAlarmLimit")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return valueAlarm_lowAlarmLimit;
}

double nt_scalar_get_value_alarm_low_warning_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double valueAlarm_lowWarningLimit = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_lowWarningLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("lowWarningLimit")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return valueAlarm_lowWarningLimit;
}

double nt_scalar_get_value_alarm_high_warning_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double valueAlarm_highWarningLimit = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_highWarningLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("highWarningLimit")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return valueAlarm_highWarningLimit;
}

double nt_scalar_get_value_alarm_high_alarm_limit(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    double valueAlarm_highAlarmLimit = 0.0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return double(0.0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_highAlarmLimit = valueAlarmField->getSubField<epics::pvData::PVDouble>("highAlarmLimit")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return double(0.0);
    }
    return valueAlarm_highAlarmLimit;
}

int32_t nt_scalar_get_value_alarm_low_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t valueAlarm_low_alarm_severity = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_low_alarm_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("lowAlarmSeverity")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
}

int32_t nt_scalar_get_value_alarm_low_warning_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t valueAlarm_low_warning_severity = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_low_warning_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("lowWarningSeverity")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return valueAlarm_low_warning_severity;
}

int32_t nt_scalar_get_value_alarm_high_warning_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t valueAlarm_high_warning_severity = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_high_warning_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("highWarningSeverity")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return valueAlarm_high_warning_severity;
}

int32_t nt_scalar_get_value_alarm_high_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t valueAlarm_high_alarm_severity = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int32_t(0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_high_alarm_severity = valueAlarmField->getSubField<epics::pvData::PVInt>("highAlarmSeverity")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int32_t(0);
    }
    return valueAlarm_high_alarm_severity;
}

int8_t nt_scalar_get_value_alarm_hysteresis(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int8_t valueAlarm_hysteresis = 0;
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return int8_t(0);
    }
    try {
        // Extract valueAlarm
        auto valueAlarmField = pvStructure->getSubField<epics::pvData::PVStructure>("valueAlarm");
        if (valueAlarmField) {
            valueAlarm_hysteresis = valueAlarmField->getSubField<epics::pvData::PVByte>("hysteresis")->get();
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return int8_t(0);
    }
    return valueAlarm_hysteresis;
}

NTTypes nt_scalar_get_value_type(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    // Retrieve the raw pointer from the shared pointer
    const epics::pvData::PVStructure* pvStructure = pvStructureSharedPtr.get();
    if (!pvStructure) {
        return -1;
    }
    try {
        auto pvField = pvStructure->getSubField<epics::pvData::PVField>("value");
        if (!pvField) {
            return -1;
        }

        switch (pvField->getField()->getType()) {
        case epics::pvData::scalarType:
            return (NTTypes)pvField->getScalar()->getScalarType();
        case epics::pvData::scalarArrayType:
            // bit shift the scalar type to get the array type
            return (NTTypes) (0x10 + (uint8_t)pvField->getScalarArray()->getElementType();
        case epics::pvData::structureType:
            return NTTypes::NTEnum;
        case epics::pvData::structureArrayType:
            return NTTypes::NTEnumArray;
        case epics::pvData::unionType:  
            return NTTypes::NTUnion;
        case epics::pvData::unionArrayType:
            return NTTypes::NTUnionArray;
        default:
            return -1;
        }

    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTScalar: " << e.what() << std::endl;
        return -1;
    }
    return -1;
}
