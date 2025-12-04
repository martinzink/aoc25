
fn part_two(input: &str) -> i32 {
    let mut paper_map = utils::matrix::parse_matrix(input);
    let mut full_sum = 0;
    let mut started = true;
    let mut removed: Vec<(i32, i32)> = Vec::new();
    while !removed.is_empty() || started {
        started = false;
        for (i, j) in removed.iter() {
            paper_map[*i as usize][*j as usize] = '.';
        }
        removed.clear();
        for i in 0..paper_map.len() {
            for j in 0..paper_map[i].len() {
                if paper_map[i][j] == '@' && count_neighbours(&paper_map, i as i32, j as i32) < 4 {
                    full_sum += 1;
                    removed.push((i as i32, j as i32));
                }
            }
        }
    }


    full_sum
}

fn count_neighbours(matrix: &[Vec<char>], i: i32, j: i32) -> usize {
    let mut neighbours = 0;
    for i_adj in i-1..=i+1 {
        for j_adj in j-1..=j+1 {
            if i_adj < 0 || j_adj < 0 { continue; }
            if i_adj == i && j_adj == j { continue; }
            match matrix.get(i_adj as usize).and_then(|row| row.get(j_adj as usize)) {
                None => {}
                Some('@') => neighbours += 1,
                Some(_) => {}
            }
        }
    }
    neighbours
}

fn part_one(input: &str) -> i32 {
    let paper_map = utils::matrix::parse_matrix(input);
    let mut full_sum = 0;
    for i in 0..paper_map.len() {
        for j in 0..paper_map[i].len() {
            if paper_map[i][j] == '@' && count_neighbours(&paper_map, i as i32, j as i32) < 4 {
                full_sum += 1;
            }
        }
    }

    full_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 13);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 43);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
