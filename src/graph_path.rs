use std::collections::{hash_map::Entry, HashMap, VecDeque};

pub mod direction;
use direction::{Direction, BACKWARD, FORWARD};

pub type DistanceMap = HashMap<usize, usize>;
pub type VisitQueue = VecDeque<(usize, Direction, usize)>;

pub fn meet_in_the_middle<Neighbors, Predicate>(
    neighbors: Neighbors,
    searchs: &mut [Search],
    stop: Predicate,
) -> Option<usize>
where
    Neighbors: Fn(usize, Direction) -> Vec<usize>,
    Predicate: Fn(&usize, &Direction, &[DistanceMap]) -> bool,
{
    let mut empty = 0;
    while empty != searchs.len() {
        empty = 0;
        for search in searchs {
            if search.queue.len() == 0 {
                empty += 1;
                continue;
            }
            let (node, direction, length) = search.queue.pop_front().unwrap();

            match search.map.entry(node) {
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
            let a: Vec<DistanceMap> = searchs.iter().map(|search| search.map).collect();
            if stop(&node, &direction, &a[..]) {
                return Some(node);
            }

            //enque neighbors
            for neighbor in neighbors(node, direction) {
                search.queue.push_front((neighbor, direction, length + 1))
            }
        }
    }
    None
    // loop {
    //     for (i, (queue, map)) in searchs.iter_mut().enumerate() {
    //         empty[i] = false;
    //         match queue.pop_front() {
    //             Some((node, direction, length)) => todo!(),
    //             None => empty[i] = true,
    //         }
    //     }
    // }
    // while let Some((node, direction, length)) = queue.pop_front() {
    //     let map = &mut searchs.get_mut(direction).unwrap().1;
    //     match map.entry(node) {
    //         Entry::Occupied(mut entry) => {
    //             *entry.get_mut() = (*entry.get()).min(length);
    //             if *entry.get() == length {
    //                 continue;
    //             }
    //         }
    //         Entry::Vacant(entry) => {
    //             entry.insert(length);
    //         }
    //     }

    //     //check reached end
    //     if until(&node, &direction, &searchs) {
    //         return Some(node);
    //     }

    //     //enque neighbors
    //     let neighbors = match direction {
    //         FORWARD => neighbors.row(node),
    //         BACKWARD => neighbors.column(node),
    //     };
    //     for (neighbor, &adjecant) in neighbors.into_iter().enumerate() {
    //         if adjecant {
    //             queue.push_front((neighbor, direction, length + 1))
    //         }
    //     }
    // }
    // None
}

#[derive(Default, Debug)]
pub struct Search {
    queue: VisitQueue,
    map: DistanceMap,
}

#[allow(dead_code)]
pub fn shortest_path<Neighbors>(neighbors: Neighbors, from: usize, to: usize) -> Option<usize>
where
    Neighbors: Fn(usize, Direction) -> Vec<usize>,
{
    let mut searches: [Search; 2];
    searches[0].queue.push_back((from, FORWARD, 0usize));
    searches[1].queue.push_back((to, BACKWARD, 0usize));

    let middle = meet_in_the_middle(neighbors, &mut searches[..], |node, direction, maps| {
        maps.iter().all(|map| map.contains_key(node))
    })?;
    Some(
        searches
            .iter()
            .map(|search| search.map.get(&middle).unwrap())
            .sum(),
    )
}

// pub fn distances(neighbors: Neighbors, searchs: &mut [Search])
// where
//     Neighbors: Fn(usize, Direction) -> Vec<usize>,
// {
//     let mut searches = BiDirection::new();
//     searches[direction] = Some((node, DistanceMap::new()));

//     let mut searches: [Search; 1];
//     searches[0].queue.push_back((node, direction, 0usize));
//     // searches[1].queue.push_back((to, BACKWARD, 0usize));

//     meet_in_the_middle(adjecancy, &mut searches, |_, _, _| false);
//     match direction {
//         FORWARD => searches.forward,
//         BACKWARD => searches.backward,
//     }
//     .unwrap()
//     .1
// }
