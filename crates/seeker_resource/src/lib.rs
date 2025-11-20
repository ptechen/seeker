use crate::color::SeekerColors;
use bevy::prelude::*;

pub mod color;
pub mod fonts;
pub mod file;

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
