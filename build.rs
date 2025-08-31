use userspace_build::info;
use userspace_build::target;

fn main() {
    userspace_build::file::print("build.rs");
    info!("cargo:rerun-if-changed=build.rs");
    info!("cargo:rerun-if-changed=linker.ld");
    info!("cargo:rerun-if-changed=src/");
    info!("cargo:rerun-if-changed=crates/");

    // Static linking flags
    info!("cargo:rustc-link-arg=-static");
    info!("cargo:rustc-link-arg=--no-dynamic-linker");
    info!("cargo:rustc-link-arg=-n");

    // Disable position independent code
    info!("cargo:rustc-link-arg=--no-pie");

    // Compile assembly startup code
    cc::Build::new()
        .file("src/start.s")
        .flag("-fno-pic")
        .flag("-fno-pie")
        .compile("start");
}
