use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Advanced color system with RGB, effects, and animations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedColor {
    pub base: String,           // Base color name or hex
    pub rgb: Option<(u8, u8, u8)>, // RGB values
    pub effects: Vec<ColorEffect>, // Visual effects
    pub animation: Option<Animation>, // Animation settings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorEffect {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Blink,
    Reverse,
    Glow,           // Neon glow effect
    Shadow,         // Text shadow
    Gradient(String), // Gradient to another color
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub duration: f32,      // Duration in seconds
    pub repeat: bool,       // Whether to repeat
    pub easing: String,     // Easing function
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    Fade,
    Pulse,
    Rainbow,
    Typewriter,
    Slide,
    Bounce,
}

// Enhanced theme structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub colors: ThemeColors,
    pub display: ThemeDisplay,
    pub info_order: Vec<String>,
    pub custom_labels: HashMap<String, String>,
    pub custom_ascii: Option<CustomAscii>,
    pub effects: ThemeEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub title: AdvancedColor,
    pub subtitle: AdvancedColor,
    pub key: AdvancedColor,
    pub value: AdvancedColor,
    pub separator: AdvancedColor,
    pub logo: AdvancedColor,
    pub accent: AdvancedColor,
    pub background: Option<AdvancedColor>,
    pub border: Option<AdvancedColor>,
    pub highlight: Option<AdvancedColor>,
    pub error: Option<AdvancedColor>,
    pub warning: Option<AdvancedColor>,
    pub success: Option<AdvancedColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDisplay {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAscii {
    pub logo: Vec<String>,
    pub small_logo: Vec<String>,
    pub decorations: HashMap<String, String>, // Custom decorative elements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEffects {
    pub transitions: bool,
    pub shadows: bool,
    pub glow_intensity: f32,
    pub particle_effects: bool,
    pub sound_effects: bool,
    pub terminal_title: Option<String>,
}

// Simple color for backward compatibility
impl From<String> for AdvancedColor {
    fn from(color: String) -> Self {
        Self {
            base: color,
            rgb: None,
            effects: vec![],
            animation: None,
        }
    }
}

impl From<&str> for AdvancedColor {
    fn from(color: &str) -> Self {
        Self {
            base: color.to_string(),
            rgb: None,
            effects: vec![],
            animation: None,
        }
    }
}

// Helper functions for creating advanced colors
impl AdvancedColor {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            rgb: None,
            effects: vec![],
            animation: None,
        }
    }

    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.rgb = Some((r, g, b));
        self
    }

    pub fn glow(mut self) -> Self {
        self.effects.push(ColorEffect::Glow);
        self
    }

    pub fn bold(mut self) -> Self {
        self.effects.push(ColorEffect::Bold);
        self
    }

    pub fn pulse(mut self, duration: f32) -> Self {
        self.animation = Some(Animation {
            animation_type: AnimationType::Pulse,
            duration,
            repeat: true,
            easing: "ease-in-out".to_string(),
        });
        self
    }

    pub fn gradient(mut self, to_color: &str) -> Self {
        self.effects.push(ColorEffect::Gradient(to_color.to_string()));
        self
    }

    pub fn as_str(&self) -> &str {
        &self.base
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default rFetch theme".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            colors: ThemeColors {
                title: AdvancedColor::new("cyan").bold(),
                subtitle: AdvancedColor::new("blue"),
                key: AdvancedColor::new("yellow"),
                value: AdvancedColor::new("white"),
                separator: AdvancedColor::new("white"),
                logo: AdvancedColor::new("cyan"),
                accent: AdvancedColor::new("magenta"),
                background: None,
                border: None,
                highlight: None,
                error: None,
                warning: None,
                success: None,
            },
            display: ThemeDisplay {
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
            },
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
            custom_labels: HashMap::new(),
            custom_ascii: None,
            effects: ThemeEffects {
                transitions: false,
                shadows: false,
                glow_intensity: 0.0,
                particle_effects: false,
                sound_effects: false,
                terminal_title: None,
            },
        }
    }
}

