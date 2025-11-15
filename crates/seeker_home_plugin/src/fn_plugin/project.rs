use crate::FnUi;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::LineHeight;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerHomeSubFnState, SeekerHomeSubLoadState};

#[derive(Component)]
pub struct ProjectItemButton;

#[derive(Component)]
pub struct Project;

impl Plugin for Project {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SeekerHomeSubLoadState::Loaded), Self::project_enter)
            .add_systems(OnEnter(SeekerHomeSubFnState::Project), Self::project_enter)
            .add_systems(OnEnter(SeekerHomeSubFnState::NewProject), Self::new_project)
            .add_systems(OnEnter(SeekerHomeSubFnState::Open), Self::open_project)
            .add_systems(
                OnEnter(SeekerHomeSubFnState::CloneRepo),
                Self::clone_project,
            )
            .add_observer(Self::button_on_interaction::<Insert, Hovered>);
    }
}

impl Project {
    fn new_project(
        mut commands: Commands,
        query: Query<Entity, With<FnUi>>,
        res: Res<SeekerResource>,
    ) {
        println!("NewProject");
        for entity in query.iter() {
            commands.entity(entity).with_children(|parent| {});
        }
    }

    fn open_project(
        mut commands: Commands,
        query: Query<Entity, With<FnUi>>,
        res: Res<SeekerResource>,
    ) {
        println!("OpenProject");
        for entity in query.iter() {
            commands.entity(entity).with_children(|parent| {});
        }
    }

    fn clone_project(
        mut commands: Commands,
        query: Query<Entity, With<FnUi>>,
        res: Res<SeekerResource>,
    ) {
        println!("CloneRepo");
        for entity in query.iter() {
            commands.entity(entity).with_children(|parent| {});
        }
    }

    fn project_enter(
        mut commands: Commands,
        query: Query<Entity, With<FnUi>>,
        res: Res<SeekerResource>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn((
                        DespawnOnExit(SeekerHomeSubFnState::Project),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            box_sizing: BoxSizing::BorderBox,
                            padding: UiRect::new(
                                Val::Px(20.0),
                                Val::Px(20.0),
                                Val::ZERO,
                                Val::ZERO,
                            ),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Name::new("ProjectListHeader"),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(80.0),
                                    box_sizing: BoxSizing::BorderBox,
                                    border: UiRect::bottom(Val::Px(2.0)),
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Start,
                                    justify_items: JustifyItems::Center,
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                BorderColor::all(res.colors.home_menu),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Node {
                                        height: Val::Percent(100.0),
                                        width: Val::Percent(40.0),
                                        ..default()
                                    },
                                    Name::new("Search"),
                                ));
                                parent
                                    .spawn((Node {
                                        width: Val::Percent(60.0),
                                        height: Val::Percent(100.0),
                                        box_sizing: BoxSizing::BorderBox,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceAround,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        for text in vec!["NewProject", "Open", "CloneRepo"] {
                                            parent
                                                .spawn((
                                                    Name::new(text),
                                                    Node {
                                                        padding: UiRect::new(
                                                            Val::Px(15.),
                                                            Val::Px(15.),
                                                            Val::Px(5.0),
                                                            Val::Px(5.0),
                                                        ),
                                                        box_sizing: BoxSizing::BorderBox,
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        justify_content: JustifyContent::Center,
                                                        align_self: AlignSelf::Center,
                                                        ..default()
                                                    },
                                                    BorderRadius::all(Val::Px(3.)),
                                                    ProjectItemButton,
                                                    Button,
                                                    Hovered::default(),
                                                    BorderColor::all(res.colors.button_border),
                                                    BackgroundColor(res.colors.home_menu),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        Text::new(text),
                                                        TextFont {
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                        TextColor(res.colors.home_font_color),
                                                    ));
                                                });
                                        }
                                    });
                            });
                        parent.spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(20.0),
                            ..default()
                        });
                        parent
                            .spawn((
                                Name::new("ProjectList"),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    padding: UiRect::new(
                                        Val::Px(10.0),
                                        Val::Px(10.0),
                                        Val::ZERO,
                                        Val::ZERO,
                                    ),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::scroll_y(),
                                    ..default()
                                },
                            ))
                            .with_children(|parent| {
                                for i in 0..20 {
                                    parent
                                        .spawn((
                                            ProjectItemButton,
                                            Button,
                                            Hovered::default(),
                                            Name::new("ProjectListItem"),
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(50.0),
                                                min_height: Val::Px(50.0),
                                                max_height: Val::Px(50.0),
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::left(Val::Px(10.0)),
                                                ..default()
                                            },
                                            BorderRadius::all(Val::Px(3.)),
                                        ))
                                        .with_children(|parent| {
                                            parent
                                                .spawn((
                                                    Node {
                                                        width: Val::Px(30.0),
                                                        height: Val::Px(30.0),
                                                        box_sizing: BoxSizing::BorderBox,
                                                        display: Display::Flex,
                                                        flex_direction: FlexDirection::Column,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb_u8(50, 50, 50)),
                                                    BorderRadius::all(Val::Px(3.)),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        Text::new(format!("P{}", i)),
                                                        TextLayout {
                                                            justify: Justify::Center,
                                                            ..default()
                                                        },
                                                        TextFont {
                                                            line_height: LineHeight::RelativeToFont(
                                                                1.2,
                                                            ),
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                        TextColor(res.colors.home_font_color),
                                                    ));
                                                });
                                            parent
                                                .spawn((
                                                    Node {
                                                        width: Val::Percent(100.0),
                                                        height: Val::Px(30.0),
                                                        padding: UiRect::left(Val::Px(10.0)),
                                                        box_sizing: BoxSizing::BorderBox,
                                                        display: Display::Flex,
                                                        flex_direction: FlexDirection::Column,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    BorderRadius::all(Val::Px(3.)),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        Text::new(format!("Project {}", i)),
                                                        TextColor(res.colors.home_font_color),
                                                    ));
                                                });
                                        });
                                }
                            });
                    });
            });
        }
    }

    pub fn button_on_interaction<E: EntityEvent, C: Component>(
        _event: On<E, C>,
        mut query: Query<(&mut BackgroundColor, &Hovered), With<ProjectItemButton>>,
        res: Res<SeekerResource>,
    ) {
        for (mut color, has_hovered) in &mut query.iter_mut() {
            if has_hovered.get() {
                *color = BackgroundColor(res.colors.home_hovered);
            } else {
                *color = BackgroundColor(Color::NONE);
            }
        }
    }
}
