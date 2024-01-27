mod new;
mod run;

mod constants;

use clap::{Parser, Subcommand};
use std::fs;
use toml::Table;

/// Command line program that saves storage space by symlinking all repeating file names to a cache.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs symlinker, symlinking all discoverable files
    Run,
    /// Creates a new symlinker archive at the current location
    New {
        /// name of archive to be created
        #[arg(short, long)]
        name: String,
    },
}

/// checks if symlinker is called from a symlinker archive root and panics if it is not
// NOTE: we are making the design choice, deviating from CLI like cargo, to not allow execution
// unless the path explictly has a symlinker archive root.
fn verify_archive_root() {
    // try to load the config TOML as a string
    let config = fs::read_to_string(format!("./{}", constants::CONFIG_NAME)).expect(&format!(
        "Did not detect {}: not running from a symlinker archive.",
        constants::CONFIG_NAME
    ));

    // try to parse the TOML
    let config = config
        .parse::<Table>()
        .expect("Config file could not be parsed: not running from a symlinker archive.");

    // check if the TOML matches the symlinker "magic number" identifier
    if !match &config["Config"]["symlinker_root_here"] {
        toml::Value::Boolean(b) => *b,
        _ => panic!("Not a symlinker config file: not running from a symlinker archive."),
    } {
        panic!("Not a symlinker config file: not running from a symlinker archive.");
    }
}

fn main() {
    // take in command line arguments
    let args = Args::parse();

    // run corresponding initialization functions
    match &args.command {
        Commands::Run => {
            verify_archive_root();
            run::symlink_all();
        }
        Commands::New { name } => {
            new::create_archive(name);
        }
    }
}
