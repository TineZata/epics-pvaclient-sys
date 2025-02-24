#pragma once
#include <string>
#include <rust/cxx.h>  // Required for Rust interop
#include <pva/client.h>

rust::String get_pv_value(rust::Str name);
