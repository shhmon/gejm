use bevy::prelude::*;

use super::camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player)
            .add_systems(Update, camera::update_camera_controller);
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();

    commands.spawn((
        Player,
        Camera3d { ..default() },
        Projection::Perspective(PerspectiveProjection {
            fov: fov,
            ..default()
        }),
        Transform::from_translation(Vec3::new(0., 10., 0.)),
        camera::CameraController::default(),
    ));
}
