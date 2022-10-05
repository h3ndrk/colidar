use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.0;
    commands.spawn_bundle(camera).insert(ZoomCamera {
        min_scale: 0.1,
        max_scale: 10.0,
        scroll_speed: 0.01,
    });
}

#[derive(Component)]
pub struct ZoomCamera {
    min_scale: f32,
    max_scale: f32,
    scroll_speed: f32,
}

pub fn zoom_camera(
    mut query: Query<(&ZoomCamera, &mut OrthographicProjection)>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if scroll_events.is_empty() {
        return;
    }

    let pixels_per_line = 10.0;
    let scroll = scroll_events
        .iter()
        .map(|event| match event.unit {
            MouseScrollUnit::Pixel => event.y,
            MouseScrollUnit::Line => event.y * pixels_per_line,
        })
        .sum::<f32>();

    for (zoom_camera, mut projection) in &mut query {
        projection.scale = (projection.scale * (1.0 - scroll * zoom_camera.scroll_speed))
            .clamp(zoom_camera.min_scale, zoom_camera.max_scale);
    }
}
