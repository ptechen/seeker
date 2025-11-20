use bevy::color::Color;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use seeker_resource::fonts::MAPLE_MONO_BOLD_ITALIC;
use seeker_resource::SeekerResource;

pub trait SeekerTrait {
    fn update_color_state<T: Component, S: FreelyMutableState + From<String>>(
        mut query: ParamSet<(
            Query<(&Interaction, &mut BackgroundColor, &Name), (Changed<Interaction>, With<T>)>,
            Query<&mut BackgroundColor, With<T>>,
        )>,
        mut state: ResMut<NextState<S>>,
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
                    let name = name.to_string();
                    info!("{} pressed", name);
                    state.set(S::from(name));
                    *color = BackgroundColor(res.colors.home_hovered);
                }
                _ => {}
            }
        }
    }

    fn button_on_hovered_changed_color<E: EntityEvent, C: Component, BUTTON: Component>(
        _event: On<E, C>,
        mut query: Query<(&mut BackgroundColor, &Hovered), With<BUTTON>>,
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

    fn button_on_pressed_changed_color<BUTTON: Component>(
        mut query: ParamSet<(
            Query<
                (Entity, &mut BackgroundColor, &Interaction),
                (Changed<Interaction>, With<BUTTON>),
            >,
            Query<(Entity, &mut BackgroundColor), With<BUTTON>>,
        )>,
        res: Res<SeekerResource>,
    ) {
        let mut entity_id = 0;
        query
            .p0()
            .iter_mut()
            .for_each(|(entity, mut color, interaction)| {
                if *interaction == Interaction::Pressed {
                    *color = BackgroundColor(res.colors.home_hovered);
                    entity_id = entity.index();
                }
            });

        if entity_id > 0 {
            query.p1().iter_mut().for_each(|(entity, mut color)| {
                if entity.index() != entity_id {
                    *color = BackgroundColor(Color::NONE);
                }
            });
        }
    }

    fn ui_button<BUTTON: Component>(
        parent: &mut RelatedSpawnerCommands<ChildOf>,
        button: BUTTON,
        name: &str,
        text: &str,
        res: &Res<SeekerResource>,
        assets: &Res<AssetServer>,
    ) {
        parent
            .spawn((
                Name::new(name.to_string()),
                button,
                Node {
                    padding: UiRect::new(Val::Px(15.), Val::Px(15.), Val::Px(5.0), Val::Px(5.0)),
                    box_sizing: BoxSizing::BorderBox,
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                BorderRadius::all(Val::Px(3.)),
                Hovered::default(),
                BorderColor::all(res.colors.button_border),
                BackgroundColor(res.colors.home_menu),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: 16.0,
                        font: assets.load(MAPLE_MONO_BOLD_ITALIC),
                        ..default()
                    },
                    TextColor(res.colors.home_font_color),
                ));
            });
    }

    /// text name 相同
    fn ui_button_same<BUTTON: Component>(
        parent: &mut RelatedSpawnerCommands<ChildOf>,
        button: BUTTON,
        name: &str,
        res: &Res<SeekerResource>,
        assets: &Res<AssetServer>,
    ) {
        Self::ui_button(parent, button, name, name, res, assets);
    }
}
