use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";
const TIME_STEPS: f32 = 1.0 / 60.0;
// region: Resources
struct Materials {
    player_materials: Handle<ColorMaterial>,
}
// endregiom: Resources

// region: Components
struct Player;
struct PlayerSpeed(f32);
// endregion: Components

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
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(assert_server.load(PLAYER_SPRITE).into()),
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    // spawn sprite
    let window = windows.get_primary().unwrap();
    let bottom = -window.height() / 2.0;

    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75.0 / 4.0 + 5., 10.0),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
