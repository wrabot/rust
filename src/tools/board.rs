use crate::tools::point::{Point, DIRECTIONS_4, DIRECTIONS_8};
use itertools::Itertools;
use std::fmt;

#[derive()]
pub struct Board<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>,
}

impl<T> Board<T> {
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.cells.get(y * self.width + x)
    }

    #[allow(unused)]
    pub fn is_valid_point(&self, point: &Point) -> bool {
        self.is_valid(point.x as usize, point.y as usize)
    }

    #[allow(unused)]
    pub fn get_point(&self, point: &Point) -> Option<&T> {
        self.get(point.x as usize, point.y as usize)
    }

    pub fn neighbors(&self, point: &Point, directions: &[Point]) -> Vec<Point> {
        directions
            .iter()
            .map(|d| point + d)
            .filter(|p| self.is_valid_point(p))
            .collect_vec()
    }

    pub fn neighbors4(&self, point: &Point) -> Vec<Point> {
        self.neighbors(point, &DIRECTIONS_4)
    }

    pub fn neighbors8(&self, point: &Point) -> Vec<Point> {
        self.neighbors(point, &DIRECTIONS_8)
    }

    pub fn zone<N: Fn(&Point) -> Vec<Point>, P: Fn(&Point) -> bool>(
        &self,
        start: Point,
        neighbors: N,
        predicate: P,
    ) -> Vec<Point> {
        let mut zone = Vec::<Point>::new();
        zone.push(start);
        let mut index = 0;
        while index < zone.len() {
            for point in neighbors(&zone[index]) {
                if !zone.contains(&point) && predicate(&point) {
                    zone.push(point)
                }
            }
            index += 1;
        }
        zone
    }

    #[allow(unused)]
    pub fn zone4<P: Fn(&Point) -> bool>(&self, point: Point, predicate: P) -> Vec<Point> {
        self.zone(point, |p| self.neighbors4(p), predicate)
    }

    #[allow(unused)]
    pub fn zone8<P: Fn(&Point) -> bool>(&self, point: Point, predicate: P) -> Vec<Point> {
        self.zone(point, |p| self.neighbors8(p), predicate)
    }
}

impl<T: fmt::Display> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .chunks(self.width)
                .map(|line| line.iter().join(""))
                .join("\n")
        )
    }
}