pub fn get_builtin_themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();
    
    // Default theme
    themes.insert("default".to_string(), Theme::default());
    
    // True Neon theme with real neon effects
    themes.insert("neon".to_string(), Theme {
        name: "neon".to_string(),
        description: "True neon theme with glowing effects and vibrant colors".to_string(),
        version: "2.0.0".to_string(),
        author: Some("rFetch Team".to_string()),
        colors: ThemeColors {
            title: AdvancedColor::new("bright_magenta")
                .rgb(255, 0, 255)
                .glow()
                .bold()
                .pulse(2.0),
            subtitle: AdvancedColor::new("bright_cyan")
                .rgb(0, 255, 255)
                .glow(),
            key: AdvancedColor::new("bright_green")
                .rgb(0, 255, 0)
                .glow()
                .gradient("bright_yellow"),
            value: AdvancedColor::new("bright_white")
                .rgb(255, 255, 255)
                .glow(),
            separator: AdvancedColor::new("bright_magenta")
                .rgb(255, 0, 255)
                .glow(),
            logo: AdvancedColor::new("bright_cyan")
                .rgb(0, 255, 255)
                .glow()
                .pulse(3.0),
            accent: AdvancedColor::new("bright_yellow")
                .rgb(255, 255, 0)
                .glow(),
            background: Some(AdvancedColor::new("black").rgb(0, 0, 0)),
            border: Some(AdvancedColor::new("bright_magenta")
                .rgb(255, 0, 255)
                .glow()),
            highlight: Some(AdvancedColor::new("bright_yellow")
                .rgb(255, 255, 0)
                .glow()),
            error: Some(AdvancedColor::new("bright_red")
                .rgb(255, 0, 0)
                .glow()),
            warning: Some(AdvancedColor::new("bright_yellow")
                .rgb(255, 255, 0)
                .glow()),
            success: Some(AdvancedColor::new("bright_green")
                .rgb(0, 255, 0)
                .glow()),
        },
        display: ThemeDisplay {
            logo_type: "ascii".to_string(),
            separator: " ⚡ ".to_string(),
            padding: 3,
            show_borders: true,
            show_color_bar: true,
            color_bar_style: "neon_gradient".to_string(),
            alignment: "left".to_string(),
            max_width: None,
            line_spacing: 1.2,
            indent: 2,
            show_icons: true,
            icon_style: "neon".to_string(),
        },
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
        custom_labels: {
            let mut labels = HashMap::new();
            labels.insert("os".to_string(), "⚡ SYSTEM".to_string());
            labels.insert("kernel".to_string(), "🔥 KERNEL".to_string());
            labels.insert("uptime".to_string(), "⏰ UPTIME".to_string());
            labels.insert("packages".to_string(), "📦 PACKAGES".to_string());
            labels.insert("shell".to_string(), "🐚 SHELL".to_string());
            labels.insert("terminal".to_string(), "💻 TERMINAL".to_string());
            labels.insert("cpu".to_string(), "🚀 CPU".to_string());
            labels.insert("gpu".to_string(), "🎮 GPU".to_string());
            labels.insert("memory".to_string(), "🧠 MEMORY".to_string());
            labels.insert("disk".to_string(), "💾 STORAGE".to_string());
            labels.insert("battery".to_string(), "🔋 BATTERY".to_string());
            labels.insert("date".to_string(), "📅 DATE".to_string());
            labels
        },
        custom_ascii: Some(CustomAscii {
            logo: vec![
                "    ███╗   ██╗███████╗ ██████╗ ███╗   ██╗    ".to_string(),
                "    ████╗  ██║██╔════╝██╔═══██╗████╗  ██║    ".to_string(),
                "    ██╔██╗ ██║█████╗  ██║   ██║██╔██╗ ██║    ".to_string(),
                "    ██║╚██╗██║██╔══╝  ██║   ██║██║╚██╗██║    ".to_string(),
                "    ██║ ╚████║███████╗╚██████╔╝██║ ╚████║    ".to_string(),
                "    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝    ".to_string(),
                "                                             ".to_string(),
                "    ⚡⚡⚡ NEON POWERED SYSTEM INFO ⚡⚡⚡     ".to_string(),
            ],
            small_logo: vec!["⚡".to_string()],
            decorations: {
                let mut decorations = HashMap::new();
                decorations.insert("border_top".to_string(), "═══════════════════════════════════════".to_string());
                decorations.insert("border_bottom".to_string(), "═══════════════════════════════════════".to_string());
                decorations.insert("corner_tl".to_string(), "╔".to_string());
                decorations.insert("corner_tr".to_string(), "╗".to_string());
                decorations.insert("corner_bl".to_string(), "╚".to_string());
                decorations.insert("corner_br".to_string(), "╝".to_string());
                decorations.insert("vertical".to_string(), "║".to_string());
                decorations
            },
        }),
        effects: ThemeEffects {
            transitions: true,
            shadows: true,
            glow_intensity: 0.8,
            particle_effects: true,
            sound_effects: false,
            terminal_title: Some("⚡ NEON SYSTEM INFO ⚡".to_string()),
        },
    });
    
    // Minimal theme (simplified)
    themes.insert("minimal".to_string(), Theme {
        name: "minimal".to_string(),
        description: "Clean and minimal theme".to_string(),
        version: "1.0.0".to_string(),
        author: None,
        colors: ThemeColors {
            title: AdvancedColor::new("white"),
            subtitle: AdvancedColor::new("bright_black"),
            key: AdvancedColor::new("bright_black"),
            value: AdvancedColor::new("white"),
            separator: AdvancedColor::new("bright_black"),
            logo: AdvancedColor::new("white"),
            accent: AdvancedColor::new("white"),
            background: None,
            border: None,
            highlight: None,
            error: None,
            warning: None,
            success: None,
        },
        display: ThemeDisplay {
            logo_type: "small".to_string(),
            separator: " ".to_string(),
            padding: 1,
            show_borders: false,
            show_color_bar: false,
            color_bar_style: "minimal".to_string(),
            alignment: "left".to_string(),
            max_width: Some(60),
            line_spacing: 1.0,
            indent: 0,
            show_icons: false,
            icon_style: "minimal".to_string(),
        },
        info_order: vec![
            "os".to_string(),
            "kernel".to_string(),
            "uptime".to_string(),
            "shell".to_string(),
            "cpu".to_string(),
            "memory".to_string(),
        ],
        custom_labels: HashMap::new(),
        custom_ascii: None,
        effects: ThemeEffects {
            transitions: false,
            shadows: false,
            glow_intensity: 0.0,
            particle_effects: false,
            sound_effects: false,
            terminal_title: None,
        },
    });
    
    // Retro theme
    themes.insert("retro".to_string(), Theme {
        name: "retro".to_string(),
        description: "Retro terminal theme".to_string(),
        version: "1.0.0".to_string(),
        author: None,
        colors: ThemeColors {
            title: AdvancedColor::new("green").bold(),
            subtitle: AdvancedColor::new("bright_green"),
            key: AdvancedColor::new("green"),
            value: AdvancedColor::new("bright_green"),
            separator: AdvancedColor::new("green"),
            logo: AdvancedColor::new("bright_green"),
            accent: AdvancedColor::new("yellow"),
            background: None,
            border: Some(AdvancedColor::new("green")),
            highlight: None,
            error: None,
            warning: None,
            success: None,
        },
        display: ThemeDisplay {
            logo_type: "ascii".to_string(),
            separator: ": ".to_string(),
            padding: 2,
            show_borders: true,
            show_color_bar: true,
            color_bar_style: "blocks".to_string(),
            alignment: "left".to_string(),
            max_width: Some(80),
            line_spacing: 1.0,
            indent: 0,
            show_icons: false,
            icon_style: "retro".to_string(),
        },
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
        custom_labels: {
            let mut labels = HashMap::new();
            labels.insert("os".to_string(), "SYSTEM".to_string());
            labels.insert("kernel".to_string(), "KERNEL".to_string());
            labels.insert("uptime".to_string(), "UPTIME".to_string());
            labels.insert("packages".to_string(), "PKGS".to_string());
            labels.insert("shell".to_string(), "SHELL".to_string());
            labels.insert("terminal".to_string(), "TERM".to_string());
            labels.insert("cpu".to_string(), "CPU".to_string());
            labels.insert("gpu".to_string(), "GPU".to_string());
            labels.insert("memory".to_string(), "MEM".to_string());
            labels.insert("disk".to_string(), "DISK".to_string());
            labels.insert("battery".to_string(), "BAT".to_string());
            labels.insert("date".to_string(), "DATE".to_string());
            labels
        },
        custom_ascii: None,
        effects: ThemeEffects {
            transitions: false,
            shadows: false,
            glow_intensity: 0.0,
            particle_effects: false,
            sound_effects: false,
            terminal_title: Some("RETRO SYSTEM INFO".to_string()),
        },
    });
    
    themes
}

pub fn load_theme(theme_name: &str) -> Option<Theme> {
    let builtin_themes = get_builtin_themes();
    builtin_themes.get(theme_name).cloned()
}

pub fn list_themes() -> Vec<String> {
    get_builtin_themes().keys().cloned().collect()
}