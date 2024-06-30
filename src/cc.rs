use std::process::{Command, ExitCode, ExitStatus};

use crate::{cli, config::Config};

#[derive(Debug)]
pub struct CC {
    config: Config,
    args: cli::Args,
    cc: String,
    cc_args: Vec<String>,
}

fn concat_cmd(out: &mut String, parts: &[&str]) {
    out.push_str(parts.join(" ").as_str());
}

impl CC {
    pub fn new(config: Config, args: cli::Args) -> Self {
        Self {
            config,
            args,
            cc: String::new(),
            cc_args: Vec::new(),
        }
    }

    /// Prepare commands
    pub fn prepare(&mut self) {
        let cc = if self.args.release {
            if let Some(ref release) = self.config.compiler.release {
                release
                    .overrides
                    .cc
                    .as_ref()
                    .unwrap_or(&self.config.compiler.default.cc)
            } else {
                &self.config.compiler.default.cc
            }
        } else {
            &self.config.compiler.default.cc
        };

        self.cc = cc.to_string();
    }
    /// Run all the commands.
    pub fn compile(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.cc);

        let mut process = cmd.args(&self.cc_args).spawn()?;

        match process.wait() {
            Ok(status) if !status.success() => {
                return Err(format!("process `{cmd:?}` returned non-zero exitcode.").into())
            }
            Err(e) => return Err(format!("Failed to wait on process `{cmd:?}`: {e}").into()),
            _ => {}
        }

        Ok(())
    }
    pub fn compile_command(&self) -> String {
        format!("{} {}", self.cc, self.cc_args.join(" "))
    }
}
