fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input.split(',')
        .map(|range| {
            let (low_str, high_str) = range.split_once('-').unwrap();
            (low_str.parse::<u64>().unwrap(), high_str.parse::<u64>().unwrap())
        })
        .collect()
}

fn part_two(input: &str) -> u64 {
    let ranges = parse_ranges(input);
    let mut result = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            let i_str = i.to_string();
            let len = i_str.len();
            for chunk_len in 1..=len/2 {
                let chunk = &i_str[..chunk_len];
                if chunk.repeat(len/chunk_len) == i_str {
                    result += i;
                    break;
                }
            }
        }
    }
    result
}

fn part_one(input: &str) -> u64 {
    let ranges = parse_ranges(input);
    let mut result = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            let i_str = i.to_string();
            let len = i_str.len();
            if len % 2 == 1 {
                continue;
            }
            let first = &i_str[len/2..];
            let second = &i_str[..len/2];
            if first == second {
                result += i;
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
        assert_eq!(part_one(EXAMPLE), 1227775554);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 4174379265);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
