use image;
use util::{IPoint, Dimensions};

pub fn calculate_convex<D1, D2>(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                                offset: D1,
                                size: D2)
                                -> Vec<(IPoint, IPoint)>
    where D1: Into<Dimensions>,
          D2: Into<Dimensions>
{
    use std::u8;

    let offset = offset.into();
    let size = size.into();

    let mut result = vec![];
    let offset_x = offset.x() as i32;
    let offset_y = offset.y() as i32;
    let width = size.x() as i32;
    let height = size.y() as i32;
    let mut convex_point = IPoint::new(0, 0);
    let mut convex_direction = IPoint::new(0, 0);
    let mut top_right_found = false;

    'top_right_loop: for y in 0..height {
        for x in (0..width).rev() {
            if image_buffer.get_pixel((offset_x + x) as u32, (offset_y + y) as u32)[3] != u8::MIN {
                convex_point = IPoint::new(x, y);
                convex_direction = IPoint::new(1, if y > height / 2 { 1 } else { 0 });
                top_right_found = true;
                break 'top_right_loop;
            }
        }
    }
    if !top_right_found {
        return result;
    }

    let start_convex = convex_point;
    let mut prev_convex = convex_point;
    let mut prev_direction = convex_direction;

    for boundary_point in BoundaryIterator::new(width, height, (width - 1, convex_point.y())) {
        for point in LineIterator::new(boundary_point, convex_point) {
            if image_buffer.get_pixel((offset_x + point.x()) as u32,
                                      (offset_y + point.y()) as u32)
                   [3] != u8::MIN {
                if prev_convex != convex_point &&
                   !is_points_on_line(prev_convex, convex_point, point) {
                    result.push((prev_convex, prev_direction));
                    prev_convex = convex_point;
                    prev_direction = convex_direction;
                }
                convex_point = point;
                convex_direction = IPoint::new(if boundary_point.x() > width / 2 { 1 } else { 0 },
                                               if boundary_point.y() > height / 2 {
                                                   1
                                               } else {
                                                   0
                                               });
                break;
            }
        }
    }

    result.push((prev_convex, prev_direction));
    if !is_points_on_line(prev_convex, convex_point, start_convex) {
        result.push((convex_point, convex_direction));
    }

    result
}

fn is_points_on_line<P1, P2, P3>(a: P1, b: P2, c: P3) -> bool
    where P1: Into<IPoint>,
          P2: Into<IPoint>,
          P3: Into<IPoint>
{
    let a = a.into();
    let b = b.into();
    let c = c.into();

    if a.x() == b.x() {
        b.x() == c.x()
    } else if a.y() == b.y() {
        b.y() == c.y()
    } else {
        let (ab_x_delta, ab_y_delta) = (b - a).into();
        let (bc_x_delta, bc_y_delta) = (c - b).into();

        ab_x_delta.abs() == ab_y_delta.abs() && bc_x_delta.abs() == bc_y_delta.abs() &&
        ab_x_delta.signum() == bc_x_delta.signum() &&
        ab_y_delta.signum() == bc_y_delta.signum()
    }
}

struct BoundaryIterator {
    from_point: IPoint,
    cur_point: IPoint,
    max_x: i32,
    max_y: i32,
}

impl BoundaryIterator {
    pub fn new<P>(width: i32, height: i32, from_point: P) -> BoundaryIterator
        where P: Into<IPoint>
    {
        let from_point = from_point.into();
        BoundaryIterator {
            from_point: from_point,
            cur_point: from_point,
            max_x: width - 1,
            max_y: height - 1,
        }
    }
}

impl Iterator for BoundaryIterator {
    type Item = IPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = match self.cur_point.into() {
            (0, 0) => (1, 0),
            (0, h) if h == self.max_y => (0, -1),
            (w, 0) if w == self.max_x => (0, 1),
            (w, h) if w == self.max_x && h == self.max_y => (-1, 0),
            (0, _) => (0, -1),
            (_, 0) => (1, 0),
            (w, _) if w == self.max_x => (0, 1),
            (_, h) if h == self.max_y => (-1, 0),
            (_, _) => (0, 0),
        };

        self.cur_point += IPoint::from(delta);

        if self.cur_point == self.from_point {
            None
        } else {
            Some(self.cur_point)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineIterator {
    from: IPoint,
    to: IPoint,
    cur: IPoint,
    main_step: IPoint,
    secondary_step: IPoint,
    max_delta: i32,
    min_delta: i32,
    steps_taken: i32,
    secondary_steps_taken: i32,
}

impl LineIterator {
    pub fn new<P1, P2>(from: P1, to: P2) -> LineIterator
        where P1: Into<IPoint>,
              P2: Into<IPoint>
    {
        use std::cmp::{max, min};
        let from = from.into();
        let to = to.into();
        let delta = to - from;

        let mut main_step = IPoint::new(0, 0);
        let mut secondary_step = IPoint::new(0, 0);

        match delta.into() {
            (0, 0) => {}
            (0, y_delta) => {
                main_step = IPoint::new(0, y_delta.signum());
            }
            (x_delta, 0) => {
                main_step = IPoint::new(x_delta.signum(), 0);
            }
            (x_delta, y_delta) => {
                if x_delta.abs() >= y_delta.abs() {
                    main_step = IPoint::new(x_delta.signum(), 0);
                    secondary_step = IPoint::new(0, y_delta.signum());
                } else {
                    main_step = IPoint::new(0, y_delta.signum());
                    secondary_step = IPoint::new(x_delta.signum(), 0);
                }
            }
        }

        LineIterator {
            from: from,
            to: to,
            cur: from,
            main_step: main_step,
            secondary_step: secondary_step,
            max_delta: max(delta.x().abs(), delta.y().abs()),
            min_delta: min(delta.x().abs(), delta.y().abs()),
            steps_taken: 1,
            secondary_steps_taken: 0,
        }
    }

    fn advance(&mut self) {
        self.cur += self.main_step;

        self.steps_taken += 1;

        if self.secondary_step == IPoint::new(0, 0) {
            return;
        }

        if self.steps_taken * (self.min_delta + 1) / (1 + self.secondary_steps_taken) >
           self.max_delta + 1 {
            self.cur += self.secondary_step;
            self.secondary_steps_taken += 1;
        }
    }
}

impl Iterator for LineIterator {
    type Item = IPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.cur == self.to {
            None
        } else {
            Some(self.cur)
        };

        self.advance();

        result
    }
}
