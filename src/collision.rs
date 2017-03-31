use scene::SceneObject;
use sprites::SpriteData;
use util::{UPoint, FPoint, Angle, Dimensions};

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
    normals: Vec<Angle>,
    range: FPoint,
}

impl CollisionData {
    pub fn new<P>(sprite_data: &SpriteData, sprite_angle: Angle, sprite_scale: P) -> CollisionData
        where P: Into<FPoint>
    {

        let angle_sin = sprite_angle.as_rad().sin();
        let angle_cos = sprite_angle.as_rad().cos();
        let sprite_scale = sprite_scale.into();

        let rotated_points = sprite_data
            .convex()
            .iter()
            .map(|&p| {
                     let scaled_p = p * sprite_scale;
                     FPoint::new(scaled_p.x() * angle_cos - scaled_p.y() * angle_sin,
                                 scaled_p.x() * angle_sin + scaled_p.y() * angle_cos)
                 })
            .collect::<Vec<_>>();

        let (left, top, right, bottom) = rotated_points
            .iter()
            .fold((1.0f32, 1.0f32, 0.0f32, 0.0f32),
                  |(left, top, right, bottom), p| {
                      (left.min(p.x()), top.min(p.y()), right.max(p.x()), bottom.max(p.y()))
                  });
        let normals = CollisionData::compute_normals(&rotated_points);

        CollisionData {
            hitbox: Hitbox {
                left: -left,
                top: -top,
                right: right,
                bottom: bottom,
            },
            convex: rotated_points,
            range: FPoint::new(left.abs().max(right.abs()), top.abs().max(bottom.abs())),
            normals: normals,
        }
    }

    fn compute_normals(convex: &Vec<FPoint>) -> Vec<Angle> {
        use std::f32::consts;

        let mut result = vec![];
        if convex.is_empty() || convex.len() == 1 {
            return result;
        }

        let mut prev_point = convex[0];
        for &point in &convex[1..] {
            let delta = point - prev_point;
            result.push(Angle::from_rad(delta.y().atan2(delta.x()) - consts::FRAC_PI_2));
            prev_point = point;
        }

        let delta = convex[0] - prev_point;
        result.push(Angle::from_rad(delta.y().atan2(delta.x()) - consts::FRAC_PI_2));
        result
    }

    pub fn hitbox(&self) -> &Hitbox {
        &self.hitbox
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CollisionDetector {
    allowance: FPoint,
}

impl CollisionDetector {
    pub fn new(virtual_dimensions: Dimensions) -> CollisionDetector {
        CollisionDetector { allowance: FPoint::new(0.25, 0.25) / virtual_dimensions.as_f32() }
    }

    pub fn detect_collisions<'a, 'b, I, J, F>(&self, objects_a: I, objects_b: J, on_collision: F)
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
            for segment in SegmentsIterator::new(b) {
                segments[segment as usize].push(i);
            }
        }

        for a in objects_a {
            for segment in SegmentsIterator::new(a) {
                for i in &segments[segment as usize] {
                    let b = &mut bs[*i];
                    if !self.range_collision(a, b) {
                        continue;
                    }

                    if !self.hitbox_collision(a, b) {
                        continue;
                    }

                    if !self.convex_collision(a, b) {
                        continue;
                    }

                    on_collision(a, b);
                }
            }
        }
    }

    fn range_collision(&self, a: &SceneObject, b: &SceneObject) -> bool {
        let distance = a.pos - b.pos;
        distance.x().abs() < (a.collision_data().range.x() + b.collision_data().range.x()) &&
        distance.y().abs() < (a.collision_data().range.y() + b.collision_data().range.y())
    }

    fn hitbox_collision(&self, a: &SceneObject, b: &SceneObject) -> bool {
        let a_hitbox = a.collision_data().hitbox();
        let b_hitbox = b.collision_data().hitbox();
        a.pos.x() + a_hitbox.right > b.pos.x() - b_hitbox.left &&
        b.pos.x() + b_hitbox.right > a.pos.x() - a_hitbox.left &&
        a.pos.y() + a_hitbox.bottom > b.pos.y() - b_hitbox.top &&
        b.pos.y() + b_hitbox.bottom > a.pos.y() - a_hitbox.top
    }

    fn convex_collision(&self, a: &SceneObject, b: &SceneObject) -> bool {

        for angle in (&a.collision_data().normals)
                .into_iter()
                .chain(&b.collision_data().normals) {
            let (a_min, a_max) = a.collision_data()
                .convex
                .iter()
                .fold((3.0f32, -3.0f32), |(min_p, max_p), p| {
                    let proj = project_point(a.pos + *p, *angle);
                    (min_p.min(proj), max_p.max(proj))
                });
            let (b_min, b_max) = b.collision_data()
                .convex
                .iter()
                .fold((3.0f32, -3.0f32), |(min_p, max_p), p| {
                    let proj = project_point(b.pos + *p, *angle);
                    (min_p.min(proj), max_p.max(proj))
                });
            let projected_allowance = project_point(self.allowance, *angle);

            if a_max - projected_allowance < b_min || b_max - projected_allowance < a_min {
                return false;
            }
        }
        true
    }
}

fn project_point(p: FPoint, normal: Angle) -> f32 {
    let hypotenuse = (p.x() * p.x() + p.y() * p.y()).sqrt();
    let point_angle = p.y().atan2(p.x());
    hypotenuse * (normal.as_rad() - point_angle).sin()
}

fn segment_coords(p: FPoint) -> UPoint {
    use std::cmp::min;
    UPoint::new(min((p.x().max(0.0).min(1.0) * SEGMENTS_SIDE as f32) as u32,
                    SEGMENTS_SIDE - 1),
                min((p.y().max(0.0).min(1.0) * SEGMENTS_SIDE as f32) as u32,
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
                                                    so.pos.y() - hitbox.top));
        let to_point = segment_coords(FPoint::new(so.pos.x() + hitbox.right,
                                                  so.pos.y() + hitbox.bottom));
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
