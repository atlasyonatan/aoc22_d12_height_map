use ndarray::{self, Array2};
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(path).unwrap();
    let mut grid = array2_from_vec2(
        io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.chars().collect())
            .collect(),
    );
    let shape = grid.dim();

    let (start, end) = {
        let mut start = Vec::new();
        let mut end = Vec::new();
        for (position, character) in grid.indexed_iter_mut() {
            match character {
                'S' => {
                    *character = 'a';
                    start.push(position);
                }
                'E' => {
                    *character = 'z';
                    end.push(position);
                }
                _ => (),
            }
        }
        if start.len() != 1 {
            panic!("Invalid input: {} start positions", start.len())
        }
        if end.len() != 1 {
            panic!("Invalid input: {} end positions", start.len())
        }
        (start[0], end[0])
    };

    let steps: HashSet<(i32, i32)> = HashSet::from_iter([(1, 0), (-1, 0), (0, 1), (0, -1)]);

    fn can_traverse(from: char, to: char) -> bool {
        let difference = to as i8 - from as i8;
        difference <= 1
    }

    let count = shape.0 * shape.1;
    let adjecancy = Array2::from_shape_fn((count, count), |(a, b)| {
        let (x1, y1) = (a % shape.0, a / shape.0);
        let (x2, y2) = (b % shape.0, b / shape.0);
        let step = (x1 as i32 - x2 as i32, y1 as i32 - y2 as i32);
        if steps.contains(&step) && can_traverse(grid[[x1, y1]], grid[[x2, y2]]) {
            true
        } else {
            false
        }
    });

    let start_node = start.1 * shape.0 + start.0;
    let end_node = end.1 * shape.0 + end.0;


    match meet_in_the_middle(adjecancy, start_node, end_node) {
        Some((middle, lengths)) => {
            let total_length = lengths.iter().sum::<usize>();
            println!("Found a shortest path of length {}", total_length);
            let (x, y) = (middle % shape.0, middle / shape.0);
            println!("Path mid point (x: {}, y: {})", x, y);
        }
        None => println!("No path"),
    }

    // let mut adj_pow = Array2::from_diag_elem(count, 1);
    // let mut len = 1;
    // while len <= count {
    //     adj_pow = adj_pow.dot(&adjecancy);
    //     if adj_pow[[start_p, end_p]] > 1 {
    //         break;
    //     }
    //     len += 1;
    // }
    // if len > count {
    //     println!("Found no paths between start and end");
    // } else {
    //     let paths = adj_pow[[start_p, end_p]];
    //     let plural = if paths == 1 { "s" } else { "" };
    //     println!(
    //         "Found {} shortest path{} from start to end with length={} steps",
    //         paths, plural, len
    //     );
    // }
}

fn array2_from_vec2<T>(mut input_vec2: Vec<Vec<T>>) -> Array2<T> {
    let mut data = Vec::new();

    let ncols = input_vec2.first().map_or(0, |row| row.len());
    let mut nrows = 0;

    for i in 0..input_vec2.len() {
        data.append(&mut input_vec2[i]);
        nrows += 1;
    }

    Array2::from_shape_vec((nrows, ncols), data).unwrap()
}

fn meet_in_the_middle(
    adjecancy: Array2<bool>,
    from_node: usize,
    to_node: usize,
) -> Option<(usize, [usize; 2])> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((from_node, 0usize, true));
    queue.push_back((to_node, 0usize, false));

    while let Some((node, length, forward)) = queue.pop_front() {
        //visit node
        match visited.entry((node, forward)) {
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
        if let Entry::Occupied(complement) = visited.entry((node, !forward)) {
            let mut lengths = [length, *complement.get()];
            if !forward {
                lengths.reverse();
            }
            return Some((node, lengths));
        }

        //enque neighbors
        let neighbors = if forward {
            adjecancy.row(node)
        } else {
            adjecancy.column(node)
        };
        for (neighbor, &adjecant) in neighbors.into_iter().enumerate() {
            if adjecant {
                queue.push_front((neighbor, length + 1, forward))
            }
        }
    }
    None
}
