use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

struct Rucksack {
    left: String,
    right: String,
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() - 1);
            Rucksack {
                left: left.to_string(),
                right: right.to_string(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve1(input: &[Rucksack]) -> usize {
    for Rucksack { left, right } in input {
        left.
    }
}
