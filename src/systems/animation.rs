use crate::components::AnimatedSprite;
use bevy::prelude::*;

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimatedSprite,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animated_sprite, mut sprite, texture_atlas_handle) in query.iter_mut() {
        animated_sprite.timer.tick(time.delta());
        if animated_sprite.timer.just_finished() {
            let texture_atlas = texture_atlases
                .get(texture_atlas_handle)
                .expect("animated sprites must have an atlas handle");
            sprite.index += 1;
            if sprite.index >= animated_sprite.end_idx
                || sprite.index >= texture_atlas.textures.len()
            {
                sprite.index = animated_sprite.start_idx;
            }
        }
    }
}
