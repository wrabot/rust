use itertools::Itertools;
use std::time::Instant;

use crate::tools::board::Board;
use crate::tools::graph::short_path;
use crate::tools::point::points;

pub fn part1(input: &str, expected: u32) {
    let start = Instant::now();
    let lines = input.lines().collect_vec();
    let board = Board {
        width: lines[0].len(),
        height: lines.len(),
        cells: lines
            .iter()
            .flat_map(|&line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect_vec(),
    };
    let result = min_risk(board);
    println!("year2021 day15 part1 duration {:?}", start.elapsed());
    assert_eq!(result, expected)
}

pub fn part2(input: &str, expected: u32) {
    let start = Instant::now();
    let lines = input.lines().collect_vec();
    let board = Board {
        width: lines[0].len() * 5,
        height: lines.len() * 5,
        cells: (0..5)
            .flat_map(|row| {
                lines.iter().flat_map(move |line| {
                    (0..5).flat_map(move |column| {
                        line.chars()
                            .map(move |c| (c.to_digit(10).unwrap() + column + row - 1) % 9 + 1)
                    })
                })
            })
            .collect_vec(),
    };
    let result = min_risk(board);
    println!("year2021 day15 part2 duration {:?}", start.elapsed());
    assert_eq!(result, expected)
}

fn min_risk(cave: Board<u32>) -> u32 {
    let points = points(cave.width, cave.height);
    let neighbors = (0..points.len())
        .map(|id| {
            cave.neighbors4(&points[id])
                .iter()
                .map(|n| {
                    (
                        (n.y * cave.width as i32 + n.x) as usize,
                        *cave.get_point(n).unwrap() as i32,
                    )
                })
                .collect_vec()
        })
        .collect_vec();
    short_path(0, points.len() - 1, |id| &neighbors[id])
        .unwrap()
        .iter()
        .dropping(1)
        .map(|id| cave.get_point(&points[*id]).unwrap())
        .sum()
}
