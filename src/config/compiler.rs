use serde::{Deserialize, Serialize};

use super::guess_default::GuessDefault;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerBase {
    pub cc: String,
    pub cflags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerWithMode {
    pub cc: Option<String>,
    pub cflags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    #[serde(flatten)]
    pub overrides: CompilerWithMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Debug {
    #[serde(flatten)]
    pub overrides: CompilerWithMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compiler {
    #[serde(flatten)]
    pub default: CompilerBase,
    pub release: Option<Release>,
    pub debug: Option<Debug>,
}

fn file_exists(file: &str) -> bool {
    if std::path::Path::new(file).exists() {
        return true;
    }

    // Check if spawn
    if let Ok(mut proc) = std::process::Command::new(file).spawn() {
        let _ = proc.kill();
        return true;
    }

    false
}

fn search_files<'a>(files: &[&'a str], default: &'a str) -> &'a str {
    for file in files {
        if file_exists(file) {
            return file;
        }
    }
    default
}

impl Compiler {

}


impl GuessDefault for Compiler {
    /// Try to search the user's system for installed binaries, and use those that are found.
    fn guess_default() -> Self {
        let cc = search_files(&["gcc", "clang"], "cc");

        Self {
            default: CompilerBase { cc: cc.to_string(), cflags: None },
            release: None,
            debug: None,
        }
    }
}