use std::env;

use colored::Colorize;

/// (Short, Long, Desc)
pub struct Arg(&'static str, &'static str, &'static str);
impl Arg {
    pub fn positional(&self) -> String {
        format!("[{}/{}]", self.0, self.1)
    }
    pub fn listed(&self) -> String {
        format!("{}, {}", self.0, self.1)
    }
}
impl std::cmp::PartialEq<Arg> for &str {
    fn eq(&self, other: &Arg) -> bool {
        *self == other.0 || *self == other.1
    }
}

pub const HELP: Arg = Arg("-h", "--help", "Prints this help message.");
pub const RELEASE: Arg = Arg("-r", "--release", "Whether or not to run cuild in release mode, when this is passed, additional steps such as those under [compiler.release] will be taken.");
pub const INIT: Arg = Arg("-i", "--init", "Initializes a cuild project.");
pub const DEBUG: Arg = Arg("-d", "--debug", "Whether or not to run cuild in debug mode, this will use settings under [compiler.debug] in cuild.toml");

#[derive(Debug, Default)]
pub struct Args {
    pub release: bool,
    pub init: bool,
    pub help: bool,
    pub debug: bool,
}

fn current_exe_name() -> Option<String> {
    std::env::current_dir()
        .and_then(|p| {
            Ok(p.file_name()
                .and_then(|n| Some(n.to_string_lossy().to_string())))
        })
        .map_or(None, |p| p)
}

pub fn print_usage() {
    println!(
        "{usage} {program} {help} {release}",
        usage = "USAGE:".bold(),
        program = current_exe_name().unwrap_or(env!("CARGO_PKG_NAME").into()),
        release = RELEASE.positional(),
        help = HELP.positional(),
    );
    println!("{}", "FLAGS:".cyan().bold());
    for flag in [HELP, RELEASE] {
        println!("       {}: {}", flag.listed().bold(), flag.2);
    }
}

impl Args {
    pub fn parse(args: env::Args) -> Result<Self, String> {
        let mut s = Self::default();

        // Parse args
        let args = args.skip(1).into_iter();

        for arg in args {
            let arg = arg.to_lowercase();
            let arg = arg.as_str();

            if arg == HELP {
                s.help = true;
            } else if arg == INIT {
                s.init = true;
            } else if arg == RELEASE {
                s.release = true;
            } else if arg == DEBUG {
                s.debug = true;
            } else {
                return Err(format!("Invalid command: {arg}"));
            }
        }

        Ok(s)
    }
}
