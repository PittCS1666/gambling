use bevy::{prelude::*, window::PresentMode, transform, pbr::ScreenSpaceAmbientOcclusionBundle};

const TITLE: &str = "School Sanctioned Gambling";
const WIN_WIDTH: f32 = 1280.;
const WIN_HEIGHT: f32 = 720.;

#[derive(Component, Deref, DerefMut)]
struct SlideTimer(Timer);



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Fifo,
                resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                title: TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, next_slide)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let paths = vec!["matts_slide.png", "sams_slide.png", "garretts_slide.png", "griffins_slide.png"];
    let mut timer = 0.;

    for path in paths {
        if timer == 0. {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                ..default()
            });
        }
        else {
            commands.spawn(SpriteBundle {
                texture: asset_server.load(path),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            }).insert(SlideTimer(Timer::from_seconds(timer,TimerMode::Once)));
        }
        timer += 3.;
    }
}

fn next_slide(time: Res<Time>, mut timer: Query<(&mut SlideTimer, &mut Transform)>) {
    let mut position = 2.;

    for (mut timer, mut transform) in timer.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = position;
            position += 1.;   
        }
    }
    
}
