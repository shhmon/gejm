use bevy::prelude::*;

pub struct TracerPlugin;

impl Plugin for TracerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tracers);
    }
}

#[derive(Component)]
pub struct BulletTracer {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub lifetime: f32,
    pub time_alive: f32,
}

impl BulletTracer {
    pub fn new(start: Vec3, end: Vec3, speed: f32) -> Self {
        BulletTracer {
            start_pos: start,
            end_pos: end,
            lifetime: Vec3::distance(start, end) / speed,
            time_alive: 0.,
        }
    }
}

fn update_tracers(
    mut commands: Commands,
    mut query: Query<(&mut BulletTracer, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut tracer, mut transform, entity) in &mut query {
        tracer.time_alive += time.delta_secs();
        let progress = f32::clamp(tracer.time_alive / tracer.lifetime, 0., 1.);

        transform.translation = Vec3::lerp(tracer.start_pos, tracer.end_pos, progress);

        transform.rotate_local_y(time.delta_secs() * 10.);
        transform.rotate_local_x(time.delta_secs() * 5.);

        if tracer.time_alive > tracer.lifetime {
            commands.entity(entity).despawn();
        }
    }
}
