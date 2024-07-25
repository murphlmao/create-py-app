// modules
use crate::os_detection;

// function to initialize the module creation
pub fn init() -> String {
    let os = os_detection::detect_os();
    match os.as_str() {
        "Linux" => println!("Detected Linux."), // TODO this may very well only support Ubuntu. Need to add additional compatibility later down the line.
        "Windows" => println!("Detected Windows."),
        _ => println!("Unsupported OS"),
    }

    // return linux or windows 
    return os;
}
