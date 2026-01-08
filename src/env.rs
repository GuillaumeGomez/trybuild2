use crate::error::{Error, Result};
use std::env;

#[derive(PartialEq, Debug, Default)]
pub enum Update {
    #[default]
    Wip,
    Overwrite,
}

impl Update {
    pub fn env() -> Result<Self> {
        let Some(var) = env::var_os("TRYBUILD2") else {
            return Ok(Update::default())
        };

        match var.as_os_str().to_str() {
            Some("wip") => Ok(Update::Wip),
            Some("overwrite") => Ok(Update::Overwrite),
            _ => Err(Error::UpdateVar(var)),
        }
    }
}
