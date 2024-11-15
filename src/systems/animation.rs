use crate::components::AnimatedSprite;
use bevy::prelude::*;

pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimatedSprite, &mut TextureAtlas)>,
) {
    for (mut animated_sprite, mut sprite) in query.iter_mut() {
        animated_sprite.timer.tick(time.delta());
        if animated_sprite.timer.just_finished() {
            animated_sprite.idx = (animated_sprite.idx + 1) % animated_sprite.frames.len();
        }
        sprite.index = animated_sprite.frames[animated_sprite.idx];
    }
}
