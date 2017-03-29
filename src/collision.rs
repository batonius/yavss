use scene::SceneObject;
use sprites::SpriteData;
use util::{UPoint, FPoint, Angle};

const SEGMENTS_SIDE: u32 = 10;
const SEGMENTS_COUNT: u32 = SEGMENTS_SIDE * SEGMENTS_SIDE;

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
    range: FPoint,
}

impl CollisionData {
    pub fn new(sprite_data: &SpriteData, sprite_angle: Angle) -> CollisionData {

        let angle_sin = sprite_angle.as_rad().sin();
        let angle_cos = sprite_angle.as_rad().cos();

        let rotated_points = sprite_data
            .convex()
            .iter()
            .map(|p| {
                     FPoint::new(p.x() * angle_cos - p.y() * angle_sin,
                                 p.x() * angle_sin + p.y() * angle_cos)
                 })
            .collect::<Vec<_>>();

        let (left, top, right, bottom) = rotated_points
            .iter()
            .fold((1.0f32, 0.0f32, 0.0f32, 1.0f32),
                  |(left, top, right, bottom), p| {
                      (left.min(p.x()), top.max(p.y()), right.max(p.x()), bottom.min(p.y()))
                  });

        CollisionData {
            hitbox: Hitbox {
                left: -left,
                top: top,
                right: right,
                bottom: -bottom,
            },
            convex: rotated_points,
            range: FPoint::new(left.abs().max(right.abs()), top.abs().max(bottom.abs())),
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
    let mut segments: Vec<Vec<usize>> = Vec::with_capacity(SEGMENTS_COUNT as usize);

    for _ in 0..SEGMENTS_COUNT {
        segments.push(vec![]);
    }

    let mut bs: Vec<_> = objects_b.into_iter().collect();

    for (i, b) in bs.iter().enumerate() {
        for segment in SegmentsIterator::new(&b) {
            segments[segment as usize].push(i);
        }
    }

    for a in objects_a.into_iter() {
        for segment in SegmentsIterator::new(&a) {
            for i in &segments[segment as usize] {
                let b = &mut bs[*i];
                if !range_collision(&a, &b) {
                    continue;
                }

                if !hitbox_collision(&a, &b) {
                    continue;
                }

                if !convex_collision(&a, &b) {
                    continue;
                }

                on_collision(a, b);
            }
        }
    }
}

fn range_collision(a: &SceneObject, b: &SceneObject) -> bool {
    let distance = a.pos - b.pos;
    distance.x().abs() < (a.collision_data().range.x() + b.collision_data().range.x()) &&
    distance.y().abs() < (a.collision_data().range.y() + b.collision_data().range.y())
}

fn hitbox_collision(a: &SceneObject, b: &SceneObject) -> bool {
    let a_hitbox = a.collision_data().hitbox();
    let b_hitbox = b.collision_data().hitbox();
    a.pos.x() + a_hitbox.right > b.pos.x() - b_hitbox.left &&
    b.pos.x() + b_hitbox.right > a.pos.x() - a_hitbox.left &&
    a.pos.y() + a_hitbox.top > b.pos.y() - b_hitbox.bottom &&
    b.pos.y() + b_hitbox.top > a.pos.y() - a_hitbox.bottom
}

fn convex_collision(_: &SceneObject, _: &SceneObject) -> bool {
    true
}

fn segment_coords(p: FPoint) -> UPoint {
    use std::cmp::min;
    UPoint::new(min((p.x().max(0.0).min(1.0) * SEGMENTS_SIDE as f32) as u32,
                    SEGMENTS_SIDE - 1),
                min(((1.0 - p.y()).max(0.0).min(1.0) * SEGMENTS_SIDE as f32) as u32,
                    SEGMENTS_SIDE - 1))
}

fn segment_no(p: UPoint) -> u32 {
    p.y() * SEGMENTS_SIDE + p.x()
}

struct SegmentsIterator {
    from_point: UPoint,
    to_point: UPoint,
    cur_point: UPoint,
}

impl SegmentsIterator {
    pub fn new(so: &SceneObject) -> SegmentsIterator {
        let hitbox = so.collision_data().hitbox();
        let from_point = segment_coords(FPoint::new(so.pos.x() - hitbox.left,
                                                    so.pos.y() + hitbox.top));
        let to_point = segment_coords(FPoint::new(so.pos.x() + hitbox.right,
                                                  so.pos.y() - hitbox.bottom));
        SegmentsIterator {
            from_point: from_point,
            cur_point: from_point,
            to_point: to_point,
        }
    }
}

impl Iterator for SegmentsIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_point.x() > self.to_point.x() {
            return None;
        }

        let result = self.cur_point;

        *self.cur_point.mut_x() += 1;

        if self.cur_point.x() > self.to_point.x() && self.cur_point.y() < self.to_point.y() {
            *self.cur_point.mut_y() += 1;
            *self.cur_point.mut_x() = self.from_point.x();
        }

        Some(segment_no(result))
    }
}
