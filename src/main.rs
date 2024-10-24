// modules
mod cli;
use cli::args::Cli;
use clap::Parser;
mod template_handler;
use template_handler::create_std_template;
use template_handler::create_dirs;


fn main() {
    let cli = Cli::parse();
    match cli {
        Cli::Create(args) => {
            println!("Creating project with name: {}", args.name);
            let python_version: String = "3.9".to_string(); // TODO get currently installed python version from other func
            let name = &args.name;
            let dirs = create_dirs::DirectoryManager::new(&name);
            dirs.create();

            let res = create_std_template::render_all(name, &python_version);
            println!("{}", res);

        }
    }
}
