use serde::{Serialize, Deserialize};

use super::guess_default::GuessDefault;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

fn current_dir_name() -> Option<String> {
    std::env::current_dir().map_or(None, |p| {
        p.file_name()
            .and_then(|n| Some(n.to_string_lossy().to_string()))
    })
}

impl GuessDefault for Package {
    fn guess_default() -> Self {
        Self {
            name: current_dir_name().unwrap_or("name".to_string()),
            version: "0.1.0".to_string(),
        }
    }
}