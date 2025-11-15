use bevy::prelude::*;

#[derive(Resource)]
pub struct SeekerColors {
    pub home_menu: Color,
    pub project_list: Color,
    pub home_hovered: Color,
    pub button_border: Color,
    pub home_font_color: Color,
}

impl SeekerColors {
    pub fn new() -> Self {
        SeekerColors {
            home_menu: Color::srgb_u8(43, 45, 48),
            project_list: Color::srgb_u8(30, 31, 34),
            home_hovered: Color::srgb_u8(50, 66, 107),
            button_border: Color::srgb_u8(79, 81, 86),
            home_font_color: Color::srgb_u8(218, 220, 224),
        }
    }
}
