mod config;
mod log;
mod run;
mod cli;
mod cc;
use std::process::ExitCode;

use config::Config;

#[repr(u8)]
pub enum ExitReason {
    Ok = 0,
    InvalidCommand,
    FailedReadingConfig,
    CommandFailed,
}

fn main() -> ExitCode {
    let argv = std::env::args();
    let args = match cli::Args::parse(argv) {
        Ok(a) => a,
        Err(e) => {
            log::error(e);
            cli::print_usage();
            return ExitCode::from(ExitReason::InvalidCommand as u8);
        }
    };

    let config = Config::read();

    let code = match config {
        Ok(config) => if let Err(e) = run::run(config, args) {
            log::error(format!("Failed to run Command: {e}"));
            ExitReason::CommandFailed
        } else {ExitReason::Ok}
        ,
        Err(e) => {
            let mut msg = String::from("Failed to read config: ");
            let description = match e {
                config::Error::IOError(e) => e.to_string(),
                config::Error::SerializeError(e) => e.to_string(),
                config::Error::DeserializeError(e) => e.to_string(),
            };
            msg.push_str(&description);

            log::error(&msg);
            ExitReason::FailedReadingConfig
        }
    };

    ExitCode::from(code as u8)
}
