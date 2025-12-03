use crate::color::SeekerColors;
use bevy::prelude::*;

pub mod color;
pub mod file;
pub mod fonts;
pub mod project_list;

#[derive(Resource)]
pub struct SeekerResource {
    pub colors: SeekerColors,
}

impl SeekerResource {
    pub fn new() -> Self {
        SeekerResource {
            colors: SeekerColors::new(),
        }
    }
}
