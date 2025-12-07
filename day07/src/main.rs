use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct TachyonManifold {
    start_coord: usize,
    splitter_coords: Vec<Vec<usize>>
}

impl TachyonManifold {
    fn new(input: &str) -> Self {
        let mut start_coord = 0;
        let mut splitter_coords = vec![];
        for (_line_num, line) in input.lines().enumerate() {
            let mut line_splitters = vec![];
            for (i, char) in line.chars().enumerate() {
                match char {
                    'S' => { start_coord = i }
                    '.' => {}
                    '^' => { line_splitters.push(i) }
                    _ => panic!("Invalid character: {}", char)
                }
            }
            if !line_splitters.is_empty() {
                splitter_coords.push(line_splitters);
            }
        }
        TachyonManifold{start_coord, splitter_coords}
    }

    fn calculate_splits(&self) -> u64 {
        let mut splits = 0;
        let mut curr_rays: HashSet<usize> = HashSet::new();
        curr_rays.insert(self.start_coord);
        for splitter_coords in &self.splitter_coords {
            for coord in splitter_coords {
                if curr_rays.contains(coord) {
                    splits += 1;
                    curr_rays.remove(coord);
                    curr_rays.insert(*coord + 1);
                    curr_rays.insert(*coord - 1);
                }
            }
        }
        splits
    }

    fn calculate_quantum_splits(&self) -> u64 {
        let mut curr_rays: HashMap<usize, usize> = HashMap::new();
        curr_rays.insert(self.start_coord, 1);
        for splitter_coords in &self.splitter_coords {
            let mut next_rays: HashMap<usize, usize> = HashMap::new();
            for current_coord in curr_rays.keys() {
                let number_of_timelines = *curr_rays.get(&current_coord).unwrap();
                if splitter_coords.contains(current_coord) {
                    *next_rays.entry(*current_coord-1).or_default() += number_of_timelines;
                    *next_rays.entry(*current_coord+1).or_default() += number_of_timelines;
                } else {
                    *next_rays.entry(*current_coord).or_default() += number_of_timelines;
                }
            }
            curr_rays = next_rays;
        }
        curr_rays.values().sum::<usize>() as u64
    }
}

fn part_one(input: &str) -> u64 {
    let tachyon_manifold = TachyonManifold::new(input);
    tachyon_manifold.calculate_splits()
}

fn part_two(input: &str) -> u64 {
    let tachyon_manifold = TachyonManifold::new(input);
    tachyon_manifold.calculate_quantum_splits()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 21);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 40);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
