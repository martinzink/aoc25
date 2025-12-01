fn parse_input(input: &str) -> Vec<(i32, u32)> {
    input
        .lines()
        .map(|line| {
            let amount = line[1..].parse::<u32>().unwrap();
            let direction = match line.as_bytes()[0] as char {
                'L' => -1i32,
                'R' => 1i32,
                _ => panic!("invalid input"),
            };
            (direction, amount)
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let inputs = parse_input(input);
    let mut pos = 50i32;
    let mut result = 0u32;
    for input in inputs {
        pos += input.0 * input.1 as i32;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            result += 1;
        }
    }
    result
}

fn part_two(input: &str) -> u32 {
    let inputs = parse_input(input);
    let mut pos = 50i32;
    let mut result = 0u32;
    for input in inputs {
        for _ in 0..input.1 {
            pos += input.0;
            match pos {
                0 => result += 1,
                -1 => {
                    pos = 99;
                }
                100 => {
                    pos = 0;
                    result += 1;
                }
                _ => {}
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 3);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 6);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
