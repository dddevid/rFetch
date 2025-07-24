use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedColor {
    pub base: String,
    pub rgb: Option<(u8, u8, u8)>,
    pub effects: Vec<ColorEffect>,
    pub animation: Option<Animation>,
    pub gradient: Option<Gradient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorEffect {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Blink,
    Reverse,
    Dim,
    Glow(u8),
    Shadow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub animation_type: AnimationType,
    pub duration: f32,
    pub repeat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    Fade,
    Pulse,
    Rainbow,
    Wave,
    Typewriter,
    Slide,
    Bounce,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub colors: Vec<String>,
    pub direction: GradientDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub colors: ThemeColors,
    pub display: ThemeDisplay,
    pub ascii: Option<String>,
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
    pub accent: Option<AdvancedColor>,
    pub background: Option<AdvancedColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDisplay {
    pub logo_type: String,
    pub separator: String,
    pub padding: usize,
    pub layout: String,
    pub border: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEffects {
    pub transitions: bool,
    pub animations: bool,
    pub glow: bool,
    pub shadows: bool,
    pub transparency: Option<f32>,
}

impl From<&str> for AdvancedColor {
    fn from(color: &str) -> Self {
        Self {
            base: color.to_string(),
            rgb: None,
            effects: Vec::new(),
            animation: None,
            gradient: None,
        }
    }
}

impl From<String> for AdvancedColor {
    fn from(color: String) -> Self {
        Self {
            base: color,
            rgb: None,
            effects: Vec::new(),
            animation: None,
            gradient: None,
        }
    }
}

impl AdvancedColor {
    pub fn new(base: &str) -> Self {
        Self::from(base)
    }

    pub fn with_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.rgb = Some((r, g, b));
        self
    }

    pub fn with_effect(mut self, effect: ColorEffect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn with_effects(mut self, effects: Vec<ColorEffect>) -> Self {
        self.effects = effects;
        self
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn with_gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }

    pub fn bold(self) -> Self {
        self.with_effect(ColorEffect::Bold)
    }

    pub fn italic(self) -> Self {
        self.with_effect(ColorEffect::Italic)
    }

    pub fn underline(self) -> Self {
        self.with_effect(ColorEffect::Underline)
    }

    pub fn glow(self, intensity: u8) -> Self {
        self.with_effect(ColorEffect::Glow(intensity))
    }

    pub fn pulse(mut self, duration: f32) -> Self {
        self.animation = Some(Animation {
            animation_type: AnimationType::Pulse,
            duration,
            repeat: true,
        });
        self
    }

    pub fn rainbow(mut self, duration: f32) -> Self {
        self.animation = Some(Animation {
            animation_type: AnimationType::Rainbow,
            duration,
            repeat: true,
        });
        self
    }
}

pub fn load_theme(name: &str) -> Option<Theme> {
    match name {
        "default" => Some(create_default_theme()),
        "neon" => Some(create_neon_theme()),
        "minimal" => Some(create_minimal_theme()),
        "retro" => Some(create_retro_theme()),
        _ => None,
    }
}

pub fn list_themes() -> Vec<String> {
    vec![
        "default".to_string(),
        "neon".to_string(),
        "minimal".to_string(),
        "retro".to_string(),
    ]
}

fn create_default_theme() -> Theme {
    Theme {
        name: "Default".to_string(),
        description: "Default rFetch theme with balanced colors".to_string(),
        author: "rFetch Team".to_string(),
        version: "1.0.0".to_string(),
        colors: ThemeColors {
            title: AdvancedColor::new("cyan").bold(),
            subtitle: AdvancedColor::new("blue"),
            key: AdvancedColor::new("yellow"),
            value: AdvancedColor::new("white"),
            separator: AdvancedColor::new("white"),
            logo: AdvancedColor::new("cyan"),
            accent: Some(AdvancedColor::new("magenta")),
            background: None,
        },
        display: ThemeDisplay {
            logo_type: "auto".to_string(),
            separator: ": ".to_string(),
            padding: 2,
            layout: "horizontal".to_string(),
            border: None,
        },
        ascii: None,
        effects: ThemeEffects {
            transitions: false,
            animations: false,
            glow: false,
            shadows: false,
            transparency: None,
        },
    }
}

fn create_neon_theme() -> Theme {
    Theme {
        name: "Neon".to_string(),
        description: "Bright neon theme with glowing effects and animations".to_string(),
        author: "rFetch Team".to_string(),
        version: "1.0.0".to_string(),
        colors: ThemeColors {
            title: AdvancedColor::new("bright_cyan")
                .bold()
                .glow(80)
                .pulse(2.0),
            subtitle: AdvancedColor::new("bright_magenta")
                .italic()
                .glow(60),
            key: AdvancedColor::new("bright_yellow")
                .bold()
                .glow(70),
            value: AdvancedColor::new("bright_white")
                .glow(40),
            separator: AdvancedColor::new("bright_blue")
                .glow(50),
            logo: AdvancedColor::new("bright_cyan")
                .bold()
                .glow(90)
                .rainbow(3.0),
            accent: Some(AdvancedColor::new("bright_magenta")
                .glow(85)
                .pulse(1.5)),
            background: Some(AdvancedColor::new("black")
                .with_rgb(5, 5, 15)),
        },
        display: ThemeDisplay {
            logo_type: "ascii".to_string(),
            separator: " ▶ ".to_string(),
            padding: 3,
            layout: "horizontal".to_string(),
            border: Some("neon".to_string()),
        },
        ascii: Some(r#"
    ██████╗ ███████╗███████╗████████╗ ██████╗██╗  ██╗
    ██╔══██╗██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
    ██████╔╝█████╗  █████╗     ██║   ██║     ███████║
    ██╔══██╗██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
    ██║  ██║██║     ███████╗   ██║   ╚██████╗██║  ██║
    ╚═╝  ╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
        "#.to_string()),
        effects: ThemeEffects {
            transitions: true,
            animations: true,
            glow: true,
            shadows: true,
            transparency: Some(0.95),
        },
    }
}

fn create_minimal_theme() -> Theme {
    Theme {
        name: "Minimal".to_string(),
        description: "Clean and minimal theme with essential information only".to_string(),
        author: "rFetch Team".to_string(),
        version: "1.0.0".to_string(),
        colors: ThemeColors {
            title: AdvancedColor::new("white").bold(),
            subtitle: AdvancedColor::new("bright_black"),
            key: AdvancedColor::new("bright_black"),
            value: AdvancedColor::new("white"),
            separator: AdvancedColor::new("bright_black"),
            logo: AdvancedColor::new("white"),
            accent: None,
            background: None,
        },
        display: ThemeDisplay {
            logo_type: "small".to_string(),
            separator: " ".to_string(),
            padding: 1,
            layout: "vertical".to_string(),
            border: None,
        },
        ascii: None,
        effects: ThemeEffects {
            transitions: false,
            animations: false,
            glow: false,
            shadows: false,
            transparency: None,
        },
    }
}

fn create_retro_theme() -> Theme {
    Theme {
        name: "Retro".to_string(),
        description: "Vintage terminal theme with classic green colors".to_string(),
        author: "rFetch Team".to_string(),
        version: "1.0.0".to_string(),
        colors: ThemeColors {
            title: AdvancedColor::new("bright_green").bold(),
            subtitle: AdvancedColor::new("green"),
            key: AdvancedColor::new("green"),
            value: AdvancedColor::new("bright_green"),
            separator: AdvancedColor::new("green"),
            logo: AdvancedColor::new("bright_green").bold(),
            accent: Some(AdvancedColor::new("yellow")),
            background: Some(AdvancedColor::new("black")),
        },
        display: ThemeDisplay {
            logo_type: "ascii".to_string(),
            separator: ": ".to_string(),
            padding: 2,
            layout: "horizontal".to_string(),
            border: Some("classic".to_string()),
        },
        ascii: Some(r#"
    ┌─────────────────────────────────────┐
    │  ██████  ███████ ███████ ████████   │
    │  ██   ██ ██      ██         ██      │
    │  ██████  █████   █████      ██      │
    │  ██   ██ ██      ██         ██      │
    │  ██   ██ ██      ███████    ██      │
    └─────────────────────────────────────┘
        "#.to_string()),
        effects: ThemeEffects {
            transitions: false,
            animations: false,
            glow: false,
            shadows: false,
            transparency: None,
        },
    }
}