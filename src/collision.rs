use scene::{SceneObject, SpriteDataCache};

pub fn detect_collisions<'a, 'b, I, J, F>(sprite_data_cache: &SpriteDataCache,
                                          objects_a: I,
                                          objects_b: J,
                                          on_collision: F)
    where I: IntoIterator<Item = &'a mut SceneObject>,
          J: IntoIterator<Item = &'b mut SceneObject>,
          F: Fn(&mut SceneObject, &mut SceneObject)
{
    let mut bs: Vec<_> = objects_b.into_iter().collect();

    for a in objects_a.into_iter() {
        for b in &mut bs {
            let a_hitbox = sprite_data_cache.get_sprite_data(&a.object_type).get_virtual_hitbox();
            let b_hitbox = sprite_data_cache.get_sprite_data(&b.object_type).get_virtual_hitbox();
            if a.pos.0 + a_hitbox.right > b.pos.0 - b_hitbox.left &&
               b.pos.0 + b_hitbox.right > a.pos.0 - a_hitbox.left &&
               a.pos.1 + a_hitbox.top > b.pos.1 - b_hitbox.bottom &&
               b.pos.1 + b_hitbox.top > a.pos.1 - a_hitbox.bottom {
                on_collision(a, *b);
            }
        }
    }
}
