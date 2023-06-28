#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[allow(unused)]
pub fn points(width: usize, height: usize) -> Vec<Point> {
    let mut points = Vec::with_capacity(width * height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            points.push(Point { x, y })
        }
    }
    points
}

#[allow(unused)]
pub const DIRECTIONS_4: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
];

#[allow(unused)]
pub const DIRECTIONS_DIAGONALS: [Point; 4] = [
    Point { x: 1, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: -1 },
];

#[allow(unused)]
pub const DIRECTIONS_8: [Point; 8] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: -1 },
];

impl std::ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<i32> for &Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
