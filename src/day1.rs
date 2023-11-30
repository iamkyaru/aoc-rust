use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            result.push(current);
            current = Vec::new();
            continue;
        }

        let calories: usize = line.parse().unwrap();
        current.push(calories);
    }

    result
}

#[aoc(day1, part1)]
pub fn solver1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn solver2(input: &Vec<Vec<usize>>) -> usize {
    let mut sums = input
        .iter()
        .map(|calories| calories.iter().sum())
        .collect::<Vec<usize>>();
    sums.sort();

    sums.iter().rev().take(3).sum()
}
