// libraries
use clap::Parser;

// modules
mod commands;
mod os_detection;
use crate::os_detection::pyenv_detection;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // name of the application
    #[arg(short, long)]
    name: String,

    // python version to install for this app
    #[arg(short, long, default_value = "latest")]
    pyver: String,


}

fn main() {
    let args = Args::parse();

    println!("App name: {:?}", args.name);
    println!("Python Version: {:?}", args.pyver);


    // ask user if they're sure they want to create a python app in this current directory
    println!("Are you sure you want to create a python app in this directory? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input != "y" {
        println!("Exiting...");
        return;
    }

    // init module creation
    let os: String = commands::init();
    let pyenv_status = pyenv_detection::install_pyenv(os);
    //println!("pyenv status: {}", pyenv_status);


    // create python app
    commands::create_app(args.name, args.pyver);
}
