use super::components::SlideTimer;
use bevy::prelude::*;

pub fn setup_credits(mut commands: Commands, asset_server: Res<AssetServer>) {
    let paths = vec![
        "matts_slide.png",
        "sams_slide.png",
        "garretts_slide.png",
        "marias_slide.png",
        "griffins_slide.png",
        "alexs_slide.png",
        "makyes_slide.png",
    ];
    let mut timer = 0.;

    for path in paths {
        if timer == 0. {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                ..default()
            });
        } else {
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load(path),
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..default()
                })
                .insert(SlideTimer(Timer::from_seconds(timer, TimerMode::Once)));
        }
        timer += 3.;
    }
}

pub fn next_slide(time: Res<Time>, mut timer: Query<(&mut SlideTimer, &mut Transform)>) {
    let mut position = 2.;

    for (mut timer, mut transform) in timer.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = position;
            position += 1.;
        }
    }
}
