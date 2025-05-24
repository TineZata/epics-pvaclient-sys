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
