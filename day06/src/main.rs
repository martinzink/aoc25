#[derive(Debug)]
struct Equation {
    pub operands: Vec<u64>,
    pub operator: char
}

fn parse_one(input: &str) -> Vec<Equation> {
    let mut equations : Vec<Equation> = Vec::new();

    for line in input.lines() {
        let splits = line.split_ascii_whitespace().collect::<Vec<&str>>();
        for (i, split) in splits.iter().enumerate() {
            if equations.len() <= i { equations.push(Equation{operands: Vec::new(), operator: ' '}); }
            let equation: &mut Equation = equations.get_mut(i).unwrap();
            match split {
                &"+" => { equation.operator = '+' }
                &"*" => { equation.operator = '*' }
                a => { equation.operands.push(a.parse::<u64>().unwrap()); }
            }
        }
    }

    equations
}

fn part_one(input: &str) -> u64 {
    let equations = parse_one(input);
    let mut result: u64 = 0;
    for equation in equations {
        match {equation.operator} {
            '+' => { result += equation.operands.iter().fold(0, |acc, x| acc + x) }
            '*' => { result += equation.operands.iter().fold(1, |acc, x| acc * x); }
            _ => {panic!("Invalid operator");}
        }
    }
    result
}

fn part_two(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let line_length = lines[0].len();
    let mut result = 0;
    let mut operands: Vec<u64> = vec![];
    for i in (0..line_length).rev() {
        let mut operand: u64 = 0;
        for line in &lines {
            match line[i] {
                ' ' => {}
                '+' => {
                    operands.push(operand);
                    result += operands.iter().fold(0, |acc, x| acc + x);
                    operands.clear();
                    operand = 0;
                }
                '*' => {
                    operands.push(operand);
                    result += operands.iter().fold(1, |acc, x| acc * x);
                    operands.clear();
                    operand = 0;
                }
                a => {
                    let digit = a.to_digit(10).unwrap();
                    operand *= 10;
                    operand += digit as u64;
                }
            }
        }
        if operand != 0 { operands.push(operand); }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 4277556);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 3263827);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
