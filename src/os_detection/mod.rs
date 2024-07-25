// modules
pub mod pyenv_detection;


// libraries
use os_info;

pub fn detect_os() -> String {
    let info = os_info::get();
    info.os_type().to_string()
}
