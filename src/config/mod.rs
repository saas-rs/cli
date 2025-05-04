use crate::consts;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub api_url: Option<String>,
}

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    #[cfg(target_family = "unix")]
    let d = dirs::home_dir(); // ~

    #[cfg(target_family = "windows")]
    let d = dirs::data_dir(); // %APPDATA%

    match d {
        None => Err("Could not locate home dir".into()),
        Some(home_dir_path_buf) => {
            let home_dir = home_dir_path_buf.as_path();

            let mut path_buf = PathBuf::new();
            path_buf.push(home_dir);
            #[cfg(target_family = "unix")]
            path_buf.push(format!(".{}", consts::BRAND));
            #[cfg(target_family = "windows")]
            path_buf.push(consts::BRAND);
            path_buf.push("config");

            match File::open(path_buf.as_path()) {
                Ok(f) => {
                    if let Ok(config) = serde_yaml::from_reader(f) {
                        return Ok(config);
                    }
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => return Err(Box::new(e)),
                },
            };

            Ok(Config::default())
        }
    }
}

pub fn save(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_family = "unix")]
    let d = dirs::home_dir(); // ~

    #[cfg(target_family = "windows")]
    let d = dirs::data_dir(); // %APPDATA%

    match d {
        None => Err("Could not locate home dir".into()),
        Some(home_dir_path_buf) => {
            let home_dir = home_dir_path_buf.as_path();

            // Create ~/.<brand>/ if needed
            let mut path_buf = PathBuf::new();
            path_buf.push(home_dir);
            #[cfg(target_family = "unix")]
            path_buf.push(format!(".{}", consts::BRAND));
            #[cfg(target_family = "windows")]
            path_buf.push(consts::BRAND);
            std::fs::create_dir_all(path_buf.as_path())?;

            // Write ~/.<brand>/config
            path_buf.push("config");
            let mut f = File::create(path_buf.as_path())?;
            let mut s = serde_yaml::to_string(&config)?;
            s = format!("{s}\n");
            f.write_all(s.as_bytes())?;

            Ok(String::from(path_buf.as_path().to_string_lossy()))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load() {
        // load config
        let mut config = super::load().unwrap();
        let orig_api_key = config.api_key;

        // make a change and save
        config.api_key = Some("abc".to_string());
        super::save(&config).unwrap();

        // reload
        let config_reloaded = super::load().unwrap();
        assert!(config_reloaded.api_key.is_some());
        assert_eq!(config_reloaded.api_key.unwrap(), "abc");

        // restore original value
        config.api_key = orig_api_key;
        super::save(&config).unwrap();
    }
}
