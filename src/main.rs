use crate::resources::{AppState, AssetsLoading};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Swing".to_string(),
                    name: Some("Swing".to_string()),
                    resolution: (1920., 1080.).into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin,
            AudioPlugin,
            bevy_framepace::FramepacePlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .insert_resource(AssetsLoading(vec![]))
        .insert_state(AppState::Loading)
        // Loading systems
        .add_systems(OnEnter(AppState::Loading), systems::load_all_assets)
        .add_systems(
            Update,
            systems::check_loaded_system.run_if(in_state(AppState::Loading)),
        )
        // Menu systems
        .add_systems(OnEnter(AppState::Menu), systems::start_menu)
        .add_systems(
            Update,
            systems::menu_window_system.run_if(in_state(AppState::Menu)),
        )
        // Game systems
        .add_systems(OnEnter(AppState::InGame), systems::start_game)
        .add_systems(
            Update,
            (systems::animate_sprite_system, systems::attack_system)
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}
