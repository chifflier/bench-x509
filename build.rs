fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/util-decode-der.c");
    println!("cargo:rerun-if-changed=src/util-decode-der.h");
    println!("cargo:rerun-if-changed=src/suricata-common-stub.h");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/util-decode-der.c")
        .flag("-Wno-unused")
        .flag("-O2")
        .compile("util-decode-der");
}
