use std::fs;
use cfg_aliases::cfg_aliases;

mod utils;

fn main() {
    cfg_aliases! {
        macos: { target_os = "macos" },
        linux: { target_os = "linux" }
    }

    let main_folder = utils::get_main_folder();

    if let Ok(exist) = main_folder.try_exists() {
        if !exist {
            match fs::create_dir(main_folder) {
                Ok(_) => println!("Created main directory successfully"),
                Err(e) => panic!("Failed to create main directory: {}", e)
            };
        } else if !main_folder.is_dir() {
            panic!("Existing main folder is not a directory!")
        }
    } else {
        panic!("Unable to access main folder!")
    }

    println!("Main directory is valid.");
}
