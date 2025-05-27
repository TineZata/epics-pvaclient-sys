#include "wrapper.h"
#include "helpers.h"

/*
epics:nt/NTEnum:1.0
        enum_t value
            int index
            string[] choices
        alarm_t alarm
            int severity
            int status
            string message
        structure timeStamp
            long secondsPastEpoch
            int nanoseconds
            int userTag
*/

int nt_enum_get_value_index(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int value_index = -1; // Default value if not found
    
    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            // Extract the value index from the NTEnum structure
            value_index = enumWrapper->getValue()->getSubField<epics::pvData::PVInt>("index")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int(-1);
    }
    return value_index;
}

const char* const* nt_enum_get_value_choices(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* const* value_choices = nullptr;

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            auto choicesField = enumWrapper->getValue()->getSubField<epics::pvData::PVValueArray<std::string>>("choices");
            auto castedChoicesField = std::dynamic_pointer_cast<const epics::pvData::PVStringArray>(choicesField);
            if (castedChoicesField) {
                value_choices = extract_string_array(castedChoicesField).data();
            }
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return nullptr;
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return nullptr;
    }
    return value_choices;
}

size_t nt_enum_get_value_choices_count(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    size_t choices_count = 0;

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            auto choicesField = enumWrapper->getValue()->getSubField<epics::pvData::PVValueArray<std::string>>("choices");
            auto castedChoicesField = std::dynamic_pointer_cast<const epics::pvData::PVStringArray>(choicesField);
            if (castedChoicesField) {
                choices_count = castedChoicesField->getLength();
            }
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return size_t(0);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return size_t(0);
    }
    return choices_count;
}

int nt_enum_get_alarm_severity(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int severity = -1; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            severity = enumWrapper->getAlarm()->getSubField<epics::pvData::PVInt>("severity")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int(-1);
    }
    return severity;
}

int nt_enum_get_alarm_status(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int status = -1; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            status = enumWrapper->getAlarm()->getSubField<epics::pvData::PVInt>("status")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int(-1);
    }
    return status;
}

const char* nt_enum_get_alarm_message(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    const char* message = nullptr; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            message = enumWrapper->getAlarm()->getSubField<epics::pvData::PVString>("message")->get().c_str();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return nullptr;
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return nullptr;
    }
    return message;
}

int64_t nt_enum_get_timestamp_seconds(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int64_t seconds = -1; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            seconds = enumWrapper->getTimeStamp()->getSubField<epics::pvData::PVLong>("secondsPastEpoch")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int64_t(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int64_t(-1);
    }
    return seconds;
}

int32_t nt_enum_get_timestamp_nanoseconds(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t nanoseconds = -1; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            nanoseconds = enumWrapper->getTimeStamp()->getSubField<epics::pvData::PVInt>("nanoseconds")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int32_t(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int32_t(-1);
    }
    return nanoseconds;
}

int32_t nt_enum_get_timestamp_user_tag(std::shared_ptr<PVStructure> pvStructureSharedPtr) {
    int32_t user_tag = -1; // Default value if not found

    try {
        std::shared_ptr<EpicsNtNTEnum> enumWrapper = EpicsNtNTEnum::wrap(pvStructureSharedPtr);
        if (enumWrapper.get() != 0) {
            user_tag = enumWrapper->getTimeStamp()->getSubField<epics::pvData::PVInt>("userTag")->get();
        } else {
            std::cerr << "Invalid NTEnum structure." << std::endl;
            return int32_t(-1);
        }
    } catch (std::exception &e) {
        // Handle exceptions and clean up
        std::cerr << "Error extracting NTEnum: " << e.what() << std::endl;
        return int32_t(-1);
    }
    return user_tag;
}
