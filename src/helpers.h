#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <vector>
#include <memory>
#include <pv/pvData.h>

std::vector<const char *> extract_string_array(const std::shared_ptr<const epics::pvData::PVStringArray> &array);
