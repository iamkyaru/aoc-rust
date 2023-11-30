use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

pub struct Rucksack {
    left: String,
    right: String,
}

impl Rucksack {
    pub fn find_shared_item(&self) -> char {
        let shared_item = self
            .left
            .chars()
            .find(|left_char| {
                self.right
                    .chars()
                    .any(|right_char| left_char == &right_char)
            })
            .unwrap();

        shared_item
    }
}

fn prioritize_item(item: char) -> usize {
    match item {
        'a'..='z' => (item as usize) - ('a' as usize) + 1,
        'A'..='Z' => (item as usize) - ('A' as usize) + 1 + 26,
        _ => 0,
    }
}

#[aoc_generator(day3, part1)]
pub fn generator1(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let mid = line.len() / 2;
            let (left, right) = line.split_at(mid);
            Rucksack {
                left: left.to_string(),
                right: right.to_string(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve1(input: &[Rucksack]) -> usize {
    let mut priorities = 0;

    for rucksack in input {
        let shared_item = rucksack.find_shared_item();
        priorities += prioritize_item(shared_item);
    }

    priorities
}
