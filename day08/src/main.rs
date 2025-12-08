use std::collections::{HashMap, HashSet};
use petgraph::graph::UnGraph;
use itertools::Itertools;
use petgraph::algo::connected_components;
use petgraph::visit::Walker;

fn parse_input(input: &str) -> (Vec<[f64; 3]>, UnGraph::<usize, f64>, HashMap<usize, petgraph::graph::NodeIndex>) {
    let coords: Vec<[f64;3]> = input.lines().map(|line| {
        let (x_str, yz_str) = line.split_once(',').unwrap();
        let (y_str, z_str) = yz_str.split_once(',').unwrap();
        [x_str.parse().unwrap(), y_str.parse().unwrap(), z_str.parse().unwrap()]
    }).collect();
    let mut g = UnGraph::<usize, f64>::new_undirected();
    let mut node_ids = HashMap::new();
    for (id, _coord) in coords.iter().enumerate() {
        node_ids.insert(id, g.add_node(id));
    }
    (coords, g, node_ids)
}

fn calc_connections(coords: &[[f64; 3]]) -> Vec<(f64, usize, usize)> {
    let mut connections = Vec::new();

    for coord_pair in coords.iter().enumerate().combinations(2) {
        let (id1, c1) = coord_pair[0];
        let (id2, c2) = coord_pair[1];
        let dist_sq =  (c2[0] - c1[0]).powi(2) + (c2[1] - c1[1]).powi(2) + (c2[2] - c1[2]).powi(2);
        connections.push((dist_sq, id1, id2));
    }
    connections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    connections
}

fn part_one(input: &str, connection_size: usize) -> usize {
    let (coords, mut g, node_ids) = parse_input(input);

    for connection in calc_connections(&coords).iter().take(connection_size) {
        g.add_edge(node_ids[&connection.1], node_ids[&connection.2], connection.0);
    }

    let mut visited = HashSet::new();
    let mut component_sizes = Vec::new();
    for coord in node_ids {
        if visited.contains(&coord.1) {
            continue;
        }
        let bfs = petgraph::visit::Bfs::new(&g, coord.1);
        let mut size = 0;
        for nx in bfs.iter(&g) {
            visited.insert(nx);
            size += 1;
        }
        component_sizes.push(size);
    }
    component_sizes.sort_unstable();
    component_sizes.reverse();
    component_sizes.iter().take(3).product()
}

fn part_two(input: &str) -> u64 {
    let (coords, mut g, node_ids) = parse_input(input);

    let mut connections = calc_connections(&coords);
    connections.reverse();

    let mut res = 0u64;
    while connected_components(&g) > 1 {
        let c = connections.pop().unwrap();
        g.add_edge(node_ids[&c.1], node_ids[&c.2], c.0);
        res = coords[c.1][0] as u64 * coords[c.2][0] as u64
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE, 10), 40);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 25272);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT, 1000));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
