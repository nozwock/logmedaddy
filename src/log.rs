use crate::config::Profile;
use std::process::Command;

pub fn log_profiles(profiles: Vec<&Profile>) -> String {
    let mut out = Vec::new();
    for profile in profiles {
        out.push(wrap_string(&profile.name, "#", 10)); // push profile name
        for logger in &profile.loggers {
            out.push(wrap_string(&logger, "+", 10)); // push logger name
            let log = log_from_logger(&logger);
            if let Some(log) = log {
                out.push(log); // push the logs
            }
        }
    }
    out.join("\n\n")
}
fn log_from_logger(logger: &str) -> Option<String> {
    let mut logger = logger.split_whitespace();
    let cmd = logger.next()?;
    let args: Vec<_> = logger.collect();
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8(out.stdout).ok()?)
}

fn wrap_string(string: &str, with: &str, count: usize) -> String {
    let wrap = with.repeat(count);
    format!("{} {} {}", wrap, string, wrap)
}
