use clap::{Parser, Subcommand};

mod init;
use init::init_command_handler;

mod request;
use request::request_command_handler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize application (create config file, set autorun)
    Init {},
    /// Request quiz question
    Request {
        #[arg(long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            init_command_handler();
        }
        Some(Commands::Request { force }) => {
            request_command_handler(*force);
        }
        None => {}
    }
}
