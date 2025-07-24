use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::themes::{Theme, AdvancedColor, ThemeColors, ThemeDisplay, CustomAscii, ThemeEffects, ColorEffect, Animation, AnimationType};

// Theme Definition Language (TDL) Parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlTheme {
    pub meta: TdlMeta,
    pub colors: TdlColors,
    pub display: TdlDisplay,
    pub layout: TdlLayout,
    pub effects: Option<TdlEffects>,
    pub ascii: Option<TdlAscii>,
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlMeta {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlColors {
    pub title: TdlColor,
    pub subtitle: TdlColor,
    pub key: TdlColor,
    pub value: TdlColor,
    pub separator: TdlColor,
    pub logo: TdlColor,
    pub accent: TdlColor,
    pub background: Option<TdlColor>,
    pub border: Option<TdlColor>,
    pub highlight: Option<TdlColor>,
    pub error: Option<TdlColor>,
    pub warning: Option<TdlColor>,
    pub success: Option<TdlColor>,
    pub custom_colors: Option<HashMap<String, TdlColor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlColor {
    pub base: String,
    pub rgb: Option<[u8; 3]>,
    pub hex: Option<String>,
    pub effects: Option<Vec<String>>,
    pub animation: Option<TdlAnimation>,
    pub gradient: Option<TdlGradient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlAnimation {
    pub animation_type: String,
    pub duration: f32,
    pub repeat: bool,
    pub easing: Option<String>,
    pub delay: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlGradient {
    pub to_color: String,
    pub direction: Option<String>,
    pub stops: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlDisplay {
    pub logo_type: String,
    pub separator: String,
    pub padding: usize,
    pub show_borders: bool,
    pub show_color_bar: bool,
    pub color_bar_style: String,
    pub alignment: String,
    pub max_width: Option<usize>,
    pub line_spacing: f32,
    pub indent: usize,
    pub show_icons: bool,
    pub icon_style: String,
    pub border_style: Option<String>,
    pub corner_style: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlLayout {
    pub info_order: Vec<String>,
    pub custom_labels: Option<HashMap<String, String>>,
    pub sections: Option<Vec<TdlSection>>,
    pub columns: Option<usize>,
    pub responsive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlSection {
    pub name: String,
    pub title: Option<String>,
    pub items: Vec<String>,
    pub style: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlEffects {
    pub transitions: bool,
    pub shadows: bool,
    pub glow_intensity: f32,
    pub particle_effects: bool,
    pub sound_effects: bool,
    pub terminal_title: Option<String>,
    pub cursor_style: Option<String>,
    pub typing_effect: Option<bool>,
    pub fade_in: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlAscii {
    pub logo: Vec<String>,
    pub small_logo: Vec<String>,
    pub decorations: Option<HashMap<String, String>>,
    pub frames: Option<Vec<Vec<String>>>, // For animated ASCII
    pub frame_delay: Option<f32>,
}

// TDL Parser implementation
pub struct TdlParser;

impl TdlParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<TdlTheme, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Self::parse_string(&content)
    }

    pub fn parse_string(content: &str) -> Result<TdlTheme, Box<dyn std::error::Error>> {
        // Support multiple formats
        if content.trim_start().starts_with('{') {
            // JSON format
            Ok(serde_json::from_str(content)?)
        } else if content.contains("name =") || content.contains("[meta]") {
            // TOML format
            Ok(toml::from_str(content)?)
        } else {
            // YAML format
            Ok(serde_yaml::from_str(content)?)
        }
    }

    pub fn to_theme(tdl: TdlTheme) -> Theme {
        Theme {
            name: tdl.meta.name,
            description: tdl.meta.description,
            version: tdl.meta.version,
            author: tdl.meta.author,
            colors: Self::convert_colors(tdl.colors),
            display: Self::convert_display(tdl.display),
            info_order: tdl.layout.info_order,
            custom_labels: tdl.layout.custom_labels.unwrap_or_default(),
            custom_ascii: tdl.ascii.map(Self::convert_ascii),
            effects: Self::convert_effects(tdl.effects),
        }
    }

    fn convert_colors(colors: TdlColors) -> ThemeColors {
        ThemeColors {
            title: Self::convert_color(colors.title),
            subtitle: Self::convert_color(colors.subtitle),
            key: Self::convert_color(colors.key),
            value: Self::convert_color(colors.value),
            separator: Self::convert_color(colors.separator),
            logo: Self::convert_color(colors.logo),
            accent: Self::convert_color(colors.accent),
            background: colors.background.map(Self::convert_color),
            border: colors.border.map(Self::convert_color),
            highlight: colors.highlight.map(Self::convert_color),
            error: colors.error.map(Self::convert_color),
            warning: colors.warning.map(Self::convert_color),
            success: colors.success.map(Self::convert_color),
        }
    }

    fn convert_color(color: TdlColor) -> AdvancedColor {
        let mut advanced_color = AdvancedColor::new(&color.base);

        // Set RGB from array or hex
        if let Some(rgb) = color.rgb {
            advanced_color.rgb = Some((rgb[0], rgb[1], rgb[2]));
        } else if let Some(hex) = color.hex {
            if let Ok(rgb) = Self::hex_to_rgb(&hex) {
                advanced_color.rgb = Some(rgb);
            }
        }

        // Convert effects
        if let Some(effects) = color.effects {
            for effect_str in effects {
                if let Some(effect) = Self::string_to_effect(&effect_str) {
                    advanced_color.effects.push(effect);
                }
            }
        }

        // Convert animation
        if let Some(anim) = color.animation {
            advanced_color.animation = Some(Animation {
                animation_type: Self::string_to_animation_type(&anim.animation_type),
                duration: anim.duration,
                repeat: anim.repeat,
                easing: anim.easing.unwrap_or_else(|| "ease-in-out".to_string()),
            });
        }

        // Handle gradient
        if let Some(gradient) = color.gradient {
            advanced_color.effects.push(ColorEffect::Gradient(gradient.to_color));
        }

        advanced_color
    }

    fn convert_display(display: TdlDisplay) -> ThemeDisplay {
        ThemeDisplay {
            logo_type: display.logo_type,
            separator: display.separator,
            padding: display.padding,
            show_borders: display.show_borders,
            show_color_bar: display.show_color_bar,
            color_bar_style: display.color_bar_style,
            alignment: display.alignment,
            max_width: display.max_width,
            line_spacing: display.line_spacing,
            indent: display.indent,
            show_icons: display.show_icons,
            icon_style: display.icon_style,
        }
    }

    fn convert_ascii(ascii: TdlAscii) -> CustomAscii {
        CustomAscii {
            logo: ascii.logo,
            small_logo: ascii.small_logo,
            decorations: ascii.decorations.unwrap_or_default(),
        }
    }

    fn convert_effects(effects: Option<TdlEffects>) -> ThemeEffects {
        if let Some(effects) = effects {
            ThemeEffects {
                transitions: effects.transitions,
                shadows: effects.shadows,
                glow_intensity: effects.glow_intensity,
                particle_effects: effects.particle_effects,
                sound_effects: effects.sound_effects,
                terminal_title: effects.terminal_title,
            }
        } else {
            ThemeEffects {
                transitions: false,
                shadows: false,
                glow_intensity: 0.0,
                particle_effects: false,
                sound_effects: false,
                terminal_title: None,
            }
        }
    }

    fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color format".into());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        Ok((r, g, b))
    }

    fn string_to_effect(effect: &str) -> Option<ColorEffect> {
        match effect.to_lowercase().as_str() {
            "bold" => Some(ColorEffect::Bold),
            "italic" => Some(ColorEffect::Italic),
            "underline" => Some(ColorEffect::Underline),
            "strikethrough" => Some(ColorEffect::Strikethrough),
            "blink" => Some(ColorEffect::Blink),
            "reverse" => Some(ColorEffect::Reverse),
            "glow" => Some(ColorEffect::Glow),
            "shadow" => Some(ColorEffect::Shadow),
            _ => None,
        }
    }

    fn string_to_animation_type(anim_type: &str) -> AnimationType {
        match anim_type.to_lowercase().as_str() {
            "fade" => AnimationType::Fade,
            "pulse" => AnimationType::Pulse,
            "rainbow" => AnimationType::Rainbow,
            "typewriter" => AnimationType::Typewriter,
            "slide" => AnimationType::Slide,
            "bounce" => AnimationType::Bounce,
            _ => AnimationType::Pulse,
        }
    }
}

// Theme generator for creating TDL templates
pub struct TdlGenerator;

impl TdlGenerator {
    pub fn generate_template(format: &str) -> String {
        match format.to_lowercase().as_str() {
            "json" => Self::generate_json_template(),
            "toml" => Self::generate_toml_template(),
            "yaml" | "yml" => Self::generate_yaml_template(),
            _ => Self::generate_yaml_template(),
        }
    }

    fn generate_json_template() -> String {
        serde_json::to_string_pretty(&Self::create_template()).unwrap_or_default()
    }

    fn generate_toml_template() -> String {
        toml::to_string_pretty(&Self::create_template()).unwrap_or_default()
    }

    fn generate_yaml_template() -> String {
        serde_yaml::to_string(&Self::create_template()).unwrap_or_default()
    }

    fn create_template() -> TdlTheme {
        TdlTheme {
            meta: TdlMeta {
                name: "my_theme".to_string(),
                description: "My custom theme".to_string(),
                version: "1.0.0".to_string(),
                author: Some("Your Name".to_string()),
                license: Some("MIT".to_string()),
                tags: Some(vec!["custom".to_string(), "colorful".to_string()]),
                created: Some("2024-01-01".to_string()),
                updated: Some("2024-01-01".to_string()),
            },
            colors: TdlColors {
                title: TdlColor {
                    base: "cyan".to_string(),
                    rgb: Some([0, 255, 255]),
                    hex: Some("#00FFFF".to_string()),
                    effects: Some(vec!["bold".to_string(), "glow".to_string()]),
                    animation: Some(TdlAnimation {
                        animation_type: "pulse".to_string(),
                        duration: 2.0,
                        repeat: true,
                        easing: Some("ease-in-out".to_string()),
                        delay: None,
                    }),
                    gradient: None,
                },
                subtitle: TdlColor {
                    base: "blue".to_string(),
                    rgb: None,
                    hex: None,
                    effects: None,
                    animation: None,
                    gradient: None,
                },
                key: TdlColor {
                    base: "yellow".to_string(),
                    rgb: None,
                    hex: None,
                    effects: Some(vec!["bold".to_string()]),
                    animation: None,
                    gradient: None,
                },
                value: TdlColor {
                    base: "white".to_string(),
                    rgb: None,
                    hex: None,
                    effects: None,
                    animation: None,
                    gradient: None,
                },
                separator: TdlColor {
                    base: "white".to_string(),
                    rgb: None,
                    hex: None,
                    effects: None,
                    animation: None,
                    gradient: None,
                },
                logo: TdlColor {
                    base: "cyan".to_string(),
                    rgb: None,
                    hex: None,
                    effects: Some(vec!["glow".to_string()]),
                    animation: None,
                    gradient: None,
                },
                accent: TdlColor {
                    base: "magenta".to_string(),
                    rgb: None,
                    hex: None,
                    effects: None,
                    animation: None,
                    gradient: None,
                },
                background: None,
                border: None,
                highlight: None,
                error: None,
                warning: None,
                success: None,
                custom_colors: None,
            },
            display: TdlDisplay {
                logo_type: "auto".to_string(),
                separator: ": ".to_string(),
                padding: 2,
                show_borders: false,
                show_color_bar: true,
                color_bar_style: "blocks".to_string(),
                alignment: "left".to_string(),
                max_width: None,
                line_spacing: 1.0,
                indent: 0,
                show_icons: false,
                icon_style: "unicode".to_string(),
                border_style: None,
                corner_style: None,
            },
            layout: TdlLayout {
                info_order: vec![
                    "os".to_string(),
                    "kernel".to_string(),
                    "uptime".to_string(),
                    "packages".to_string(),
                    "shell".to_string(),
                    "terminal".to_string(),
                    "cpu".to_string(),
                    "gpu".to_string(),
                    "memory".to_string(),
                    "disk".to_string(),
                    "battery".to_string(),
                    "date".to_string(),
                ],
                custom_labels: None,
                sections: None,
                columns: None,
                responsive: None,
            },
            effects: Some(TdlEffects {
                transitions: false,
                shadows: false,
                glow_intensity: 0.0,
                particle_effects: false,
                sound_effects: false,
                terminal_title: None,
                cursor_style: None,
                typing_effect: None,
                fade_in: None,
            }),
            ascii: Some(TdlAscii {
                logo: vec![
                    "  ██████╗ ███████╗███████╗████████╗ ██████╗██╗  ██╗".to_string(),
                    "  ██╔══██╗██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║".to_string(),
                    "  ██████╔╝█████╗  █████╗     ██║   ██║     ███████║".to_string(),
                    "  ██╔══██╗██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║".to_string(),
                    "  ██║  ██║██║     ███████╗   ██║   ╚██████╗██║  ██║".to_string(),
                    "  ╚═╝  ╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝".to_string(),
                ],
                small_logo: vec!["🎨".to_string()],
                decorations: None,
                frames: None,
                frame_delay: None,
            }),
            custom: None,
        }
    }
}

// Theme validation
pub struct TdlValidator;

impl TdlValidator {
    pub fn validate(theme: &TdlTheme) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate meta
        if theme.meta.name.is_empty() {
            errors.push("Theme name cannot be empty".to_string());
        }

        if theme.meta.description.is_empty() {
            errors.push("Theme description cannot be empty".to_string());
        }

        // Validate version format
        if !Self::is_valid_version(&theme.meta.version) {
            errors.push("Invalid version format (use semantic versioning)".to_string());
        }

        // Validate colors
        Self::validate_color(&theme.colors.title, "title", &mut errors);
        Self::validate_color(&theme.colors.subtitle, "subtitle", &mut errors);
        Self::validate_color(&theme.colors.key, "key", &mut errors);
        Self::validate_color(&theme.colors.value, "value", &mut errors);

        // Validate display settings
        if theme.display.padding > 10 {
            errors.push("Padding should not exceed 10".to_string());
        }

        if theme.display.line_spacing < 0.5 || theme.display.line_spacing > 3.0 {
            errors.push("Line spacing should be between 0.5 and 3.0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_color(color: &TdlColor, name: &str, errors: &mut Vec<String>) {
        if color.base.is_empty() {
            errors.push(format!("Color '{}' base cannot be empty", name));
        }

        if let Some(hex) = &color.hex {
            if !Self::is_valid_hex(hex) {
                errors.push(format!("Invalid hex color for '{}'", name));
            }
        }

        if let Some(rgb) = &color.rgb {
            if rgb.len() != 3 {
                errors.push(format!("RGB array for '{}' must have exactly 3 values", name));
            }
        }
    }

    fn is_valid_version(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() == 3 && parts.iter().all(|part| part.parse::<u32>().is_ok())
    }

    fn is_valid_hex(hex: &str) -> bool {
        let hex = hex.trim_start_matches('#');
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(TdlParser::hex_to_rgb("#FF0000").unwrap(), (255, 0, 0));
        assert_eq!(TdlParser::hex_to_rgb("00FF00").unwrap(), (0, 255, 0));
        assert_eq!(TdlParser::hex_to_rgb("#0000FF").unwrap(), (0, 0, 255));
    }

    #[test]
    fn test_template_generation() {
        let template = TdlGenerator::create_template();
        assert_eq!(template.meta.name, "my_theme");
        assert!(!template.colors.title.base.is_empty());
    }

    #[test]
    fn test_validation() {
        let template = TdlGenerator::create_template();
        assert!(TdlValidator::validate(&template).is_ok());
    }
}