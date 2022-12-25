use ndarray::{self, Array2};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

mod graph_path;
use graph_path::{direction::BACKWARD, distances};

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

    println!("Creating adjecancy matrix.");
    let timestamp = Instant::now();
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
    println!("Done {:?}", timestamp.elapsed());

    let start_node = start.1 * shape.0 + start.0;
    let end_node = end.1 * shape.0 + end.0;

    println!("Computing graph distances.");
    let timestamp = Instant::now();
    let distances = distances(&adjecancy, end_node, BACKWARD);
    println!("Done {:?}", timestamp.elapsed());

    println!("Part 1:");
    match distances.get(&start_node) {
        Some(distance) => {
            println!("Found a shortest path of length {}", distance);
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

    println!();
    println!("Part 2:");
    let mut starting_nodes: Vec<(usize, &usize)> = grid
        .indexed_iter()
        .filter_map(|(position, &height)| {
            if height == 'a' {
                Some(position.1 * shape.0 + position.0)
            } else {
                None
            }
        })
        .filter_map(|node| match distances.get(&node) {
            Some(distance) => Some((node, distance)),
            None => None,
        })
        .collect();
    starting_nodes.sort_by_key(|(_, &distance)| distance);
    match starting_nodes.first() {
        Some((node, distance)) => {
            let (x, y) = (node / shape.0, node % shape.0);
            println!("Found a nearest starting node: (x: {}, y: {}). Distance from it to end node is {}.",x,y,distance);
        }
        None => println!("No starting points in reach"),
    }
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
