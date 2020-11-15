use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// Implementation heavily influenced by https://github.com/prscoelho/aoc2019/blob/master/src/aoc18/mod.rs
// Ashamedly, I was a quite lazy with this, and basically retyped the implementation from the
// above source to get some experience learning how a more seasoned Rust programmer
// would implement graph search.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn neighbors(&self) -> [Coordinate; 4] {
        [
            Coordinate(self.0 - 1, self.1),
            Coordinate(self.0 + 1, self.1),
            Coordinate(self.0, self.1 - 1),
            Coordinate(self.0, self.1 + 1),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Wall,
    Empty,
    Value(char),
}

fn parse_input(input: &str) -> HashMap<Coordinate, Tile> {
    let mut grid = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                _ => Tile::Value(c),
            };
            grid.insert(Coordinate(x as i32, y as i32), tile);
        }
    }
    grid
}

fn reachable_from(grid: &HashMap<Coordinate, Tile>, coord: Coordinate) -> HashMap<char, usize> {
    let mut visited = HashSet::new();
    let mut result = HashMap::new();

    let mut queue = VecDeque::from(vec![(coord, 0)]);
    queue.push_back((coord, 0));

    visited.insert(coord);
    while let Some((current, cost)) = queue.pop_front() {
        for neighbor in &current.neighbors() {
            if let Some(tile) = grid.get(neighbor) {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(*neighbor);
                match tile {
                    Tile::Empty => {
                        queue.push_back((*neighbor, cost + 1));
                    }
                    Tile::Value(c) => {
                        result.insert(*c, cost + 1);
                    }
                    _ => {}
                }
            }
        }
    }

    result
}

fn build_graph(grid: &HashMap<Coordinate, Tile>) -> HashMap<char, HashMap<char, usize>> {
    let mut graph = HashMap::new();
    for (coord, tile) in grid.iter() {
        if let Tile::Value(c) = tile {
            let reachable = reachable_from(grid, *coord);
            graph.insert(*c, reachable);
        }
    }
    graph
}

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    current: char,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search_keys(
    graph: &HashMap<char, HashMap<char, usize>>,
    keys: &BTreeSet<char>,
    start: char,
) -> Vec<(char, usize)> {
    let mut distances: HashMap<char, usize> =
        graph.keys().map(|&key| (key, usize::max_value())).collect();

    let mut heap = BinaryHeap::new();
    *distances.get_mut(&start).unwrap() = 0;
    heap.push(DijkstraState {
        cost: 0,
        current: start,
    });
    let mut reach = HashSet::new();
    while let Some(DijkstraState { cost, current }) = heap.pop() {
        if current.is_lowercase() && !keys.contains(&current) {
            reach.insert(current);
            continue;
        }

        if cost > distances[&current] {
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&current).unwrap().iter() {
            // Permissions check
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }

            let next = DijkstraState {
                cost: cost + next_cost,
                current: next_node,
            };
            if next.cost < distances[&next_node] {
                distances.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }

    reach
        .into_iter()
        .map(|node| (node, distances[&node]))
        .collect()
}

fn search(graph: HashMap<char, HashMap<char, usize>>, start: char) -> usize {
    let mut p_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(key, _)| key.is_lowercase()).count();

    let mut distances: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    distances.insert((start, BTreeSet::new()), 0);

    let initial = State {
        cost: 0,
        node: start,
        keys: BTreeSet::new(),
    };

    p_queue.push(initial);
    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(current) = p_queue.pop() {
        if current.keys.len() == key_count {
            return current.cost;
        }

        let cache_key = (current.node, current.keys.clone());
        if let Some(&best_cost) = distances.get(&cache_key) {
            if current.cost > best_cost {
                continue;
            }
        }

        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| search_keys(&graph, &current.keys, current.node));

        for &(next_node, cost) in cached_entry.iter() {
            let mut next_keys = current.keys.clone();
            next_keys.insert(next_node);
            let next_steps = current.cost + cost;

            let distances_entry = distances
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());

            if next_steps < *distances_entry {
                *distances_entry = next_steps;

                p_queue.push(State {
                    cost: current.cost + cost,
                    node: next_node,
                    keys: next_keys,
                });
            }
        }
    }
    usize::max_value()
}

fn search_four(graph: HashMap<char, HashMap<char, usize>>) -> usize {
    let mut p_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    let mut distances: HashMap<([char; 4], BTreeSet<char>), usize> = HashMap::new();
    let robots = ['@', '=', '%', '$'];

    distances.insert((robots.clone(), BTreeSet::new()), 0);

    let start = FourState {
        cost: 0,
        robots: robots,
        keys: BTreeSet::new(),
    };

    p_queue.push(start);

    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();
    while let Some(current) = p_queue.pop() {
        if current.keys.len() == key_count {
            return current.cost;
        }

        if let Some(&best_cost) = distances.get(&(current.robots, current.keys.clone())) {
            if current.cost > best_cost {
                continue;
            }
        }

        for (robot_number, &robot_location) in current.robots.iter().enumerate() {
            let cache_key = (robot_location, current.keys.clone());

            let cached_entry = cache
                .entry(cache_key)
                .or_insert_with(|| search_keys(&graph, &current.keys, robot_location));

            for &(next_node, cost) in cached_entry.iter() {
                let mut next_keys = current.keys.clone();
                next_keys.insert(next_node);

                let mut next_robots = current.robots.clone();
                next_robots[robot_number] = next_node;

                let next_cost = current.cost + cost;

                let distances_entry = distances
                    .entry((next_robots.clone(), next_keys.clone()))
                    .or_insert(usize::max_value());

                if next_cost < *distances_entry {
                    *distances_entry = next_cost;
                    let next_state = FourState {
                        cost: next_cost,
                        robots: next_robots,
                        keys: next_keys,
                    };

                    p_queue.push(next_state);
                }
            }
        }
    }
    usize::max_value()
}

fn four_robots(grid: &mut HashMap<Coordinate, Tile>) {
    let robot_coord = grid
        .iter()
        .find(|(_, &v)| v == Tile::Value('@'))
        .map(|(k, _)| k.clone())
        .unwrap();

    grid.insert(robot_coord, Tile::Wall);
    for &neighbor in &robot_coord.neighbors() {
        grid.insert(neighbor, Tile::Wall);
    }
    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 - 1),
        Tile::Value('@'),
    );
    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 + 1),
        Tile::Value('='),
    );

    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 + 1),
        Tile::Value('%'),
    );
    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 - 1),
        Tile::Value('$'),
    );
}

#[derive(PartialEq, Eq)]
struct FourState {
    cost: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for FourState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for FourState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut grid = parse_input(input);
    four_robots(&mut grid);
    let graph = build_graph(&grid);
    search_four(graph)
}

fn solve_first(input: &str) -> usize {
    let grid = parse_input(input);
    let graph = build_graph(&grid);
    search(graph, '@')
}

fn main() {
    let input = include_str!("18.txt");
    println!("Part 1: {:?}", solve_first(input));
    println!("Part 2: {:?}", solve_second(input));
}
