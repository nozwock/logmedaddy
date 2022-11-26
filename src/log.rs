use crate::config::Profile;
use colored::*;
use std::process::Command;

#[derive(Debug, Default, Clone)]
struct LoggerCmd {
    cmd: String,
    output: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct LoggerProfile {
    name: String,
    loggers: Vec<LoggerCmd>,
}

#[derive(Debug, Default, Clone)]
pub struct Logger {
    profiles: Vec<LoggerProfile>,
}

impl Logger {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn from_profiles(profiles: &Vec<Profile>) -> Self {
        Self {
            profiles: profiles
                .iter()
                .map(|prof| LoggerProfile {
                    name: prof.name.clone(),
                    loggers: prof
                        .loggers
                        .iter()
                        .map(|cmd| LoggerCmd {
                            cmd: cmd.clone(),
                            output: command_output(cmd),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
    pub fn to_string_lossy(self) -> String {
        self.profiles
            .into_iter()
            .fold(String::new(), |acc, profile| {
                // for each profile
                let mut out = Vec::new();
                if !acc.is_empty() {
                    out.push(acc);
                }
                out.push(wrap_str(&profile.name, "#", 10)); // push profile name
                profile.loggers.into_iter().for_each(|logger| {
                    if let Some(output) = logger.output {
                        out.push(wrap_str(&logger.cmd, "+", 10)); // push logger name
                        out.push(output);
                    } else {
                        eprintln!("{} '{}'", "Command failed:".red(), logger.cmd.yellow());
                        // for now, this is fine I suppose
                    };
                });
                out.join("\n\n")
            })
    }
    pub fn to_string(self) -> Option<String> {
        let mut out = Vec::new();
        for profile in self.profiles {
            out.push(wrap_str(&profile.name, "#", 10)); // push profile name
            for logger in profile.loggers {
                if let Some(output) = logger.output {
                    out.push(wrap_str(&logger.cmd, "+", 10)); // push logger name
                    out.push(output); // push the logs
                } else {
                    eprintln!("{} '{}'", "Command failed:".red(), logger.cmd.yellow());
                    // for now, this is fine I suppose
                    return None;
                };
            }
        }
        Some(out.join("\n\n"))
    }
}

fn command_output(cmd: &str) -> Option<String> {
    let mut args = cmd.trim().split_whitespace();
    let cmd = args.next()?;
    let args = args.collect::<Vec<_>>();
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8(out.stdout).ok()?)
}

fn wrap_str(string: &str, with: &str, count: usize) -> String {
    let wrap = with.repeat(count);
    format!("{} {} {}", wrap, string, wrap)
}
