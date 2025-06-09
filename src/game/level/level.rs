use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn((
        Collider::cuboid(1000., 0., 1000.),
        MeshMaterial3d(level_material.clone()),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.)))),
        Transform::IDENTITY,
    ));

    commands.spawn((
        Collider::cuboid(30., 30., 30.),
        MeshMaterial3d(level_material.clone()),
        Mesh3d(meshes.add(Cuboid::from_length(60.))),
        Transform::from_xyz(0., 0., -100.),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
