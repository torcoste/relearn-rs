use clap::{Parser, Subcommand};

mod configs;

mod init;
use init::init_command_handler;

mod learn;
use learn::learn_command_handler;

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
    Learn {
        #[arg(long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // TODO: if it is a first run, run init command automatically before any other command

    match &cli.command {
        Some(Commands::Init {}) => {
            init_command_handler();
        }
        Some(Commands::Learn { force }) => {
            learn_command_handler(*force);
        }
        None => {
            learn_command_handler(true);
        }
    }
}
