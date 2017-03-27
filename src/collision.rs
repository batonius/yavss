use scene::SceneObject;
use sprites::SpriteData;
use util::{FPoint, Angle};

#[derive(Debug, Copy, Clone)]
pub struct Hitbox {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug, Clone)]
pub struct CollisionData {
    hitbox: Hitbox,
    convex: Vec<FPoint>,
}

impl CollisionData {
    pub fn new(sprite_data: &SpriteData, sprite_angle: Angle) -> CollisionData {

        let angle_sin = sprite_angle.as_rad().sin();
        let angle_cos = sprite_angle.as_rad().cos();

        let rotated_points = sprite_data.convex()
            .iter()
            .map(|p| {
                FPoint::new(p.x() * angle_cos - p.y() * angle_sin,
                            p.x() * angle_sin + p.y() * angle_cos)
            })
            .collect::<Vec<_>>();

        let (left, top, right, bottom) = rotated_points.iter()
            .fold((1.0f32, 0.0f32, 0.0f32, 1.0f32),
                  |(left, top, right, bottom), p| {
                      (left.min(p.x()), top.max(p.y()), right.max(p.x()), bottom.min(p.y()))
                  });

        CollisionData {
            hitbox: Hitbox {
                left: left.abs(),
                top: top.abs(),
                right: right.abs(),
                bottom: bottom.abs(),
            },
            convex: rotated_points,
        }
    }

    pub fn hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

pub fn detect_collisions<'a, 'b, I, J, F>(objects_a: I, objects_b: J, on_collision: F)
    where I: IntoIterator<Item = &'a mut SceneObject>,
          J: IntoIterator<Item = &'b mut SceneObject>,
          F: Fn(&mut SceneObject, &mut SceneObject)
{
    let mut bs: Vec<_> = objects_b.into_iter().collect();

    for a in objects_a.into_iter() {
        for b in &mut bs {
            let mut collision = false;
            {
                let a_hitbox = a.collision_data().hitbox();
                let b_hitbox = b.collision_data().hitbox();
                if a.pos.x() + a_hitbox.right > b.pos.x() - b_hitbox.left &&
                   b.pos.x() + b_hitbox.right > a.pos.x() - a_hitbox.left &&
                   a.pos.y() + a_hitbox.top > b.pos.y() - b_hitbox.bottom &&
                   b.pos.y() + b_hitbox.top > a.pos.y() - a_hitbox.bottom {
                    collision = true;
                }
            }
            if collision {
                on_collision(a, *b);
            }
        }
    }
}
