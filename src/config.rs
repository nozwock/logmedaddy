use crate::defines::app_config_path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub loggers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profiles: vec![
                Profile {
                    name: "systemd".to_owned(),
                    loggers: vec![
                        "journalctl -b".to_owned(),
                        "systemd-analyze blame".to_owned(),
                    ],
                },
                Profile {
                    name: "hwinfo".to_owned(),
                    loggers: vec!["inxi -Fxxc0z".to_owned(), "hwinfo --short".to_owned()],
                },
                Profile {
                    name: "system".to_owned(),
                    loggers: vec!["cat /etc/fstab".to_owned(), "lsblk -f -o+SIZE".to_owned()],
                },
                Profile {
                    name: "boot".to_owned(),
                    loggers: vec!["sudo cat /boot/grub/grub.cfg".to_owned()],
                },
            ],
        }
    }
}

impl Config {
    pub fn load() -> Config {
        confy::load_path(app_config_path()).unwrap_or_default()
    }
    pub fn store(self) -> anyhow::Result<()> {
        Ok(confy::store_path(app_config_path(), self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_config() {
        confy::store_path("./target/logmedaddy.ron", Config::default()).unwrap();
    }
}
