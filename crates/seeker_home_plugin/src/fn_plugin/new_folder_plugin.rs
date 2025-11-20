use bevy::camera::RenderTarget;
use bevy::picking::PickingSystems::Input;
use bevy::prelude::*;
use bevy::window::{WindowRef, WindowResolution};
use seeker_config::SEEKER_CONFIG;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerFileDialogFnState, SeekerNewFolderState};

pub struct NewFolderPlugin;

impl Plugin for NewFolderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerNewFolderState::Open), Self::on_enter);
    }
}

impl NewFolderPlugin {
    fn on_enter(mut commands: Commands, res: Res<SeekerResource>) {
        let new_folder_window = commands
            .spawn((
                DespawnOnExit(SeekerNewFolderState::None),
                BorderRadius::all(Val::Px(3.)),
                Window {
                    title: "New Folder".to_string(),
                    window_theme: Some(SEEKER_CONFIG.window_theme),
                    resolution: WindowResolution::new(320, 160),
                    titlebar_show_buttons: false,
                    // titlebar_show_title: false,
                    // titlebar_shown: false,
                    ..default()
                },
            ))
            .id();
        let new_folder_camera = commands
            .spawn((
                Camera2d::default(),
                Camera {
                    target: RenderTarget::Window(WindowRef::Entity(new_folder_window)),
                    ..default()
                },
            ))
            .id();
        let mut parent = commands.spawn((
            UiTargetCamera(new_folder_camera),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                height: Val::Percent(100.),
                width: Val::Percent(100.), // 明确设置宽度
                // overflow: Overflow::scroll(),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
        ));

        parent.with_children(|parent| {
            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            ));
        });
    }
}