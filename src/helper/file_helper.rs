use std::path::PathBuf;

#[cfg(debug_assertions)]
pub fn get_root_dir() -> PathBuf {
    env!("CARGO_MANIFEST_DIR").into()
}

#[cfg(not(debug_assertions))]
pub fn get_root_dir() -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path
    } else {
        PathBuf::new()
    }
}
