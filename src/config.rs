use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::error::RFetchError;
use crate::themes::AdvancedColor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub info: InfoConfig,
    pub colors: ColorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub logo_type: String,
    pub color_mode: String,
    pub output_format: String,
    pub minimal: bool,
    pub verbose: bool,
    pub separator: String,
    pub padding: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoConfig {
    pub show_os: bool,
    pub show_kernel: bool,
    pub show_uptime: bool,
    pub show_packages: bool,
    pub show_shell: bool,
    pub show_resolution: bool,
    pub show_de: bool,
    pub show_wm: bool,
    pub show_theme: bool,
    pub show_icons: bool,
    pub show_font: bool,
    pub show_cursor: bool,
    pub show_terminal: bool,
    pub show_cpu: bool,
    pub show_gpu: bool,
    pub show_memory: bool,
    pub show_disk: bool,
    pub show_battery: bool,
    pub show_locale: bool,
    pub show_local_ip: bool,
    pub show_public_ip: bool,
    pub show_users: bool,
    pub show_date: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub title: AdvancedColor,
    pub subtitle: AdvancedColor,
    pub key: AdvancedColor,
    pub value: AdvancedColor,
    pub separator: AdvancedColor,
    pub logo: AdvancedColor,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                logo_type: "auto".to_string(),
                color_mode: "auto".to_string(),
                output_format: "normal".to_string(),
                minimal: false,
                verbose: false,
                separator: ": ".to_string(),
                padding: 2,
            },
            info: InfoConfig {
                show_os: true,
                show_kernel: true,
                show_uptime: true,
                show_packages: true,
                show_shell: true,
                show_resolution: true,
                show_de: true,
                show_wm: true,
                show_theme: false,
                show_icons: false,
                show_font: false,
                show_cursor: false,
                show_terminal: true,
                show_cpu: true,
                show_gpu: true,
                show_memory: true,
                show_disk: true,
                show_battery: true,
                show_locale: false,
                show_local_ip: false,
                show_public_ip: false,
                show_users: false,
                show_date: true,
            },
            colors: ColorConfig {
                title: AdvancedColor::from("cyan"),
                subtitle: AdvancedColor::from("blue"),
                key: AdvancedColor::from("yellow"),
                value: AdvancedColor::from("white"),
                separator: AdvancedColor::from("white"),
                logo: AdvancedColor::from("cyan"),
            },
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&String>) -> Result<Self, RFetchError> {
        let config_file = if let Some(path) = config_path {
            path.clone()
        } else {
            Self::default_config_path()?
        };

        if Path::new(&config_file).exists() {
            let content = fs::read_to_string(&config_file)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: Option<&str>) -> Result<(), RFetchError> {
        let config_file = if let Some(path) = path {
            path.to_string()
        } else {
            Self::default_config_path()?
        };

        let content = toml::to_string_pretty(self)
            .map_err(|e| RFetchError::config(format!("Failed to serialize config: {}", e)))?;

        if let Some(parent) = Path::new(&config_file).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&config_file, content)?;
        Ok(())
    }

    fn default_config_path() -> Result<String, RFetchError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| RFetchError::config("Could not find config directory"))?;
        
        Ok(config_dir
            .join("rfetch")
            .join("config.toml")
            .to_string_lossy()
            .to_string())
    }

    pub fn apply_minimal(&mut self) {
        self.info.show_theme = false;
        self.info.show_icons = false;
        self.info.show_font = false;
        self.info.show_cursor = false;
        self.info.show_locale = false;
        self.info.show_local_ip = false;
        self.info.show_public_ip = false;
        self.info.show_users = false;
        self.display.logo_type = "small".to_string();
    }

    pub fn apply_verbose(&mut self) {
        self.info.show_theme = true;
        self.info.show_icons = true;
        self.info.show_font = true;
        self.info.show_cursor = true;
        self.info.show_locale = true;
        self.info.show_local_ip = true;
        self.info.show_users = true;
    }
}