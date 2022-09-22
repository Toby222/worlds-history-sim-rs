use {
    crate::components::panning::Pan2d,
    bevy::{
        app::{App, Plugin},
        ecs::{
            event::EventReader,
            query::With,
            system::{Query, Res},
        },
        input::{
            mouse::{MouseButton, MouseMotion},
            Input,
        },
        sprite::Sprite,
        transform::components::Transform,
    },
};

#[derive(Default)]
pub(crate) struct PanningPlugin;

impl Plugin for PanningPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_system(panning_system_2d);
    }
}

fn panning_system_2d(
    mut query: Query<'_, '_, (&mut Transform, &Pan2d), With<Sprite>>,
    mut mouse_motion_events: EventReader<'_, '_, MouseMotion>,
    input_mouse: Res<'_, Input<MouseButton>>,
) {
    if !input_mouse.pressed(MouseButton::Left) {
        return;
    }

    let mut horizontal = 0.0;
    for movement in mouse_motion_events.iter() {
        horizontal += movement.delta.x;
    }

    query.for_each_mut(|(mut transform, pan)| {
        if pan.enabled {
            transform.translation.x += horizontal;
        }
    });
}
