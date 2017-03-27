use std::f32;
use std::ops::{Add, Div, Sub, AddAssign};
use std::convert::From;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point<T> {
    a: T,
    b: T,
}

impl<T> Point<T>
    where T: Copy
{
    pub fn new(a: T, b: T) -> Point<T> {
        Point { a: a, b: b }
    }

    pub fn x(&self) -> T {
        self.a
    }

    pub fn width(&self) -> T {
        self.a
    }

    pub fn mut_x(&mut self) -> &mut T {
        &mut self.a
    }

    #[allow(dead_code)]
    pub fn mut_width(&mut self) -> &mut T {
        &mut self.a
    }

    pub fn y(&self) -> T {
        self.b
    }

    pub fn height(&self) -> T {
        self.b
    }

    pub fn mut_y(&mut self) -> &mut T {
        &mut self.b
    }

    #[allow(dead_code)]
    pub fn mut_height(&mut self) -> &mut T {
        &mut self.b
    }
}

impl Point<u32> {
    pub fn as_f32(&self) -> Point<f32> {
        Point {
            a: self.a as f32,
            b: self.b as f32,
        }
    }

    #[allow(dead_code)]
    pub fn as_i32(&self) -> Point<i32> {
        Point {
            a: self.a as i32,
            b: self.b as i32,
        }
    }
}

impl Point<i32> {
    #[allow(dead_code)]
    pub fn as_f32(&self) -> Point<f32> {
        Point {
            a: self.a as f32,
            b: self.b as f32,
        }
    }

    #[allow(dead_code)]
    pub fn as_u32(&self) -> Point<u32> {
        Point {
            a: self.a as u32,
            b: self.b as u32,
        }
    }
}

impl<T> Add for Point<T>
    where T: Add
{
    type Output = Point<<T as Add<T>>::Output>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<T> AddAssign for Point<T>
    where T: AddAssign
{
    fn add_assign(&mut self, rhs: Point<T>) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}

impl<T> Sub for Point<T>
    where T: Sub
{
    type Output = Point<<T as Sub<T>>::Output>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl<T> Div for Point<T>
    where T: Div
{
    type Output = Point<<T as Div<T>>::Output>;

    fn div(self, rhs: Point<T>) -> Self::Output {
        Point {
            a: self.a / rhs.a,
            b: self.b / rhs.b,
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((a, b): (T, T)) -> Point<T> {
        Point { a: a, b: b }
    }
}

impl<T> Into<(T, T)> for Point<T> {
    fn into(self) -> (T, T) {
        (self.a, self.b)
    }
}

impl<T> Into<[T; 2]> for Point<T> {
    fn into(self) -> [T; 2] {
        [self.a, self.b]
    }
}

pub type Dimensions = Point<u32>;
pub type FDimensions = Point<f32>;
pub type UPoint = Point<u32>;
pub type IPoint = Point<i32>;
pub type FPoint = Point<f32>;

#[derive(Clone, Copy, Debug)]
pub struct Angle {
    rad: f32,
}

impl Angle {
    #[allow(dead_code)]
    pub fn from_rad(r: f32) -> Angle {
        Angle { rad: r }
    }

    pub fn from_deg(d: f32) -> Angle {
        Angle { rad: d * f32::consts::PI / 180.0 }
    }

    pub fn as_rad(&self) -> f32 {
        self.rad
    }

    pub fn as_deg(&self) -> f32 {
        (self.rad * 180.0 / f32::consts::PI) % 360.0
    }

    pub fn add_deg(&self, deg: f32) -> Angle {
        Angle::from_deg(self.as_deg() + deg)
    }
}
