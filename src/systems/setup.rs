use crate::{
    components::{AnimatedSprite, LoadingScreen, Player},
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
    loading_assets
        .0
        .push(server.load::<Image, _>("sprites/jocat.png").clone_untyped());
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

pub fn start_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: server.load("sprites/BG.png"),
        ..SpriteBundle::default()
    });
    audio.play(server.load("sound/music.ogg"));

    let texture_handle = server.load("sprites/jocat.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1920., 1080.), 5, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(AnimatedSprite {
            // Timer is equivalent to 122 BPM
            timer: Timer::from_seconds(0.098360656, true),
            frames: vec![13, 14, 15, 16, 17],
            idx: 0,
        })
        .insert(Player);
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
