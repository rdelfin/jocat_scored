use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
}

pub struct AssetsLoading(pub Vec<HandleUntyped>);
