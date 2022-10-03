use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Erase loaded config file (or user config file and exit)
    #[arg(long, action = ArgAction::SetTrue)]
    eraseconf: bool,
    /// Erase loaded mapper file (or user mapper file and exit)
    #[arg(long, action = ArgAction::SetTrue)]
    erasemapper: bool,

    ///Create user level config file
    #[arg(long, action = ArgAction::SetTrue)]
    userconf: bool,

    /// Start DOSBox-X with the specific config file
    #[arg(long, value_name = "conf file")]
    conf: Option<PathBuf>,

    /// Turn debugging information on 12323
    #[arg(long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    println!("Value for erase_conf: {}", cli.eraseconf);

    if let Some(config_path) = cli.conf.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
}
