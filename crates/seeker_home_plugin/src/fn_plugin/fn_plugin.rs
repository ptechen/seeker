use bevy::prelude::*;
use crate::fn_plugin::project::Project;

#[derive(Component)]
pub struct FnPlugin;

impl Plugin for FnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Project)
        ;
    }
}