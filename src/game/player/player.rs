use core::f32;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use super::{camera, tracer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tracer::TracerPlugin)
            .add_systems(Startup, init_player)
            .add_systems(Update, (update_player, camera::update_camera_controller));
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov = 90.0_f32.to_radians();

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

fn update_player(
    time: Res<Time>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    rapier_context: ReadRapierContext,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window = window_query.single().unwrap();
    let rapier = rapier_context.single().unwrap();

    if let Ok((_player, mut transform, global_transform, camera)) = player_query.single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Ok(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };

            let hit = rapier.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                QueryFilter::default(),
            );

            if let Some((_entity, ray_intersection)) = hit {
                let tracer_material = StandardMaterial {
                    base_color: Color::srgb(1., 1., 0.),
                    unlit: false,
                    ..default()
                };

                commands.spawn((
                    transform.looking_at(ray_intersection.point, Vec3::Y),
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(1., 1., 1.)))),
                    MeshMaterial3d(materials.add(tracer_material)),
                    tracer::BulletTracer::new(transform.translation, ray_intersection.point, 75.),
                ));
            }
        }

        let speed = 40.0;
        let quat = Quat::from_rotation_y(transform.rotation.to_euler(EulerRot::YXZ).0);
        let mut forward = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            forward += quat * -Vec3::Z;
        }

        if keys.pressed(KeyCode::KeyS) {
            forward += quat * Vec3::Z;
        }

        if keys.pressed(KeyCode::KeyA) {
            forward += quat * -Vec3::X;
        }

        if keys.pressed(KeyCode::KeyD) {
            forward += quat * Vec3::X;
        }

        let target = forward.clamp_length(0., 1.);
        transform.translation += target * speed * time.delta_secs();
    }
}
