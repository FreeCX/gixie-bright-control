use std::fs;

use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Coord {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Clock {
    pub timezone: i32,
    pub server: String,
    #[serde(default = "date_fmt_default")]
    pub date_fmt: String,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Brightness {
    pub min: u8,
    pub max: u8,
    pub step: u8,
    pub num: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub coord: Coord,
    pub clock: Clock,
    pub brightness: Brightness,
}

impl Config {
    pub fn load(filename: &str) -> Result<Config, String> {
        let data = fs::read_to_string(filename).map_err(|_| "Cannot load config file")?;
        Ok(serde_yaml::from_str(&data).map_err(|_| "Cannot parse config")?)
    }
}

fn date_fmt_default() -> String {
    "%Y-%m-%d %H:%M".to_string()
}
