use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    Menu,
    InGame,
}

struct MusicHandle(Option<Handle<AudioSource>>);

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
        .insert_resource(MusicHandle(None))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_state(AppState::Loading)
        .add_startup_system(load_all_assets)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loaded_system))
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(start_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(start_background_audio))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu_window_system))
        .add_system(exit_on_esc_system)
        .run();
}

fn load_all_assets(asset_server: Res<AssetServer>, mut music_handle: ResMut<MusicHandle>) {
    music_handle.0 = Some(asset_server.load("sound/music.ogg"));
}

fn check_loaded_system(
    server: Res<AssetServer>,
    music_handle: Res<MusicHandle>,
    mut app_state: ResMut<State<AppState>>,
) {
    use bevy::asset::LoadState;

    if let Some(ref music_handle) = music_handle.0 {
        match server.get_group_load_state(vec![music_handle.id]) {
            LoadState::Failed => {
                // one of our assets had an error
            }
            LoadState::Loaded => {
                app_state.set(AppState::Menu).unwrap();
            }
            _ => {
                // NotLoaded/Loading: not fully ready yet
            }
        }
    }
}

fn start_background_audio(
    mut commands: Commands,
    audio: Res<Audio>,
    music_handle: Res<MusicHandle>,
) {
    let music_handle = music_handle.0.clone();
    audio.play(music_handle.unwrap());
    commands.remove_resource::<MusicHandle>();
}

fn start_menu(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn menu_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::Window::new("MenuWindow").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
            app_state.set(AppState::InGame).unwrap();
        }
    });
}
