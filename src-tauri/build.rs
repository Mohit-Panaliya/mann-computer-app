fn main() {
    tauri_build::build();
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=-SUBSYSTEM:WINDOWS");
    }
}
