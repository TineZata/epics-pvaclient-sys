use std::env;

fn main() {

    let epics_base = env::var("EPICS_BASE").expect("EPICS_BASE environment variable not set");

    cxx_build::bridge("src/lib.rs")
        .file("src/helpers.cpp")
        .file("src/wrapper_nt_enum.cpp")
        .file("src/wrapper_nt_scalar.cpp")
        .file("src/wrapper.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/EHsc") // Enable exception handling for MSVC
        .include("include")
        .include(format!("{}/include", epics_base))
        .include(format!("{}/include/compiler/msvc", epics_base))
        .include(format!("{}/include/os/WIN32", epics_base))
        .compile("epics_pvaclient_sys");

        println!("cargo:rustc-link-search=native={}/lib/windows-x64", epics_base);

    // EPICS libraries that must be linked
    println!("cargo:rustc-link-lib=static=pvaClient");
    println!("cargo:rustc-link-lib=static=pvAccess");
    println!("cargo:rustc-link-lib=static=pvData"); // This is likely missing
    println!("cargo:rustc-link-lib=static=ca");
    println!("cargo:rustc-link-lib=static=Com");
}
