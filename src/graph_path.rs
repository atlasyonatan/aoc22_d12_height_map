use ndarray::Array2;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

pub type DistanceMap = HashMap<usize, usize>;
pub type DirectionMap = HashMap<Direction, DistanceMap>;

pub fn meet_in_the_middle<F>(
    adjecancy: &Array2<bool>,
    nodes: &Vec<(usize, Direction)>,
    maps: &mut DirectionMap,
    until: F,
) -> Option<usize>
where
    F: Fn(&usize, &Direction, &DirectionMap) -> bool,
{
    let mut queue = VecDeque::new();
    for (node, direction) in nodes {
        queue.push_back((*node, *direction, 0usize))
    }

    while let Some((node, direction, length)) = queue.pop_front() {
        let mapped = maps.get_mut(&direction).unwrap();
        match mapped.entry(node) {
            Entry::Occupied(mut entry) => {
                let current_length = entry.get_mut();
                if length < *current_length {
                    *current_length = length;
                } else {
                    continue;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(length);
            }
        }

        //check reached end
        if until(&node, &direction, &maps) {
            return Some(node);
        }

        //enque neighbors
        let neighbors = match direction {
            Direction::Forward => adjecancy.row(node),
            Direction::Backward => adjecancy.column(node),
        };
        for (neighbor, &adjecant) in neighbors.into_iter().enumerate() {
            if adjecant {
                queue.push_front((neighbor, direction, length + 1))
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn shortest_path(adjecancy: &Array2<bool>, from: usize, to: usize) -> Option<usize> {
    let nodes = vec![(from, Direction::Forward), (to, Direction::Backward)];
    let mut maps = HashMap::new();
    maps.entry(Direction::Forward).or_default();
    maps.entry(Direction::Backward).or_default();
    let middle = meet_in_the_middle(adjecancy, &nodes, &mut maps, |node, direction, maps| {
        maps.get(&direction.reverse()).unwrap().contains_key(node)
    })?;
    Some(maps.values().map(|map| map.get(&middle).unwrap()).sum())
}

pub fn distances(
    adjecancy: &Array2<bool>,
    node: usize,
    direction: Direction,
) -> HashMap<usize, usize> {
    let nodes = vec![(node, direction)];
    let mut maps = HashMap::new();
    maps.entry(direction).or_default();
    meet_in_the_middle(adjecancy, &nodes, &mut maps, |_, _, _| false);
    maps.remove(&direction).unwrap()
}
