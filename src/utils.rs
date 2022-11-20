use std::path::PathBuf;

pub fn get_main_folder() -> PathBuf {
    if let Some(base_dirs) = directories::BaseDirs::new() {
        base_dirs.data_dir().join("rusty-craft-launcher")
    } else {
        panic!("Unable to get main folder!")
    }
}