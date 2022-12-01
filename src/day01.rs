const INPUT: &str = include_str!("../inputs/day_01.txt");

fn parse() -> Vec<u64> {
    INPUT
        .split("\n\n")
        .map(|c| c.lines().filter_map(|x| x.parse::<u64>().ok()).sum())
        .collect()
}

fn part1() -> u64 {
    *parse().iter().max().unwrap()
}

fn part2() -> u64 {
    let mut calories = parse();
    calories.sort_by_key(|&c| std::cmp::Reverse(c));
    calories.iter().take(3).sum()
}

pub fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
