// libraries
use std::env;
use std::fs;
use std::process::{Command, exit};
use std::path::PathBuf;

// modules
mod cli;
use cli::args::Cli;
use clap::Parser;
mod template_handler;
use template_handler::create_std_template;
use template_handler::create_dirs;


fn get_python_version() -> String {
    let home_dir = env::var("USERPROFILE").or_else(|_| env::var("HOME")).expect("Could not determine home directory");

    // construct the path to the pyenv-win version file
    // TODO make this compatible with other OS'
    let pyenv_version_path = PathBuf::from(home_dir)
        .join(".pyenv")
        .join("pyenv-win")
        .join("version");

    if pyenv_version_path.exists() {
        if let Ok(version) = fs::read_to_string(pyenv_version_path) {
            let version = version.trim().to_string();
            if !version.is_empty() {
                return version;
            }
        }
    }

    // fallback to system's default python3 if pyenv version file is not found or invalid
    let python_output = Command::new("python3")
        .arg("--version")
        .output();

    if let Ok(output) = python_output {
        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            let version = version_output.split_whitespace().nth(1).unwrap_or("unknown").to_string();
            return version;
        }
    }

    eprintln!("Failed to get Python version. Please ensure Python is properly configured.");
    exit(1);
}


fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Create(args) => {
            let name = &args.name;

            if PathBuf::from(name).exists() {
                eprintln!("A directory with the name '{}' already exists in the current directory.", name);
                exit(1);
            }

            // TODO: perform some validation to see if user input is a valid python version
            let python_version = match &args.python_version {
                Some(version) => version.clone(),
                None => {
                    //println!("Python version not provided. Attempting to get Python version from system...");
                    let version = get_python_version(); // get the actual Python version

                    if version == "unknown" {
                        println!("Failed to get Python version. Please make sure Python is installed.");
                        println!("Defaulting to python 3.12.7");
                        "3.12.7".to_string()
                    } else {
                        //println!("Python version found: {}", version);
                        version
                    }
                }
            };

            println!("Creating project: {} with python version {}", args.name, python_version);

            let dirs = create_dirs::DirectoryManager::new(&name);
            dirs.create();

            let res = create_std_template::render_all(name, &python_version);
            println!("{}", res);
        }
    }
}