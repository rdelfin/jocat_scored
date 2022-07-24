use crate::resources::{AppState, AssetsLoading};
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            title: "Swing".to_string(),
            width: 1920.,
            height: 1080.,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .insert_resource(AssetsLoading(vec![]))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_state(AppState::Loading)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(systems::load_all_assets),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(systems::check_loaded_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(systems::start_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(systems::start_game))
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(systems::animate_sprite_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Menu).with_system(systems::menu_window_system),
        )
        .add_system(exit_on_esc_system)
        .run();
}
