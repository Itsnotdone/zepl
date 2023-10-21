use serde::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub game: GameConfig,
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub name: String,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub dylib: String,
    pub main: String,
}
