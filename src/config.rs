use std::fmt::Error;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub listen: String,
    pub memcached: Memcached,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Memcached {
    pub address: String,
}

// This function open file by given path, decodes YAML and returns it.
pub fn load(path: String) -> Result<Config, Error> {
    let mut file = File::open(path).unwrap();

    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("Loaded plain YAML {:?}", content);

    // TODO: how to return an error to handle it by caller func?
    let config: Config = serde_yaml::from_str(content.as_str()).expect("failed to decode YAML");

    return Ok(config);
}
