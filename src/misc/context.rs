use clap::{ArgAction, Parser};
use std::error::Error;
use std::path::{Path, PathBuf};
use crate::misc::config::Config;
use crate::misc::msg::Messages;

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

    /// Start DOSBox-X with the specific language file
    #[arg(long, value_name = "language file")]
    lang: Option<PathBuf>,

    /// Turn debugging information on dosbox
    #[arg(long, action = clap::ArgAction::Count)]
    debug: u8,
}

pub struct Context {
    config: Config,
    message: Messages,

}

pub type Err = Box<dyn Error>;

impl Context {
    pub fn new() -> Option<Self> {
        Some(
            Context {
                config: Config::new()?,
                message: Messages::new()?
            }
        )
    }

    pub fn parse_args(&mut self) {
        let cli = Cli::parse();

        if let Some(file_path) = cli.conf {
            log::info!("Load user config {:#?}", file_path);
            self.load_config(&file_path).expect("Cannot parse config file.");
            log::trace!("Config is {:#?}", self.config);
        }

        if let Some(file_path) = cli.lang {
            log::info!("Load language file {:#?}", file_path);
            self.load_language(&file_path).expect("Cannot load language file.");
        }
        log::trace!("Language is {:#?}", self.message);
        self.save_language(&PathBuf::from("target/1.lang")).expect("Cannot save language file.");
    }

    pub fn load_config(&mut self, file_path: &PathBuf) -> Result<(), Err> {
        self.config.load(file_path)
    }

    pub fn load_language(&mut self, file_path: &PathBuf) -> Result<(), Err> {
        log::info!("Load language config {:#?}", file_path);
        self.message.load(file_path)
    }

    pub fn save_language(&mut self, file_path: &PathBuf) -> Result<(), Err> {
        log::info!("Save language config {:#?}", file_path);
        self.message.save(file_path)
    }

    pub fn msg_get(&self, id: &String) -> Option<&String> {
        self.msg_get(id)
    }
}
