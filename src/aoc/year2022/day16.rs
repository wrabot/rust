use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

use crate::tools::graph::distances;

pub fn part1(input: &str) -> i32 {
    let data = Data::parse(input);
    let all_distances = all_distances(&data);
    max(&data, &all_distances, &data.targets, data.start, 0, 0, 30)
}

pub fn part2(input: &str) -> i32 {
    let data = Data::parse(input);
    let all_distances = all_distances(&data);
    let mut result = 0;
    let mut mine = Vec::with_capacity(data.targets.len());
    let mut elephant = Vec::with_capacity(data.targets.len());
    for i in 0..(1 << (data.targets.len() - 1)) {
        mine.clear();
        elephant.clear();
        for index in 0..data.targets.len() {
            if (i & (1 << index)) == 0 {
                mine.push(data.targets[index])
            } else {
                elephant.push(data.targets[index])
            }
        }
        if mine.len() < data.targets.len() / 2 || elephant.len() < data.targets.len() / 2 {
            continue;
        }
        let max = max(&data, &all_distances, &mine, data.start, 0, 0, 26)
            + max(&data, &all_distances, &elephant, data.start, 0, 0, 26);
        if max > result {
            result = max
        }
    }
    result
}

fn all_distances(data: &Data) -> HashMap<usize, HashMap<usize, i32>> {
    let mut all_distances = HashMap::<usize, HashMap<usize, i32>>::new();
    all_distances.insert(data.start, HashMap::<usize, i32>::new());
    distances(
        data.start,
        |id| &data.neighbors[id],
        all_distances.get_mut(&data.start).unwrap(),
    );
    for target in &data.targets {
        all_distances.insert(*target, HashMap::<usize, i32>::new());
        distances(
            *target,
            |id| &data.neighbors[id],
            all_distances.get_mut(&target).unwrap(),
        );
    }
    all_distances
}

fn max(
    data: &Data,
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
                &data,
                all_distances,
                &r,
                *id,
                release + data.valves[*id].rate,
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
    fn parse(input: &str) -> Self {
        let regex =
            Regex::new("Valve (\\w*) has flow rate=(\\d*); tunnels? leads? to valves? (.*)")
                .unwrap();
        let valves = regex
            .captures_iter(input)
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
