mod fn_plugin;
mod menu_plugin;

use bevy::prelude::*;
use seeker_resource::SeekerResource;
use seeker_state::{
    SeekerFileDialogFnState, SeekerHomeSubFnState, SeekerHomeSubLoadState, SeekerState,
};

#[derive(Component)]
pub struct MenuUi;

#[derive(Component)]
pub struct FnUi;

#[derive(Component)]
pub struct SeekerHomePlugin;

impl Plugin for SeekerHomePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(SeekerHomeSubFnState::Project)
            .insert_state(SeekerHomeSubLoadState::default())
            .insert_state(SeekerFileDialogFnState::default())
            .add_systems(OnEnter(SeekerState::Home), Self::home_enter)
            .add_plugins(menu_plugin::MenuPlugin)
            .add_plugins(fn_plugin::fn_plugin::FnPlugin);
    }
}

impl SeekerHomePlugin {
    fn home_enter(
        mut commands: Commands,
        res: Res<SeekerResource>,
        mut state: ResMut<NextState<SeekerHomeSubLoadState>>,
    ) {
        commands
            .spawn((
                DespawnOnExit(SeekerState::Home),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    MenuUi,
                    Node {
                        width: Val::Px(225.),
                        height: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(res.colors.home_menu),
                ));
                parent.spawn((
                    FnUi,
                    Node {
                        flex_grow: 1.,
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(res.colors.project_list),
                ));
            });
        state.set(SeekerHomeSubLoadState::Loaded);
    }
}
