use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;
use std::time::Instant;

use crate::tools::graph::distances;

pub fn part1() {
    let all_distances = all_distances();
    let start = Instant::now();
    let result = max(&all_distances, &DATA.targets, DATA.start, 0, 0, 30);
    println!("year2022 day16 part1 duration {:?}", start.elapsed());
    assert_eq!(result, 2077)
}

pub fn part2() {
    let all_distances = all_distances();
    let start = Instant::now();
    let mut result = 0;
    let mut mine = Vec::with_capacity(DATA.targets.len());
    let mut elephant = Vec::with_capacity(DATA.targets.len());
    for i in 0..(1 << (DATA.targets.len() - 1)) {
        mine.clear();
        elephant.clear();
        for index in 0..DATA.targets.len() {
            if (i & (1 << index)) == 0 {
                mine.push(DATA.targets[index])
            } else {
                elephant.push(DATA.targets[index])
            }
        }
        if mine.len() < DATA.targets.len() / 2 || elephant.len() < DATA.targets.len() / 2 {
            continue;
        }
        let max = max(&all_distances, &mine, DATA.start, 0, 0, 26)
            + max(&all_distances, &elephant, DATA.start, 0, 0, 26);
        if max > result {
            result = max
        }
    }
    println!("year2022 day16 part2 duration {:?}", start.elapsed());
    assert_eq!(result, 2741)
}

fn all_distances() -> HashMap<usize, HashMap<usize, i32>> {
    let mut all_distances = HashMap::<usize, HashMap<usize, i32>>::new();
    all_distances.insert(DATA.start, HashMap::<usize, i32>::new());
    distances(
        DATA.start,
        |id| &DATA.neighbors[id],
        all_distances.get_mut(&DATA.start).unwrap(),
    );
    for target in &DATA.targets {
        all_distances.insert(*target, HashMap::<usize, i32>::new());
        distances(
            *target,
            |id| &DATA.neighbors[id],
            all_distances.get_mut(&target).unwrap(),
        );
    }
    all_distances
}

fn max(
    all_distances: &HashMap<usize, HashMap<usize, i32>>,
    remaining: &Vec<usize>,
    current: usize,
    release: i32,
    total: i32,
    time: i32,
) -> i32 {
    let mut result = total + release * time;
    for id in remaining {
        let delay = all_distances[&current][id] + 1;
        if delay < time {
            let mut r = remaining.clone();
            r.remove(r.iter().position(|&x| x == *id).unwrap());
            let max = max(
                all_distances,
                &r,
                *id,
                release + DATA.valves[*id].rate,
                total + release * delay,
                time - delay,
            );
            if max > result {
                result = max
            }
        }
    }
    result
}

// init

lazy_static! {
    static ref DATA: Data = Data::new();
}

#[derive(Debug)]
struct Valve {
    id: usize,
    name: String,
    rate: i32,
    neighbors: Vec<String>,
}

#[derive(Debug)]
struct Data {
    valves: Vec<Valve>,
    start: usize,
    neighbors: Vec<Vec<(usize, i32)>>,
    targets: Vec<usize>,
}

impl Data {
    fn new() -> Self {
        let regex =
            Regex::new("Valve (\\w*) has flow rate=(\\d*); tunnels? leads? to valves? (.*)")
                .unwrap();
        let valves = regex
            .captures_iter(include_str!("day16_input.txt"))
            .enumerate()
            .map(|(id, captures)| Valve {
                id,
                name: captures.index(1).to_string(),
                rate: captures.index(2).parse().unwrap(),
                neighbors: captures
                    .index(3)
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect_vec(),
            })
            .collect_vec();
        let start = valves.iter().position(|v| v.name == "AA").unwrap();
        let neighbors = valves
            .iter()
            .map(|v| {
                v.neighbors
                    .iter()
                    .map(|n| (valves.iter().position(|v| &v.name == n).unwrap(), 1))
                    .collect_vec()
            })
            .collect_vec();
        let targets = valves
            .iter()
            .filter(|v| v.rate > 0)
            .map(|v| v.id)
            .collect_vec();
        Self {
            valves,
            start,
            neighbors,
            targets,
        }
    }
}
