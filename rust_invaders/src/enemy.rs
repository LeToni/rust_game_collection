use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

use crate::{ActiveEnemies, Enemy, FromEnemy, Laser, Materials, Speed, WinSize, SCALE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawn.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.9))
                .with_system(enemy_fire.system()),
        );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    materials: Res<Materials>,
) {
    // Compute random enemy position
    let mut rng = thread_rng();
    let w_span = win_size.width / 2.0 - 100.0;
    let h_span = win_size.height / 2.0 - 100.0;
    let x = rng.gen_range(-w_span..w_span) as f32;
    let y = rng.gen_range(-h_span..h_span) as f32;

    //  spawn enemy
    if active_enemies.0 < 1 {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SCALE, SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy);

        active_enemies.0 += 1
    }
}

fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let x = tf.translation.x;
        let y = tf.translation.y;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_laser_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15.0, 0.0),
                    scale: Vec3::new(SCALE, -SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(Speed::default);
    }
}
