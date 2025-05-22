#include "helpers.h"

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

