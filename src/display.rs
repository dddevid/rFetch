use colored::*;
use serde_json;
use std::cmp;
use unicode_width::UnicodeWidthChar;

use crate::config::Config;
use crate::error::RFetchError;
use crate::info::SystemInfo;
use crate::logo::{get_logo, get_color_codes};
use crate::themes::Theme;
use crate::utils::*;

pub struct DisplayManager<'a> {
    config: &'a Config,
    theme: Option<&'a Theme>,
}

impl<'a> DisplayManager<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config, theme: None }
    }

    pub fn with_theme(config: &'a Config, theme: &'a Theme) -> Self {
        Self { config, theme: Some(theme) }
    }

    pub fn display(&self, info: &SystemInfo) -> Result<(), RFetchError> {
        match self.config.display.output_format.as_str() {
            "json" => self.display_json(info),
            _ => self.display_normal(info),
        }
    }

    fn display_json(&self, info: &SystemInfo) -> Result<(), RFetchError> {
        let json = serde_json::to_string_pretty(info)?;
        println!("{}", json);
        Ok(())
    }

    fn display_normal(&self, info: &SystemInfo) -> Result<(), RFetchError> {
        let logo_lines = self.get_logo_lines(&info.os);
        let info_lines = self.build_info_lines(info);

        let max_logo_width = self.calculate_max_visual_width(&logo_lines);
        let max_lines = cmp::max(logo_lines.len(), info_lines.len());

        if !self.config.display.minimal {
            let title = format!("{}@{}", info.user, info.hostname);
            let separator = "─".repeat(title.len());
            
            if self.should_use_colors() {
                println!("{}", title.cyan().bold());
                println!("{}", separator.cyan());
            } else {
                println!("{}", title);
                println!("{}", separator);
            }
            println!();
        }

        for i in 0..max_lines {
            let empty_string = String::new();
            let logo_line = logo_lines.get(i).unwrap_or(&empty_string);
            let info_line = info_lines.get(i).unwrap_or(&empty_string);

            let visual_width = self.calculate_visual_width(logo_line);
            let padding_needed = if max_logo_width > visual_width {
                max_logo_width - visual_width
            } else {
                0
            };

            if self.should_use_colors() {
                print!("{}", logo_line.color(self.config.colors.logo.base.as_str()));
            } else {
                print!("{}", logo_line);
            }

            print!("{}", " ".repeat(padding_needed));
            print!("{}", " ".repeat(self.config.display.padding));
            println!("{}", info_line);
        }

        if !self.config.display.minimal {
            println!();
            self.display_color_bar(&info.colors, max_logo_width)?;
        }

        Ok(())
    }

    fn calculate_max_visual_width(&self, lines: &[String]) -> usize {
        lines.iter()
            .map(|line| self.calculate_visual_width(line))
            .max()
            .unwrap_or(0)
    }

    fn calculate_visual_width(&self, line: &str) -> usize {
        let cleaned = self.strip_ansi_codes(line);
        
        cleaned.chars().map(|c| {
            match c {
                '\u{2500}'..='\u{257F}' => 1,
                '\u{2580}'..='\u{259F}' => 1,
                '\u{25A0}'..='\u{25FF}' => 1,
                '\u{2600}'..='\u{26FF}' => 2,
                '\u{2700}'..='\u{27BF}' => 1,
                '\u{1F300}'..='\u{1F9FF}' => 2,
                _ => {
                    UnicodeWidthChar::width(c).unwrap_or(1)
                }
            }
        }).sum()
    }

    fn strip_ansi_codes(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                if chars.peek() == Some(&'[') {
                    chars.next();
                    while let Some(next_ch) = chars.next() {
                        if next_ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    fn get_logo_lines(&self, os_name: &str) -> Vec<String> {
        if let Some(theme) = self.theme {
            if let Some(ascii) = &theme.ascii {
                match self.config.display.logo_type.as_str() {
                    "ascii" | "auto" => {
                        return ascii.lines().map(|s| s.to_string()).collect();
                    }
                    "none" => return vec![],
                    _ => {}
                }
            }
        }
        
        get_logo(os_name, &self.config.display.logo_type)
    }

    fn build_info_lines(&self, info: &SystemInfo) -> Vec<String> {
        let mut lines = Vec::new();

        if self.config.info.show_os && !info.os.is_empty() {
            lines.push(self.format_info_line("OS", &info.os));
        }

        if self.config.info.show_kernel && !info.kernel.is_empty() {
            lines.push(self.format_info_line("Kernel", &info.kernel));
        }

        if self.config.info.show_uptime && !info.uptime.is_empty() {
            lines.push(self.format_info_line("Uptime", &info.uptime));
        }

        if self.config.info.show_packages && info.packages > 0 {
            lines.push(self.format_info_line("Packages", &info.packages.to_string()));
        }

        if self.config.info.show_shell && !info.shell.is_empty() {
            lines.push(self.format_info_line("Shell", &info.shell));
        }

        if self.config.info.show_resolution && !info.resolution.is_empty() && info.resolution != "unknown" {
            lines.push(self.format_info_line("Resolution", &info.resolution));
        }

        if self.config.info.show_de && !info.desktop_environment.is_empty() && info.desktop_environment != "unknown" {
            lines.push(self.format_info_line("DE", &info.desktop_environment));
        }

        if self.config.info.show_wm && !info.window_manager.is_empty() && info.window_manager != "unknown" {
            lines.push(self.format_info_line("WM", &info.window_manager));
        }

        if self.config.info.show_terminal && !info.terminal.is_empty() && info.terminal != "unknown" {
            lines.push(self.format_info_line("Terminal", &info.terminal));
        }

        if self.config.info.show_cpu && !info.cpu.is_empty() && info.cpu != "unknown" {
            lines.push(self.format_info_line("CPU", &info.cpu));
        }

        if self.config.info.show_gpu && !info.gpu.is_empty() && info.gpu != "unknown" {
            lines.push(self.format_info_line("GPU", &info.gpu));
        }

        if self.config.info.show_memory && info.memory.total > 0 {
            let memory_str = format!(
                "{} / {} ({}%)",
                format_bytes(info.memory.used),
                format_bytes(info.memory.total),
                info.memory.percentage as u8
            );
            lines.push(self.format_info_line("Memory", &memory_str));
        }

        if self.config.info.show_disk && !info.disk.is_empty() {
            for disk in &info.disk {
                if disk.mount_point == "/" || disk.mount_point == "C:\\" {
                    let disk_str = format!(
                        "{} / {} ({}%)",
                        format_bytes(disk.used),
                        format_bytes(disk.total),
                        disk.percentage as u8
                    );
                    lines.push(self.format_info_line("Disk", &disk_str));
                    break;
                }
            }
        }

        if self.config.info.show_battery {
            if let Some(battery) = &info.battery {
                let battery_str = format!("{}% ({})", battery.percentage, battery.status);
                lines.push(self.format_info_line("Battery", &battery_str));
            }
        }

        if self.config.info.show_locale && !info.locale.is_empty() && info.locale != "unknown" {
            lines.push(self.format_info_line("Locale", &info.locale));
        }

        if self.config.info.show_local_ip && !info.local_ip.is_empty() && info.local_ip != "unknown" {
            lines.push(self.format_info_line("Local IP", &info.local_ip));
        }

        if self.config.info.show_users && !info.users.is_empty() {
            let users_str = info.users.join(", ");
            lines.push(self.format_info_line("Users", &users_str));
        }

        if self.config.info.show_date {
            lines.push(self.format_info_line("Date", &info.date));
        }

        lines
    }

    fn format_info_line(&self, key: &str, value: &str) -> String {
        let separator = &self.config.display.separator;
        
        if self.should_use_colors() {
            format!(
                "{}{}{}",
                key.color(self.config.colors.key.base.as_str()).bold(),
                separator.color(self.config.colors.separator.base.as_str()),
                value.color(self.config.colors.value.base.as_str())
            )
        } else {
            format!("{}{}{}", key, separator, value)
        }
    }

    fn display_color_bar(&self, colors: &[String], logo_width: usize) -> Result<(), RFetchError> {
        if !self.should_use_colors() {
            return Ok(());
        }

        let color_names = [
            "black", "red", "green", "yellow", 
            "blue", "magenta", "cyan", "white"
        ];

        let padding = " ".repeat(logo_width + self.config.display.padding);
        print!("{}", padding);

        for (i, color_name) in color_names.iter().enumerate() {
            if i < colors.len() {
                print!("{}", "██".color(*color_name));
            }
        }
        println!();

        print!("{}", padding);
        for (i, color_name) in color_names.iter().enumerate() {
            if i < colors.len() {
                let bright_color = format!("bright_{}", color_name);
                print!("{}", "██".color(bright_color.as_str()));
            }
        }
        println!();

        Ok(())
    }

    fn should_use_colors(&self) -> bool {
        match self.config.display.color_mode.as_str() {
            "always" => true,
            "never" => false,
            "auto" => atty::is(atty::Stream::Stdout),
            _ => true,
        }
    }
}

trait ColorExt {
    fn color(&self, color_name: &str) -> ColoredString;
}

impl ColorExt for &str {
    fn color(&self, color_name: &str) -> ColoredString {
        match color_name {
            "black" => self.black(),
            "red" => self.red(),
            "green" => self.green(),
            "yellow" => self.yellow(),
            "blue" => self.blue(),
            "magenta" => self.magenta(),
            "purple" => self.purple(),
            "cyan" => self.cyan(),
            "white" => self.white(),
            "bright_black" => self.bright_black(),
            "bright_red" => self.bright_red(),
            "bright_green" => self.bright_green(),
            "bright_yellow" => self.bright_yellow(),
            "bright_blue" => self.bright_blue(),
            "bright_magenta" => self.bright_magenta(),
            "bright_purple" => self.bright_purple(),
            "bright_cyan" => self.bright_cyan(),
            "bright_white" => self.bright_white(),
            _ => self.normal(),
        }
    }
}

impl ColorExt for String {
    fn color(&self, color_name: &str) -> ColoredString {
        self.as_str().color(color_name)
    }
}