use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_systems(Update, update_cursor_locking)
            .add_systems(Startup, init_cursor);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    locked: bool,
}

impl Cursor {
    pub fn toggle_lock(&mut self, window: &mut Mut<'_, Window>) {
        self.locked = !self.locked;
        window.cursor_options.visible = !self.locked;

        if self.locked {
            let window_w = window.width();
            let window_h = window.height();
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.set_cursor_position(Some(Vec2::new(window_w / 2., window_h / 2.)));
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
        }
    }
}

fn init_cursor(mut cursor: ResMut<Cursor>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = query.single_mut().unwrap();
    cursor.toggle_lock(&mut window);
}

fn update_cursor_locking(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor: ResMut<Cursor>,
    mut query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = query.single_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        cursor.toggle_lock(&mut window);
    }
}
