// main.rs
mod cli;
mod presets;
use presets::{common, python};
use cli::args::Cli;
use clap::Parser;  // Make sure to bring the Parser trait into scope




fn main() {
    let cli = Cli::parse();  // `parse` is now available
    match cli {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            let create = true;
            let python_version: String = "3.9".to_string();
            let prefix = common(&args.name, create);
            python(&args.name, &prefix, &python_version);
        }
    }
}
