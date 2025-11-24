use crate::fn_plugin::new_folder_plugin::{NewFolderPlugin, NewFolderWindow};
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
    SeekerFileDialogFnState, SeekerNewFolderState, SeekerState,
};
use seeker_trait::SeekerTrait;
use std::os::unix::fs::MetadataExt;

#[derive(Component)]
pub struct FileDialogPlugin;

#[derive(Component)]
pub struct FileDialogWindow;

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
                (
                    Self::button_on_pressed_changed_color::<FileDialogButton>,
                    Self::update_file,
                    Self::update_fn,
                )
                    .run_if(in_state(SeekerFileDialogFnState::Open)),
            )
            .add_observer(
                Self::button_on_hovered_changed_color::<Insert, Hovered, FileDialogFnButton>,
            );
    }
}

impl FileDialogPlugin {
    fn enter(
        mut commands: Commands,
        res: Res<SeekerResource>,
        assets: Res<AssetServer>,
        mut window: Single<&mut Window>,
    ) {
        window.visible = false;
        let file_dialog_window = commands
            .spawn((
                DespawnOnExit(SeekerFileDialogFnState::Open),
                FileDialogWindow,
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
                DespawnOnExit(SeekerFileDialogFnState::Open),
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
            DespawnOnExit(SeekerFileDialogFnState::Open),
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
                Self::render_dir(&files, parent, &assets, dialog_level);
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
        let mut is_dir = false;
        let mut file_ = None;
        let mut dialog_level = Level::default();
        for (file, interaction) in query.p0().iter_mut() {
            if *interaction == Interaction::Pressed {
                is_dir = file.is_dir;
                path = Some(file.path.clone());
                entity = Some(file.root_entity);
                dialog_level = Level::new(file.level.level + 1);
                current_file.file = Some(file.clone());
                file_ = Some(file.clone());
            }
        }
        if dialog_level.level >= 2 {
            for (entity, level) in query.p1().iter_mut() {
                if level.level >= dialog_level.level {
                    commands.entity(entity).despawn();
                }
            }
            if is_dir {
                if let (Some(path), Some(entity)) = (path, entity) {
                    let files = get_files(path, entity, dialog_level.clone());
                    commands.entity(entity).with_children(|parent| {
                        Self::render_dir(&files, parent, &asset, dialog_level);
                    });
                }
            } else {
                if let (Some(file), Some(entity)) = (file_, entity) {
                    commands.entity(entity).with_children(|parent| {
                        Self::render_file(file, parent, &asset, dialog_level);
                    });
                }
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
        mut window: Single<&mut Window, Without<FileDialogWindow>>,
    ) {
        for (entity, name, interaction) in query.iter_mut() {
            if *interaction == Interaction::Pressed {
                match name.as_str() {
                    "Open" => {
                        window.visible = true;
                        state.set(SeekerState::Edit);
                        dialog_file_state.set(SeekerFileDialogFnState::None);
                    }
                    "Cancel" => {
                        window.visible = true;
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

    fn render_dir(
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
                    min_width: Val::Px(400.),
                    max_width: Val::Px(400.),
                    overflow: Overflow::scroll(),
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
                                    Text::new(truncate_filename(&file.filename, 20)),
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
                            Text::new(truncate_filename(&file.filename, 23)),
                            TextFont {
                                font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                ..default()
                            },
                        ));
                    }
                }
            });
    }

    fn render_file(
        file: File,
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
                    min_width: Val::Px(400.),
                    overflow: Overflow::scroll(),
                    padding: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(0.), Val::Px(0.)),
                    border: UiRect::right(Val::Px(1.)),
                    ..default()
                },
                BorderColor::all(Color::srgb_u8(78, 81, 87)),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        file.clone(),
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        if let Ok(metadata) = std::fs::metadata(&file.path) {
                            parent.spawn((
                                Text::new(format!("filename: {}", file.filename)),
                                TextFont {
                                    font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                    font_size: 14.,
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new(format!("size: {}", metadata.size())),
                                TextFont {
                                    font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                    font_size: 14.,
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new(format!(
                                    "created_time: {}",
                                    system_time_to_datetime_string(metadata.created().unwrap())
                                )),
                                TextFont {
                                    font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                    font_size: 14.,
                                    ..default()
                                },
                            ));
                            parent.spawn((
                                Text::new(format!(
                                    "modified_time: {}",
                                    system_time_to_datetime_string(metadata.modified().unwrap())
                                )),
                                TextFont {
                                    font: asset.load(MAPLE_MONO_BOLD_ITALIC),
                                    font_size: 14.,
                                    ..default()
                                },
                            ));
                        }
                    });
            });
    }
}

