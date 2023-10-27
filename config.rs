use std::collections::HashMap;
use serde::Deserialize;
use dirs::home_dir;

#[derive(Deserialize, Clone)]
pub struct Profile {
    pub dir: String,
    pub cmds: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub windows: Vec<String>,
    pub fresh: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub presets: HashMap<String, Preset>,
    pub fresh_cmds: Vec<String>,
}

pub fn get_config() -> Config {
    let root = home_dir().expect("failed to find home directory");
    let path = format!("{}/.config/le-config.yaml", root.display());
    let file = std::fs::File::open(path).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(file).expect("Could not read values.");
    return config;
}

impl Config {
    pub fn get_preset(&self, name: String) -> Preset {
        self.presets
            .get(&name)
            .expect(&format!("Error: Could not find preset \"{}\"", &name))
            .clone()
    }

    pub fn get_profile(&self, name: &str) -> Profile {
        self.profiles
            .get(name)
            .expect(&format!("Error: Could not find profile \"{}\"", &name))
            .clone()
    }
}
