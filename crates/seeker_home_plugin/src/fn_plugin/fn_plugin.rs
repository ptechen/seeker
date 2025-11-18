use bevy::prelude::*;
use crate::fn_plugin::file_dialog::FileDialogPlugin;
use crate::fn_plugin::project::Project;

#[derive(Component)]
pub struct FnPlugin;

impl Plugin for FnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Project)
            .add_plugins(FileDialogPlugin)
        ;
    }
}