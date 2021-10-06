use bevy::prelude::*;

const WIN_H: f32 = 600.;
const WIN_W: f32 = 600.;
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;

const PLAYER_SPRITE: &str = "player.png";
const PLAYER_LASER_SPRITE: &str = "laser_player.png";

// region: Resources

struct Materials {
    player: Handle<ColorMaterial>,
    player_laser: Handle<ColorMaterial>,
}

// endregion: Resources

// region Components

struct Player;
struct PlayerLaser;
struct PlayerReadyToFire(bool);

struct Laser;
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}
// endregion Components

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "Rust Space Invaders (Classic version)".to_string(),
            width: WIN_W,
            height: WIN_H,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup",
            SystemStage::parallel().with_system(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(player_shoots.system())
        .add_system(laser_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create main resources
    commands.insert_resource(Materials {
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        player_laser: materials.add(asset_server.load(PLAYER_LASER_SPRITE).into()),
    });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>) {
    let bottom = -WIN_H / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyToFire(true))
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut tf, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        tf.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_shoots(
    mut commands: Commands,
    materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut PlayerReadyToFire, With<Player>)>,
) {
    if let Ok((tf, mut ready_to_fire, _)) = query.single_mut() {
        if ready_to_fire.0 && keyboard_input.pressed(KeyCode::Space) {
            let pos_x = tf.translation.x;
            let pos_y = tf.translation.y;

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos_x, pos_y, 0.),
                        scale: Vec3::new(SCALE, SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(PlayerLaser)
                .insert(Speed::default());

            ready_to_fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) {
            ready_to_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<PlayerLaser>)>,
) {
    for (laser_entity, speed, mut laser_tf) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;

        if translation.y > WIN_H {
            commands.entity(laser_entity).despawn();
        }
    }
}
