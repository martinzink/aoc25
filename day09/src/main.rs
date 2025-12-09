use geo::{Contains, Coord, Intersects};

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.lines().map(|line| {
        let (x_str, y_str) = line.split_once(',').unwrap();
        (x_str.parse().unwrap(), y_str.parse().unwrap())
    }).collect()
}

fn part_one(input: &str) -> i64 {
    let coords = parse_input(input);
    
    let mut max = 0;
    for coord in coords.iter() {
        for other_coord in coords.iter() {
            let size = (coord.0-other_coord.0 + 1).abs() * (coord.1-other_coord.1 + 1).abs();
            max = max.max(size);
        }
    }
    max
}

fn parse_input_geo(input: &str) -> Vec<geo::Coord> {
    input.lines().map(|line| {
        let (x_str, y_str) = line.split_once(',').unwrap();
        geo::Coord{x:x_str.parse().unwrap(), y:y_str.parse().unwrap()}
    }).collect()
}


fn part_two(input: &str) -> i64 {
    let coords = parse_input_geo(input);
    let outline = geo::LineString::new(coords.clone());
    let polygon = geo::Polygon::new(outline, vec![]);
    let mut max = 0;
    const EPSILON: f64 = 0.0001;
    let mut counter = 0f64;
    for coord in coords.iter() {
        println!("{}%", ((100f64 * counter) / coords.len() as f64).round());
        counter += 1f64;
        for other_coord in coords.iter() {
            let top_left = geo::Coord{x: coord.x.min(other_coord.x), y: coord.y.min(other_coord.y)} + geo::Coord{x: EPSILON, y: EPSILON};
            let bottom_right = geo::Coord{x: coord.x.max(other_coord.x), y: coord.y.max(other_coord.y)} + geo::Coord{x: -EPSILON, y: -EPSILON};
            let rect = geo::Rect::new(top_left, bottom_right);
            if polygon.contains(&rect) {
                let size = ((coord.x-other_coord.x).abs() + 1f64) * ((coord.y-other_coord.y).abs() + 1f64);
                max = max.max(size.round() as i64);
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 50);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 24);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
