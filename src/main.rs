// modules

// lib
use std::process;
use clap::Parser;
use regex::Regex;

// define modules
mod cli;
mod presets;
use presets::{base, common, python};
use cli::args::{Cli, Language};



#[allow(clippy::needless_return)]
fn validate_preset(preset: &str) -> Language {
    if preset == "base" {
        return Language {
            language: "base".to_string(),
            ver: "".to_string(),
        };
    }

    let re = Regex::new(r"python(3\.\d+|4\.\d+)").unwrap();
    if let Some(caps) = re.captures(preset) {
        return Language {
            language: "python".to_string(),
            ver: caps[1].to_string(),
        };
    } else {
        eprintln!("Python version not recognized in --preset, invalid input. Expected format: 'python3.xx'");
        process::exit(1);
    }
}

fn main() {
    match Cli::parse() {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            let lang = validate_preset(&args.preset);
            let create = true;
            if lang.language == "python" {
                let prefix = common(&args.name, create, &lang);
                python(&args.name, &prefix, &lang);
            } else if lang.language == "base" {
                let _prefix = base(&args.name, create, &lang);
            } else {
                eprintln!("Preset: {:?} not supported", args.preset);
            }
        }
        Cli::Update(args) => {
            println!("Updating project with name: {}", args.name);
            println!("Using preset: {:?} ", args.preset);
            let lang = validate_preset(&args.preset);
            let create = false;
            if lang.language == "python" {
                let prefix = common(&args.name, create, &lang);
                python(&args.name, &prefix, &lang);
            } else if lang.language == "base" {
                let _prefix = base(&args.name, create, &lang);
            } else {
                eprintln!("Preset: {:?} not supported", args.preset);
            }
        }
    }
}
