use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct LoadingScreen;

#[derive(Component, Debug, Clone)]
pub struct AnimatedSprite {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub idx: usize,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Player;
