fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/pva_client_bridge.cpp")
        .flag_if_supported("-std=c++17")
        .include("include")
        .include("D:/repos/3rdParty/epics-base/include")
        .include("D:/repos/3rdParty/epics-base/include/compiler/msvc")
        .include("D:/repos/3rdParty/epics-base/include/os/WIN32")
        .compile("epics_pvaclient_sys");

    println!("cargo:rustc-link-search=native=D:/repos/3rdParty/epics-base/lib/windows-x64");

    // EPICS libraries that must be linked
    println!("cargo:rustc-link-lib=static=pvaClient");
    println!("cargo:rustc-link-lib=static=pvAccess");
    println!("cargo:rustc-link-lib=static=pvData"); // This is likely missing
    println!("cargo:rustc-link-lib=static=ca");
    println!("cargo:rustc-link-lib=static=Com");
}
