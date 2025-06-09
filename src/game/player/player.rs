use bevy::prelude::*;

use crate::game::player::camera_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player)
            .add_systems(Update, camera_controller::update_camera_controller);
    }
}

#[derive(Component)]
pub struct Player;

// fn debug(time: Res<Time>, mut query: Query<&mut Transform, With<Player>>) {
//     for mut player in &mut query {
//         player.rotate_y(time.delta_secs());
//     }
// }

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
        camera_controller::CameraController::default(),
    ));
}
