use clap::{ArgAction, Parser};
use log::LevelFilter;
use std::path::PathBuf;

mod misc;

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

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        1 => {
            log::debug!("Debug mode is kind of on");
            log::set_max_level(LevelFilter::Debug);
        }
        2 => {
            log::trace!("Debug mode is on");
            log::set_max_level(LevelFilter::Trace);
        }
        _ => {
            log::info!("Debug mode is off");
            log::set_max_level(LevelFilter::Info);
        }
    }
    env_logger::init();

    let mut config = misc::setup::Config::new().expect("Cannot parse config manifest.");
    log::debug!("Config is {:#?}", config);

    if let Some(config_path) = cli.conf.as_deref() {
        config
            .parse(config_path)
            .expect("Cannot parse config file.");
    }

    let mut msg = misc::msg::MessageMap::new();
    msg.load_lang_file(PathBuf::from("res/default.lang"));

    log::trace!("config is {:#?}", config);
}
