use crate::fn_plugin::file_dialog_plugin::FileDialogPlugin;
use crate::fn_plugin::project::ProjectPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct FnPlugin;

impl Plugin for FnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectPlugin).add_plugins(FileDialogPlugin);
    }
}
