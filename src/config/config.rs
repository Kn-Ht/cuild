use crate::config::error;
use serde::{Deserialize, Serialize};
use std::fs;

use super::{guess_default::GuessDefault, Compiler, Package, Overrides};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub package: Package,
    pub compiler: Compiler,
    pub src: Option<Overrides>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            package: Package::guess_default(),
            compiler: Compiler::guess_default(),
            src: None
        }
    }
}

impl Config {
    pub fn read() -> error::Result<Self> {
        let content = fs::read_to_string("cuild.toml")?;
        let s = toml::from_str(&content)?;

        Ok(s)
    }
    #[inline]
    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        Ok(toml::to_string_pretty(self)?)
    }
}
