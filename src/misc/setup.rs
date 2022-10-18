use std::error::Error;
use crate::misc::config::Config;
use crate::misc::msg::Messages;

pub struct Setup {
    config: Config,
    message: Messages,

}

pub type Err = Box<dyn Error>;

impl Setup {
    pub fn new() -> Result<Self, Err> {
        Ok(
            Setup{
                config: Config::new()?,
                message: Messages::new()?
            }
        )
    }
}