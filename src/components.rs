use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct AnimatedSprite {
    pub timer: Timer,
    pub start_idx: usize,
    pub end_idx: usize,
}
