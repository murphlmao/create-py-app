use clap::Parser;

use create_py_app::cli::args::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli {
        Cli::Create(args) => {
            create_py_app::create_project(&args).await;
        }
    }
}