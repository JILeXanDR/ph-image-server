use std::fs::File;
use std::io::Read;

use serde::Deserialize;

/// Configuration for app.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub listen: String,
    #[serde(rename(deserialize = "reportToV2"))]
    pub report_to_v2: String,
    pub metrics: Metrics,
}

impl Config {
    pub fn from_yaml_file_path(path: String) -> Result<Self, ErrKind> {
        load(path)
    }
}

/// Configuration for prometheus metrics.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Metrics {
    pub enabled: bool,
    pub addr: String,
}

#[derive(Debug)]
pub enum ErrKind {
    ReadError(std::io::Error),
    DecodeError(serde_yaml::Error),
}

impl From<std::io::Error> for ErrKind {
    fn from(err: std::io::Error) -> Self {
        ErrKind::ReadError(err)
    }
}

impl From<serde_yaml::Error> for ErrKind {
    fn from(err: serde_yaml::Error) -> Self {
        ErrKind::DecodeError(err)
    }
}

/// Build config using a YAML file loaded from given path.
pub fn load(path: String) -> Result<Config, ErrKind> {
    let mut file = File::open(path)?;

    let mut content: String = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = serde_yaml::from_str(content.as_str())?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file() {
        let result = load("wefwefwefwef.yaml".to_string());

        match result {
            Err(ErrKind::ReadError(_)) => {}
            Err(e) => panic!("unexpected error kind {:?}", e),
            _ => (),
        }
    }

    #[test]
    fn load_file_with_bad_yaml() {
        let result = load("testdata/bad_config.yaml".to_string());

        match result {
            Err(ErrKind::DecodeError(_)) => {}
            Err(e) => panic!("unexpected error kind {:?}", e),
            _ => (),
        }
    }

    #[test]
    fn load_valid_file() {
        let config = load("testdata/config.yaml".to_string()).expect("failed to load config");

        assert_eq!(
            config,
            Config {
                listen: "127.0.0.1:9123".to_string(),
                report_to_v2: "http://some-host:9999/report-stats-v2?token=token".to_string(),
                metrics: Metrics {
                    enabled: true,
                    addr: "127.0.0.1:9010".to_string()
                }
            }
        );
    }
}
