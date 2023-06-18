#[allow(unused)]
pub fn short_path<'a, N: Fn(usize) -> &'a Vec<(usize, i32)>>(
    start: usize,
    end: usize,
    neighbors: N,
) -> Option<Vec<usize>> {
    short_path_a(start, |id| id == end, neighbors, |_| 0)
}

#[allow(unused)]
pub fn short_path_a<
    'a,
    E: Fn(usize) -> bool,
    N: Fn(usize) -> &'a Vec<(usize, i32)>,
    C: Fn(usize) -> i32,
>(
    start: usize,
    is_end: E,
    neighbors: N,
    to_end_minimal_cost: C,
) -> Option<Vec<usize>> {
    struct Node {
        predecessor: Option<usize>,
        from_start_cost: i32,
        to_end_minimal_cost: i32,
    }

    let mut nodes = std::collections::HashMap::<usize, Node>::new();
    let mut todo = Vec::<usize>::new();
    nodes.insert(
        start,
        Node {
            predecessor: None,
            from_start_cost: 0,
            to_end_minimal_cost: to_end_minimal_cost(start),
        },
    );
    todo.push(start);

    loop {
        let current = match todo.pop() {
            Some(id) => id,
            None => return None,
        };

        if is_end(current) {
            let mut path = itertools::unfold(Some(current), |current| {
                let ret = *current;
                if let Some(id) = current {
                    *current = nodes.get(id).unwrap().predecessor;
                }
                ret
            })
            .collect::<Vec<_>>();
            path.reverse();
            return Some(path);
        }

        let current_node = nodes.get(&current).unwrap();
        let current_from_start_cost = current_node.from_start_cost;

        for (next, cost) in neighbors(current).iter() {
            let from_start_cost = current_from_start_cost + cost;
            let to_end_minimal_cost = match nodes.get(next) {
                None => Some((from_start_cost + to_end_minimal_cost(start))),
                Some(next_node) => {
                    if from_start_cost < next_node.from_start_cost {
                        let to_end_minimal_cost = from_start_cost + next_node.to_end_minimal_cost
                            - next_node.from_start_cost;
                        if let Ok(index) = todo.binary_search_by(|id| {
                            next_node
                                .to_end_minimal_cost
                                .cmp(&nodes[id].to_end_minimal_cost)
                                .then(next.cmp(id))
                        }) {
                            todo.remove(index);
                        }
                        Some(to_end_minimal_cost)
                    } else {
                        None
                    }
                }
            };
            if let Some(to_end_minimal_cost) = to_end_minimal_cost {
                nodes.insert(
                    *next,
                    Node {
                        predecessor: Some(current),
                        from_start_cost,
                        to_end_minimal_cost,
                    },
                );
                let index = match todo.binary_search_by(|id| {
                    to_end_minimal_cost
                        .cmp(&nodes[id].to_end_minimal_cost)
                        .then(next.cmp(id))
                }) {
                    Ok(index) => index,
                    Err(index) => index,
                };
                todo.insert(index, *next)
            }
        }
    }
}

#[allow(unused)]
pub fn distances<'a, N: Fn(usize) -> &'a Vec<(usize, i32)>>(
    start: usize,
    neighbors: N,
    distances: &mut std::collections::HashMap<usize, i32>,
) {
    let mut todo = Vec::<usize>::new();
    distances.insert(start, 0);
    todo.push(start);
    loop {
        let current = match todo.pop() {
            Some(id) => id,
            None => return,
        };

        let current_distance = distances[&current];
        for (next, cost) in neighbors(current).iter() {
            let next_distance = current_distance + cost;
            if match distances.get(next) {
                None => true,
                Some(&distance) => {
                    if next_distance < distance {
                        if let Ok(index) = todo
                            .binary_search_by(|id| distance.cmp(&distances[id]).then(next.cmp(id)))
                        {
                            todo.remove(index);
                        }
                        true
                    } else {
                        false
                    }
                }
            } {
                distances.insert(*next, next_distance);
                let index = match todo
                    .binary_search_by(|id| next_distance.cmp(&distances[id]).then(next.cmp(id)))
                {
                    Ok(index) => index,
                    Err(index) => index,
                };
                todo.insert(index, *next)
            }
        }
    }
}
