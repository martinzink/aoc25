use std::cmp::Ordering;

fn parse_banks(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
        })
        .collect()
}

fn get_max_digit(bank: &[u8], index_from: usize, index_up_to: usize) -> (usize, u8) {
    bank[index_from..index_up_to].iter().enumerate().max_by(|(ai, ad), (bi, bd)| {
        match ad.cmp(bd) {
            Ordering::Less => { Ordering::Less}
            Ordering::Equal => { bi.cmp(ai) }
            Ordering::Greater => { Ordering::Greater}
        }
    }).map(|(index, val)| (index + index_from, *val)).unwrap()
}

fn get_max_joltage(bank: &[u8], number_of_batteries: usize) -> u64 {
    let mut digits = Vec::new();
    let mut next_digit_start_index = 0;
    for battery_i in 0..number_of_batteries {
        let (digit_loc, digit_val) = get_max_digit(bank, next_digit_start_index, bank.len() - (number_of_batteries - battery_i - 1));
        next_digit_start_index = digit_loc + 1;
        digits.push(digit_val);
    }
    digits.iter().fold(0u64, |acc, digit| acc*10 + *digit as u64)
}

fn part_one(input: &str) -> u64 {
    parse_banks(input).iter().fold(0u64, |acc, bank| acc + get_max_joltage(bank, 2))
}

fn part_two(input: &str) -> u64 {
    parse_banks(input).iter().fold(0u64, |acc, bank| acc + get_max_joltage(bank, 12))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 357);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 3121910778619);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
