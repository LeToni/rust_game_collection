use bevy::{input::keyboard::keyboard_input_system, prelude::*};

const PLAYER_SPRITE: &str = "player.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEPS: f32 = 1.0 / 60.0;
// region: Resources
struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
}
// endregiom: Resources

// region: Components
struct Player;
struct Laser;
struct Speed(f32);
// endregion: Components
impl Default for Speed {
    fn default() -> Self {
        Self(500.0)
    }
}

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
        .add_system(player_movement.system())
        .add_system(player_shoots.system())
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
        laser_materials: materials.add(assert_server.load(LASER_SPRITE).into()),
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    // spawn sprite
    let window = windows.get_primary().unwrap();
    let bottom = -window.height() / 2.0;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75.0 / 4.0 + 5., 10.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        transform.translation.x += dir * speed.0 * TIME_STEPS;
    }
}

fn player_shoots(
    mut commands: Commands,
    materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, With<Player>)>,
) {
    if let Ok((player_tf, _)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            let pos_x = player_tf.translation.x;
            let pos_y = player_tf.translation.y;

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.laser_materials.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(Speed::default());
        }
    }
}
