use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_threshold_mins: u64,
    pub duration: Option<String>,
    #[serde(default)]
    pub daily_goal_hours: f64,
    #[serde(default)]
    pub show_timer_in_menubar: bool,
    #[serde(default = "default_show_state_icon")]
    pub show_state_icon: bool,
    #[serde(default)]
    pub launch_at_login: bool,
    #[serde(default = "default_auto_check_updates")]
    pub auto_check_updates: bool,
    #[serde(default = "default_show_motivational_messages")]
    pub show_motivational_messages: bool,
}

fn default_show_state_icon() -> bool {
    true
}

fn default_auto_check_updates() -> bool {
    true
}

fn default_show_motivational_messages() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_threshold_mins: 5,
            duration: None,
            daily_goal_hours: 4.0,
            show_timer_in_menubar: false,
            show_state_icon: true,
            launch_at_login: false,
            auto_check_updates: true,
            show_motivational_messages: true,
        }
    }
}

pub fn load_config() -> Result<Config> {
    let mut path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    path.push(".neflo");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    path.push("config.json");

    if !path.exists() {
        let config = Config::default();
        let data = serde_json::to_string_pretty(&config)?;
        fs::write(&path, data)?;
        return Ok(config);
    }

    let data = fs::read_to_string(&path)?;
    let config = serde_json::from_str(&data)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let mut path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    path.push(".neflo");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    path.push("config.json");

    let data = serde_json::to_string_pretty(config)?;
    fs::write(&path, data)?;
    Ok(())
}
