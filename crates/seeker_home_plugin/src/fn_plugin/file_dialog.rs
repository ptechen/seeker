use bevy::camera::RenderTarget;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::{WindowRef, WindowResolution};
use seeker_config::SEEKER_CONFIG;
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerFileDialogFnState, SeekerHomeSubFnState};
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

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerFileDialogFnState::Open), Self::enter)
            .add_systems(
                Update,
                Self::button_on_pressed_changed_color::<FileDialogButton>,
            )
            .add_systems(Update, Self::update_file);
    }
}

impl FileDialogPlugin {
    fn enter(mut commands: Commands, res: Res<SeekerResource>, asset: Res<AssetServer>) {
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

        let mut parent = commands
            .spawn((
                UiTargetCamera(file_dialog_camera),
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    height: percent(90.),
                    overflow: Overflow::scroll(),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
            ));
            let files = get_files(home, parent.id());
            parent.with_children(|parent| {

                parent
                    .spawn(
                        (Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        }),
                    )
                    .with_children(|parent| {
                        for file in &files {
                            file.render(parent, &asset);
                        }
                    });
            });
    }
    fn update_file(
        mut commands: Commands,
        asset: Res<AssetServer>,
        mut query: Query<
            (&mut File, &Interaction),
            (Changed<Interaction>, With<FileDialogButton>),
        >,
    ) {
        for (mut file, interaction) in query.iter_mut() {
            if *interaction == Interaction::Pressed {
                println!("{}", file.filename);
                let files = get_files(file.path.clone(), file.root_entity);
                commands.entity(file.root_entity).with_children(|parent| {
                    parent
                        .spawn(
                            (Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            }),
                        )
                        .with_children(|parent| {
                            for file in &files {
                                file.render(parent, &asset);
                            }
                        });
                });
            }
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Files {
    pub files: Vec<File>,
    pub entity: Option<Entity>,
}

#[derive(Component, Resource, Clone)]
pub struct File {
    pub path: PathBuf,
    pub filename: String,
    pub is_dir: bool,
    pub root_entity: Entity,
}

impl File {
    fn render(&self, parent: &mut RelatedSpawnerCommands<ChildOf>, asset: &Res<AssetServer>) {
        if self.is_dir {
            parent
                .spawn((
                    self.clone(),
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
                        Text::new(&self.filename),
                        TextFont {
                            font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(">"),
                        TextFont {
                            font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                            ..default()
                        },
                    ));
                });
        } else {
            parent.spawn((
                self.clone(),
                FileDialogButton,
                Text::new(format!("{}  ", self.filename)),
                TextFont {
                    font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                    ..default()
                },
            ));
        }
    }
}

fn get_files(path: PathBuf, entity: Entity) -> Vec<File> {
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
                root_entity: entity,
            });
        }
    }
    files
}
