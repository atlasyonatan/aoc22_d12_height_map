use ndarray::Array2;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

pub mod direction;
use direction::{BiDirection, Direction, BACKWARD, FORWARD};

pub type DistanceMap = HashMap<usize, usize>;
pub type Search = BiDirection<(usize, DistanceMap)>;

pub fn meet_in_the_middle<F>(
    adjecancy: &Array2<bool>,
    search: &mut Search,
    until: F,
) -> Option<usize>
where
    F: Fn(&usize, &Direction, &Search) -> bool,
{
    let mut queue = VecDeque::new();
    for (direction, &(node, _)) in search.iter() {
        queue.push_back((node, direction, 0usize));
    }
    // let mut maps = searches.iter().map(|(_, map)| &map);
    while let Some((node, direction, length)) = queue.pop_front() {
        let map = &mut search.get_mut(direction).unwrap().1;
        match map.entry(node) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = (*entry.get()).min(length);
                if *entry.get() == length {
                    continue;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(length);
            }
        }

        //check reached end
        if until(&node, &direction, &search) {
            return Some(node);
        }

        //enque neighbors
        let neighbors = match direction {
            FORWARD => adjecancy.row(node),
            BACKWARD => adjecancy.column(node),
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
    let mut searches = Search {
        forward: Some((from, DistanceMap::new())),
        backward: Some((to, DistanceMap::new())),
    };
    let middle = meet_in_the_middle(adjecancy, &mut searches, |node, direction, maps| {
        maps.get(!direction).unwrap().1.contains_key(node)
    })?;
    Some(
        searches
            .iter()
            .map(|(_, (_, map))| map.get(&middle).unwrap())
            .sum(),
    )
}

pub fn distances(adjecancy: &Array2<bool>, node: usize, direction: Direction) -> DistanceMap {
    let mut searches = BiDirection::new();
    searches[direction] = Some((node, DistanceMap::new()));
    meet_in_the_middle(adjecancy, &mut searches, |_, _, _| false);
    match direction {
        FORWARD => searches.forward,
        BACKWARD => searches.backward,
    }
    .unwrap()
    .1
}
