use crate::fn_plugin::file_dialog_plugin::{FileDialogFnButton, FileDialogWindow};
use bevy::camera::RenderTarget;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::window::{WindowRef, WindowResolution};
use seeker_config::SEEKER_CONFIG;
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::SeekerResource;
use seeker_state::SeekerNewFolderState;
use seeker_trait::SeekerTrait;
use std::mem;
use seeker_resource::file::CurrentFile;

#[derive(Component)]
pub struct NewFolderPlugin;
#[derive(Component)]
pub struct NewFolderWindow;

impl SeekerTrait for NewFolderPlugin {}

#[derive(Component)]
pub struct NewFolderInput;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct NewFolderFnButton;

#[derive(Component)]
struct Bubble {
    timer: Timer,
}

impl Plugin for NewFolderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerNewFolderState::Open), Self::on_enter)
            .add_systems(
                Update,
                (
                    Self::toggle_ime,
                    Self::listen_ime_events,
                    Self::listen_keyboard_input_events,
                    Self::bubbling_text,
                    Self::update_fn,
                )
                    .run_if(in_state(SeekerNewFolderState::Open)),
            )
            .add_observer(
                Self::button_on_hovered_changed_color::<Insert, Hovered, NewFolderFnButton>,
            );
    }
}

impl NewFolderPlugin {
    fn on_enter(mut commands: Commands, res: Res<SeekerResource>, assets: Res<AssetServer>, mut window: Single<&mut Window, With<FileDialogWindow>>) {
        window.visible = false;
        let new_folder_window = commands
            .spawn((
                DespawnOnExit(SeekerNewFolderState::Open),
                NewFolderWindow,
                BorderRadius::all(Val::Px(3.)),
                Window {
                    title: "New Folder".to_string(),
                    window_theme: Some(SEEKER_CONFIG.window_theme),
                    resolution: WindowResolution::new(400, 160),
                    titlebar_show_buttons: false,
                    // titlebar_show_title: false,
                    // titlebar_shown: false,
                    ..default()
                },
            ))
            .id();
        let new_folder_camera = commands
            .spawn((
                DespawnOnExit(SeekerNewFolderState::Open),
                Camera2d::default(),
                Camera {
                    target: RenderTarget::Window(WindowRef::Entity(new_folder_window)),
                    ..default()
                },
            ))
            .id();
        let font = assets.load(MAPLE_MONO_BOLD_ITALIC);
        let mut parent = commands.spawn((
            DespawnOnExit(SeekerNewFolderState::Open),
            UiTargetCamera(new_folder_camera),
            Button,
            Node {
                height: Val::Percent(100.),
                width: Val::Percent(100.), // 明确设置宽度
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                // overflow: Overflow::scroll(),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
        ));
        parent.with_children(|parent| {
            parent
                .spawn(Node {
                    height: Val::Percent(80.),
                    width: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::new(
                                    Val::Px(15.),
                                    Val::Px(15.),
                                    Val::Px(5.0),
                                    Val::Px(5.0),
                                ),
                                box_sizing: BoxSizing::BorderBox,
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(98.),
                                height: Val::Px(32.),
                                border: UiRect::all(Val::Px(1.)),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(3.)),
                            Hovered::default(),
                            BorderColor::all(res.colors.button_border),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                NewFolderInput,
                                Text::new(""),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 16.0,
                                    ..default()
                                },
                            ));
                        });
                });
        });

        parent.with_children(|parent| {
            parent
                .spawn(
                    (Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    }),
                )
                .with_children(|parent| {
                    Self::ui_button_same(parent, NewFolderFnButton, "Cancel", &res, &assets);
                    Self::ui_button_same(parent, NewFolderFnButton, "Create", &res, &assets);
                });
        });
    }

    fn toggle_ime(
        input: Res<ButtonInput<MouseButton>>,
        mut window: Single<&mut Window, With<NewFolderWindow>>,
    ) {
        if input.just_pressed(MouseButton::Left) {
            window.ime_position = window.cursor_position().unwrap();
            window.ime_enabled = !window.ime_enabled;
        }
    }

    fn bubbling_text(
        mut commands: Commands,
        mut bubbles: Query<(Entity, &mut Transform, &mut Bubble), With<NewFolderInput>>,
        time: Res<Time>,
    ) {
        for (entity, mut transform, mut bubble) in bubbles.iter_mut() {
            if bubble.timer.tick(time.delta()).just_finished() {
                commands.entity(entity).despawn();
            }
            transform.translation.y += time.delta_secs() * 100.0;
        }
    }

    fn listen_ime_events(
        mut ime_reader: MessageReader<Ime>,
        mut edit_text: Single<&mut Text, With<NewFolderInput>>,
    ) {
        for ime in ime_reader.read() {
            match ime {
                Ime::Preedit { value, cursor, .. } if !cursor.is_none() => {
                    // *ui_writer.text(*status_text, 7) = format!("{value}\n");
                }
                Ime::Preedit { cursor, .. } if cursor.is_none() => {
                    // *ui_writer.text(*status_text, 7) = "\n".to_string();
                }
                Ime::Commit { value, .. } => {
                    if edit_text.len() < 32 {
                        edit_text.push_str(value);
                    }
                }
                Ime::Enabled { .. } => {
                    // *ui_writer.text(*status_text, 5) = "true\n".to_string();
                }
                Ime::Disabled { .. } => {
                    // *ui_writer.text(*status_text, 5) = "false\n".to_string();
                }
                _ => (),
            }
        }
    }

    fn listen_keyboard_input_events(
        mut commands: Commands,
        mut keyboard_input_reader: MessageReader<KeyboardInput>,
        edit_text: Single<(&mut Text, &TextFont), With<NewFolderInput>>,
    ) {
        let (mut text, style) = edit_text.into_inner();
        for keyboard_input in keyboard_input_reader.read() {
            // Only trigger changes when the key is first pressed.
            if !keyboard_input.state.is_pressed() {
                continue;
            }

            match (&keyboard_input.logical_key, &keyboard_input.text) {
                (Key::Enter, _) => {
                    if text.is_empty() {
                        continue;
                    }
                    let old_value = mem::take(&mut **text);

                    commands.spawn((
                        Text::new(old_value),
                        style.clone(),
                        Bubble {
                            timer: Timer::from_seconds(5.0, TimerMode::Once),
                        },
                    ));
                }
                (Key::Backspace, _) => {
                    text.pop();
                }
                (_, Some(inserted_text)) => {
                    // Make sure the text doesn't have any control characters,
                    // which can happen when keys like Escape are pressed
                    if inserted_text.chars().all(Self::is_printable_char) {
                        if text.len() <32 {
                            text.push_str(inserted_text);
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    // this logic is taken from egui-winit:
    // https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
    fn is_printable_char(chr: char) -> bool {
        let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
            || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
            || ('\u{100000}'..='\u{10fffd}').contains(&chr);

        !is_in_private_use_area && !chr.is_ascii_control()
    }

    fn update_fn(
        mut state: ResMut<NextState<SeekerNewFolderState>>,
        res: Res<CurrentFile>,
        mut query: Query<(&Name, &Interaction), (Changed<Interaction>, With<NewFolderFnButton>)>,
        mut window: Single<&mut Window, With<FileDialogWindow>>,
        text: Single<&Text, With<NewFolderInput>>,
    ) {
        for (name, interaction) in query.iter_mut() {
            if *interaction == Interaction::Pressed {
                info!("{name}");
                match name.as_str() {
                    "Cancel" => {
                        window.visible = true;
                        state.set(SeekerNewFolderState::None);
                    }
                    "Create" => {
                        window.visible = true;
                        state.set(SeekerNewFolderState::None);
                        if let Some(file) = &res.file {
                            if let Err(e) = std::fs::create_dir(file.path.join(text.as_str())) {
                                error!("create_dir: {e}");
                            };
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
