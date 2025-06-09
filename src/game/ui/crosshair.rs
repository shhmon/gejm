use bevy::prelude::*;

pub fn spawn_crosshair(mut commands: Commands) {
    let crosshair_size = 2.0;

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(crosshair_size),
                    height: Val::Px(crosshair_size),
                    left: Val::Percent(50.),
                    top: Val::Percent(50.),
                    margin: UiRect {
                        top: Val::Px(-crosshair_size / 2.),
                        left: Val::Px(-crosshair_size / 2.),
                        ..default()
                    },
                    ..default()
                },
                ImageNode {
                    image: ImageNode::solid_color(Color::srgb(0., 1., 0.)).image,
                    ..default()
                },
            ));
        });
}
