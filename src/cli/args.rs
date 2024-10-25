// libraries
use clap::Parser;

// modules

#[derive(Parser)]
pub struct CreateModuleArgs { // create module subparser
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub python_version: Option<String>,
}


#[derive(Parser)]
#[clap( // this is basically the description base description of the CLI
    name = "create-py-app",
    version, // derived from cargo.toml
    about = "create-py-app automatically sets up a new python project structure for you.",
    long_about = "create-py-app is a small, Rust based utility designed to automatically configure & setup
    a standard python project structure based on generally accepted community best practices.",
)]
pub enum Cli {
    // serves as an 'execute' method for your de-facto CLI
    Create(CreateModuleArgs),
}