// libraries
use clap::Parser;

#[derive(Parser)]
pub struct CreateModuleArgs {
    #[clap(long, required = true)]
    pub name: String,
    #[clap(long, required = false, default_value = "python3.10")]
    pub preset: String,
}



#[derive(Parser)]
pub struct UpdateModuleArgs {
    #[clap(long, required = true)]
    pub name: String,
    #[clap(long, required = false, default_value = "python3.10")]
    pub preset: String,
}