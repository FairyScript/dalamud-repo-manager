use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub enable: bool,
    pub priority: i16,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub repo: Vec<Repo>,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    //let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let config: Config = toml::from_str(contents.as_str())?;

    Ok(config)
}
