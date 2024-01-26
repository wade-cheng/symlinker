mod new;
mod run;
mod stats;

mod constants;

use clap::{Parser, Subcommand};
use std::fs;
use toml::Table;

/// Command line program that saves storage space by symlinking all repeating file names to a cache.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// test
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs symlinker, symlinking all discoverable files
    Run,
    /// Lists statistics for this symlinker archive
    Stats,
    /// Creates a new symlinker archive at the current location
    New,
}

/// checks returns whether we are called from a symlinker archive root and panics if not
/// TODO: make it like cargo where you can call from any subdirectory of the archive
fn in_archive() {
    let config = fs::read_to_string(format!("./{}", constants::CONFIG_NAME)).expect(&format!(
        "Did not detect {}: not running from a symlinker archive.",
        constants::CONFIG_NAME
    ));

    let config = config
        .parse::<Table>()
        .expect("Config file could not be parsed: not running from a symlinker archive.");

    let x = match &config["Config"]["symlinker_root_here"] {
        toml::Value::Boolean(b) => b,
        _ => panic!("Not a symlinker config file: not running from a symlinker archive."),
    };

    if !x {
        // TODO: weird and unintuitive: make a magic number? something that will identify a config file instead.
        panic!("Not at symlinker root file.")
    }
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Run => {
            in_archive();
            run::symlink_all();
        }
        Commands::Stats => {
            in_archive();
            stats::print_stats();
        }
        Commands::New => {
            new::create_archive();
        }
    }
}
