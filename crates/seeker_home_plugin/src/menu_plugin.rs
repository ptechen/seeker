use bevy::prelude::*;
use bevy::text::LineHeight;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerHomeSubFnState, SeekerHomeSubLoadState, SeekerState};
use crate::MenuUi;

#[derive(Component)]
pub struct MenuPlugin;

#[derive(Component)]
pub struct HomeMenuButton;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerHomeSubLoadState::Loaded), Self::menu_enter);
        app.add_systems(OnEnter(SeekerState::Home), Self::menu_enter)
            .add_systems(Update, Self::menu_update);
    }
}

impl MenuPlugin {
    fn menu_enter(
        mut commands: Commands,
        res: Res<SeekerResource>,
        mut query: Query<Entity, With<MenuUi>>,
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
                        ));
                        parent.spawn((
                            Text::new("0.0.1"),
                            TextFont {
                                font_size: 12.0,
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
                                Button,
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
                                        ..default()
                                    },
                                    TextColor(res.colors.home_font_color),
                                ));
                            });
                        parent
                            .spawn((
                                Name::new("Projects1"),
                                HomeMenuButton,
                                Button,
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
                                        ..default()
                                    },
                                    TextColor(res.colors.home_font_color),
                                ));
                            });
                    });
            });
        }
    }

    fn menu_update(
        mut query: ParamSet<(
            Query<(&Interaction, &mut BackgroundColor, &Name), (Changed<Interaction>, With<Button>)>,
            Query<&mut BackgroundColor, With<HomeMenuButton>>,
        )>,
        mut state: ResMut<NextState<SeekerHomeSubFnState>>,
        res: Res<SeekerResource>,
    ) {
        let mut has_pressed = false;
        query.p0().iter().for_each(|(interaction, _, _)| {
            if *interaction == Interaction::Pressed {
                has_pressed = true;
            }
        });
        if has_pressed {
            query.p1().iter_mut().for_each(|mut color| {
                *color = BackgroundColor(Color::NONE);
            });
        }
        for (interaction, mut color, name) in &mut query.p0().iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    state.set(match name.as_str() {
                        "Project" => SeekerHomeSubFnState::Project,
                        "Open" => SeekerHomeSubFnState::FileDialog,
                        "NewProject" => SeekerHomeSubFnState::NewProject,
                        "CloneRepo" => SeekerHomeSubFnState::CloneRepo,
                        _ => SeekerHomeSubFnState::None,
                    });
                    *color = BackgroundColor(res.colors.home_hovered);
                }
                _ => {}
            }
        }
    }
}