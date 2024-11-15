use crate::{
    components::{AnimatedSprite, LoadingScreen, Player},
    resources::{AppState, AssetsLoading},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

pub fn load_all_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut loading_assets: ResMut<AssetsLoading>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: 1000.,
            far: -1000.,
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn(SpriteBundle {
            texture: server.load("sprites/loading.png"),
            ..default()
        })
        .insert(LoadingScreen);
    loading_assets
        .0
        .push(server.load::<AudioSource>("sound/music.ogg").untyped());
    loading_assets
        .0
        .push(server.load::<Image>("sprites/BG.png").untyped());
    loading_assets
        .0
        .push(server.load::<Image>("sprites/jocat.png").untyped());
}

pub fn check_loaded_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading_assets: Res<AssetsLoading>,
    mut next_app_state: ResMut<NextState<AppState>>,
    q: Query<Entity, With<LoadingScreen>>,
) {
    use bevy::asset::LoadState;

    if loading_assets.0.len() > 0 {
        let all_loaded = loading_assets
            .0
            .iter()
            .all(|asset| match server.load_state(asset.id()) {
                LoadState::Loaded => true,
                _ => false,
            });

        if all_loaded {
            for entity in q.iter() {
                commands.entity(entity).despawn();
            }
            next_app_state.set(AppState::Menu);
        }
    }
}

pub fn start_game(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(SpriteBundle {
        texture: server.load("sprites/BG.png"),
        ..SpriteBundle::default()
    });
    audio.play(server.load("sound/music.ogg"));

    let texture_handle = server.load("sprites/jocat.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1920, 1080), 5, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            ..default()
        },
        AnimatedSprite {
            // Timer is equivalent to 122 BPM
            timer: Timer::from_seconds(0.098360656, TimerMode::Repeating),
            frames: vec![13, 14, 15, 16, 17],
            idx: 0,
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Player,
    ));
}

pub fn start_menu(mut egui_context: EguiContexts) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn menu_window_system(
    mut egui_context: EguiContexts,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    egui::Window::new("MenuWindow").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
            next_app_state.set(AppState::InGame);
        }
    });
}
