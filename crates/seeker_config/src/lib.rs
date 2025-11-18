use std::sync::LazyLock;
use bevy::prelude::*;
use bevy::window::WindowTheme;
use serde::{Deserialize, Serialize};

pub static SEEKER_CONFIG: LazyLock<SeekerConfig> = LazyLock::new(|| {
    let path = std::env::home_dir().unwrap();
    let path = path.join(".Seeker/config.toml");
    let data = std::fs::read(path).unwrap_or_default();
    toml::from_slice::<SeekerConfig>(&data).unwrap_or_default()
});

#[derive(Clone, Deserialize ,Serialize)]
pub struct SeekerConfig {
    pub window_theme: WindowTheme,
    #[serde(skip)]
    pub colors: SeekerColors,
}

impl Default for SeekerConfig {
    fn default() -> Self {
        Self {
            window_theme: WindowTheme::Dark,
            colors: SeekerColors::from(WindowTheme::Dark),
        }
    }
}

#[derive(Clone, Default, Deserialize ,Serialize)]
pub struct SeekerColors{
    pub background: Color,
}

impl From<WindowTheme> for SeekerColors {
    fn from(value: WindowTheme) -> Self {
        match value {
            WindowTheme::Dark => SeekerColors {
                background: Color::BLACK,
            },
            WindowTheme::Light => SeekerColors {
                background: Color::WHITE,
            },
        }
    }
}