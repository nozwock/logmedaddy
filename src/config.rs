use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
    loggers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profiles: vec![Profile {
                name: "systemd".to_owned(),
                loggers: vec![
                    "journalctl -b".to_owned(),
                    "systemd-analyze blame".to_owned(),
                ],
            }],
        }
    }
}
