use bevy::color::Color;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
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
}
