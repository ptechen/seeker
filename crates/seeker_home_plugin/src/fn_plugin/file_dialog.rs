use crate::FnUi;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::ui::widget::ImageNode;
use bevy::window::{WindowRef, WindowResolution};
use seeker_config::{SeekerConfig, SEEKER_CONFIG};
use seeker_resource::fonts::FIRASSANS_BOLD;
use seeker_resource::SeekerResource;
use seeker_state::SeekerHomeSubFnState;
use std::fs;
use std::path::PathBuf;

#[derive(Component)]
pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerHomeSubFnState::FileDialog), Self::enter);
    }
}

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub filename: String,
    pub is_dir: bool,
}

impl FileDialogPlugin {
    fn enter(mut commands: Commands, res: Res<SeekerResource>, asset: Res<AssetServer>) {
        let file_dialog_window = commands
            .spawn((
                DespawnOnExit(SeekerHomeSubFnState::FileDialog),
                BorderRadius::all(Val::Px(3.)),
                Window {
                    window_theme: Some(SEEKER_CONFIG.window_theme),
                    resolution: WindowResolution::new(1000, 400),
                    titlebar_show_buttons: false,
                    titlebar_show_title: false,
                    titlebar_shown: false,
                    ..default()
                },
            ))
            .id();
        let file_dialog_camera = commands
            .spawn((
                Camera2d::default(),
                Camera {
                    target: RenderTarget::Window(WindowRef::Entity(file_dialog_window)),
                    ..default()
                },
            ))
            .id();

        let Some(home) = std::env::home_dir() else {
            return;
        };
        let files = get_files(home);
        let image = asset.load("embedded://seeker_embed_plugin/assets/pngs/folder.png");

        commands
            .spawn((
                UiTargetCamera(file_dialog_camera),
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    height: percent(90.),
                    overflow: Overflow::scroll(),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                for file in &files {
                    if file.is_dir {
                        parent.spawn((
                            ImageNode::new(image.clone()),
                            Text::new(format!("> {}", file.filename)),
                            TextFont {
                                font: asset.load(FIRASSANS_BOLD),
                                ..default()
                            },
                        ));
                    } else {
                        parent.spawn((
                            Text::new(&file.filename),
                            TextFont {
                                font: asset.load(FIRASSANS_BOLD),
                                ..default()
                            },
                        ));
                    }
                }
            });
    }
}

fn get_files(path: PathBuf) -> Vec<File> {
    let Ok(entries) = fs::read_dir(path) else {
        return vec![];
    };
    let mut files = Vec::with_capacity(100);
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(".") {
                continue;
            }
            files.push(File {
                is_dir: path.is_dir(),
                path,
                filename: entry.file_name().to_string_lossy().to_string(),
            });
        }
    }
    files
}
