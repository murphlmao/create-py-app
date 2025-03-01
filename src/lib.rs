pub mod cli;
pub mod py_detector;
pub mod template_handler;

use std::path::PathBuf;
use std::process::exit;

pub const DEFAULT_PYTHON_VERSION: &str = "3.12.7";
pub const DEFAULT_VCS_PLATFORM: &str = "gitlab";

use cli::args::CreateModuleArgs;
use py_detector::{get_python_version, fetch_python_versions, check_python_version_exists};
use template_handler::{create_std_template, create_dirs};

pub fn validate_project_name(name: &str) -> bool {
    let path = PathBuf::from(name);
    if path.exists() {
        eprintln!("A directory with the name '{}' already exists in the current directory.", name);
        return false;
    }
    true
}

pub async fn resolve_python_version(specified_version: Option<String>) -> String {
    match specified_version {
        Some(version) => {
            if check_python_version_exists(&version).await.unwrap_or(false) {
                version
            } else {
                eprintln!("Warning: The specified Python version '{}' may not be a valid Python version.", version);
                version
            }
        },
        None => {
            println!("Python version not provided. Attempting to get Python version from system...");
            match get_python_version().as_str() {
                "unknown" => {
                    println!("Failed to get Python version. Attempting to fetch the latest available version...");
                    match fetch_python_versions().await {
                        Ok(versions) => versions.get(0).unwrap_or(&DEFAULT_PYTHON_VERSION.to_string()).clone(),
                        Err(_) => {
                            eprintln!("Failed to fetch available Python versions from the registry. Defaulting to Python {}.", DEFAULT_PYTHON_VERSION);
                            DEFAULT_PYTHON_VERSION.to_string()
                        }
                    }
                }
                version => version.to_string(),
            }
        }
    }
}

pub fn resolve_vcs_platform(specified_platform: Option<String>) -> String {
    match specified_platform {
        Some(platform) => {
            let normalized = platform.to_lowercase();
            if normalized == "gitlab" || normalized == "github" {
                normalized
            } else {
                eprintln!("Choose from 'GitHub' or GitLab' not {}", platform);
                exit(1);
            }
        },
        None => DEFAULT_VCS_PLATFORM.to_string()
    }
}

pub async fn create_project(args: &CreateModuleArgs) -> () {
    let name = &args.name;
    
    if !validate_project_name(name) {
        exit(1);
    }
    
    let python_version = resolve_python_version(args.python_version.clone()).await;
    
    let vcs_platform = resolve_vcs_platform(args.vcs_platform.clone());
    
    println!("Creating project: {} with python version {}, using {} templates", 
             name, python_version, vcs_platform);
    
    let dirs = create_dirs::DirectoryManager::new(name, &vcs_platform);
    dirs.create();
    
    let res = create_std_template::render_all(name, &python_version, &vcs_platform, args.retype);
    println!("{}", res);
}