// 估算显示宽度（中文等宽字符算作2个单位宽度）
fn char_display_width(ch: char) -> usize {
    if ch.len_utf8() > 1 {
        2
    } else {
        1
    }
}

use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn system_time_to_datetime_string(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn truncate_filename(filename: &str, max_length: usize) -> String {
    let mut filename_len = 0;
    for char in filename.chars() {
        filename_len += char_display_width(char);
    }
    // 如果字符数小于等于最大长度，直接返回
    if filename_len <= max_length {
        return filename.to_string();
    }

    // 获取文件扩展名（如果有）
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if !extension.is_empty() {
        // 包含扩展名的情况：保留扩展名并在名称主体部分中间截断
        let basename = std::path::Path::new(filename)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(filename);

        let extension_width: usize = extension.chars().map(char_display_width).sum();
        let basename_chars: Vec<char> = basename.chars().collect();
        let basename_width: usize = basename_chars.iter().map(|&c| char_display_width(c)).sum();

        // 计算可用于basename的空间（预留 "..."+extension+"." 的空间）
        let available_width = max_length.saturating_sub(extension_width + 4); // "...".len() + ".".len()

        if available_width > 0 && basename_width > 0 {
            // 根据显示宽度而不是字符数来分割
            let half_width = available_width / 2;

            // 构建前半部分
            let mut first_part = String::new();
            let mut first_width = 0;
            let mut split_index = 0;
            for (i, &ch) in basename_chars.iter().enumerate() {
                let ch_width = char_display_width(ch);
                if first_width + ch_width <= half_width {
                    first_part.push(ch);
                    first_width += ch_width;
                    split_index = i + 1;
                } else {
                    break;
                }
            }

            // 构建后半部分
            let mut second_part = String::new();
            let mut remaining_width = available_width - first_width;
            for &ch in basename_chars[split_index..].iter().rev() {
                let ch_width = char_display_width(ch);
                if ch_width <= remaining_width {
                    second_part.push(ch);
                    remaining_width -= ch_width;
                } else {
                    break;
                }
            }
            second_part = second_part.chars().rev().collect();

            format!("{}...{}.{}", first_part, second_part, extension)
        } else {
            // 如果空间不足，显示前几个字符加扩展名
            let display_len = max_length.saturating_sub(extension_width + 2); // "..".len() + ".".len()
            if display_len > 0 {
                let mut shortened = String::new();
                let mut current_width = 0;
                for ch in filename.chars() {
                    let ch_width = char_display_width(ch);
                    if current_width + ch_width <= display_len {
                        shortened.push(ch);
                        current_width += ch_width;
                    } else {
                        break;
                    }
                }
                format!("{}..{}", shortened, extension)
            } else {
                format!(".{}", extension)
            }
        }
    } else {
        // 没有扩展名的情况：简单地从中间截断
        let filename_chars: Vec<char> = filename.chars().collect();
        let half_width = (max_length - 3) / 2; // 3 是 "..." 的长度

        // 构建前半部分
        let mut first_part = String::new();
        let mut first_width = 0;
        let mut split_index = 0;
        for (i, &ch) in filename_chars.iter().enumerate() {
            let ch_width = char_display_width(ch);
            if first_width + ch_width <= half_width {
                first_part.push(ch);
                first_width += ch_width;
                split_index = i + 1;
            } else {
                break;
            }
        }

        // 构建后半部分
        let mut second_part = String::new();
        let mut remaining_width = (max_length - 3) - first_width;
        for &ch in filename_chars[split_index..].iter().rev() {
            let ch_width = char_display_width(ch);
            if ch_width <= remaining_width {
                second_part.push(ch);
                remaining_width -= ch_width;
            } else {
                break;
            }
        }
        second_part = second_part.chars().rev().collect();

        format!("{}...{}", first_part, second_part)
    }
}
