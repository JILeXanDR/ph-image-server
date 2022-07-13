use std::fs::File;
use std::io::Read;

use serde::Deserialize;

/// Config of app.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub listen: String,
    pub memcached: Memcached,
}

/// Configuration for memcache storage.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Memcached {
    pub address: String,
}

/// This function open file by given path, decodes YAML and returns it.
pub fn load(path: String) -> Result<Config, serde_yaml::Error> {
    let mut file = File::open(path).unwrap();

    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();

    return serde_yaml::from_str(content.as_str());
}
