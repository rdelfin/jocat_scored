use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

mod components;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    Menu,
    InGame,
}

struct AssetsLoading(Vec<HandleUntyped>);

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
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_all_assets))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loaded_system))
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(start_menu))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(start_game))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu_window_system))
        .add_system(exit_on_esc_system)
        .run();
}

fn load_all_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: server.load("sprites/loading.png"),
            ..default()
        })
        .insert(components::LoadingScreen);
    loading_assets.0.push(
        server
            .load::<AudioSource, _>("sound/music.ogg")
            .clone_untyped(),
    );
    loading_assets
        .0
        .push(server.load::<Image, _>("sprites/BG.png").clone_untyped());
}

fn check_loaded_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading_assets: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
    q: Query<Entity, With<components::LoadingScreen>>,
) {
    use bevy::asset::LoadState;

    if loading_assets.0.len() > 0 {
        let all_loaded = loading_assets.0.iter().all(|asset| {
            match server.get_group_load_state(vec![asset.id]) {
                LoadState::Loaded => true,
                _ => false,
            }
        });

        if all_loaded {
            for entity in q.iter() {
                commands.entity(entity).despawn();
            }
            app_state.set(AppState::Menu).unwrap();
        }
    }
}

fn start_game(mut commands: Commands, server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("sprites/BG.png"),
        ..SpriteBundle::default()
    });
    audio.play(server.load("sound/music.ogg"));
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
