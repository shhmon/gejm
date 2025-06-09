use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct CameraController {
    pub rotation: Vec2,
    pub vertical_limit: f32,
    pub sensitivity: f32,
    pub locked: bool,
}

impl CameraController {}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            rotation: Vec2::ZERO,
            sensitivity: 0.3,
            vertical_limit: 80.0,
            locked: false,
        }
    }
}

pub fn update_camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    if let Ok((mut camera_controller, mut transform)) = query.single_mut() {
        // Update camera lock
        if keys.just_pressed(KeyCode::Escape) {
            camera_controller.locked = !camera_controller.locked;
        }

        if camera_controller.locked {
            return;
        }

        for ev in mouse_motion.read() {
            camera_controller.rotation.y -= ev.delta.x * camera_controller.sensitivity;
            camera_controller.rotation.x -= ev.delta.y * camera_controller.sensitivity;

            camera_controller.rotation.x = f32::clamp(
                camera_controller.rotation.x,
                -camera_controller.vertical_limit,
                camera_controller.vertical_limit,
            )
        }

        let y_quat = Quat::from_axis_angle(Vec3::Y, camera_controller.rotation.y.to_radians());
        let x_quat = Quat::from_axis_angle(Vec3::X, camera_controller.rotation.x.to_radians());
        transform.rotation = y_quat * x_quat;
    }
}
