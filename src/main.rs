use bevy::{input::system::exit_on_esc_system, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            title: "Swing".to_string(),
            width: 1280.,
            height: 800.,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system)
        .run();
}
