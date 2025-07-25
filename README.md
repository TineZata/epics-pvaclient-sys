# epics-pvaclient-sys

**epics-pvaclient-sys** is a low-level Rust FFI crate that provides direct bindings to the [EPICS pvAccess and pvData](https://epics-controls.org/resources-and-support/documents/pvaccess/). It acts as the bridge layer between EPICS 7’s powerful network and data manipulation API written in C++ and modern Rust-based control system tooling.

This crate is the foundation for a safe, idiomatic Rust API for interacting with EPICS pvAccess and pvData, allowing C++ interoperability to be tightly controlled and safely isolated.

## Features
- Compiles and links directly against [EPICS Base 7](https://github.com/epics-base/epics-base) libraries using [`cxx`](https://docs.rs/cxx), providing a static and predictable integration with EPICS libraries.

- Interfaces with the following EPICS components: pvaClient, pvAccess, pvData, nt, ca, and Com.

- Encapsulates all interaction with unsafe and unmanaged C++ code within this crate.

- Designed to expose only safe, idiomatic Rust interfaces to downstream crates.

- Ideal for building high-level Rust APIs and tools for distributed control systems and scientific instrumentation.

## Build Tooling

This crate uses the following crates to bridge Rust and EPICS C++ code:

- [`cxx`](https://docs.rs/cxx): Enables safe, idiomatic interoperability between Rust and C++. It generates bindings based on `#[cxx::bridge]` definitions.
- [`cxx-build`](https://docs.rs/cxx-build): Used in `build.rs` to compile and link C++ source files as part of the crate build process.
- [`cc`](https://docs.rs/cc): Used for compiling additional C/C++ sources that fall outside of `cxx`'s build scope (e.g. static helper libraries or shims).

## Requirements
- **EPICS Base 7** must be installed and built on your system.

- The environment variable `EPICS_BASE` must be set and included in your `PATH` and build environment.

- Currently supports **Windows x64 (MSVC)** targets.

- **Linux support is planned** in a future release.

## Status
This crate is intended for use as an internal dependency of a future `epics-pvaclient` crate, which will provide a safe and ergonomic API for application developers. It is not designed for standalone use.

The API and build system are actively evolving and may change as the project matures.
