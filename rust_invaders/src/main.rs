mod enemy;
mod player;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy.png";
const TIME_STEPS: f32 = 1.0 / 60.0;
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;
const SCALE: f32 = 0.5;

// region: Resources
struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
    enemy_materials: Handle<ColorMaterial>,
}

#[allow(unused)]
struct WinSize {
    width: f32,
    height: f32,
}

struct ActiveEnemies(u32);
// endregiom: Resources

// region: Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Enemy;
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
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(laser_hit_enemy.system())
        .run();
}

fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(assert_server.load(PLAYER_SPRITE).into()),
        laser_materials: materials.add(assert_server.load(LASER_SPRITE).into()),
        enemy_materials: materials.add(assert_server.load(ENEMY_SPRITE).into()),
    });
    commands.insert_resource(WinSize {
        width: window.width(),
        height: window.height(),
    })
}

fn laser_hit_enemy(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &Transform, &Sprite, With<Laser>)>,
    mut enemy_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    for (laser_entity, laser_tf, laser_sprite, _) in laser_query.iter_mut() {
        for (enemy_entity, enemy_tf, enemy_sprite, _) in enemy_query.iter_mut() {
            let laser_scale = Vec2::from(laser_tf.scale);
            let enemy_scale = Vec2::from(enemy_tf.scale);
            let collision = collide(
                laser_tf.translation,
                laser_sprite.size * laser_scale,
                enemy_tf.translation,
                enemy_sprite.size * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                active_enemies.0 -= 1;

                commands.entity(laser_entity).despawn();
            }
        }
    }
}
