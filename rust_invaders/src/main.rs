use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn sprite
    let window = windows.get_primary().unwrap();
    let bottom = -window.height() / 2.0;

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(assert_server.load(PLAYER_SPRITE).into()),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75.0 / 4.0 + 5., 10.0),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
