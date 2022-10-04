use bevy::prelude::*;

#[derive(Default)]
pub struct Cursor {
    pub position: Vec2,
}

pub fn update_cursor(
    windows: Res<Windows>,
    mut cursor: ResMut<Cursor>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    let window = windows.get_primary().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos = world_pos.truncate();
        cursor.position = world_pos;
    }
}
