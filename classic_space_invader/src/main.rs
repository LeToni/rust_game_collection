use bevy::prelude::*;

const WIN_H: f32 = 600.;
const WIN_W: f32 = 600.;

// region: Resources

// endregion: Resources

// region Components

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
        .run();
}
