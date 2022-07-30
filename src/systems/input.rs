use crate::components::{AnimatedSprite, Player};
use bevy::prelude::*;

pub fn attack_system(
    input: Res<Input<KeyCode>>,
    mut q: Query<(&mut AnimatedSprite, &mut TextureAtlasSprite), With<Player>>,
) {
    if input.pressed(KeyCode::Space) {
        for (mut animated_sprite, mut texture_atlas_sprite) in q.iter_mut() {
            animated_sprite.timer.reset();
            animated_sprite.frames = vec![7, 6, 5, 4, 3, 2, 1, 0];
            texture_atlas_sprite.index = 0;
        }
    }
}
