use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

use crate::tools::graph::distances;

pub fn part1(input: &str) -> i32 {
    let data = Data::parse(input);
    max(&data, &data.targets, data.start, 0, 0, 30)
}

pub fn part2(input: &str) -> i32 {
    let data = Data::parse(input);
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
        let max =
            max(&data, &mine, data.start, 0, 0, 26) + max(&data, &elephant, data.start, 0, 0, 26);
        if max > result {
            result = max
        }
    }
    result
}

fn max(
    data: &Data,
    remaining: &Vec<usize>,
    current: usize,
    release: i32,
    total: i32,
    time: i32,
) -> i32 {
    let mut result = total + release * time;
    for id in remaining {
        let delay = data.all_distances[&current][id] + 1;
        if delay < time {
            let mut r = remaining.clone();
            r.remove(r.iter().position(|&x| x == *id).unwrap());
            let max = max(
                &data,
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
    targets: Vec<usize>,
    all_distances: HashMap<usize, HashMap<usize, i32>>,
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

        let targets = valves
            .iter()
            .filter(|v| v.rate > 0)
            .map(|v| v.id)
            .collect_vec();

        let neighbors = valves
            .iter()
            .map(|v| {
                v.neighbors
                    .iter()
                    .map(|n| (valves.iter().position(|v| &v.name == n).unwrap(), 1))
                    .collect_vec()
            })
            .collect_vec();

        let mut all_distances = HashMap::<usize, HashMap<usize, i32>>::new();
        all_distances.insert(start, HashMap::<usize, i32>::new());
        distances(
            start,
            |id| &neighbors[id],
            all_distances.get_mut(&start).unwrap(),
        );
        for target in &targets {
            all_distances.insert(*target, HashMap::<usize, i32>::new());
            distances(
                *target,
                |id| &neighbors[id],
                all_distances.get_mut(target).unwrap(),
            );
        }
        Self {
            valves,
            start,
            targets,
            all_distances,
        }
    }
}
