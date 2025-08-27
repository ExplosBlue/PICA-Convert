fn main() {
    cc::Build::new()
        .cpp(true)
        .file("rg_etc1/rg_etc1.cpp")
        .file("rg_etc1/wrapper.cpp")
        .include("rg_etc1")
        .compile("rg_etc1_wrapper");

    println!("cargo:rerun-if-changed=rg_etc1/rg_etc1.cpp");
    println!("cargo:rerun-if-changed=rg_etc1/wrapper.cpp");
    println!("cargo:rerun-if-changed=rg_etc1/rg_etc1.h");
}