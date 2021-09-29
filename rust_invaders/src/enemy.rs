use std::f32::consts::PI;

use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

use crate::{ActiveEnemies, Enemy, FromEnemy, Laser, Materials, Speed, WinSize, SCALE, TIME_STEPS};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(enemy_laser_movement.system())
            .add_system(enemy_movement.system())
            .add_system_set(
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
            .insert(Enemy)
            .insert(Speed::default());

        active_enemies.0 += 1
    }
}

fn enemy_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Speed), With<Enemy>>) {
    let now = time.seconds_since_startup() as f32;

    for (mut tf, speed) in query.iter_mut() {
        let max_distance = TIME_STEPS * speed.0;
        let x_org = tf.translation.x;
        let y_org = tf.translation.y;

        let (x_offset, y_offset) = (0., 100.);
        let (x_radius, y_radius) = (150.0, 100.0);

        let angle = speed.0 * TIME_STEPS * now % 360.0 / PI;

        let x_dst = x_radius * angle.cos() + x_offset;
        let y_dst = y_radius * angle.sin() + y_offset;

        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0. {
            0.
        } else {
            max_distance / distance
        };

        let x = x_org - dx * distance_ratio;
        let x = if dx > 0.0 { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0.0 { y.max(y_dst) } else { y.min(y_dst) };

        tf.translation.x = x;
        tf.translation.y = y;
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
            .insert(Speed::default());
    }
}

fn enemy_laser_movement(
    window: Res<WinSize>,
    mut commands: Commands,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromEnemy>)>,
) {
    for (enemy_laser_entity, speed, mut laser_tf) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y -= speed.0 * TIME_STEPS;

        if translation.y < -window.height / 2.0 - 50.0 {
            commands.entity(enemy_laser_entity).despawn();
        }
    }
}
