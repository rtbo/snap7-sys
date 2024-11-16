use std::env;
use std::path::PathBuf;

fn main() {
    let this_dir = env::current_dir().expect("should get current dir");
    let snap7_dir = this_dir.join("snap7");
    let wrapper_h = this_dir.join("wrapper.h");

    println!("cargo:rerun-if-changed={}", snap7_dir.display());
    println!("cargo:rerun-if-changed={}", wrapper_h.display());

    let dst = cmake::Config::new(snap7_dir)
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=snap7");

    let bindings = bindgen::Builder::default()
        .allowlist_function("Cli_.*")
        .allowlist_function("Srv_.*")
        .allowlist_function("Par_.*")
        .allowlist_var("S7.*")
        .allowlist_var("err.*")
        .allowlist_var("p_.*")
        .allowlist_var("Job.*")
        .allowlist_var("Max.*")
        .allowlist_var("CONN.*")
        .allowlist_var("Block.*")
        .allowlist_var("SubBlk.*")
        .allowlist_var("Operation.*")
        .rust_target(bindgen::RustTarget::Stable_1_71)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .header(wrapper_h.display().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
