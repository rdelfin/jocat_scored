use crate::{
    components::LoadingScreen,
    resources::{AppState, AssetsLoading},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_kira_audio::{Audio, AudioSource};

pub fn load_all_assets(
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
        .insert(LoadingScreen);
    loading_assets.0.push(
        server
            .load::<AudioSource, _>("sound/music.ogg")
            .clone_untyped(),
    );
    loading_assets
        .0
        .push(server.load::<Image, _>("sprites/BG.png").clone_untyped());
}

pub fn check_loaded_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading_assets: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
    q: Query<Entity, With<LoadingScreen>>,
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

pub fn start_game(mut commands: Commands, server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("sprites/BG.png"),
        ..SpriteBundle::default()
    });
    audio.play(server.load("sound/music.ogg"));
}

pub fn start_menu(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn menu_window_system(
    mut egui_context: ResMut<EguiContext>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::Window::new("MenuWindow").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
            app_state.set(AppState::InGame).unwrap();
        }
    });
}
