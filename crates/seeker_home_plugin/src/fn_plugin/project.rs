use crate::FnUi;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::LineHeight;
use bevy::ui::FocusPolicy;
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::project_list::ProjectListResource;
use seeker_resource::SeekerResource;
use seeker_state::{SeekerFileDialogFnState, SeekerHomeSubFnState, SeekerHomeSubLoadState};
use seeker_trait::SeekerTrait;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct ProjectItemButton;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct FileDialogButton;

#[derive(Component)]
pub struct ProjectPlugin;

impl SeekerTrait for ProjectPlugin {}

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ProjectListResource::default())
            .add_systems(OnEnter(SeekerHomeSubLoadState::Loaded), Self::project_enter)
            .add_systems(OnEnter(SeekerHomeSubFnState::Project), Self::project_enter)
            // .add_systems(OnEnter(SeekerHomeSubFnState::NewProject), Self::new_project)
            // .add_systems(OnEnter(SeekerHomeSubFnState::Open), Self::open_project)
            .add_systems(
                OnEnter(SeekerHomeSubFnState::CloneRepo),
                Self::clone_project,
            )
            .add_systems(
                Update,
                Self::update_color_state::<ProjectItemButton, SeekerHomeSubFnState>
                    .run_if(in_state(SeekerHomeSubFnState::Project)),
            )
            .add_systems(
                Update,
                Self::update_color_state::<FileDialogButton, SeekerFileDialogFnState>
                    .run_if(in_state(SeekerHomeSubFnState::Project)),
            )
            .add_observer(
                Self::button_on_hovered_changed_color::<Insert, Hovered, ProjectItemButton>,
            )
            .add_observer(
                Self::button_on_hovered_changed_color::<Insert, Hovered, FileDialogButton>,
            );
    }
}

impl ProjectPlugin {
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
        assets: Res<AssetServer>,
        project_list: Res<ProjectListResource>,
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
                                            if text == "Open" {
                                                Self::ui_button_same(
                                                    parent,
                                                    FileDialogButton,
                                                    text,
                                                    &res,
                                                    &assets,
                                                );
                                            } else {
                                                Self::ui_button_same(
                                                    parent,
                                                    ProjectItemButton,
                                                    text,
                                                    &res,
                                                    &assets,
                                                );
                                            }
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
                                for project in &project_list.projects {
                                    parent
                                        .spawn((
                                            ProjectItemButton,
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
                                                        Text::new(get_project_name_short(&project.project_name)),
                                                        TextLayout {
                                                            justify: Justify::Center,
                                                            ..default()
                                                        },
                                                        TextFont {
                                                            line_height: LineHeight::RelativeToFont(
                                                                1.2,
                                                            ),
                                                            font_size: 16.0,
                                                            font: assets
                                                                .load(MAPLE_MONO_BOLD_ITALIC),
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
                                                        Text::new(&project.project_name),
                                                        TextFont {
                                                            font: assets
                                                                .load(MAPLE_MONO_BOLD_ITALIC),
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                        TextColor(res.colors.home_font_color),
                                                    ));
                                                    parent.spawn((
                                                        Text::new(&project.path),
                                                        TextFont {
                                                            font: assets
                                                                .load(MAPLE_MONO_BOLD_ITALIC),
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                        TextColor(res.colors.home_font_grey_color),
                                                    ));
                                                });
                                        });
                                }
                            });
                    });
            });
        }
    }
}

/// 获取项目名称的简称（两个字符）
fn get_project_name_short(project_name: &str) -> String {
    let project_name:String = project_name.chars().take(2).collect();
    project_name.to_uppercase()
}