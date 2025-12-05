#[derive(Debug)]
struct FreshIdRange {
    start: u64,
    end: u64,
}

impl FreshIdRange {
    fn from_str(input: &str) -> Vec<Self> {
        input.lines().map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            Self {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        }).collect()
    }

    fn contains(&self, id: u64) -> bool {
        self.start <= id && id <= self.end
    }
}

fn parse(input: &str) -> (Vec<FreshIdRange>, Vec<u64>) {
    let (fresh_id_ranges_str, ids_str)= input.split_once("\n\n").unwrap();
    (FreshIdRange::from_str(fresh_id_ranges_str), ids_str.lines().map(|line| line.parse().unwrap()).collect())
}

fn part_one(input: &str) -> u32 {
    let (fresh_ranges, ids) = parse(input);
    let mut result = 0u32;
    for id in ids.iter() {
        if fresh_ranges.iter().any(|range| range.contains(*id)) {
            result += 1;
            continue;
        }
    }
    result
}

fn part_two(input: &str) -> u64 {
    let (mut fresh_ranges, _ids) = parse(input);
    fresh_ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    struct Acc {
        count: u64,
        last_end: u64
    }
    fresh_ranges.iter().fold(Acc{ count:0, last_end:0}, |acc: Acc, range| {
        let start = range.start.max(acc.last_end);
        let end = range.end + 1;
        if start < end {
            Acc{ count: acc.count +(end-start), last_end: end}
        } else {
            acc
        }
    }).count
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
        assert_eq!(part_two(EXAMPLE), 14);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
