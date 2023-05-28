use dirs::home_dir;
/* use log::debug; */
use log::error;
use log::info;
use std::path::PathBuf;

pub fn load_config(path: Option<PathBuf>) {
    /*  getting default path to config file  */

    match path {
        None => {
            if let Some(home_path) = home_dir() {
                let path = home_path.join(".config").join("newm").join("config.py");
                info!("Default config path used : {:?}", path);
            } else {
                error!("Failed to get home directory path");
            }
        }
        Some(_) => {
            info!("Provided config used");
        }
    }
}
