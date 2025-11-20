use crate::fn_plugin::new_folder_plugin::NewFolderPlugin;
use bevy::camera::RenderTarget;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::{WindowRef, WindowResolution};
use seeker_config::SEEKER_CONFIG;
use seeker_resource::file::{get_files, CurrentFile, File, Level};
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::SeekerResource;
use seeker_state::{
    SeekerFileDialogFnState, SeekerHomeSubFnState, SeekerNewFolderState, SeekerState,
};
use seeker_trait::SeekerTrait;
use std::fs;
use std::path::PathBuf;

#[derive(Component)]
pub struct FileDialogPlugin;

impl SeekerTrait for FileDialogPlugin {}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct FileDialogButton;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct FileDialogFnButton;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFile::default())
            .insert_state(SeekerNewFolderState::None)
            .add_plugins(NewFolderPlugin)
            .add_systems(OnEnter(SeekerFileDialogFnState::Open), Self::enter)
            .add_systems(
                Update,
                Self::button_on_pressed_changed_color::<FileDialogButton>,
            )
            .add_systems(Update, Self::update_file)
            .add_observer(
                Self::button_on_hovered_changed_color::<Insert, Hovered, FileDialogFnButton>,
            )
            .add_systems(Update, Self::update_fn);
    }
}

impl FileDialogPlugin {
    fn enter(mut commands: Commands, res: Res<SeekerResource>, assets: Res<AssetServer>) {
        let file_dialog_window = commands
            .spawn((
                DespawnOnExit(SeekerFileDialogFnState::Open),
                BorderRadius::all(Val::Px(3.)),
                Window {
                    window_theme: Some(SEEKER_CONFIG.window_theme),
                    resolution: WindowResolution::new(1000, 400),
                    titlebar_show_buttons: false,
                    titlebar_show_title: false,
                    // titlebar_shown: false,
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

        let mut parent = commands.spawn((
            UiTargetCamera(file_dialog_camera),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                width: Val::Percent(100.), // 明确设置宽度
                // overflow: Overflow::scroll(),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
        ));

        parent.with_children(|parent| {
            let mut parent = parent.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Val::Percent(90.),
                width: Val::Percent(100.), // 明确设置宽度
                overflow: Overflow::scroll(),
                // padding: UiRect::all(Val::Px(10.)),
                ..default()
            });

            let dialog_level = Level::new(1);
            let files = get_files(home, parent.id(), dialog_level.clone());
            parent.with_children(|parent| {
                Self::render(&files, parent, &assets, dialog_level);
            });
        });
        parent.with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::top(Val::Px(10.)),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(
                            (Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                width: Val::Percent(70.),
                                ..default()
                            }),
                        )
                        .with_children(|parent| {
                            Self::ui_button_same(
                                parent,
                                FileDialogFnButton,
                                "NewFolder",
                                &res,
                                &assets,
                            );
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(30.),
                            ..default()
                        })
                        .with_children(|parent| {
                            Self::ui_button_same(
                                parent,
                                FileDialogFnButton,
                                "Cancel",
                                &res,
                                &assets,
                            );
                            Self::ui_button_same(parent, FileDialogFnButton, "Open", &res, &assets);
                        });
                });
        });
    }
    fn update_file(
        mut commands: Commands,
        asset: Res<AssetServer>,
        mut current_file: ResMut<CurrentFile>,
        mut query: ParamSet<(
            Query<(&mut File, &Interaction), (Changed<Interaction>, With<FileDialogButton>)>,
            Query<(Entity, &Level)>,
        )>,
    ) {
        let mut entity = None;
        let mut path = None;
        let mut dialog_level = Level::default();
        for (file, interaction) in query.p0().iter_mut() {
            if *interaction == Interaction::Pressed && file.is_dir {
                current_file.file = Some(file.clone());
                path = Some(file.path.clone());
                entity = Some(file.root_entity);
                dialog_level = Level::new(file.level.level + 1);
            }
        }
        if dialog_level.level >= 2 {
            for (entity, level) in query.p1().iter_mut() {
                if level.level >= dialog_level.level {
                    commands.entity(entity).despawn();
                }
            }

            if let (Some(path), Some(entity)) = (path, entity) {
                let files = get_files(path, entity, dialog_level.clone());
                commands.entity(entity).with_children(|parent| {
                    Self::render(&files, parent, &asset, dialog_level);
                });
            }
        }
    }
    fn update_fn(
        mut commands: Commands,
        current_file: Res<CurrentFile>,
        mut state: ResMut<NextState<SeekerState>>,
        mut dialog_file_state: ResMut<NextState<SeekerFileDialogFnState>>,
        mut new_folder_state: ResMut<NextState<SeekerNewFolderState>>,
        mut query: Query<
            (Entity, &Name, &Interaction),
            (Changed<Interaction>, With<FileDialogFnButton>),
        >,
    ) {
        for (entity, name, interaction) in query.iter_mut() {
            if *interaction == Interaction::Pressed {
                match name.as_str() {
                    "Open" => {
                        state.set(SeekerState::Edit);
                        dialog_file_state.set(SeekerFileDialogFnState::None);
                    }
                    "Cancel" => {
                        dialog_file_state.set(SeekerFileDialogFnState::None);
                    }
                    "NewFolder" => {
                        new_folder_state.set(SeekerNewFolderState::Open);
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(
        files: &Vec<File>,
        parent: &mut RelatedSpawnerCommands<ChildOf>,
        asset: &Res<AssetServer>,
        level: Level,
    ) {
        parent
            .spawn((
                level,
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    padding: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(0.), Val::Px(0.)),
                    border: UiRect::right(Val::Px(1.)),
                    ..default()
                },
                BorderColor::all(Color::srgb_u8(78, 81, 87)),
            ))
            .with_children(|parent| {
                for file in files {
                    if file.is_dir {
                        parent
                            .spawn((
                                file.clone(),
                                FileDialogButton,
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(&file.filename),
                                    TextFont {
                                        font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                        ..default()
                                    },
                                ));

                                parent.spawn((
                                    Text::new("  >"),
                                    TextFont {
                                        font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                        ..default()
                                    },
                                ));
                            });
                    } else {
                        parent.spawn((
                            file.clone(),
                            FileDialogButton,
                            Text::new(&file.filename),
                            TextFont {
                                font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                ..default()
                            },
                        ));
                    }
                }
            });
    }
}
