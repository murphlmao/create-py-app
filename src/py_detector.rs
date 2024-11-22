// libraries
use std::env;
use std::fs;
use std::process::{Command, exit};
use std::path::PathBuf;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;


pub fn get_python_version() -> String {
    let home_dir = env::var("USERPROFILE").or_else(|_| env::var("HOME")).expect("Could not determine home directory");

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


pub async fn fetch_python_versions() -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://www.python.org/api/v2/downloads/release/"; // their public api with all of their releases
    let client = Client::new();
    let response = client.get(url).send().await?;
    let data = response.json::<Vec<Value>>().await?;

    // Extract and sort versions
    let mut versions: Vec<String> = data.iter()
        .filter_map(|item| item["name"].as_str())
        .filter(|name| !["a", "b", "c", "rc"].iter().any(|&dup| name.contains(dup)))
        .map(|name| name.replace("Python ", ""))
        .collect();

    versions.sort_by(|a, b| b.cmp(a)); // Sort in reverse order
    Ok(versions)
}

/// Check if a specific version exists in the list
pub async fn check_python_version_exists(version: &str) -> Result<bool, Box<dyn Error>> {
    let versions = fetch_python_versions().await?;
    Ok(versions.contains(&version.to_string()))
}