use image;

pub type Point = (i32, i32);

pub fn calculate_convex(image_buffer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
                        offset: (u32, u32),
                        size: (u32, u32))
                        -> Vec<Point> {
    use std::u8;

    let mut result = vec![];
    let offset_x = offset.0 as i32;
    let offset_y = offset.1 as i32;
    let width = size.0 as i32;
    let height = size.1 as i32;
    let mut convex_point = (0, 0);
    let mut top_right_found = false;

    'top_right_loop: for y in 0..height {
        for x in (0..width).rev() {
            if image_buffer.get_pixel((offset_x + x) as u32, (offset_y + y) as u32)[3] != u8::MIN {
                convex_point = (x, y);
                top_right_found = true;
                break 'top_right_loop;
            }
        }
    }
    if !top_right_found {
        return result;
    }

    for boundary_point in BoundaryIterator::new(width, height, (width - 1, convex_point.1)) {
        for point in LineIterator::new(boundary_point, convex_point) {
            if image_buffer.get_pixel((offset_x + point.0) as u32,
                           (offset_y + point.1) as u32)[3] !=
               u8::MIN {
                result.push(convex_point);
                convex_point = point;
                break;
            }
        }
    }

    result.push(convex_point);
    return result;
}

struct BoundaryIterator {
    from_point: Point,
    cur_point: Point,
    max_x: i32,
    max_y: i32,
}

impl BoundaryIterator {
    pub fn new(width: i32, height: i32, from_point: Point) -> BoundaryIterator {
        BoundaryIterator {
            from_point: from_point,
            cur_point: from_point,
            max_x: width - 1,
            max_y: height - 1,
        }
    }
}

impl Iterator for BoundaryIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = match (self.cur_point.0, self.cur_point.1) {
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

        self.cur_point.0 += delta.0;
        self.cur_point.1 += delta.1;

        if self.cur_point == self.from_point {
            None
        } else {
            Some(self.cur_point)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineIterator {
    from: Point,
    to: Point,
    cur: Point,
    main_step: Point,
    secondary_step: Point,
    max_delta: i32,
    min_delta: i32,
    steps_taken: i32,
    secondary_steps_taken: i32,
}

impl LineIterator {
    pub fn new(from: Point, to: Point) -> LineIterator {
        use std::cmp::{max, min};
        let delta = (to.0 - from.0, to.1 - from.1);
        let mut main_step = (0, 0);
        let mut secondary_step = (0, 0);

        match delta {
            (0, 0) => {}
            (0, y_delta) => {
                main_step = (0, y_delta.signum());
            }
            (x_delta, 0) => {
                main_step = (x_delta.signum(), 0);
            }
            (x_delta, y_delta) => {
                if x_delta.abs() >= y_delta.abs() {
                    main_step = (x_delta.signum(), 0);
                    secondary_step = (0, y_delta.signum());
                } else {
                    main_step = (0, y_delta.signum());
                    secondary_step = (x_delta.signum(), 0);
                }
            }
        }

        LineIterator {
            from: from,
            to: to,
            cur: from,
            main_step: main_step,
            secondary_step: secondary_step,
            max_delta: max(delta.0.abs(), delta.1.abs()),
            min_delta: min(delta.0.abs(), delta.1.abs()),
            steps_taken: 1,
            secondary_steps_taken: 0,
        }
    }

    fn advance(&mut self) {
        self.cur.0 += self.main_step.0;
        self.cur.1 += self.main_step.1;
        self.steps_taken += 1;

        if self.secondary_step == (0, 0) {
            return;
        }

        if self.steps_taken * (self.min_delta + 1) / (1 + self.secondary_steps_taken) >
           self.max_delta + 1 {
            self.cur.0 += self.secondary_step.0;
            self.cur.1 += self.secondary_step.1;
            self.secondary_steps_taken += 1;
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.cur == self.to {
            None
        } else {
            Some(self.cur)
        };

        self.advance();

        return result;
    }
}

#[cfg(test)]
mod test {
    use ::sprites::convex;

    #[test]
    fn test_boundary_iterator() {
        let bi = convex::BoundaryIterator::new(3, 3, (0, 0));
        assert!(bi.eq([(1i32, 0i32), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1)]
            .into_iter()
            .map(|&v| v)));
    }

    #[test]
    fn line_test() {
        let li = convex::LineIterator::new((1, 0), (6, 3));
        assert!(li.eq([(1i32, 0i32), (2, 1), (3, 1), (4, 2), (5, 2)]
            .into_iter()
            .map(|&v| v)));
        let li = convex::LineIterator::new((1, 1), (6, 3));
        assert!(li.eq([(1i32, 1i32), (2, 1), (3, 2), (4, 2), (5, 3)]
            .into_iter()
            .map(|&v| v)));
        let li = convex::LineIterator::new((1, 2), (6, 3));
        assert!(li.eq([(1i32, 2i32), (2, 2), (3, 2), (4, 3), (5, 3)]
            .into_iter()
            .map(|&v| v)));
        let li = convex::LineIterator::new((1, 3), (6, 3));
        assert!(li.eq([(1i32, 3i32), (2, 3), (3, 3), (4, 3), (5, 3)]
            .into_iter()
            .map(|&v| v)));
        let li = convex::LineIterator::new((6, 3), (6, 0));
        assert!(li.eq([(6i32, 3i32), (6, 2), (6, 1)]
            .into_iter()
            .map(|&v| v)));
    }
}
