use std::{fs::{self, OpenOptions}, io::{self, Read}, path::Path, process::{Command, ExitCode}};
use std::io::Write;

use crate::{cc::CC, cli, config, log};
use colored::Colorize;
use config::Config;

fn dir_exists(p: impl AsRef<Path>) -> bool {
    p.as_ref().is_dir()
}
fn file_exists(p: impl AsRef<Path>) -> bool {
    p.as_ref().is_file()
}

fn add_to_gitignore(ignored: &[&str]) -> io::Result<()> {
    println!("1");
    let mut already_added: Vec<bool> = vec![false; ignored.len()];
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .open(".gitignore")?;

    let mut prev_contents = String::new();
    if let Ok(true) = std::path::Path::new(".gitignore").try_exists() {
        file.read_to_string(&mut prev_contents)?;

        // Check if IGNORED in contents
        for line in prev_contents.split('\n') {
            for (i, ignore) in ignored.iter().enumerate() {
                if line == *ignore {
                    already_added[i] = true;
                }
            }
        }
    }

    if already_added.iter().filter(|b| !**b).next().is_none() {
        return Ok(())
    }

    let mut contents = String::from("# Added by cuild\n");
    if !prev_contents.is_empty() {
        contents.insert(0, '\n');
    }

    for (i, ignore) in ignored.iter().enumerate() {
        if !already_added[i] {
            contents.push_str(ignore);
            contents.push('\n');
        }
    }

    write!(file, "{contents}")?;

    Ok(())
}

fn init_project() -> Result<(), Box<dyn std::error::Error>> {
    if dir_exists("src") {
        log::warning("src folder already exists, it will not be touched.");
    } else {
        fs::create_dir("src")?;
        log::info("Created src/ folder");
    }
    if file_exists("cuild.toml") {
        log::warning("cuild.toml already exists, it will not be touched.");
    } else {
        fs::write("cuild.toml", Config::default().to_string()?)?;
        log::info("Created cuild.toml");
    }
    let ignored = &["/target/"];
    add_to_gitignore(ignored)?;
    log::info(format!("added {} to .gitignore", ignored.join(", ")));

    Ok(())
}
 
pub fn run(config: Config, args: cli::Args) -> Result<(), String> {
    if args.init {
        init_project().map_err(|e| e.to_string())?;
        return Ok(());
    }

    if !dir_exists("src") {
        return Err("src folder does not exist, please create it before continuing.".into());
    }

    if args.release && config.compiler.release.is_none() {
        return Err(format!("release mode was enabled ({}), but the [compiler.config] in cuild.toml was not specified.", cli::RELEASE.listed()))
    }

    let mut cc = CC::new(config, args);

    cc.prepare();

    let cmd_pref = format!("{}{}{}", "[".bold(), "CMD".bold().bright_green(), "]".bold());
    println!("{cmd_pref} {}", cc.compile_command());

    cc.compile().map_err(|e| e.to_string())
}