use once_cell::sync::Lazy;
use serde_derive::*;
use std::{fs::OpenOptions, io::Read};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    mirrors: Vec<String>,
}

impl Config {
    /// Get a reference to the config's mirrors.
    pub fn mirrors(&self) -> &Vec<String> {
        &self.mirrors
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut s = String::new();
    let mut f = OpenOptions::new()
        .write(false)
        .read(true)
        .append(false)
        .open("susrc")
        .expect("Failed to open susrc");
    f.read_to_string(&mut s).expect("Failed to read susrc");
    let out: Config = toml::from_str(&s).unwrap();
    out
});
