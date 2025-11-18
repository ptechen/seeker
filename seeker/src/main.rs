use std::time::Duration;
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::{PresentMode, WindowResolution};
use bevy::winit::{UpdateMode, WinitSettings};
use seeker_config::SEEKER_CONFIG;
use seeker_embed_plugin::SeekerEmbedPlugin;
use seeker_home_plugin::SeekerHomePlugin;
use seeker_resource::SeekerResource;
use seeker_scroll::SeekerScrollPlugin;
use seeker_state::SeekerState;

fn main() {

    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::reactive_low_power(Duration::from_secs(60)),
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(120)),
        })
        .insert_resource(SeekerResource::new())
        .init_resource::<InputFocus>()
        .add_plugins(UiWidgetsPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                title: "Seeker".to_string(),
                resolution: WindowResolution::new(1000, 700),
                resizable: true,   // 允许调整窗口大小
                decorations: true, // 显示窗口装饰
                window_theme: Some(SEEKER_CONFIG.window_theme),
                ..default()
            }),
            ..default()
        }))
        .insert_state(SeekerState::default())
        .add_plugins(SeekerEmbedPlugin)
        .add_plugins(SeekerScrollPlugin)
        .add_systems(Startup, setup)
        .add_plugins(SeekerHomePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
