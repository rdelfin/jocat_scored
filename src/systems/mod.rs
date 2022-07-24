mod animation;
mod setup;

pub use self::{
    animation::animate_sprite_system,
    setup::{check_loaded_system, load_all_assets, menu_window_system, start_game, start_menu},
};
