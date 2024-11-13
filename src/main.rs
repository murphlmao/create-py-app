// libraries
use clap::Parser;
use std::path::PathBuf;

// modules
mod cli;
use cli::args::Cli;
mod py_detector;
use py_detector::{get_python_version, fetch_python_versions, check_python_version_exists};
mod template_handler;
use template_handler::{create_std_template, create_dirs};


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Create(args) => {
            let name = &args.name;

            if PathBuf::from(name).exists() {
                eprintln!("A directory with the name '{}' already exists in the current directory.", name);
                std::process::exit(1);
            }

            // determine Python version
            let python_version = match &args.python_version {
                Some(version) => {
                    if check_python_version_exists(version).await.unwrap_or(false) {
                        version.clone()
                    } else {
                        eprintln!("Warning: The specified Python version '{}' may not be a valid Python version.", version);
                        version.clone()
                    }
                }
                None => {
                    println!("Python version not provided. Attempting to get Python version from system...");
                    match get_python_version().as_str() {
                        "unknown" => {
                            println!("Failed to get Python version. Attempting to fetch the latest available version...");
                            match fetch_python_versions().await {
                                Ok(versions) => versions.get(0).unwrap_or(&"3.12.7".to_string()).clone(),
                                Err(_) => {
                                    eprintln!("Failed to fetch available Python versions from the registry. Defaulting to Python 3.12.7.");
                                    "3.12.7".to_string()
                                }
                            }
                        }
                        version => version.to_string(),
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