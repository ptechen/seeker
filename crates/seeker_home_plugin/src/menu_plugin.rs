use crate::MenuUi;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy::text::LineHeight;
use bevy::ui::FocusPolicy;
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerHomeSubFnState, SeekerHomeSubLoadState, SeekerState};
use seeker_trait::SeekerTrait;

#[derive(Component)]
pub struct MenuPlugin;

impl SeekerTrait for MenuPlugin {}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct HomeMenuButton;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerHomeSubLoadState::Loaded), Self::menu_enter);
        app.add_systems(OnEnter(SeekerState::Home), Self::menu_enter)
            .add_systems(
                Update,
                Self::update_color_state::<HomeMenuButton, SeekerHomeSubFnState>.run_if(
                    in_state(SeekerState::Home).or(in_state(SeekerHomeSubLoadState::Loaded)),
                ),
            );
    }
}

impl MenuPlugin {
    fn menu_enter(
        mut commands: Commands,
        res: Res<SeekerResource>,
        mut query: Query<Entity, With<MenuUi>>,
        assets: Res<AssetServer>,
    ) {
        for entity in query.iter_mut() {
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn((
                        Name::new("SeekerVersion"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Seeker"),
                            TextColor(res.colors.home_font_color),
                            TextFont {
                                font: assets.load(MAPLE_MONO_BOLD_ITALIC),
                                ..default()
                            },
                        ));
                        parent.spawn((
                            Text::new("0.0.1"),
                            TextFont {
                                font_size: 12.0,
                                font: assets.load(MAPLE_MONO_BOLD_ITALIC),
                                ..default()
                            },
                            TextColor(res.colors.home_font_color),
                        ));
                    });
                parent
                    .spawn((
                        Name::new("SeekerHomeMenu"),
                        Node {
                            padding: UiRect::all(Val::Px(10.0)),
                            width: Val::Percent(100.0),
                            height: Val::Px(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Name::new("Project"),
                                HomeMenuButton,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(30.0),
                                    box_sizing: BoxSizing::BorderBox,
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    justify_self: JustifySelf::Center,
                                    padding: UiRect::left(Val::Px(20.0)),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(3.)),
                                BackgroundColor(res.colors.home_hovered),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Projects"),
                                    TextFont {
                                        font_size: 20.0,
                                        line_height: LineHeight::Px(30.0),
                                        font: assets.load(MAPLE_MONO_BOLD_ITALIC),
                                        ..default()
                                    },
                                    TextColor(res.colors.home_font_color),
                                ));
                            });
                        parent
                            .spawn((
                                Name::new("Projects1"),
                                HomeMenuButton,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(30.0),
                                    box_sizing: BoxSizing::BorderBox,
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Start,
                                    padding: UiRect::left(Val::Px(20.0)),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(3.)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Projects"),
                                    TextFont {
                                        font_size: 20.0,
                                        line_height: LineHeight::Px(30.0),
                                        font: assets.load(MAPLE_MONO_BOLD_ITALIC),
                                        ..default()
                                    },
                                    TextColor(res.colors.home_font_color),
                                ));
                            });
                    });
            });
        }
    }
}
