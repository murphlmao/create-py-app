// libraries
use clap::Parser;

// modules
use crate::cli::subparsers::{CreateModuleArgs, UpdateModuleArgs};


// base parser for your application
#[derive(Parser)]
#[clap(
    name = "create-py-app",
    version,
    about = "create-py-app automatically sets up a new python project structure for you.",
    long_about = "create-py-app is a small, Rust based utility designed to automatically configure & setup
    a standard python project structure based on generally accepted community best practices."
)]


pub enum Cli {
    #[clap(about = "Create a new project")]
    Create(CreateModuleArgs),

    #[clap(about = "Update existing project")]
    Update(UpdateModuleArgs),
}


pub struct Language {
    pub language: String,
    pub ver: String,
}