use clap::{Arg, Command};
use colored::*;
use std::process;
use std::io::{self, Write};

mod config;
mod display;
mod error;
mod info;
mod logo;
mod tdl;
mod themes;
mod utils;

use crate::config::Config;
use crate::display::DisplayManager;
use crate::error::RFetchError;
use crate::info::SystemInfo;
use crate::tdl::{TdlParser, TdlGenerator};
use crate::themes::{load_theme, list_themes};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error".red().bold(), e);
        process::exit(1);
    }
}

fn run() -> Result<(), RFetchError> {
    let matches = Command::new("rFetch")
        .version(env!("CARGO_PKG_VERSION"))
        .author("rFetch Team")
        .about("A fast and beautiful system information tool")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Use custom configuration file")
        )
        .arg(
            Arg::new("theme")
                .short('t')
                .long("theme")
                .value_name("THEME")
                .help("Use a specific theme (default, minimal, neon, retro) or path to custom TDL file")
        )
        .arg(
            Arg::new("list-themes")
                .long("list-themes")
                .help("List all available themes")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("generate-template")
                .long("generate-template")
                .value_name("FORMAT")
                .help("Generate a TDL theme template (json, yaml, toml)")
        )
        .arg(
            Arg::new("logo")
                .short('l')
                .long("logo")
                .value_name("LOGO")
                .help("Display specific logo (auto, ascii, small, none)")
        )
        .arg(
            Arg::new("color")
                .long("color")
                .value_name("WHEN")
                .help("When to use colors (auto, always, never)")
                .default_value("auto")
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output in JSON format")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("minimal")
                .short('m')
                .long("minimal")
                .help("Show minimal information")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show verbose information")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("clear")
                .long("clr")
                .help("Clear terminal before displaying information")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if let Some(format) = matches.get_one::<String>("generate-template") {
        let template = TdlGenerator::generate_template(format);
        println!("{}", template);
        return Ok(());
    }

    if matches.get_flag("list-themes") {
        println!("Available themes:");
        for theme_name in list_themes() {
            if let Some(theme) = load_theme(&theme_name) {
                println!("  {} - {}", theme_name.cyan().bold(), theme.description);
            }
        }
        return Ok(());
    }

    let config_path = matches.get_one::<String>("config");
    let mut config = Config::load(config_path)?;

    let mut loaded_theme: Option<themes::Theme> = None;
    if let Some(theme_name) = matches.get_one::<String>("theme") {
        let theme = if std::path::Path::new(theme_name).exists() {
            match TdlParser::parse_file(theme_name) {
                Ok(tdl_theme) => Some(TdlParser::to_theme(tdl_theme)),
                Err(e) => {
                    eprintln!("{}: Failed to load custom theme '{}': {}", 
                             "Error".red().bold(), theme_name, e);
                    None
                }
            }
        } else {
            load_theme(theme_name)
        };

        if let Some(theme) = theme {
            config.colors.title = theme.colors.title.clone();
            config.colors.subtitle = theme.colors.subtitle.clone();
            config.colors.key = theme.colors.key.clone();
            config.colors.value = theme.colors.value.clone();
            config.colors.separator = theme.colors.separator.clone();
            config.colors.logo = theme.colors.logo.clone();
            
            config.display.logo_type = theme.display.logo_type.clone();
            config.display.separator = theme.display.separator.clone();
            config.display.padding = theme.display.padding;
            
            loaded_theme = Some(theme);
        } else if !std::path::Path::new(theme_name).exists() {
            eprintln!("{}: Unknown theme '{}'. Use --list-themes to see available themes.", 
                     "Warning".yellow().bold(), theme_name);
        }
    }

    if let Some(logo) = matches.get_one::<String>("logo") {
        config.display.logo_type = logo.clone();
    }
    
    if let Some(color) = matches.get_one::<String>("color") {
        config.display.color_mode = color.clone();
    }

    if matches.get_flag("json") {
        config.display.output_format = "json".to_string();
    }

    if matches.get_flag("minimal") {
        config.display.minimal = true;
        config.apply_minimal();
    }

    if matches.get_flag("verbose") {
        config.display.verbose = true;
        config.apply_verbose();
    }

    if matches.get_flag("clear") {
        clear_terminal();
    }

    let system_info = SystemInfo::gather(&config)?;

    let display_manager = if let Some(ref theme) = loaded_theme {
        DisplayManager::with_theme(&config, theme)
    } else {
        DisplayManager::new(&config)
    };
    display_manager.display(&system_info)?;

    Ok(())
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap_or(());
    
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cls").status();
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}