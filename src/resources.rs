use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
}

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<UntypedHandle>);
