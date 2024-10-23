// libraries

// modules
mod cli;
mod presets;
use presets::{common, python};
use cli::args::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            let python_version: String = "3.9".to_string(); // TODO get currently installed python version from other func
            let prefix = common(&args.name);
            python(&args.name, &prefix, &python_version);
        }
    }
}
