fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++23")
        .flag("-Wno-pedantic")
        .flag("-Wno-class-memaccess")
        .flag("-Wno-type-limits")
        .flag("-Wno-ignored-attributes")
        .flag("-Wno-sign-compare")
        .flag("-Wno-maybe-uninitialized")
        .file("third_party/ExETC1/Etc1.cpp")
        .file("c_src/etc1_ffi.cpp")
        .include("third_party/ExETC1")
        .compile("etc1_wrapper");

    println!("cargo:rerun-if-changed=third_party/ExETC1/Etc1.cpp");
    println!("cargo:rerun-if-changed=third_party/ExETC1/Etc1.h");
    println!("cargo:rerun-if-changed=c_src/etc1_ffi.cpp");
}
