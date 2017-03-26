use scene::{SceneObject, SpriteDataCache};
use sprites::Hitbox;

struct CollisionData<'a> {
    scene_object: &'a mut SceneObject,
    adjusted_hitbox: Hitbox,
}

impl<'a> CollisionData<'a> {
    pub fn new(sprite_data_cache: &SpriteDataCache,
               scene_object: &'a mut SceneObject)
               -> CollisionData<'a> {
        let hibox = CollisionData::compute_adjusted_hitbox(sprite_data_cache, scene_object);
        CollisionData {
            scene_object: scene_object,
            adjusted_hitbox: hibox,
        }
    }

    fn compute_adjusted_hitbox(sprite_data_cache: &SpriteDataCache,
                               scene_object: &'a SceneObject)
                               -> Hitbox {
        Hitbox {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

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
            let a_hitbox = sprite_data_cache.sprite_data(&a.object_type).virtual_hitbox();
            let b_hitbox = sprite_data_cache.sprite_data(&b.object_type).virtual_hitbox();
            if a.pos.x() + a_hitbox.right > b.pos.x() - b_hitbox.left &&
               b.pos.x() + b_hitbox.right > a.pos.x() - a_hitbox.left &&
               a.pos.y() + a_hitbox.top > b.pos.y() - b_hitbox.bottom &&
               b.pos.y() + b_hitbox.top > a.pos.y() - a_hitbox.bottom {
                on_collision(a, *b);
            }
        }
    }
}
